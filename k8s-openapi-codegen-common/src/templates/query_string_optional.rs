pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	generics: super::Generics<'_>,
	vis: &str,
	fields: &[super::Property<'_>],
	is_watch: bool,
	operation_feature: Option<&str>,
	map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
	let local = crate::map_namespace_local_to_string(map_namespace)?;

	let type_generics_impl = generics.type_part.map(|part| format!("<{part}>")).unwrap_or_default();
	let type_generics_type = generics.type_part.map(|part| format!("<{part}>")).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {part}")).unwrap_or_default();

	let operation_feature_attribute: std::borrow::Cow<'static, str> =
		operation_feature.map_or("".into(), |operation_feature| format!("#[cfg(feature = {operation_feature:?})]\n").into());

	let mut fields_append_pair = String::new();

	for super::Property { name, field_name, field_type_name, .. } in fields {
		use std::fmt::Write;

		writeln!(fields_append_pair, "        if let Some(value) = self.{field_name} {{")?;
		match &**field_type_name {
			"Option<&'a str>" => writeln!(fields_append_pair, r#"            __query_pairs.append_pair({name:?}, value);"#)?,
			"Option<bool>" => writeln!(fields_append_pair, r#"            __query_pairs.append_pair({name:?}, if value {{ "true" }} else {{ "false" }});"#)?,
			_ => writeln!(fields_append_pair, r#"            __query_pairs.append_pair({name:?}, &value.to_string());"#)?,
		}
		writeln!(fields_append_pair, "        }}")?;
	}

	let watch_append_pair =
		if is_watch {
			"        __query_pairs.append_pair(\"watch\", \"true\");\n"
		}
		else {
			""
		};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/query_string_optional.rs")),
		local = local,
		type_name = type_name,
		type_generics_impl = type_generics_impl,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		operation_feature_attribute = operation_feature_attribute,
		vis = vis,
		fields_append_pair = fields_append_pair,
		watch_append_pair = watch_append_pair,
	)?;

	Ok(())
}
