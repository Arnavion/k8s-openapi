# v0.23.0 (2024-09-15)

- BREAKING CHANGE: Added support for Kubernetes 1.31 under the `v1_31` feature.

- BREAKING CHANGE: Dropped support for Kubernetes 1.24 and 1.25.

Corresponding Kubernetes API server versions:

- v1.26.15
- v1.27.16
- v1.28.14
- v1.29.9
- v1.30.5
- v1.31.1

## k8s-openapi-codegen-common

- No changes.

## k8s-openapi-derive

- No changes except to bump the `k8s-openapi-codegen-common` dependency to the new version.

---

# v0.22.0 (2024-05-04)

## k8s-openapi

- BREAKING CHANGE: Added support for Kubernetes 1.30 under the `v1_30` feature.

Corresponding Kubernetes API server versions:

- v1.24.17
- v1.25.16
- v1.26.15
- v1.27.13
- v1.28.9
- v1.29.4
- v1.30.0

## k8s-openapi-codegen-common

- No changes.

## k8s-openapi-derive

- No changes except to bump the `k8s-openapi-codegen-common` dependency to the new version.

---

# v0.21.1 (2024-02-17)

## k8s-openapi

- BUGFIX: Fix `schemars::JsonSchema` impl of `k8s_openapi::apimachinery::pkg::util::intstr::IntOrString` to be in line with what Kubernetes expects of int-or-string fields in CRDs.

Corresponding Kubernetes API server versions:

- v1.24.17
- v1.25.16
- v1.26.14
- v1.27.11
- v1.28.7
- v1.29.2

## k8s-openapi-codegen-common

- BUGFIX: See the bugfix mentioned above.

## k8s-openapi-derive

- No changes except to bump the `k8s-openapi-codegen-common` dependency to the new version.

---

# v0.21.0 (2024-01-19)

## k8s-openapi

- BREAKING CHANGE: Added support for Kubernetes 1.29 under the `v1_29` feature.

- BREAKING CHANGE: Dropped support for Kubernetes 1.22 and 1.23.

Corresponding Kubernetes API server versions:

- v1.24.17
- v1.25.16
- v1.26.13
- v1.27.10
- v1.28.6
- v1.29.1

## k8s-openapi-codegen-common

- No changes.

## k8s-openapi-derive

- No changes.

---

# v0.20.0 (2023-09-07)

## k8s-openapi

- BREAKING CHANGE: This release removes all associated methods of resource types that mapped API operations. For example, there is no more `fn api::core::v1::Pod::list()`, and all types related to API operations like `ListOptional` and `ResponseBody` have also been removed. See https://github.com/Arnavion/k8s-openapi/issues/149 for more details.

- BREAKING CHANGE: Added support for Kubernetes 1.28 under the `v1_28` feature.

- BREAKING CHANGE: Dropped support for Kubernetes 1.20 and 1.21.

Corresponding Kubernetes API server versions:

- v1.22.17
- v1.23.17
- v1.24.17
- v1.25.13
- v1.26.8
- v1.27.5
- v1.28.1

## k8s-openapi-codegen-common

- BREAKING CHANGE: `write_operation` and other things related to emitting API operations have been removed.

## k8s-openapi-derive

- BREAKING CHANGE: The generated resource type no longer has associated clientset methods for listing etc.

---

# v0.19.0 (2023-08-05)

## k8s-openapi

- BREAKING CHANGE: Added support for Kubernetes 1.27 under the `v1_27` feature.

- FEATURE: The `k8s-openapi` now has two additional Cargo features `earliest` and `latest`, which select the earliest and latest supported version. For example, in this release, `earliest` is equivalent to `v1_20` and `latest` is equivalent to `v1_27`.

Corresponding Kubernetes API server versions:

- v1.20.15
- v1.21.14
- v1.22.17
- v1.23.17
- v1.24.16
- v1.25.12
- v1.26.7
- v1.27.4

## k8s-openapi-codegen-common

- No changes.

## k8s-openapi-derive

- No changes.

---

# v0.18.0 (2023-04-07)

## k8s-openapi

- BREAKING CHANGE: The `k8s_openapi::DeepMerge` trait and its impls on this crate's types now have semantics in line with merge strategies in Kubernetes. Specifically, the code generator now takes the `x-kubernetes-list-map-keys`, `x-kubernetes-list-type`, `x-kubernetes-map-type`, `x-kubernetes-patch-merge-key` and `x-kubernetes-patch-strategy` annotations into account when generating the `DeepMerge` impls.

  For example, in v0.17.0, `PodSpec::merge_from` would append entries into `self.containers`, whereas now it does a "list-map" strategy to replace containers with the same `name`.

Corresponding Kubernetes API server versions:

- v1.20.15
- v1.21.14
- v1.22.17
- v1.23.17
- v1.24.12
- v1.25.8
- v1.26.3

