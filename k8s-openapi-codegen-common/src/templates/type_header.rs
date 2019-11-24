pub(crate) fn generate(
	mut writer: impl std::io::Write,
	definition_path: &crate::swagger20::DefinitionPath,
	type_comment: Option<&str>,
	derives: Option<Derives>,
	vis: &str,
) -> Result<(), crate::Error> {
	let type_comment: String =
		if let Some(type_comment) = type_comment {
			super::get_comment_text(type_comment, "")
			.map(|line| format!("///{}\n", line))
			.collect()
		}
		else {
			String::new()
		};

	let derives: std::borrow::Cow<'_, str> =
		if let Some(Derives { copy, default, eq }) = derives {
			format!(
				"#[derive(Clone, {copy}Debug, {default}{eq}PartialEq)]\n",
				copy = if copy { "Copy, " } else { "" },
				default = if default { "Default, " } else { "" },
				eq = if eq { "Eq, " } else { "" },
			).into()
		}
		else {
			"".into()
		};

	write!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/type_header.rs")),
		definition_path = definition_path,
		type_comment = type_comment,
		derives = derives,
		vis = vis,
	)?;

	Ok(())
}

#[derive(Clone, Copy)]
pub(crate) struct Derives {
	pub(crate) copy: bool,
	pub(crate) default: bool,
	pub(crate) eq: bool,
}
