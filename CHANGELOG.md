# v0.3.0 (2018-11-08)

- No notable changes except for API updates based on upstream changes to the OpenAPI specs.

Corresponding Kubernetes API server versions:

- v1.7.16
- v1.8.15
- v1.9.11
- v1.10.9
- v1.11.4
- v1.12.2


# v0.2.0 (2018-07-06)

- BREAKING CHANGE: Types that were previously emitted as type aliases are now emitted as newtypes. For example `io.k8s.apimachinery.pkg.api.resource.Quantity` was previous emitted as `pub type Quantity = String;` but is now emitted as `pub struct Quantity(pub String);`

- BREAKING CHANGE: The `IntOrString` enum in the crate root no longer exists. Previously each version's `io.k8s.apimachinery.pkg.util.intstr.IntOrString` was emitted as a type alias for the root type - `pub type IntOrString = ::IntOrString;`. Now they are emitted as the enum themselves `pub enum IntOrString { ... }`. This brings `IntOrString` in line with other types like `RawExtension` that have special replacement versions.

Corresponding Kubernetes API server versions:

- v1.7.16
- v1.8.14
- v1.9.9
- v1.10.5
- v1.11.0


# v0.1.0 (2018-06-30)

First release.

Corresponding Kubernetes API server versions:

- v1.7.16
- v1.8.14
- v1.9.9
- v1.10.5
- v1.11.0
