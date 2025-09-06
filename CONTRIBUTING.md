The repository supports the latest patch versions of each of the separate 1.x releases.


# To run all tests

```sh
# Create all node images
KIND_IMAGES="$(realpath ~/kind-images)"
mkdir -p "$KIND_IMAGES"
./test.sh all create-node-image "$KIND_IMAGES"

# Create all clusters
./test.sh all create-cluster "$KIND_IMAGES"

# Run all tests in record mode
K8S_RECORD=1 ./test.sh all run-tests

# Delete all clusters
./test.sh all delete-cluster

# Run all tests in replay mode
./test.sh all run-tests
```


# To add support for a new patch version

(Eg: The repository supports v1.50.5 and you want to add support for v1.50.6)

1. `/k8s-openapi-codegen/src/supported_version.rs`: Update the `spec_url` for the corresponding `SupportedVersion`.

1. `/k8s-openapi-codegen/src/supported_version.rs`: Add fixups for the new variant as necessary.

1. `/k8s-openapi-codegen/`: Use `cargo run` to regenerate the bindings. Inspect the diff. This combined with the changelog may indicate new fixups that could be backported to older versions.

1. `/test.sh`: Update `K8S_VERSIONS` and `KIND_VERSIONS` map entries for the new version.

1. `/`: Use `./test.sh '...' create-node-image '...'; ./test.sh '...' create-cluster '...'; K8S_RECORD=1 ./test.sh '...' run-tests` to create a cluster and run the tests against it in record mode.

1. `/`: Use `./test.sh '...' delete-cluster; ./test.sh '...' run-tests` to delete the cluster and run the tests in replay mode.


# To add support for a new minor version

(Eg: The repository supports v1.50.x and you want to add support for v1.51.0)

1. `/k8s-openapi-codegen/src/supported_version.rs`: Add a new variant to the `SupportedVersion` enum. Use the appropriate `mod_root` and `spec_url`.

1. `/k8s-openapi-codegen/src/supported_version.rs`: Add the new variant to the `ALL` list.

1. `/k8s-openapi-codegen/src/supported_version.rs`: Add fixups for the new variant as necessary.

1. `/k8s-openapi-codegen/`: Use `cargo run` to generate the bindings for the new version.

   `diff` the mod root of the previous version and the new one to see precisely what changed between the two versions. This is useful to discover new fixups that could be backported to older versions.

1. Search for `_[a-z]s:` to find new special idents that need to be handled in `get_rust_ident`. If found, handle them, and add tests in `fn special_idents`

1. `/Cargo.toml`: Add a new feature for the new version.

1. `/Cargo.toml`: Update `latest` feature to the new feature.

1. `/build.rs`: Update the value of `MAX`

1. `/src/lib.rs`: Add a new doc header attribute for the new version.

1. `/src/lib.rs`: Add a new `mod` for the new version.

1. `/k8s-openapi-tests/Cargo.toml`: Add a new feature for the new version. It should enable the corresponding feature of the `k8s-openapi` crate.

1. `/k8s-openapi-tests/build.rs`: Update the value of `MAX`

1. `/k8s-openapi-tests/src/lib.rs`: Add a new `replays_directory` in `Client::new`

1. `/k8s-openapi-tests-macro-deps/Cargo.toml`: Add a new feature for the new version. It should enable the corresponding feature of the `k8s-openapi` crate.

1. `/test.sh`: Add `K8S_VERSIONS` and `KIND_VERSIONS` map enties for the new version.

1. `/`: Create a cluster and run the tests against it in record mode.

    ```sh
    ./test.sh '...' create-node-image /path/of/node/images/directory
    ./test.sh '...' create-cluster /path/of/node/images/directory
    K8S_RECORD=1 ./test.sh '...' run-tests
    ```

1. `/k8s-openapi-tests/`: Delete the cluster and run the tests in replay mode.

    ```sh
    ./test.sh '...' delete-cluster
    ./test.sh '...' run-tests
    ```

1. `/devenv.sh`: Update `max_version`.


# To make a new crate release

1. `/Cargo.toml`: Update `package.version` value
1. `/Cargo.toml`: Update `package.documentation` value
1. `/Cargo.toml`: Update `package.links` value
1. `/k8s-openapi-codegen-common/Cargo.toml`: Update `package.version` value
1. `/k8s-openapi-codegen-common/Cargo.toml`: Update `package.documentation` value
1. `/k8s-openapi-derive/Cargo.toml`: Update `package.version` value
1. `/k8s-openapi-derive/Cargo.toml`: Update `package.documentation` value
1. `/k8s-openapi-derive/Cargo.toml`: Update version req of `k8s-openapi-codegen-common` dependency
1. Generate docs (change directory name as appropriate).

    ```sh
    rm -rf ./target/doc/ &&
    cargo rustdoc --features 'latest' -- -A 'rustdoc::bare_urls' -Z unstable-options --enable-index-page &&
    CARGO_TARGET_DIR="$(realpath ./target)" cargo rustdoc --manifest-path ./k8s-openapi-codegen-common/Cargo.toml -- -Z unstable-options --enable-index-page &&
    CARGO_TARGET_DIR="$(realpath ./target)" cargo rustdoc --manifest-path ./k8s-openapi-derive/Cargo.toml -- -Z unstable-options --enable-index-page &&
    rm -rf ../k8s-openapi-gh-pages/v0.26.x &&
    cp -R ./target/doc ../k8s-openapi-gh-pages/v0.26.x
    ```

1. `../k8s-openapi-gh-pages/index.html`: Add new anchor if this is a new major release


# Cloud offerings

Keep these in mind when dropping support for older versions:

- [Aliyun ACK](https://www.alibabacloud.com/help/en/ack/ack-managed-and-ack-dedicated/user-guide/support-for-kubernetes-versions/)

- [Amazon EKS](https://docs.aws.amazon.com/eks/latest/userguide/kubernetes-versions.html#available-versions)

- [Azure AKS](https://learn.microsoft.com/en-us/azure/aks/supported-kubernetes-versions#azure-portal-and-cli-versions)

    ```sh
    az account list-locations --query '[].name' --output tsv |
        while read -r location; do
            (</dev/null az aks get-versions --location "$location" --query 'values[].patchVersions.keys(@)[]' --output tsv 2>/dev/null) & :
        done |
        sort --version-sort |
        head -1
    ```

- [Google GKE](https://cloud.google.com/kubernetes-engine/docs/release-schedule)

- [Oracle OKE](https://docs.oracle.com/en-us/iaas/Content/ContEng/Concepts/contengaboutk8sversions.htm#supportedk8sversions)
