[![Build Status](https://travis-ci.com/Arnavion/k8s-openapi-codegen.svg?branch=master)](https://travis-ci.com/Arnavion/k8s-openapi-codegen)

This binary generates Rust types for the resource definitions in the Kubernetes OpenAPI spec.


# Usage

```toml
[dependencies]
k8s-openapi = { git = "https://github.com/Arnavion/k8s-openapi-codegen", branch = "master", features = ["v1_9"] }
```

```rust
extern crate k8s_openapi;

fn main() {
	let pod_spec: k8s_openapi::v1_9::api::core::v1::PodSpec = Default::default();
	println!("{:#?}", pod_spec);
}
```

Each supported version of Kubernetes is represented by one top-level module (like `::v1_9`) and is enabled by a feature flag of the same name (like `v1_9`).


# Build

1. Run this binary.

	```sh
	cargo run
	```

1. Build the `k8s-openapi` crate to test that the generated code compiles

	1. Enter directory

		```sh
		pushd $PWD/k8s-openapi
		```

	1. Build

		```sh
		cargo build --all-features
		```

	1. Leave directory

		```sh
		popd
		```
