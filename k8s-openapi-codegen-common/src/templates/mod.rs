use crate::swagger20::SchemaKind;

pub(crate) mod impl_deserialize;

pub(crate) mod impl_listable_resource;

pub(crate) mod impl_metadata;

pub(crate) mod impl_resource;

pub(crate) mod impl_schema;

pub(crate) mod impl_serialize;

pub(crate) mod int_or_string;

pub(crate) mod json_schema_props_or;

pub(crate) mod newtype;

pub(crate) mod operation_response_common;

pub(crate) mod patch;

pub(crate) mod query_string_optional;

pub(crate) mod r#struct;

pub(crate) mod type_alias;

pub(crate) mod type_header;

pub(crate) mod watch_event;

#[derive(Clone, Copy, Default)]
pub(crate) struct Generics<'a> {
	pub(crate) type_part: Option<&'a str>,
	pub(crate) where_part: Option<&'a str>,
}

pub(crate) struct Property<'a> {
	pub(crate) name: &'a str,
	pub(crate) comment: Option<&'a str>,
	pub(crate) field_name: std::borrow::Cow<'static, str>,
	pub(crate) field_type_name: String,
	pub(crate) field_inner_type_name: String,
	/// Irrespective of this field being required or not (ignore the wrapping `Optional` if present), does it have a `Default` implemented?
	pub(crate) field_type_impl_default: bool,
	pub(crate) collection_item_type: Option<CollectionInfo<'a>>,
	/// If `collection_item_type` is true, it must contain `Some` with `Default` for the time element
	// Kept separately to avoid having to refactor the code too much, since it's calculate from `collection_item_type`
	// somewhat later.
	pub(crate) collection_item_impls_default: Option<bool>,
	pub(crate) required: PropertyRequired,
	pub(crate) is_flattened: bool,
}

#[derive(Clone)]
pub(crate) struct CollectionInfo<'a> {
	pub(crate) items_type: std::borrow::Cow<'a, str>,
	pub(crate) items_kind: SchemaKind,
	pub(crate) type_: CollectionType,
}

#[derive(Clone)]
pub(crate) enum CollectionType {
	Vec,
	Map,
}

#[derive(Clone, Copy)]
pub(crate) enum PropertyRequired {
	Required { is_default: bool },
	Optional,

	// TODO:
	// This was added in 6cff2149f3334a220f1827b6740fcac54783b6c3 but then stopped being used because of
	// https://github.com/Arnavion/k8s-openapi/issues/103
	#[allow(unused)]
	OptionalDefault,
}

#[derive(Clone, Copy)]
pub(crate) struct ResourceMetadata<'a> {
	pub(crate) api_version: &'a str,
	pub(crate) group: &'a str,
	pub(crate) kind: &'a str,
	pub(crate) version: &'a str,
	pub(crate) list_kind: Option<&'a str>,
	pub(crate) metadata_ty: Option<&'a str>,
	pub(crate) url_path_segment_and_scope: (&'a str, &'a str),
}

#[derive(Clone, Copy)]
pub(crate) enum DateTimeSerializationFormat {
	Default,
	SixDecimalDigits,
	ZeroDecimalDigits,
}
