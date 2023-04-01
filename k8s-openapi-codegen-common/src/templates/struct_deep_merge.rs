pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	generics: super::Generics<'_>,
	fields: &[super::Property<'_>],
	map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
	let local = crate::map_namespace_local_to_string(map_namespace)?;

	let type_generics_type = generics.type_part.map(|part| format!("<{part}>")).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {part}")).unwrap_or_default();

	let mut merge_body = String::new();
	for super::Property { field_name, merge_type, .. } in fields {
		generate_field(
			&mut merge_body,
			&local,
			&format!("&mut self.{field_name}"),
			&format!("other.{field_name}"),
			merge_type,
			8,
		)?;
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

fn generate_field(
	writer: &mut impl std::fmt::Write,
	local: &str,
	self_name: &str,
	other_name: &str,
	merge_type: &crate::swagger20::MergeType,
	indent: usize,
) -> Result<(), crate::Error> {
	const S: &str = "";

	match merge_type {
		crate::swagger20::MergeType::Default => writeln!(
			writer,
			"{S:indent$}{local}DeepMerge::merge_from({self_name}, {other_name});",
		)?,

		crate::swagger20::MergeType::List {
			strategy: crate::swagger20::KubernetesListType::Atomic,
			keys: _,
			item_merge_type: _,
		} => writeln!(
			writer,
			"{S:indent$}{local}merge_strategies::list::atomic({self_name}, {other_name});",
		)?,

		crate::swagger20::MergeType::List {
			strategy: crate::swagger20::KubernetesListType::Map,
			keys,
			item_merge_type,
		} => {
			writeln!(writer, "{S:indent$}{local}merge_strategies::list::map(")?;
			writeln!(writer, "{S:indent$}    {self_name},")?;
			writeln!(writer, "{S:indent$}    {other_name},")?;
			writeln!(
				writer,
				"{S:indent$}    &[{keys}],",
				keys = keys.iter().map(|k| format!("|lhs, rhs| lhs.{k} == rhs.{k}", k = crate::get_rust_ident(k))).collect::<Vec<_>>().join(", "),
			)?;
			writeln!(writer, "{S:indent$}    |current_item, other_item| {{")?;
			generate_field(writer, local, "current_item", "other_item", item_merge_type, indent + 8)?;
			writeln!(writer, "{S:indent$}    }},")?;
			writeln!(writer, "{S:indent$});")?;
		},

		crate::swagger20::MergeType::List {
			strategy: crate::swagger20::KubernetesListType::Set,
			keys: _,
			item_merge_type: _,
		} => writeln!(
			writer,
			"{S:indent$}{local}merge_strategies::list::set({self_name}, {other_name});",
		)?,

		crate::swagger20::MergeType::Map {
			strategy: crate::swagger20::KubernetesMapType::Granular,
			value_merge_type,
		} => {
			writeln!(
				writer,
				"{S:indent$}{local}merge_strategies::map::granular({self_name}, {other_name}, |current_item, other_item| {{",
			)?;
			generate_field(writer, local, "current_item", "other_item", value_merge_type, indent + 4)?;
			writeln!(writer, "{S:indent$}}});")?;
		},

		crate::swagger20::MergeType::Map {
			strategy: crate::swagger20::KubernetesMapType::Atomic,
			value_merge_type: _,
		} => writeln!(
			writer,
			"{S:indent$}{local}merge_strategies::map::atomic({self_name}, {other_name});",
		)?,
	}

	Ok(())
}
