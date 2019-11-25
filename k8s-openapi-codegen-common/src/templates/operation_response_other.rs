pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	crate_root: &str,
	is_list: bool,
) -> Result<(), crate::Error> {
	let (
		ok_type_name,
		type_generics_where,
		response_type_generics_where,
	): (
		std::borrow::Cow<'_, str>,
		_,
		std::borrow::Cow<'_, str>,
	) =
		if is_list {
			(
				format!("{}::List<T>", crate_root).into(),
				format!(" where T: {}::ListableResource", crate_root),
				format!(" where T: serde::de::DeserializeOwned + {}::ListableResource", crate_root).into(),
			)
		}
		else {
			(
				"T".into(),
				String::new(),
				" where T: serde::de::DeserializeOwned".into(),
			)
		};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/operation_response_other.rs")),
		type_name = type_name,
		type_generics_where = type_generics_where,
		response_type_generics_where = response_type_generics_where,
		crate_root = crate_root,
		ok_type_name = ok_type_name,
	)?;

	Ok(())
}
