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

# Saves a few seconds for large crates
export CARGO_INCREMENTAL=0

pushd k8s-openapi
cargo test --verbose --no-run --features "$FEATURE"
popd

pushd k8s-openapi-tests
cargo test --verbose --no-run --features "test_$FEATURE"
popd

pushd k8s-openapi
RUST_BACKTRACE=full timeout 120 cargo test --verbose --features "$FEATURE"
popd

pushd k8s-openapi-tests
RUST_BACKTRACE=full timeout 120 cargo test --verbose --features "test_$FEATURE"
popd

pushd k8s-openapi
cargo doc --verbose --no-deps --features "$FEATURE"
popd

pushd k8s-openapi
cargo clippy --verbose --features "$FEATURE"
popd
