use crate::get_rust_ident;

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
		merge_body.push_str("        ");
		generate_field(
			&mut merge_body,
			&local,
			&format!("&mut self.{field_name}"),
			&format!("other.{field_name}"),
			merge_type,
		)?;
		merge_body.push_str(";\n");
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

fn generate_field(mut writer: &mut impl std::fmt::Write, local: &str, self_name: &str, other_name: &str, merge_type: &super::MergeType) -> Result<(), crate::Error> {
	match merge_type {
		super::MergeType::Default => write!(
			writer,
			"{local}DeepMerge::merge_from({self_name}, {other_name})",
		)?,
		super::MergeType::List {
			strategy: crate::swagger20::KubernetesListType::Atomic,
			keys: _,
			item_merge_type: _,
		} => write!(
			writer,
			"{local}merge_strategies::list::atomic({self_name}, {other_name})",
		)?,
		super::MergeType::List {
			strategy: crate::swagger20::KubernetesListType::Map,
			keys,
			item_merge_type,
		} => {
			write!(
				writer,
				"{local}merge_strategies::list::map({self_name}, {other_name}, &[{keys}], |inner_self, inner_other| {{",
				keys = keys.iter().map(|k| format!("|lhs, rhs| lhs.{k} == rhs.{k}", k = get_rust_ident(k))).collect::<Vec<_>>().join(", ")
			)?;
			generate_field(writer, local, "inner_self", "inner_other", item_merge_type)?;
			write!(writer, "}})")?;
		},
		super::MergeType::List {
			strategy: crate::swagger20::KubernetesListType::Set,
			keys: _,
			item_merge_type: _,
		} => write!(
			writer,
			"{local}merge_strategies::list::set({self_name}, {other_name})",
		)?,
		super::MergeType::Map {
			strategy: crate::swagger20::KubernetesMapType::Granular,
			value_merge_type,
		} => {
			write!(
				writer,
				"{local}merge_strategies::map::granular({self_name}, {other_name}, |inner_self, inner_other| ",
			)?;
			generate_field(writer, local, "inner_self", "inner_other", value_merge_type)?;
			write!(writer, ")")?;
		},
		super::MergeType::Map {
			strategy: crate::swagger20::KubernetesMapType::Atomic,
			value_merge_type: _,
		} => write!(
			&mut writer,
			"{local}merge_strategies::map::atomic({self_name}, {other_name})",
		)?,
	}

	Ok(())
}
