set -euo pipefail

(
	mkdir -p ~/.cargo/bin
	curl -Lo ~/.cargo/bin/rustup 'https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init'
	chmod +x ~/.cargo/bin/rustup
	ln -s ~/.cargo/bin/rustup ~/.cargo/bin/cargo
	ln -s ~/.cargo/bin/rustup ~/.cargo/bin/rustc
	ln -s ~/.cargo/bin/rustup ~/.cargo/bin/rustdoc
	export PATH="$PATH:$(realpath ~/.cargo/bin)"

	rustup install stable
	rustup default stable

	pushd k8s-openapi
	cargo test --verbose --no-run --features "$FEATURE"
	popd

	pushd k8s-openapi-tests
	cargo test --verbose --no-run --features "test_$FEATURE"
	popd
) &

(
	curl -Lo ~/minikube "https://storage.googleapis.com/minikube/releases/$MINIKUBE_VERSION/minikube-linux-amd64"
	chmod +x ~/minikube

	curl -LO "https://github.com/kubernetes-sigs/cri-tools/releases/download/$CRICTL_VERSION/crictl-$CRICTL_VERSION-linux-amd64.tar.gz"
	tar xf "./crictl-$CRICTL_VERSION-linux-amd64.tar.gz"
	sudo mv ./crictl /usr/local/bin

	sudo CHANGE_MINIKUBE_NONE_USER=true ~/minikube start --vm-driver=none "--kubernetes-version=$K8S_VERSION"
) &

children="$(jobs -pr)"
wait $children

export PATH="$PATH:$(realpath ~/.cargo/bin)"

pushd k8s-openapi
RUST_BACKTRACE=full timeout 120 cargo test --verbose --features "$FEATURE"
popd

pushd k8s-openapi-tests
RUST_BACKTRACE=full timeout 120 cargo test --verbose --features "test_$FEATURE"
popd
