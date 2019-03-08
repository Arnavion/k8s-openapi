set -euo pipefail

mkdir -p ~/.cargo/bin
curl -Lo ~/.cargo/bin/rustup 'https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init'
chmod +x ~/.cargo/bin/rustup
ln -s ~/.cargo/bin/rustup ~/.cargo/bin/cargo
ln -s ~/.cargo/bin/rustup ~/.cargo/bin/cargo-clippy
ln -s ~/.cargo/bin/rustup ~/.cargo/bin/rustc
ln -s ~/.cargo/bin/rustup ~/.cargo/bin/rustdoc
export PATH="$PATH:$(realpath ~/.cargo/bin)"

rustup install stable
rustup default stable

rustup component add clippy

pushd k8s-openapi-codegen
cargo clippy
popd
