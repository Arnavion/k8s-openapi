The repository supports the latest patch versions of each of the separate 1.x releases.


# To add support for a new patch version

(Eg: The repository supports v1.10.5 and you want to add support for v1.10.6)

1. `/k8s-openapi-codegen/src/supported_version.rs`: Update the `spec_url` for the corresponding `SupportedVersion`.

1. `/k8s-openapi-codegen/src/supported_version.rs`: Add fixups for the new variant as necessary.

1. `/k8s-openapi-codegen/`: Use `cargo run` to regenerate the bindings. Inspect the diff. This combined with the changelog may indicate new fixups that could be backported to older versions.

1. `/k8s-openapi-tests/`: Use `cargo test --features 'test_v1_<>' --no-run` to build the lib crate and test crate with the new version's feature enabled.

1. Create a minikube cluster, and run the tests against it in record mode. Details in `/k8s-openapi-tests/test-replays/README.md`. Make sure to use the same version of minikube listed in the table there. The latest version of minikube may not support deploying a cluster of an old release.

1. `/k8s-openapi-tests/test-replays/README.md`: Update the row corresponding to the Kubernetes version with the new Kubernetes version.

1. Destroy the cluster, and re-run the tests in replay mode.


# To add support for a new minor version

(Eg: The repository supports v1.10.x and you want to add support for v1.11.0)

1. `/k8s-openapi-codegen/src/supported_version.rs`: Add a new variant to the `SupportedVersion` enum. Use the appropriate `mod_root` and `spec_url`.

1. `/k8s-openapi-codegen/src/supported_version.rs`: Add the new variant to the `ALL` list.

1. `/k8s-openapi-codegen/src/supported_version.rs`: Add fixups for the new variant as necessary.

1. `/k8s-openapi-codegen/`: Use `cargo run` to generate the bindings for the new version.

   A helpful trick here is to set the `mod_root` to that of the previous version, so that the generator ends up overwriting the previous version's files instead. Then you can use `git diff` to see precisely what changed between the last version and this one. This is useful to discover new fixups that could be backported to older versions. Make sure to set the `mod_root` to the proper value before continuing.

1. `/Cargo.toml`: Add a new feature for the new version.

1. `/Cargo.toml`: Update `package.metadata.docs.rs.features` to the new feature.

1. Update feature name in the `rustdoc` command in the "To make a new crate release" section below.

1. `/build.rs`: Update the value of `MAX`

1. `/src/lib.rs`: Add a new doc header attribute for the new version.

1. `/src/lib.rs`: Add a new `mod` for the new version.

1. `/k8s-openapi-tests/Cargo.toml`: Add a new feature for the new version. It should enable the corresponding feature of the `k8s-openapi` crate.

1. `/k8s-openapi-tests/src/lib.rs`: Add a new `replays_directory` in `Client::with`

1. `/k8s-openapi-tests/`: Use `cargo test --features 'test_v1_<>' --no-run` to build the lib crate and test crate with the new version's feature enabled.

1. `/k8s-openapi-tests/test-replays/`: Create an empty directory for the new version.

1. Create a minikube cluster, and run the tests against it in record mode. Details in `/k8s-openapi-tests/test-replays/README.md`.

1. `/k8s-openapi-tests/test-replays/README.md`: Add a row to the versions table with the Kubernetes and minikube versions.

1. Destroy the cluster, and re-run the tests in replay mode.

1. `/azure-pipelines.yml`: Add a new `strategy` for the new version.


# To make a new crate release

1. `/Cargo.toml`: Update crate version
1. `/Cargo.toml`: Update docs URL
1. `/k8s-openapi-codegen-common/Cargo.toml`: Update crate version
1. `/k8s-openapi-derive/Cargo.toml`: Update crate version
1. `/k8s-openapi-derive/Cargo.toml`: Update version req of `k8s-openapi-codegen-common` dependency
1. `/k8s-openapi-derive/Cargo.toml`: Update docs URL
1. Generate docs (change feature and directory name as appropriate).

    ```sh
    rm -rf ./target/doc/ &&
    cargo rustdoc --features 'v1_17' -- -Z unstable-options --enable-index-page &&
    CARGO_TARGET_DIR="$(realpath ./target)" cargo rustdoc --manifest-path ./k8s-openapi-derive/Cargo.toml -- -Z unstable-options --enable-index-page &&
    rm -rf ../k8s-openapi-gh-pages/v0.6.x &&
    cp -R ./target/doc ../k8s-openapi-gh-pages/v0.6.x
    ```

1. `../k8s-openapi-gh-pages/index.html`: Add new anchor if this is a new major release
