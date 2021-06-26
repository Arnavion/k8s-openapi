pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	schema: &str,
) -> Result<(), crate::Error> {
	let lines = schema[2..schema.len()-1].lines().map(|s| format!("        {}", s)).collect::<Vec<_>>().join("\n");
	writeln!(writer)?;
	writeln!(writer, r#"#[cfg(feature = "schema")]"#)?;
	writeln!(writer, r#"impl {type_name} {{"#, type_name = type_name)?;
	writeln!(writer, r"    pub fn schema() -> serde_json::Value {{")?;
	writeln!(writer, r"        serde_json::json!({{")?;
	writeln!(writer, r"{}", lines)?;
	writeln!(writer, r"        }})")?;
	writeln!(writer, r"    }}")?;
	writeln!(writer, r"}}")?;
	Ok(())
}
