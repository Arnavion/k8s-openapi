pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	generics: super::Generics<'_>,
	fields: &[super::Property<'_>],
	crate_root: &str,
	resource_metadata: Option<&super::ResourceMetadata<'_>>,
) -> Result<(), crate::Error> {
	let type_generics_impl = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_type = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {}", part)).unwrap_or_default();

	let mut fields_string = String::new();
	let mut required_fields_num = 0_usize;
	let mut fields_num = vec![];

	if resource_metadata.is_some() {
		use std::fmt::Write;

		writeln!(fields_string, r#"        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as {}::Resource>::API_VERSION)?;"#, crate_root)?;
		writeln!(fields_string, r#"        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as {}::Resource>::KIND)?;"#, crate_root)?;

		required_fields_num += 2;

	}
	for super::Property { name, field_name, required, .. } in fields {
		use std::fmt::Write;

		if *required {
			writeln!(fields_string, "        serde::ser::SerializeStruct::serialize_field(&mut state, {:?}, &self.{})?;", name, field_name)?;

			required_fields_num += 1;
		}
		else {
			writeln!(fields_string, "        if let Some(value) = &self.{} {{", field_name)?;
			writeln!(fields_string, "            serde::ser::SerializeStruct::serialize_field(&mut state, {:?}, value)?;", name)?;
			writeln!(fields_string, "        }}")?;

			fields_num.push(format!("self.{}.as_ref().map_or(0, |_| 1)", field_name));
		}
	}

	let fields_num: std::borrow::Cow<'_, str> = match (required_fields_num, fields_num.is_empty()) {
		(0, true) => "            0".into(),

		(0, false) => {
			use std::fmt::Write;

			let mut fields_num_str = String::new();
			let mut first = true;
			for field_num in fields_num {
				if first {
					first = false;
				}
				else {
					writeln!(fields_num_str, " +")?;
				}

				write!(fields_num_str, "            {}", field_num)?;
			}

			fields_num_str.into()
		},

		(required_fields_num, true) => format!("            {}", required_fields_num).into(),

		(required_fields_num, false) => {
			use std::fmt::Write;

			let mut fields_num_str = format!("            {}", required_fields_num);
			for field_num in fields_num {
				writeln!(fields_num_str, " +")?;
				write!(fields_num_str, "            {}", field_num)?;
			}

			fields_num_str.into()
		},
	};

	let serialize_type_name =
		if resource_metadata.is_some() {
			format!("<Self as {}::Resource>::KIND", crate_root)
		}
		else {
			format!("{:?}", type_name)
		};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/impl_serialize.rs")),
		type_name = type_name,
		type_generics_impl = type_generics_impl,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		fields_num = fields_num,
		fields = fields_string,
		serialize_type_name = serialize_type_name,
	)?;

	Ok(())
}
