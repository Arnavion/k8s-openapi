pub(crate) mod impl_deserialize;

pub(crate) mod impl_listable_resource;

pub(crate) mod impl_metadata;

pub(crate) mod impl_resource;

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

fn get_comment_text<'a>(s: &'a str, indent: &'a str) -> impl Iterator<Item = std::borrow::Cow<'static, str>> + 'a {
	s.lines().scan(true, move |previous_line_was_empty, line|
		if line.is_empty() {
			*previous_line_was_empty = true;
			Some("".into())
		}
		else {
			let line =
				line
				.replace(r"\", r"\\")
				.replace("[", r"\[")
				.replace("]", r"\]")
				.replace("<", r"\<")
				.replace(">", r"\>")
				.replace("\t", "    ");

			let line =
				if *previous_line_was_empty && line.starts_with("    ") {
					// Collapse this line's spaces into two. Otherwise rustdoc will think this is the start of a code block containing a Rust test.
					format!("  {}", line.trim_start())
				}
				else {
					line
				};

			*previous_line_was_empty = false;

			Some(format!("{} {}", indent, line).into())
		})
}

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
	pub(crate) required: bool,
	pub(crate) is_flattened: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct ResourceMetadata<'a> {
	pub(crate) api_version: &'a str,
	pub(crate) group: &'a str,
	pub(crate) kind: &'a str,
	pub(crate) version: &'a str,
	pub(crate) is_listable: bool,
	pub(crate) metadata_ty: Option<(&'a str, bool)>,
}

#[derive(Clone, Copy)]
pub(crate) enum DateTimeSerializationFormat {
	Default,
	SixDecimalDigits,
	ZeroDecimalDigits,
}
