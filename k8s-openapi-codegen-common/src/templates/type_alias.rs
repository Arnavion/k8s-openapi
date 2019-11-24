pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	alias_type_name: &str,
) -> Result<(), crate::Error> {
	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/type_alias.rs")),
		type_name = type_name,
		alias_type_name = alias_type_name,
	)?;

	Ok(())
}
