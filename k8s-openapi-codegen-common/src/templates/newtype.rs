pub(crate) fn generate(
	mut writer: impl std::io::Write,
	vis: &str,
	type_name: &str,
	inner_type_name: &str,
	map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
	let local = crate::map_namespace_local_to_string(map_namespace)?;

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/newtype.rs")),
		local = local,
		type_name = type_name,
		vis = vis,
		inner_type_name = inner_type_name,
	)?;

	Ok(())
}
