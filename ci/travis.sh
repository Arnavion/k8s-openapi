set -euo pipefail

(
	cd k8s-openapi-tests
	cargo build --verbose --tests --features "$FEATURE"
) &

(
	curl -Lo ~/minikube 'https://storage.googleapis.com/minikube/releases/v0.25.2/minikube-linux-amd64'
	chmod +x ~/minikube

	sudo CHANGE_MINIKUBE_NONE_USER=true ~/minikube start --vm-driver=none "--kubernetes-version=$K8S_VERSION"
) &

children="$(jobs -pr)"
wait $children

cd k8s-openapi-tests
cargo test --verbose --features "$FEATURE"
