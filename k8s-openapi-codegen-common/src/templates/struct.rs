use std::fmt::Write;

pub(crate) fn generate(
	mut writer: impl std::io::Write,
	vis: &str,
	type_name: &str,
	generics: super::Generics<'_>,
	fields: &[super::Property<'_>],
) -> Result<(), crate::Error> {
	let type_generics_type = generics.type_part.map(|part| format!("<{part}>")).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {part}")).unwrap_or_default();

	let fields_body: String = {
		fields.iter()
		.try_fold(String::new(), |mut fields, super::Property { comment, field_name, field_type_name, .. }| -> Result<_, std::fmt::Error> {

			if !fields.is_empty() {
				writeln!(fields)?;
			}

			if let Some(comment) = comment {
				for line in crate::get_comment_text(comment, "") {
					writeln!(fields, "    ///{line}")?;
				}
			}

			writeln!(fields,
				"    {vis}{field_name}: {field_type_name},",
				vis = vis,
				field_name = field_name,
				field_type_name = field_type_name,
			)?;

			Ok(fields)
		})?
	};

	let mut merge_body = String::new();

	for super::Property { field_name, .. }  in fields {
		writeln!(
			&mut merge_body,
				"        self.{field_name}.merge_from(other.{field_name});",
				field_name = field_name,
		)?;
	}

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/struct.rs")),
		type_name = type_name,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		fields = fields_body,
		merge_body = merge_body,
	)?;

	Ok(())
}
