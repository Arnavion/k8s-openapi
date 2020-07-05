pub(crate) fn generate<M>(
	mut writer: impl std::io::Write,
	type_name: &str,
	generics: super::Generics<'_>,
	map_namespace: &M,
	resource_metadata: &super::ResourceMetadata<'_>,
) -> Result<(), crate::Error> where M: crate::MapNamespace {
	let local = crate::map_namespace_local_to_string(map_namespace)?;

	let type_generics_impl = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_type = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {}", part)).unwrap_or_default();

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/impl_resource.rs")),
		local = local,
		type_name = type_name,
		type_generics_impl = type_generics_impl,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		api_version = resource_metadata.api_version,
		group = resource_metadata.group,
		kind = resource_metadata.kind,
		version = resource_metadata.version,
	)?;

	Ok(())
}
