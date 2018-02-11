This binary generates Rust types for the resource definitions in the Kubernetes OpenAPI spec.


# Usage

1. Create a local copy of the [Kubernetes OpenAPI spec](https://github.com/kubernetes/kubernetes/blob/master/api/openapi-spec/swagger.json), say at `~/src/kubernetes/api/openapi-spec/swagger.json`

1. Run this binary on the spec file and place the generated code as a submodule of the `k8s-openapi` crate

	```sh
	cargo run -- ~/src/kubernetes/api/openapi-spec/swagger.json $PWD/k8s-openapi/src
	```

	For example `io.k8s.api.core.v1.PodSpec` will be emitted at `$PWD/k8s-openapi/src/api/core/v1/pod_spec.rs` and its fully-qualified name will be `::api::core::v1::PodSpec`

1. Build the `k8s-openapi` crate to test that the generated code compiles

	1. Enter directory

		```sh
		pushd $PWD/k8s-openapi
		```

	1. Build

		```sh
		cargo build
		```

	1. Leave directory

		```sh
		popd
		```
