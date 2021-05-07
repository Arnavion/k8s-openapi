pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	or: Or,
	json_schema_props_type_name: &str,
	map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
	let local = crate::map_namespace_local_to_string(map_namespace)?;

	let or_variant_name = match or {
		Or::Array => "Schemas",
		Or::Bool => "Bool",
		Or::StringArray => "Strings",
	};

	let or_variant_type: std::borrow::Cow<'_, str> = match or {
		Or::Array => format!("Vec<{}>", json_schema_props_type_name).into(),
		Or::Bool => "bool".into(),
		Or::StringArray => "Vec<String>".into(),
	};

	let mut or_visit = String::new();
	match or {
		Or::Array => {
			use std::fmt::Write;

			writeln!(or_visit, "            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: {}serde::de::SeqAccess<'de> {{", local)?;
			writeln!(or_visit,
				"                Ok({}::Schemas({local}serde::de::Deserialize::deserialize({local}serde::de::value::SeqAccessDeserializer::new(seq))?))",
				type_name, local = local)?;
			writeln!(or_visit, "            }}")?;
		},

		Or::Bool => {
			use std::fmt::Write;

			writeln!(or_visit, "            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: {}serde::de::Error {{", local)?;
			writeln!(or_visit, "                Ok({}::Bool(v))", type_name)?;
			writeln!(or_visit, "            }}")?;
		},

		Or::StringArray => {
			use std::fmt::Write;

			writeln!(or_visit, "            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: {}serde::de::SeqAccess<'de> {{", local)?;
			writeln!(or_visit,
				"                Ok({}::Strings({local}serde::de::Deserialize::deserialize({local}serde::de::value::SeqAccessDeserializer::new(seq))?))",
				type_name, local = local)?;
			writeln!(or_visit, "            }}")?;
		},
	}

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/json_schema_props_or.rs")),
		local = local,
		type_name = type_name,
		json_schema_props_type_name = json_schema_props_type_name,
		or_variant_name = or_variant_name,
		or_variant_type = or_variant_type,
		or_visit = or_visit,
	)?;

	Ok(())
}

#[derive(Clone, Copy)]
pub(crate) enum Or {
	Array,
	Bool,
	StringArray,
}
