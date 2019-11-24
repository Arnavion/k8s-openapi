pub(crate) fn generate(
	mut writer: impl std::io::Write,
	vis: &str,
	type_name: &str,
	inner_type_name: &str,
) -> Result<(), crate::Error> {
	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/newtype.rs")),
		type_name = type_name,
		vis = vis,
		inner_type_name = inner_type_name,
	)?;

	Ok(())
}
