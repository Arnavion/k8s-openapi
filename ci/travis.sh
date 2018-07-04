set -euo pipefail

: "${K8S_VERSION:=}"

(
	cd k8s-openapi-tests
	cargo test --verbose --no-run --features "$FEATURE"
) &

if [[ -n "$K8S_VERSION" ]]; then
	(
		curl -Lo ~/minikube 'https://storage.googleapis.com/minikube/releases/v0.25.2/minikube-linux-amd64'
		chmod +x ~/minikube

		sudo CHANGE_MINIKUBE_NONE_USER=true ~/minikube start --vm-driver=none "--kubernetes-version=$K8S_VERSION"
	) &
fi

children="$(jobs -pr)"
wait $children

if [[ -n "$K8S_VERSION" ]]; then
	cd k8s-openapi-tests
	RUST_BACKTRACE=full cargo test --verbose --features "$FEATURE"
fi
