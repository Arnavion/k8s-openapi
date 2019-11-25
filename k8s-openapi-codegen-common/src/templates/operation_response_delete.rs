pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	crate_root: &str,
) -> Result<(), crate::Error> {
	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/operation_response_delete.rs")),
		type_name = type_name,
		crate_root = crate_root,
	)?;

	Ok(())
}
