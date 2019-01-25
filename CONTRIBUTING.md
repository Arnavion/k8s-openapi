The repository supports the latest patch versions of each of the separate 1.x releases.


# To add support for a new patch version

(Eg: The repository supports v1.10.5 and you want to add support for v1.10.6)

1. `/src/supported_version.rs`: Update the `spec_url` for the corresponding `SupportedVersion`.

1. `/src/supported_version.rs`: Add fixups for the new variant as necessary.

1. Use `cargo run` to regenerate the bindings. Inspect the diff. This combined with the changelog may indicate new fixups that could be backported to older versions.

1. `/k8s-openapi-tests/`: Use `cargo test --features 'test_v1_<>' --no-run` to build the lib crate and test crate with the new version's feature enabled.

1. Create a minikube cluster, and run the tests against it in record mode. Details in `/k8s-openapi-tests/test-replays/README.md`

1. `/k8s-openapi-tests/test-replays/README.md`: Update the row corresponding to the Kubernetes version with the new Kubernetes version.

1. Destroy the cluster, and re-run the tests in replay mode.


# To add support for a new minor version

(Eg: The repository supports v1.10.x and you want to add support for v1.11.0)

1. `/src/supported_version.rs`: Add a new variant to the `SupportedVersion` enum. Use the appropriate `mod_root` and `spec_url`.

1. `/src/supported_version.rs`: Add the new variant to the `ALL` list.

1. `/src/supported_version.rs`: Add fixups for the new variant as necessary.

1. Use `cargo run` to generate the bindings for the new version.

   A helpful trick here is to set the `mod_root` to that of the previous version, so that the generator ends up overwriting the previous version's files instead. Then you can use `git diff` to see precisely what changed between the last version and this one. This is useful to discover new fixups that could be backported to older versions. Make sure the set the `mod_root` to the proper value before continuing.

1. `/k8s-openapi/Cargo.toml`: Add a new feature for the new version.

1. `/k8s-openapi/build.rs`: Update the value of `MAX`

1. `/k8s-openapi/src/lib.rs`: Add a new `mod` for the new version.

1. `/k8s-openapi-tests/Cargo.toml`: Add a new feature for the new version. It should enable the corresponding feature of the `k8s-openapi` crate.

1. `/k8s-openapi-tests/`: Use `cargo test --features 'test_v1_<>' --no-run` to build the lib crate and test crate with the new version's feature enabled.

1. Create a minikube cluster, and run the tests against it in record mode. Details in `/k8s-openapi-tests/test-replays/README.md`. Make sure to use the same version of minikube listed in the table there. The latest version of minikube may not support deploying a cluster of an old release.

1. `/k8s-openapi-tests/test-replays/README.md`: Add a row to the versions table with the Kubernetes and minikube versions.

1. Destroy the cluster, and re-run the tests in replay mode.
