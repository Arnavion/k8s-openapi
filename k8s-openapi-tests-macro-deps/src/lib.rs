#![cfg(test)]

#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]

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
	impl_deep_merge,
)]
#[custom_resource_definition(has_subresources = "v1")]
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

impl k8s_openapi::DeepMerge for FooBarSpec {
	fn merge_from(&mut self, other: Self) where Self: Sized {
		self.prop1.merge_from(other.prop1);
		k8s_openapi::merge_strategies::list::atomic(&mut self.prop2, other.prop2);
		self.prop3.merge_from(other.prop3);
	}
}