## k8s-openapi-codegen-common

- BREAKING CHANGE: As mentioned above, the generated code for `k8s_openapi::DeepMerge` impls now takes merge strategy annotations into account.

## k8s-openapi-derive

- No changes.

---

# v0.17.0 (2023-01-04)

## k8s-openapi

- BREAKING CHANGE: Added support for Kubernetes 1.26 under the `v1_26` feature.

- BREAKING CHANGE: Dropped support for Kubernetes 1.18 and 1.19.

- FEATURE: Allow deserializing non-optional `ByteString`s from JSON `null`. The API server is known to allow these `null`s in the `ConfigMap::binary_data` and `Secret::data` maps. The deserialization results in an empty `ByteString`, to match the behavior of the API server when given a `null` value in the `ConfigMap::data` map.

Corresponding Kubernetes API server versions:

- v1.20.15
- v1.21.14
- v1.22.17
- v1.23.15
- v1.24.9
- v1.25.5
- v1.26.0

## k8s-openapi-codegen-common

- No changes.

## k8s-openapi-derive

- No changes.

---

# v0.16.0 (2022-09-15)

## k8s-openapi

- BREAKING CHANGE: Added support for Kubernetes 1.25 under the `v1_25` feature.

- FEATURE: All spec types now implement a deep-merge API via a `DeepMerge` trait impl with a `fn merge_from(&mut self, other: Self)` method. This is useful for builder-like operations.

Corresponding Kubernetes API server versions:

- v1.18.20
- v1.19.16
- v1.20.15
- v1.21.14
- v1.22.14
- v1.23.11
- v1.24.5
- v1.25.1

## k8s-openapi-codegen-common

- No changes.

## k8s-openapi-derive

- BREAKING CHANGE: `#[derive(CustomResourceDefinition)]` no longer generates a list type alias. For example, when applied to `struct FooSpec`, previously the custom derive would generate `pub type FooList = k8s_openapi::List<Foo>;` It no longer does this, in accordance with the main k8s-openapi crate where such aliases were removed back in v0.7.0

- FEATURE: The generated custom resource type will implement `k8s_openapi::DeepMerge` if the `impl_deep_merge` custom derive attribute is used. Note that this requires you to implement `k8s_openapi::DeepMerge` on the spec type yourself; the custom derive does not do that.

---

# v0.15.0 (2022-05-22)

## k8s-openapi

- BREAKING CHANGE: The `pretty` optional parameter has been removed from all operations. Setting this parameter to `true` would've made the API server pretty-print the JSON response, which is meaningless for a programmatic client.

- BREAKING CHANGE: In addition to the previous change, the `exact` and `export` parameters have been removed from all read operations (eg `Pod::read_namespaced_pod`). These parameters were removed in Kubernetes v1.21 and were known to be broken before that, and would've caused the server response to not be able to be parsed correctly via the operation's response type anyway.

  All read operations with the exception of `Pod::read_namespaced_pod_log` had only these three optional parameters, so now that they've been removed such read operations don't have an `optional: ReadFooOptional<'_>` parameter at all.

- BREAKING CHANGE: Operation names no longer include the `_namespaced` part and the resource type name. For example, `Pod::read_namespaced_pod` is now just `Pod::read`. The corresponding optional parameters type and response type no longer include the `Namespaced` part, eg `ReadNamespacedPodResponse` is now just `ReadPodResponse`.

- BREAKING CHANGE: Added support for Kubernetes 1.24 under the `v1_24` feature.

- BREAKING CHANGE: Dropped support for Kubernetes 1.16 and 1.17.

- FEATURE: The `K8S_OPENAPI_ENABLED_VERSION` env var can now be set at build time to enable a specific API version, just like enabling a specific version feature would've done. This is only meant to be used by library developers who want to run `cargo check`, `cargo doc`, etc commands, for which the previous advice of enabling a version feature via a dev dependency would not work.

Corresponding Kubernetes API server versions:

- v1.18.20
- v1.19.16
- v1.20.15
- v1.21.12
- v1.22.9
- v1.23.6
- v1.24.0

## k8s-openapi-codegen-common

- No changes.

## k8s-openapi-derive

- No changes.

---

# v0.14.0 (2022-01-23)

## k8s-openapi

- BREAKING CHANGE: k8s-openapi now disables all default features of its dependencies and only enables the ones it needs. If your code was implicitly relying on some default feature being enabled of an indirect dependency re-exported from `k8s-openapi`, it will now not compile. You will need to enable the feature yourself in your own dependency.

  For example, if you had `use k8s_openapi::schemars; #[derive(schemars::JsonSchema)] struct YourCode { ... }` this will no longer compile because the proc macro is only compiled when the `"derive"` feature is enabled. You will need to add an explicit dependency on the `schemars` crate in your code, with its `"derive"` feature enabled.

