set -euo pipefail

: "${K8S_VERSION:=}"

(
	pushd k8s-openapi
	cargo test --verbose --no-run --features "$FEATURE"
	popd

	pushd k8s-openapi-tests
	cargo test --verbose --no-run --features "test_$FEATURE"
	popd
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

pushd k8s-openapi
RUST_BACKTRACE=full cargo test --verbose --features "$FEATURE"
popd

if [[ -n "$K8S_VERSION" ]]; then
	pushd k8s-openapi-tests
	RUST_BACKTRACE=full cargo test --verbose --features "test_$FEATURE"
	popd
fi
