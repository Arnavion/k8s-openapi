#!/bin/bash

# Usage:
#
#     test.sh create-node-image <version> <directory>
#
# Create a node image for testing <version> and save it under <directory>.
#
# Note: Use:
#
# ```sh
# docker ps -a | grep kube-build-data | awk '{ print $1 }' | xargs docker rm -f
# docker images -a | egrep '<none>|k8s|rancher|kindest|kube' | awk '{ print $3 }' | xargs -n1 docker image rm -f
# ```
#
# ... to clean up leftover gunk.
#
#
#     test.sh create-cluster <version> <directory>
#
# Create a cluster for testing <version> using the image stored under <directory>.
#
#
#     test.sh delete-cluster <version>
#
# Delete the cluster created with create-cluster.
#
#
#     test.sh run-tests <version>
#
# Run the tests for <version>. Set K8S_RECORD=1 if you want to run the tests in record mode.
#
#
#     test.sh all create-node-image <directory>
#     test.sh all create-cluster <directory>
#     test.sh all delete-cluster
#     test.sh all run-tests
#
# Run the corresponding command against all supported versions in parallel.

set -euo pipefail

declare -A K8S_VERSIONS=(
	['1.11']='1.11.10'
	['1.12']='1.12.10'
	['1.13']='1.13.12'
	['1.14']='1.14.10'
	['1.15']='1.15.12'
	['1.16']='1.16.15'
	['1.17']='1.17.12'
	['1.18']='1.18.9'
	['1.19']='1.19.3'
)

# https://github.com/kubernetes-sigs/kind/releases
declare -A KIND_VERSIONS=(
	['1.11']='0.7.0'
	['1.12']='0.8.1'
	['1.13']='0.9.0'
	['1.14']='0.9.0'
	['1.15']='0.9.0'
	['1.16']='0.9.0'
	['1.17']='0.9.0'
	['1.18']='0.9.0'
	['1.19']='0.9.0'
)

case "$1" in
	'all')
		for v in "${!K8S_VERSIONS[@]}"; do
			("$0" "$2" "$v" "${@:3}" 2>&1 | sed -e "s/^/[v$v] /") &
		done

		wait $(jobs -pr)

		exit 0
		;;

	*)
		;;
esac


K8S_VERSION="${K8S_VERSIONS["$2"]}"
KIND_VERSION="${KIND_VERSIONS["$2"]}"
K8S_CLUSTER_NAME="v$2"


# Download the appropriate version of kind
mkdir -p ~/bin
flock -x ~/bin -c "
hash -r
if ! command -v 'kind-$KIND_VERSION' >/dev/null; then
	curl -Lo ~/bin/kind-$KIND_VERSION 'https://github.com/kubernetes-sigs/kind/releases/download/v$KIND_VERSION/kind-linux-amd64'
	chmod +x ~/bin/kind-$KIND_VERSION
fi
"
hash -r
if ! command -v "kind-$KIND_VERSION" >/dev/null; then
	PATH="$PATH:$HOME/bin"
fi


case "$1" in
	'create-node-image')
		if [ -f "$3/kindest-node-v$K8S_VERSION.tar.gz" ]; then
			exit 0
		fi

		if ! docker image inspect "kindest/node:v$K8S_VERSION"; then
			docker pull "kindest/node:v$K8S_VERSION" ||
			(
				trap "rm -rf '/tmp/kubernetes-v$K8S_VERSION'" EXIT

				rm -rf "/tmp/kubernetes-v$K8S_VERSION" &&
				git clone --recurse-submodules "--branch=v$K8S_VERSION" --depth=1 https://github.com/kubernetes/kubernetes "/tmp/kubernetes-v$K8S_VERSION" &&
				"kind-$KIND_VERSION" build node-image --image "kindest/node:v$K8S_VERSION" --kube-root "/tmp/kubernetes-v$K8S_VERSION"
			)
		fi

		mkdir -p "$3"
		docker image save --output "$3/kindest-node-v$K8S_VERSION.tar.gz" "kindest/node:v$K8S_VERSION"
		docker image rm -f "kindest/node:v$K8S_VERSION"
		;;

	'create-cluster')
		if ! ("kind-$KIND_VERSION" get clusters | grep -q "$K8S_CLUSTER_NAME"); then
			docker image load --input "$3/kindest-node-v$K8S_VERSION.tar.gz"

			# Run against a temporary kubeconfig instead of ~/.kube/config, because kind tries to lock the kubeconfig to prevent concurrent modification.
			# But if it does fail to lock the file, it just fails instead of retrying.
			#
			# So we run it against a throwaway kubeconfig, then export the kubeconfig and merge it into ~/.kube/config afterwards with our own locking.

			trap "rm -f '/tmp/kubeconfig-v$K8S_VERSION'" EXIT

			rm -f "/tmp/kubeconfig-v$K8S_VERSION"
			"kind-$KIND_VERSION" create cluster --name "$K8S_CLUSTER_NAME" --image "kindest/node:v$K8S_VERSION" --kubeconfig "/tmp/kubeconfig-v$K8S_VERSION"
			rm -f "/tmp/kubeconfig-v$K8S_VERSION"
		fi

		flock -x ~/.kube -c "'kind-$KIND_VERSION' export kubeconfig --name '$K8S_CLUSTER_NAME'"
		;;

	'delete-cluster')
		if ("kind-$KIND_VERSION" get clusters | grep -q "$K8S_CLUSTER_NAME"); then
			# Run against a temporary kubeconfig instead of ~/.kube/config, because kind tries to lock the kubeconfig to prevent concurrent modification.
			# But if it does fail to lock the file, it just fails instead of retrying.
			#
			# So we run it against a throwaway kubeconfig, then delete the cluster from the real ~/.kube/config afterwards with our own locking.

			trap "rm -f '/tmp/kubeconfig-v$K8S_VERSION'" EXIT

			cp ~/.kube/config "/tmp/kubeconfig-v$K8S_VERSION"
			"kind-$KIND_VERSION" delete cluster --name "$K8S_CLUSTER_NAME" --kubeconfig "/tmp/kubeconfig-v$K8S_VERSION"
			rm -f "/tmp/kubeconfig-v$K8S_VERSION"
		fi

		docker image rm -f "kindest/node:v$K8S_VERSION"

		flock -x ~/.kube -c "kubectl config delete-context 'kind-$K8S_CLUSTER_NAME'" || :
		flock -x ~/.kube -c "kubectl config delete-cluster 'kind-$K8S_CLUSTER_NAME'" || :
		flock -x ~/.kube -c "kubectl config unset 'users.kind-$K8S_CLUSTER_NAME'" || :
		;;

	'run-tests')
		CARGO_TARGET_DIR="$PWD/target-v$K8S_VERSION" K8S_CONTEXT="kind-$K8S_CLUSTER_NAME" cargo test --features "test_v${2//./_}" "${@:3}"
		;;

	*)
		exit 1
esac
