pub(crate) fn generate(
	mut writer: impl std::io::Write,
	definition_path: &crate::swagger20::DefinitionPath,
	type_comment: Option<&str>,
	attrs: &str,
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
		if let Some(Derives { clone, copy, default, eq, ord, partial_eq, partial_ord }) = derives {
			format!(
				"#[derive({clone}{copy}Debug{default}{eq}{ord}{partial_eq}{partial_ord})]\n",
				clone = if clone { "Clone, " } else { "" },
				copy = if copy { "Copy, " } else { "" },
				default = if default { ", Default" } else { "" },
				eq = if eq { ", Eq" } else { "" },
				ord = if ord { ", Ord" } else { "" },
				partial_eq = if partial_eq { ", PartialEq" } else { "" },
				partial_ord = if partial_ord { ", PartialOrd" } else { "" },
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
		attrs = attrs,
		derives = derives,
		vis = vis,
	)?;

	Ok(())
}

#[derive(Clone, Copy)]
pub(crate) struct Derives {
	pub(crate) clone: bool,
	pub(crate) copy: bool,
	pub(crate) default: bool,
	pub(crate) eq: bool,
	pub(crate) ord: bool,
	pub(crate) partial_eq: bool,
	pub(crate) partial_ord: bool,
}
