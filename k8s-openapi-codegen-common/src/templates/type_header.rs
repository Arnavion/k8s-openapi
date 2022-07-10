pub(crate) fn generate(
	mut writer: impl std::io::Write,
	definition_path: &crate::swagger20::DefinitionPath,
	type_comment: Option<&str>,
	type_feature: Option<&str>,
	derives: Option<Derives>,
	vis: &str,
) -> Result<(), crate::Error> {
	let type_comment: String =
		type_comment
		.map(|type_comment| crate::get_comment_text(type_comment, "").map(|line| format!("///{line}\n")).collect())
		.unwrap_or_default();

	let type_feature_attribute =
		type_feature
		.map(|type_feature| format!("#[cfg(feature = {type_feature:?})]\n"))
		.unwrap_or_default();

	let derives =
		derives
		.map(|Derives { clone, copy, default, eq, ord, partial_eq, partial_ord }| format!(
			"#[derive({clone}{copy}Debug{default}{eq}{ord}{partial_eq}{partial_ord})]\n",
			clone = if clone { "Clone, " } else { "" },
			copy = if copy { "Copy, " } else { "" },
			default = if default { ", Default" } else { "" },
			eq = if eq { ", Eq" } else { "" },
			ord = if ord { ", Ord" } else { "" },
			partial_eq = if partial_eq { ", PartialEq" } else { "" },
			partial_ord = if partial_ord { ", PartialOrd" } else { "" },
		))
		.unwrap_or_default();

	write!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/type_header.rs")),
		definition_path = definition_path,
		type_comment = type_comment,
		type_feature_attribute = type_feature_attribute,
		derives = derives,
		vis = vis,
	)?;

	Ok(())
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Derives {
	pub(crate) clone: bool,
	pub(crate) copy: bool,
	pub(crate) default: bool,
	pub(crate) eq: bool,
	pub(crate) ord: bool,
	pub(crate) partial_eq: bool,
	pub(crate) partial_ord: bool,
}
