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
