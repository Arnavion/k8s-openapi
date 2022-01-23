pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	error_status_rust_type: &str,
	error_other_rust_type: &str,
	map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
	let local = crate::map_namespace_local_to_string(map_namespace)?;

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/watch_event.rs")),
		local = local,
		type_name = type_name,
		error_status_rust_type = error_status_rust_type,
		error_other_rust_type = error_other_rust_type,
	)?;

	Ok(())
}
