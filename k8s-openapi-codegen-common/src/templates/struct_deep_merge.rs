pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	generics: super::Generics<'_>,
	fields: &[super::Property<'_>],
	map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
	use std::fmt::Write;

	let local = crate::map_namespace_local_to_string(map_namespace)?;

	let type_generics_type = generics.type_part.map(|part| format!("<{part}>")).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {part}")).unwrap_or_default();

	let mut merge_body = String::new();
	for super::Property { field_name, merge_type, .. } in fields {
    match merge_type {
      super::MergeType::Default => writeln!(
			  &mut merge_body,
			  "        {local}DeepMerge::merge_from(&mut self.{field_name}, other.{field_name});",
		  )?,
      super::MergeType::List {
        strategy: crate::swagger20::KubernetesListType::Atomic,
        keys: _,
      } => writeln!(
			  &mut merge_body,
			  "        {local}merge_strategies::list::atomic(&mut self.{field_name}, other.{field_name});",
		  )?,
      super::MergeType::List {
        strategy: crate::swagger20::KubernetesListType::Map,
        keys,
      } => writeln!(
			  &mut merge_body,
			  "        {local}merge_strategies::list::map(&mut self.{field_name}, other.{field_name}, &[{keys}]);",
        keys = keys.iter().map(|k| format!("\"{k}\"")).collect::<Vec<_>>().join(",")
		  )?,
      super::MergeType::List {
        strategy: crate::swagger20::KubernetesListType::Set,
        keys: _,
      } => writeln!(
			  &mut merge_body,
			  "        {local}merge_strategies::list::set(&mut self.{field_name}, other.{field_name});",
		  )?,
      super::MergeType::Map { strategy: crate::swagger20::KubernetesMapType::Granular } => writeln!(
			  &mut merge_body,
			  "        {local}merge_strategies::map::granular(&mut self.{field_name}, other.{field_name});",
		  )?,
      super::MergeType::Map { strategy: crate::swagger20::KubernetesMapType::Atomic } => writeln!(
			  &mut merge_body,
			  "        {local}merge_strategies::map::atomic(&mut self.{field_name}, other.{field_name});",
		  )?,
    }
	}

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/struct_deep_merge.rs")),
		local = local,
		type_name = type_name,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		merge_body = merge_body,
	)?;

	Ok(())
}