- BREAKING CHANGE: Added support for Kubernetes 1.23 under the `v1_23` feature.

- BREAKING CHANGE: Dropped support for Kubernetes 1.11, 1.12, 1.13, 1.14 and 1.15.

- BUGFIX: The `serde::Deserialize` impl of some types now accepts `null` for required fields and deserializes it as the default value of that field. This is because the Kubernetes API server violates the schema and sends `null` in some cases.

  For example, a user is allowed to create a `DaemonSet` whose `PodSpec` has `"containers": null`, even though `PodSpec::containers` is a required field and emitted as a `Vec`. When querying this `DaemonSpec` back from the API server, it will return `"containers": null` in the response too. Before this fix, such a response would fail to deserialize. Note that serialization is still spec-compliant as before, so such a `DaemonSet` could not have been created with this crate's types before and still cannot be created now.

Corresponding Kubernetes API server versions:

- v1.16.15
- v1.17.17
- v1.18.20
- v1.19.16
- v1.20.15
- v1.21.9
- v1.22.6
- v1.23.2

## k8s-openapi-codegen-common

- BUGFIX: `k8s_openapi_codegen_common::Error` now implements `source()` correctly instead of always returning `None`.

## k8s-openapi-derive

- No changes except to bump the `k8s-openapi-codegen-common` dependency to the new version.

---

# v0.13.1 (2021-10-08)

## k8s-openapi

- BUGFIX: v0.13.0 added `schemars::JsonSchema` impls for resource types. For types like `k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1` that are objects with no defined structure, the impl incorrectly emitted the schema as `{}` instead of `{ "type": "object" }`. This has now been fixed.

Corresponding Kubernetes API server versions:

- v1.11.10
- v1.12.10
- v1.13.12
- v1.14.10
- v1.15.12
- v1.16.15
- v1.17.17
- v1.18.20
- v1.19.15
- v1.20.11
- v1.21.5
- v1.22.2

## k8s-openapi-codegen-common

- BUGFIX: See the bugfix mentioned above.

## k8s-openapi-derive

- No changes except to bump the `k8s-openapi-codegen-common` dependency to the new version.

---

# v0.13.0 (2021-08-09)

## k8s-openapi

- BREAKING CHANGE: The change from v0.12.0 to make `Option<Vec<T>>` and `Option<BTreeMap<K, V>>` fields into `Vec<T>` and `BTreeMap<K, V>` fields has been reverted, because there is at least one case where a resource type needs to be serialized with an empty `Vec` field. See https://github.com/Arnavion/k8s-openapi/issues/103

- BREAKING CHANGE: Added support for Kubernetes 1.22 under the `v1_22` feature.

- BREAKING CHANGE: Fixed `api::core::v1::Pod::connect_get_namespaced_pod_exec` to take its optional `command` parameter as `Option<&'a [String]>` instead of `Option<&'a str>`

- BREAKING CHANGE: Fixed `api::core::v1::ServiceSpec`'s `clusterIPs` field to be correctly emitted as `cluster_ips` instead of `cluster_i_ps`

- FEATURE: The new `schemars` crate feature enables impls of `schemars::JsonSchema` on all resource types.

Corresponding Kubernetes API server versions:

- v1.11.10
- v1.12.10
- v1.13.12
- v1.14.10
- v1.15.12
- v1.16.15
- v1.17.17
- v1.18.20
- v1.19.13
- v1.20.9
- v1.21.3
- v1.22.0

## k8s-openapi-codegen-common

- BREAKING CHANGE: `k8s_openapi_codegen_common::run` now takes an additional parameter to indicate whether the generated code of resource types should contain an impl of `schemars::JsonSchema` or not.

## k8s-openapi-derive

- FEATURE: The `#[derive(CustomResourceDefinition)]` custom derive now recognizes a new attribute `#[custom_resource_definition(generate_schema)]`. If this attribute is provided, the generated custom resource type will also implement `schemars::JsonSchema`. The `schemars` feature of the `k8s-openapi` crate must have been enabled.

---

# v0.12.0 (2021-06-15)

## k8s-openapi

- BREAKING CHANGE: Struct fields of type `Option<Vec<T>>` and `Option<BTreeMap<K, V>>` are now of type `Vec<T>` and `BTreeMap<K, V>` respectively. When deserializing from JSON, `null` is deserialized to an empty collection. When serializing, an empty collection is not serialized. This was done to improve ergonomics and is not expected to create problems with any existing Kubernetes objects. If you do find a Kubernetes object that meaningfully differentiates between a `null` collection and an empty one, please file a bug.

- BREAKING CHANGE: Added support for Kubernetes 1.21 under the `v1_21` feature.

- FEATURE: The `Resource` trait now contains two additional items. The first is an associated type `Scope: ResourceScope` that identifies whether a resource is cluster-scoped, namespace-scoped or a subresource. The second is an associated const `URL_PATH_SEGMENT: &'static str` that can be used to dynamically construct a URL for operations on the resource - for cluster- and namespace-scoped resources it is their plural name, for subresources it is the subresource name.

