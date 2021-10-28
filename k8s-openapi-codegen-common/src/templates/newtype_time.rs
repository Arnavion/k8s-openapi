pub(crate) fn generate(
	mut writer: impl std::io::Write,
	vis: &str,
	type_name: &str,
	inner_type_name: &str,
	datetime_serialization_format: super::DateTimeSerializationFormat,
	map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
	let local = crate::map_namespace_local_to_string(map_namespace)?;

	let serialize_format = match datetime_serialization_format {
		super::DateTimeSerializationFormat::SixDecimalDigits => "RFC3339_SUBSECONDS_SIX",
		super::DateTimeSerializationFormat::ZeroDecimalDigits => "RFC3339_SUBSECONDS_ZERO",
	};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/newtype_time.rs")),
		local = local,
		type_name = type_name,
		vis = vis,
		inner_type_name = inner_type_name,
		serialize_format = serialize_format,
	)?;

	Ok(())
}
