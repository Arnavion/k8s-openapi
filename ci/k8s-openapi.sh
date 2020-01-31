set -euo pipefail

bash ./ci/rustup.sh

# Saves a few seconds for large crates
export CARGO_INCREMENTAL=0

case "$OP" in
	'clippy')
		cargo clippy --features "$FEATURE" -- -D warnings
		;;

	'doc')
		cargo doc --no-deps --features "$FEATURE"
		;;

	'lib-tests')
		RUST_BACKTRACE=full cargo test --features "$FEATURE"
		;;

	'tests')
		pushd k8s-openapi-tests
		RUST_BACKTRACE=full cargo test --features "test_$FEATURE"
		popd
		;;

	*)
		exit 1
		;;
esac
