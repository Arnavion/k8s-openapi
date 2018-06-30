[![Build Status](https://travis-ci.com/Arnavion/k8s-openapi-codegen.svg?branch=master)](https://travis-ci.com/Arnavion/k8s-openapi-codegen)

This binary generates Rust types for the resource definitions in the Kubernetes OpenAPI spec.


# Generating the bindings

Run this binary:

```sh
cargo run
```

Bindings will now be generated in the `k8s-openapi/` directory.


# Using the bindings

See `k8s-openapi/README.md` for information about the bindings crate itself.

See the `k8s-openapi-tests/` directory for examples.
