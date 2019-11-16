set -euo pipefail

if [ ! -d ~/.cargo/bin ]; then
	mkdir -p ~/.cargo/bin
	curl -Lo ~/.cargo/bin/rustup 'https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init'
	chmod +x ~/.cargo/bin/rustup
	ln -s ~/.cargo/bin/rustup ~/.cargo/bin/cargo
	ln -s ~/.cargo/bin/rustup ~/.cargo/bin/cargo-clippy
	ln -s ~/.cargo/bin/rustup ~/.cargo/bin/rustc
	ln -s ~/.cargo/bin/rustup ~/.cargo/bin/rustdoc
	export PATH="$PATH:$(realpath ~/.cargo/bin)"
fi

rustup set profile minimal
rustup install --no-self-update stable
rustup default stable
rustup component add clippy

# Saves a few seconds for large crates
export CARGO_INCREMENTAL=0

cargo test --verbose --no-run --features "$FEATURE"

pushd k8s-openapi-tests
cargo test --verbose --no-run --features "test_$FEATURE"
popd

RUST_BACKTRACE=full timeout 120 cargo test --verbose --features "$FEATURE"

pushd k8s-openapi-tests
RUST_BACKTRACE=full timeout 120 cargo test --verbose --features "test_$FEATURE"
popd

cargo doc --verbose --no-deps --features "$FEATURE"

cargo clippy --verbose --features "$FEATURE" -- -D warnings