Corresponding Kubernetes API server versions:

- v1.11.10
- v1.12.10
- v1.13.12
- v1.14.10
- v1.15.12
- v1.16.15
- v1.17.17
- v1.18.19
- v1.19.11
- v1.20.7
- v1.21.1

## k8s-openapi-derive

- BUGFIX: The generated code of `#[derive(CustomResourceDefinition)]` implicitly expected the crate to have added a dependency on the `http`, `serde` and `serde_json` crates. It has now been fixed to use the re-exports from the `k8s-openapi` crate instead.

## k8s-openapi-codegen-common

- BREAKING CHANGE: `run` now takes an impl of `RunState` for writing generated code and imports instead of two separate closures. This allows the impl of `RunState` to share state between invocations of the two functions instead of needing `RefCell`, etc.

- BREAKING CHANGE: `swagger20::Type::JSONSchemaPropsOrArray`, `swagger20::Type::JSONSchemaPropsOrBool` and `swagger20::Type::JSONSchemaPropsOrStringArray` have been renamed to `swagger20::Type::JsonSchemaPropsOrArray`, `swagger20::Type::JsonSchemaPropsOrArray` and `swagger20::Type::JsonSchemaPropsOrArray` respectively to match Rust naming conventions.

---

# v0.11.0 (2021-01-23)

- BREAKING CHANGE: This version partially reverts the change in v0.9.0 that made `k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent<T>` require `T: k8s_openapi::Resource`; now it only requires `T: serde::Deserialize<'de>` once more. This has been done to make it possible to use `WatchEvent` with custom user-provided resource types that do not implement `k8s_openapi::Resource`.

  The `k8s_openapi::Resource` bound in v0.9.0 was added to be able to enforce that the `WatchEvent::<T>::Bookmark` events contain the correct `apiVersion` and `kind` fields for the specified `T` during deserialization. Without the bound now, it is no longer possible to do that. So it is now possible to deserialize, say, a `WatchEvent::<Pod>::Bookmark` as a `WatchEvent::<Node>::Bookmark` without any runtime error. Take care to deserialize `watch_*` API responses into the right `crate::clientset::WatchResponse<T>` type, such as by relying on the returned `k8s_openapi::ResponseBody<T>` as documented in the crate docs.

- BREAKING CHANGE: The `bytes` dependency has been updated to match the `tokio` v1 ecosystem.

- FEATURE: Added support for Kubernetes 1.20 under the `v1_20` feature.

Corresponding Kubernetes API server versions:

- v1.11.10
- v1.12.10
- v1.13.12
- v1.14.10
- v1.15.12
- v1.16.15
- v1.17.17
- v1.18.15
- v1.19.7
- v1.20.2

---

# v0.10.0 (2020-10-11)

- FEATURE: Added support for Kubernetes 1.19 under the `v1_19` feature.

Corresponding Kubernetes API server versions:

- v1.11.10
- v1.12.10
- v1.13.12
- v1.14.10
- v1.15.12
- v1.16.15
- v1.17.12
- v1.18.9
- v1.19.2

---

# v0.9.0 (2020-07-19)

- BREAKING CHANGE: Resource types that used to have a `metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>` field now have a `metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta` field instead. That is, metadata is now a required field for resource types. Most client requests and server responses need to set the field, so dealing with it being optional required unnecessary boilerplate in client code for both creating requests and using responses.

  Likewise, the `k8s_openapi::Metadata` trait's `metadata` getter now returns `&Self::Ty` instead of `Option<&Self::Ty>`

  Note that the fields inside the `ObjectMeta` type are themselves still optional.

  There are some sitations like PATCH requests where the metadata truly is optional. In these cases, you can create an empty metadata value via `Default::default()`, which will get serialized as an empty JSON object `{}`. If there is a situation where the empty object does not act the same as if the field had been omitted entirely, please file an issue.

- BREAKING CHANGE: `k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent::<T>::Bookmark` used to be a tuple variant containing a `T`, but is now a struct variant containing a `resource_version: String` field.

  While the Kubernetes OpenAPI spec indicates that bookmark events contain the resource type, in fact they contain a stripped down form of that type with only the `apiVersion`, `kind` and `metadata.resourceVersion` fields set to useful values. Previously this would cause deserialization of bookmark events to fail if the `T` had some required field that was actually unset or `null`. Now the deserializer only looks for those three values in the event and ignores any others.

- BREAKING CHANGE: `k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent<T>` now requires `T` to also implement `k8s_openapi::Resource`. Previously it only required `T` to implement `serde::de::DeserializeOwned`. This is required to support the change mentioned in the previous item, since the deserialization of a `WatchEvent` now needs to take the `apiVersion`, `kind` and `metadata.resourceVersion` fields into consideration itself instead of relying on `T`'s `serde::Deserialize` impl.

