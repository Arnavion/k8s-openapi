use crate::safe_field;

pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	generics: super::Generics<'_>,
	vis: &str,
	fields: &[super::Property<'_>],
	is_watch: bool,
) -> Result<(), crate::Error> {
	let type_generics_impl = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_type = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {}", part)).unwrap_or_default();

	let mut fields_append_pair = String::new();

	for super::Property { name, field_name, field_type_name, .. } in fields {
		use std::fmt::Write;

		writeln!(fields_append_pair, "        if let Some(value) = &self.{} {{", safe_field(field_name))?;
		if field_type_name == "Option<&'a str>" {
			writeln!(fields_append_pair, r#"            __query_pairs.append_pair({:?}, value);"#, name)?;
		}
		else {
			writeln!(fields_append_pair, r#"            __query_pairs.append_pair({:?}, &value.to_string());"#, name)?;
		}
		writeln!(fields_append_pair, "        }}")?;
	}

	let watch_append_pair =
		if is_watch {
			"        __query_pairs.append_pair(\"watch\", \"true\");\n"
		} else {
			""
		};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/query_string_optional.rs")),
		type_name = type_name,
		type_generics_impl = type_generics_impl,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		vis = vis,
		fields_append_pair = fields_append_pair,
		watch_append_pair = watch_append_pair,
	)?;

	Ok(())
}
