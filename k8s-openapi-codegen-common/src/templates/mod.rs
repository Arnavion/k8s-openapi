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

pub(crate) mod struct_deep_merge;

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
	pub(crate) required: PropertyRequired,
	pub(crate) is_flattened: bool,
	pub(crate) merge_type: &'a crate::swagger20::MergeType,
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