- FEATURE: The `k8s_openapi::Metadata` trait now has a `fn metadata_mut(&mut self) -> &mut<Self as Metadata>::Ty` method that can be used to mutate the metadata of the resource.

- FEATURE: The `k8s-openapi-codegen-common` crate is now stable and documented. It can be used by other code generators that want to generate code for Kubernetes-like API servers such as OpenShift.

Corresponding Kubernetes API server versions:

- v1.11.10
- v1.12.10
- v1.13.12
- v1.14.10
- v1.15.12
- v1.16.13
- v1.17.9
- v1.18.6

---

# v0.8.0 (2020-05-02)

- BREAKING CHANGE: Support for v1.8, v1.9 and v1.10 API servers has been dropped. These versions became hard to test with `kubectl` and `kind` are are not supported by major cloud providers.

- BREAKING CHANGE: `k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray`, `JSONSchemaPropsOrBool` and `JSONSchemaPropsOrStringArray` types  now wrap the `v1::JSONSchemaProps` type. Previously they incorrectly wrapped the `v1beta1::JSONSchemaProps` type.

- BREAKING CHANGE: Turning the `api` feature off now also disables the `k8s_openapi::{http,percent_encoding,url}` re-exports, the `k8s_openapi::percent_encoding2` module, the `k8s_openapi::{RequestError,ResponseError,ResponseBody}` types, the `k8s_openapi::Response` trait, and the `k8s_openapi::{Create,Delete,List,Patch,Replace,Watch}{Optional,Response}` types. All of these were only used by the API functions which had been disabled by turning the `api` feature off.

- BUGFIX: `k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime` and `Time` are now serialized with six and zero decimals in their seconds respectively, to match the API server's expectations. Previously they would be serialized with a variable number of decimals up to nine.

- FEATURE: Added support for Kubernetes 1.18 under the `v1_18` feature.

- FEATURE: `k8s_openapi::ByteString` now impls `PartialOrd` and `Ord`.

- FEATURE: `k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime` and `Time` now impl `Eq`, `PartialOrd` and `Ord`.

- FEATURE: If your crate has a dependency on `k8s-openapi`, you can now use a build script to detect which version feature has been enabled on the crate. This is a more verbose but also more flexible alternative to using the `k8s_if_*` version detection macros in your crate code.

- FEATURE: `k8s-openapi-derive`'s `#[derive(CustomResourceDefinition)]` now supports emitting a subresources field in the generated CR type.

Corresponding Kubernetes API server versions:

- v1.11.10
- v1.12.10
- v1.13.12
- v1.14.10
- v1.15.11
- v1.16.9
- v1.17.5
- v1.18.2

---

# v0.7.1 (2020-01-23)

Fixed docs URLs. No other changes since v0.7.0.

---

# v0.7.0 (2020-01-23)

- BREAKING CHANGE: The `http` and `bytes` dependencies have been updated. They now match the `tokio` 0.2 ecosystem.

