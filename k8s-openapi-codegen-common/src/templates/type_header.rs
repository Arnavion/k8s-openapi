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
		.map(|type_comment| super::get_comment_text(type_comment, "").map(|line| format!("///{}\n", line)).collect())
		.unwrap_or_default();

	let type_feature_attribute =
		type_feature
		.map(|type_feature| format!("#[cfg(feature = \"{}\")]\n", type_feature))
		.unwrap_or_default();

	let derives =
		derives
		.map(|Derives { clone, copy, default, eq, ord, partial_eq, partial_ord, json_schema }| format!(
			"{use_json_schema}#[derive({clone}{copy}Debug{default}{eq}{ord}{partial_eq}{partial_ord})]\n{json_schema}\n",
			clone = if clone { "Clone, " } else { "" },
			copy = if copy { "Copy, " } else { "" },
			default = if default { ", Default" } else { "" },
			eq = if eq { ", Eq" } else { "" },
			ord = if ord { ", Ord" } else { "" },
			partial_eq = if partial_eq { ", PartialEq" } else { "" },
			partial_ord = if partial_ord { ", PartialOrd" } else { "" },
			json_schema = if json_schema { "#[cfg_attr(feature = \"schema\", derive(JsonSchema), schemars(rename_all = \"camelCase\"))]" } else { "" },
            use_json_schema = if json_schema { "#[cfg(feature = \"schema\")]\nuse schemars::JsonSchema;\n" } else { "" },
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

#[derive(Clone, Copy)]
pub(crate) struct Derives {
	pub(crate) clone: bool,
	pub(crate) copy: bool,
	pub(crate) default: bool,
	pub(crate) eq: bool,
	pub(crate) ord: bool,
	pub(crate) partial_eq: bool,
	pub(crate) partial_ord: bool,
	pub(crate) json_schema: bool,
}
