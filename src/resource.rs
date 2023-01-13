/// A trait applied to all Kubernetes resources.
pub trait Resource {
    /// The API version of the resource. This is a composite of [`Resource::GROUP`] and [`Resource::VERSION`] (eg `"apiextensions.k8s.io/v1beta1"`)
    /// or just the version for resources without a group (eg `"v1"`).
    ///
    /// This is the string used in the `apiVersion` field of the resource's serialized form.
    const API_VERSION: &'static str;

    /// The group of the resource, or the empty string if the resource doesn't have a group.
    const GROUP: &'static str;

    /// The kind of the resource.
    ///
    /// This is the string used in the `kind` field of the resource's serialized form.
    const KIND: &'static str;

    /// The version of the resource.
    const VERSION: &'static str;

    /// The URL path segment used to construct URLs related to this resource.
    ///
    /// For cluster- and namespaced-scoped resources, this is the plural name of the resource that is followed by the resource name.
    /// For example, [`api::core::v1::Pod`](crate::api::core::v1::Pod)'s value is `"pods"` and its URLs look like `.../pods/{name}`.
    ///
    /// For subresources, this is the subresource name that comes after the parent resource's name.
    /// For example, [`api::authentication::v1::TokenRequest`](crate::api::authentication::v1::TokenRequest)'s value is `"token"`,
    /// and its URLs look like `.../serviceaccounts/{name}/token`.
    const URL_PATH_SEGMENT: &'static str;

    /// Indicates whether the resource is namespace-scoped or cluster-scoped or a subresource.
    ///
    /// If you need to restrict some generic code to resources of a specific scope, use this associated type to create a bound on the generic.
    /// For example, `fn foo<T: k8s_openapi::Resource<Scope = k8s_openapi::ClusterResourceScope>>() { }` can only be called with cluster-scoped resources.
    type Scope: ResourceScope;
}

/// The scope of a [`Resource`].
pub trait ResourceScope {}

/// Indicates that a [`Resource`] is cluster-scoped.
pub struct ClusterResourceScope {}
impl ResourceScope for ClusterResourceScope {}

/// Indicates that a [`Resource`] is namespace-scoped.
pub struct NamespaceResourceScope {}
impl ResourceScope for NamespaceResourceScope {}

/// Indicates that a [`Resource`] is neither cluster-scoped nor namespace-scoped.
pub struct SubResourceScope {}
impl ResourceScope for SubResourceScope {}

/// A trait applied to all Kubernetes resources that can be part of a corresponding list.
pub trait ListableResource: Resource {
    /// The kind of the list type of the resource.
    ///
    /// This is the string used in the `kind` field of the list type's serialized form.
    const LIST_KIND: &'static str;
}

/// A trait applied to all Kubernetes resources that have metadata.
pub trait Metadata: Resource {
    /// The type of the metadata object.
    type Ty;

    /// Gets a reference to the metadata of this resource value.
    fn metadata(&self) -> &<Self as Metadata>::Ty;

    /// Gets a mutable reference to the metadata of this resource value.
    fn metadata_mut(&mut self) -> &mut<Self as Metadata>::Ty;
}

/// Extracts the API version of the given resource value.
///
/// This just returns the [`Resource::API_VERSION`] value for the argument's type, but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn api_version<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::API_VERSION
}

/// Extracts the group of the given resource value.
///
/// This just returns the [`Resource::GROUP`] value for the argument's type, but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn group<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::GROUP
}

/// Extracts the kind of the given resource value.
///
/// This just returns the [`Resource::KIND`] value for the argument's type, but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn kind<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::KIND
}

/// Extracts the version of the given resource value.
///
/// This just returns the [`Resource::VERSION`] value for the argument's type, but is useful when you already have a value
/// and don't want to explicitly write its type.
pub fn version<T>(_: &T) -> &'static str where T: Resource {
    <T as Resource>::VERSION
}