- BREAKING CHANGE: The `Resource` trait's `api_version`, `group`, `kind` and `version` methods are now `API_VERSION`, `GROUP`, `KIND` and `VERSION` associated consts of `&'static str` type.

- BREAKING CHANGE: The `*List` resource types like `PodList` and `NodeList` have now been combined into a single generic `k8s_openapi::List<T>` type. The API response types that contained these list types have been updated accordingly.

- BREAKING CHANGE: The optional parameters of create and replace operations are now emitted as a single common type - `k8s_openapi::CreateOptional` and `k8s_openapi::ReplaceOptional` respectively.

- BREAKING CHANGE: The response types of create, delete, delete-collection, list, patch, replace and watch operations have now been combined into generic `CreateResponse<T>`, `DeleteResponse<T>`, `DeleteResponse<List<T>>`, `ListResponse<T>`, `PatchResponse<T>`, `ReplaceResponse<T>` and `WatchResponse<T>` types respectively.

- BUGFIX: v1.16's `k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray`, `JSONSchemaPropsOrBool` and `JSONSchemaPropsOrStringArray` types are now generated correctly, just like their `v1beta` cousins.

- FEATURE: Added support for Kubernetes 1.17 under the `v1_17` feature.

- FEATURE: A new `k8s_openapi::ListableResource` trait has been added to connect a resource type like `Pod` to its corresponding list type like `PodList`. Currently the trait only has one member - an associated const `LIST_KIND` that is the same as the list type's `Resource::KIND`.

Here are some demonstrative examples of the API changes:

- `Pod::delete_namespaced_pod` used to return `DeleteNamespacedPodResponse`. It now returns `DeleteResponse<Self>`.
- `Pod::delete_collection_namespaced_pod` used to return `DeleteCollectionNamespacedPodResponse`. It now returns `DeleteResponse<List<Self>>`.
- `Pod::list_namespaced_pod` used to return `ListNamespacedPodResponse` which had an `Ok(PodList)` variant. It now returns `ListResponse<Self>`, which has an `Ok(List<Self>)` variant.

Combining these response types has reduced the compile time and memory usage of the Rust compiler when compiling this crate. Notably, the compiler's memory usage now peaks at ~3 GiB from its earlier ~5 GiB, making it easier to use with environments limited to 4 GiB RAM, like CI VMs and Raspberry Pi's.

Corresponding Kubernetes API server versions:

- v1.8.15
- v1.9.11
- v1.10.13
- v1.11.10
- v1.12.10
- v1.13.12
- v1.14.10
- v1.15.9
- v1.16.6
- v1.17.2

---

# v0.6.0 (2019-10-18)

- BREAKING CHANGE: Updated `url` dependency, and thus the re-export, to v2. The re-export is only used internally by code-generated API functions and is not part of any public signatures, so it is only a breaking change for you if you were also using it for your own code.

- FEATURE: Added support for Kubernetes 1.16 under the `v1_16` feature.

- FEATURE: The `k8s-openapi-derive` crate is now out of beta.

Corresponding Kubernetes API server versions:

- v1.8.15
- v1.9.11
- v1.10.13
- v1.11.10
- v1.12.10
- v1.13.12
- v1.14.8
- v1.15.5
- v1.16.2

---

# v0.5.1 (2019-09-08)

- FEATURE: The `k8s-openapi` crate now has a default-enabled feature named `api`. If the feature is disabled, the library will only contain the resource types like `api::core::v1::Pod` and not the associated operation functions like `api::core::v1::Pod::read_namespaced_pod`. The corresponding `Response` and `Optional` types will also not be accessible. If your crate does not need the operation functions, you can disable this feature to save on compile time and resources.

Corresponding Kubernetes API server versions:

- v1.8.15
- v1.9.11
- v1.10.13
- v1.11.10
- v1.12.10
- v1.13.10
- v1.14.6
- v1.15.3

---

# v0.5.0 (2019-08-04)

- BREAKING CHANGE: The optional parameters of delete, list, patch and watch operations are now emitted as a single common type - `k8s_openapi::DeleteOptional`, `k8s_openapi::ListOptional`, `k8s_openapi::PatchOptional` and `k8s_openapi::WatchOptional` respectively. For example, where an operation like `k8s_openapi::api::core::v1::Pod::list_namespaced_pod` used to have an `optional: k8s_openapi::api::core::v1::ListNamespacedPodOptional<'_>`, it now has an `optional: k8s_openapi::ListOptional<'_>` parameter instead.

  This is because these per-operation optional structs all had the same members.

  Furthermore, delete-collection operations like `Pod::delete_collection_namespaced_pod` now take two optional parameters, one of type `k8s_openapi::ListOptional` that determines which items will be selected for deletion, and the other of type `k8s_openapi::DeleteOptional` which determines how the selected items will be deleted.

- BREAKING CHANGE: Most response types had an empty `Unauthorized` variant, and did not have other useful variants like `Forbidden` or `Conflict`. To handle those variants, you would have had to match on the empty `Other` variant and manually parse the response body into a JSON value.

  Now these empty variants like `Unauthorized` are no longer emitted, and the previously empty `Other` variant is now emitted as `Other(Result<Option<serde_json::Value>, Error>)`. If the response body is empty, the response will be parsed as `Other(Ok(None))`. Otherwise, it will be parsed as JSON into `Other(Ok(Some(response)))` or `Other(Err(err))`.

- BREAKING CHANGE: The `apimachinery::pkg::apis::meta::v1::Patch` type used to be incorrectly emitted as an empty struct. It is now emitted as an enum with variants corresponding to the three types of patches supported by Kubernetes - `Json(Vec<serde_json::Value>)`, `Merge(serde_json::Value)` and `StrategicMerge(serde_json::Value)`.

- BREAKING CHANGE: The `apimachinery::pkg::apis::meta::v1::WatchEvent` type used to be emitted as a struct containing a weakly-typed `object` and stringly-typed `type_` fields. It is now generic on the type of object and is emitted as `enum WatchEvent<T>` with `Added(T)`, `Deleted(T)`, `Modified(T)`, `ErrorStatus(metav1::Status)` and `ErrorOther(RawExtension)` members. For v1.15 and above, the enum also has a `Bookmark(T)` variant.

- BREAKING CHANGE: The `apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresourceStatus` type used to be incorrectly emitted as an empty struct. It is now emitted as a newtype around `serde_json::Value`

- BREAKING CHANGE: The connect and exec operations on `Node`, `Pod` and `Service` no longer have a corresponding response type. The response types were bogus, and these operations are not HTTP requests but SPDY or WebSocket requests anyway. The functions still return `http::Request`, and you will need to decompose these into types that your SPDY / WebSocket crate uses.

- BUGFIX: Operation parameters that were used as path components of the request URL (such as `namespace`) are now correctly encoded instead of being used verbatim.

- FEATURE: The `http::Request` returned by API operation functions now has the `Content-Type` header set if the request has a body. Particularly for patch operations, this sets the correct `Content-Type` header corresponding to the type of patch used.

- FEATURE: A new crate `k8s-openapi-derive` is now released. This crate contains a custom derive that can be used on a CRD spec type to generate the corresponding CRD type, its `k8s_openapi::Resource` and `k8s_openapi::Metadata` impls, and CRUD operations. See that crate's docs for more details.

Corresponding Kubernetes API server versions:

- v1.8.15
- v1.9.11
- v1.10.13
- v1.11.10
- v1.12.10
- v1.13.8
- v1.14.4
- v1.15.1

---

# v0.4.0 (2019-03-07)

- A new `Resource` trait is implemented on all resource types to get their API version, group, kind and version properties. See docs for details.

  Since the API version and kind of resource types is statically associated with their Rust types, the `api_version` and `kind` fields have been removed from these types. The fields are serialized and deserialized automatically without needing to be set.

- A new `Metadata` trait is implemented on all resource types that have metadata (`ObjectMeta` or `ListMeta`), and can be used to get access to this metadata.

- Fixed string responses like `ReadNamespacedPodLogResponse` to return `ResponseError::NeedMoreData` when given an empty slice, instead of returning an empty string.

- Fixed string responses like `ReadNamespacedPodLogResponse` to return as much data as possible before an invalid utf-8 sequence in the input slice. In the previous release, the parser would return `Err` even if there was valid utf-8 before the first invalid byte. It now only returns `Err` if the first byte of the slice is invalid.

- A new `ResponseBody::advance` function can be used to remove bytes from the front of the `ResponseBody`'s internal buffer. This is useful in cases like reading a pod's logs, where the parser indicates that some bytes in the beginning of the slice are invalid, and you want to skip over them and continue parsing.

There are many breaking changes in this release, as part of modifying the generated code to be easier and safer to use:

- The crate no longer supports enabling multiple version features ('v1_8', 'v1_9', etc). One and only one version feature can be enabled.

  As before, it's recommended that only application crates enable features of this crate in their `Cargo.toml`, based on the version of Kubernetes they want to run against. Library crates should not enable specific features. If library crates need to emit different code based on the feature that gets enabled, they should continue to use the version detection macros (`k8s_if_ge_1_8! { }`, etc) around such code.

  The build script of this crate will panic and fail the build if it detects zero or more than one feature has been enabled.

- In the previous release, every enabled version was represented by a top-level module, eg enabling the `v1_8` feature granted access to the `k8s_openapi::v1_8` module and the `k8s_openapi::v1_8::api::core::v1::Pod` type. Since only one feature can be enabled now, the contents of this corresponding module are re-exported from the crate root, and the module itself is hidden. Thus enabling the `v1_8` feature allows access to v1.8's `Pod` type through the `k8s_openapi::api::core::v1::Pod` path. This means crates that support multiple versions no longer need to write code like:

  ```rust
  k8s_if_1_8! { use k8s_openapi::v1_8::api::core::v1 as api; }
  k8s_if_1_9! { use k8s_openapi::v1_9::api::core::v1 as api; }
  k8s_if_1_10! { use k8s_openapi::v1_10::api::core::v1 as api; }
  ...
  ```

  and can instead write:

  ```rust
  use k8s_openapi::api::core::v1 as api;
  ```

- Support for v1.7 API servers has been dropped. Its API structure was very different from the later versions (upstream rearranged the OpenAPI spec in v1.8). Continuing to support it would've required crates to continue using `k8s_if_1_7! { }` and `k8s_if_ge_1_7! { }`. By dropping support for it, there is much less reason to use the version detection macros.

- API operation functions now have shorter names in many cases. For example `Pod::read_core_v1_namespaced_pod_log` is now called just `Pod::read_namespaced_pod_log`. The thing that is stripped from the name is a combination of the API kind ("core") and the API version ("v1"). In general, this means supporting different versions of API, like supporting `Deployment::list_apps_v1beta2_namespaced_deployment` on v1.8 and `Deployment::list_apps_v1_namespaced_deployment` on v1.9+ no longer requires version detection macros, since the function is now called `Deployment::list_namespaced_deployment` in both cases.

- API operations now take a single parameter for all optional parameters. This parameter implements `Default` such that passing in `Default::default()` for this parameter is equivalent to not passing any optional parameters.

  For example, the `api::core::v1::Pod::list_namespaced_pod` operation has one required parameter for the `namespace`, and the optional parameters are fields of the `api::core::v1::ListNamespacedPodOptional` struct which is taken as the last parameter of the function. To set one or more of these optional parameters, use struct update syntax to set those parameters and `Default` the rest. See the crate docs for details.

  This reduces the number of `None` parameters that need to be passed to all operation functions. It also means that function that differed in the number of optional parameters between Kubernetes versions no longer need version detection macros if you don't want to set any of the optional parameters. For example, `Job::create_namespaced_job` takes one optional parameter in v1.11 and three in v1.12. With this release, it can be called as `Job::create_namespaced_job("name", &spec, Default::default())` regardless of the version.

- `ResponseBody` is now generic on the response type at the type level. Previously only its `append_slice_and_parse` function was generic on the response type.

- `ResponseBody::append_slice_and_parse` has been removed. This function gave the wrong impression that appending a slice would allow zero or one responses to be parsed from its buffer. In fact, more than one response can be parsed, say when parsing a sequence of `WatchEvent`s and the appended slice contains more than one `WatchEvent`. Instead, there are now separate `ResponseBody::append_slice` and `ResponseBody::parse` functions. You should call `ResponseBody::parse` in a loop as long as it returns `Ok(parsed_response)`.

- API operation functions now return two values in a tuple. The first value is the same `http::Request<Vec<u8>>` that they used to return in the previous release - this is the HTTP request with the path, query string and request body filled out according to the API operation. The second value is a function with the signature `fn(http::StatusCode) -> ResponseBody<ResponseType>`, where `ResponseType` is the type that should be used to parse the response. For example, `Pod::list_namespaced_pod` returns `(http::Request<Vec<u8>>, fn(http::StatusCode) -> ResponseBody<ListNamespacedPodResponse>)`.

  In previous releases, you would've needed to read the docs to know that `ListNamespacedPodResponse` is the type you should use to parse the response. With this change, you can use the constructor to let the type system enforce it.

- API operations that allowed you to list or watch in the same operation are now more strongly typed. For example, in the previous release, `Pod::list_core_v1_namespaced_pod`'s docs indicated that you should use `ListCoreV1NamespacedPodResponse` to parse its response. However, this function has an optional `watch` parameter which can be used to get a stream of `WatchEvent` responses instead and thus you would need to parse the response with `WatchCoreV1NamespacedPodResponse` instead. This `WatchCoreV1NamespacedPodResponse` type was the response type of the otherwise unrelated `Pod::watch_core_v1_namespaced_pod` operation.

  Furthermore, Kubernetes has deprecated many of these watch and watchlist operations, like `Pod::watch_core_v1_namespaced_pod` and `Pod::watch_core_v1_namespaced_pod_list` respectively, in favor of using the `list` operation with the `watch` parameter set.

  With this release, both of these issues are solved.

  The list function, like `Pod::list_namespaced_pod`, no longer has a `watch` parameter. This function always returns a list response.

  The watch function, like `Pod::watch_namespaced_pod`, is no longer generated from the (potentially deprecated) watch function in the OpenAPI spec. Instead, it has the same URL and parameters as the `list` operation. It too does not have a `watch` parameter, but it sets `watch=true` in the URL of the generated `http::Request` automatically. This function always returns a `WatchEvent` stream response.

  The watchlist function, if any, is no longer emitted. It is redundant with the watch function, since watching a single item for changes or multiple items for changes is determined by the specificity of the `field_selector` parameter.

Corresponding Kubernetes API server versions:

- v1.8.15
- v1.9.11
- v1.10.13
- v1.11.8
- v1.12.6
- v1.13.4

---

# v0.3.0 (2018-11-08)

- No notable changes except for API updates based on upstream changes to the OpenAPI specs.

Corresponding Kubernetes API server versions:

- v1.7.16
- v1.8.15
- v1.9.11
- v1.10.9
- v1.11.4
- v1.12.2

---

# v0.2.0 (2018-07-06)

- BREAKING CHANGE: Types that were previously emitted as type aliases are now emitted as newtypes. For example `io.k8s.apimachinery.pkg.api.resource.Quantity` was previous emitted as `pub type Quantity = String;` but is now emitted as `pub struct Quantity(pub String);`

- BREAKING CHANGE: The `IntOrString` enum in the crate root no longer exists. Previously each version's `io.k8s.apimachinery.pkg.util.intstr.IntOrString` was emitted as a type alias for the root type - `pub type IntOrString = ::IntOrString;`. Now they are emitted as the enum themselves `pub enum IntOrString { ... }`. This brings `IntOrString` in line with other types like `RawExtension` that have special replacement versions.

Corresponding Kubernetes API server versions:

- v1.7.16
- v1.8.14
- v1.9.9
- v1.10.5
- v1.11.0

---

# v0.1.0 (2018-06-30)

First release.

Corresponding Kubernetes API server versions:

- v1.7.16
- v1.8.14
- v1.9.9
- v1.10.5
- v1.11.0
