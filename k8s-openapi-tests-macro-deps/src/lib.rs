#![cfg(test)]

#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::default_trait_access,
	clippy::indexing_slicing,
	clippy::must_use_candidate,
	clippy::unseparated_literal_suffix,
)]

//! Test that the custom derive expansion compiles successfully. All third-party crates in the expansion
//! must be referenced as the re-exports from the k8s-openapi crate, so k8s-openapi is the only required dependency.

#[derive(
	Clone, Debug, PartialEq,
	k8s_openapi_derive::CustomResourceDefinition,
)]
#[custom_resource_definition(
	group = "k8s-openapi-tests-custom-resource-definition.com",
	version = "v1",
	plural = "foobars",
	namespaced,
)]
#[cfg_attr(k8s_apiextensions = "v1beta1", custom_resource_definition(has_subresources = "v1beta1"))]
#[cfg_attr(k8s_apiextensions = "v1", custom_resource_definition(has_subresources = "v1"))]
struct FooBarSpec {
	prop1: String,
	prop2: Vec<bool>,
	prop3: Option<i32>,
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for FooBarSpec {
	fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: k8s_openapi::serde::Deserializer<'de> {
		unimplemented!();
	}
}

impl k8s_openapi::serde::Serialize for FooBarSpec {
	fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error> where S: k8s_openapi::serde::Serializer {
		unimplemented!();
	}
}
