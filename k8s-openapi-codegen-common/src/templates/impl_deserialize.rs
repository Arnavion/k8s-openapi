use crate::safe_field;

pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	generics: super::Generics<'_>,
	fields: &[super::Property<'_>],
	crate_root: String,
	resource_metadata: Option<&super::ResourceMetadata<'_>>,
) -> Result<(), crate::Error> {
	use std::fmt::Write;

	let type_generics_impl: std::borrow::Cow<'_, str> = match generics.type_part {
		Some(part) => format!("<'de, {}>", part).into(),
		None => "<'de>".into(),
	};
	let type_generics_type = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {}", part)).unwrap_or_default();

	let (visitor_field, visitor_create_field) =
		if type_generics_type.is_empty() {
			(String::new(), "")
		}
		else {
			(format!("(std::marker::PhantomData{})", type_generics_type), "(Default::default())")
		};

	let mut fields_string = String::new();
	let mut str_to_field_match_arms = String::new();
	let mut field_value_defs = String::new();
	let mut field_value_match_arms = String::new();
	let mut field_name_list = String::new();
	let mut field_value_assignment = String::new();

	let mut flattened_field = None;

	if resource_metadata.is_some() {
		writeln!(fields_string, "            Key_api_version,")?;
		writeln!(fields_string, "            Key_kind,")?;

		writeln!(str_to_field_match_arms, r#"                            "apiVersion" => Field::Key_api_version,"#)?;
		writeln!(str_to_field_match_arms, r#"                            "kind" => Field::Key_kind,"#)?;

		writeln!(field_value_match_arms, r#"                        Field::Key_api_version => {{"#)?;
		writeln!(field_value_match_arms, r#"                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;"#)?;
		writeln!(field_value_match_arms, r#"                            if value_api_version != <Self::Value as {}::Resource>::API_VERSION {{"#, crate_root)?;
		writeln!(field_value_match_arms, r#"                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as {}::Resource>::API_VERSION));"#, crate_root)?;
		writeln!(field_value_match_arms, r#"                            }}"#)?;
		writeln!(field_value_match_arms, r#"                        }},"#)?;

		writeln!(field_value_match_arms, r#"                        Field::Key_kind => {{"#)?;
		writeln!(field_value_match_arms, r#"                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;"#)?;
		writeln!(field_value_match_arms, r#"                            if value_kind != <Self::Value as {}::Resource>::KIND {{"#, crate_root)?;
		writeln!(field_value_match_arms, r#"                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as {}::Resource>::KIND));"#, crate_root)?;
		writeln!(field_value_match_arms, r#"                            }}"#)?;
		writeln!(field_value_match_arms, r#"                        }},"#)?;

		writeln!(field_name_list, r#"                "apiVersion","#)?;
		writeln!(field_name_list, r#"                "kind","#)?;
	}
	for super::Property { name, field_name, field_type_name, required, is_flattened, .. } in fields {
		if *is_flattened {
			if flattened_field.is_some() {
				return Err(format!("{} has two flattened fields", type_name).into());
			}

			flattened_field = Some((field_name, field_type_name));

			writeln!(field_value_defs,
				r#"                let mut value_{}: std::collections::BTreeMap<{}::serde_value::Value, {}::serde_value::Value> = Default::default();"#,
				field_name, crate_root, crate_root)?;
		}
		else {
			writeln!(fields_string, "            Key_{},", field_name)?;

			writeln!(str_to_field_match_arms, r#"                            {:?} => Field::Key_{},"#, name, field_name)?;

			if *required {
				writeln!(field_value_defs, r#"                let mut value_{}: Option<{}> = None;"#, field_name, field_type_name)?;

				writeln!(field_value_match_arms,
					r#"                        Field::Key_{} => value_{} = Some(serde::de::MapAccess::next_value(&mut map)?),"#,
					field_name, field_name)?;

				writeln!(field_value_assignment, "                    {}: value_{}.ok_or_else(|| serde::de::Error::missing_field({:?}))?,", safe_field(field_name), field_name, name)?;
			}
			else {
				writeln!(field_value_defs, r#"                let mut value_{}: {} = None;"#, field_name, field_type_name)?;

				writeln!(field_value_match_arms, r#"                        Field::Key_{} => value_{} = serde::de::MapAccess::next_value(&mut map)?,"#, field_name, field_name)?;

				writeln!(field_value_assignment, "                    {}: value_{},", safe_field(field_name), field_name)?;
			}

			writeln!(field_name_list, r#"                {:?},"#, name)?;
		}
	}

	if let Some((field_name, _)) = flattened_field {
		writeln!(fields_string, "            Other(String),")?;

		writeln!(str_to_field_match_arms, "                            v => Field::Other(v.to_owned()),")?;

		writeln!(field_value_match_arms,
			"                        Field::Other(key) => {{ value_{}.insert({}::serde_value::Value::String(key), serde::de::MapAccess::next_value(&mut map)?); }},",
			field_name, crate_root)?;

		writeln!(field_value_assignment,
			"                    {}: {{",
			safe_field(field_name))?;
		writeln!(field_value_assignment,
			"                        let value_{} = {}::serde_value::Value::Map(value_{});",
			field_name, crate_root, field_name)?;
		writeln!(field_value_assignment,
			"                        let value_{} = {}::serde_value::ValueDeserializer::new(value_{});",
			field_name, crate_root, field_name)?;
		writeln!(field_value_assignment,
			"                        let value_{} = serde::Deserialize::deserialize(value_{})?;",
			field_name, field_name)?;
		writeln!(field_value_assignment,
			"                        value_{}",
			field_name)?;
		writeln!(field_value_assignment,
			"                    }},")?;
	}
	else {
		writeln!(fields_string, "            Other,")?;

		writeln!(str_to_field_match_arms, "                            _ => Field::Other,")?;

		writeln!(field_value_match_arms,
			"                        Field::Other => {{ let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; }},")?;
	}

	let deserialize_type_name =
		if resource_metadata.is_some() {
			format!("<Self as {}::Resource>::KIND", crate_root)
		}
		else {
			format!("{:?}", type_name)
		};

	let visitor_expecting_type_name =
		if resource_metadata.is_some() {
			format!("<Self::Value as {}::Resource>::KIND", crate_root)
		}
		else {
			format!("{:?}", type_name)
		};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/impl_deserialize.rs")),
		type_name = type_name,
		type_generics_impl = type_generics_impl,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		fields = fields_string,
		str_to_field_match_arms = str_to_field_match_arms,
		field_value_defs = field_value_defs,
		field_value_match_arms = field_value_match_arms,
		field_value_assignment = field_value_assignment,
		field_name_list = field_name_list,
		visitor_field = visitor_field,
		visitor_create_field = visitor_create_field,
		deserialize_type_name = deserialize_type_name,
		visitor_expecting_type_name = visitor_expecting_type_name,
	)?;

	Ok(())
}
