use crate::templates::{PropertyRequired, CollectionInfo, CollectionType};

pub(crate) fn generate(
	mut writer: impl std::io::Write,
	vis: &str,
	type_name: &str,
	generics: super::Generics<'_>,
	fields: &[super::Property<'_>],
) -> Result<(), crate::Error> {
	let type_generics_type = generics.type_part.map(|part| format!("<{part}>")).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {part}")).unwrap_or_default();

	let fields_body: String = {
		fields.iter()
		.try_fold(String::new(), |mut fields, super::Property { comment, field_name, field_type_name, .. }| -> Result<_, std::fmt::Error> {
			use std::fmt::Write;

			if !fields.is_empty() {
				writeln!(fields)?;
			}

			if let Some(comment) = comment {
				for line in crate::get_comment_text(comment, "") {
					writeln!(fields, "    ///{line}")?;
				}
			}

			writeln!(fields,
				"    {vis}{field_name}: {field_type_name},",
				vis = vis,
				field_name = field_name,
				field_type_name = field_type_name,
			)?;

			Ok(fields)
		})?
	};

	let methods_body : String = {
		fields.iter()
		.try_fold(String::new(), |mut fields, super::Property {
			field_name,
			field_type_name,
			field_inner_type_name,
			field_type_impl_default,
			required,
			collection_item_type, collection_item_impls_default, .. }| -> Result<_, std::fmt::Error> {
			use std::fmt::Write;

			if !fields.is_empty() {
				writeln!(fields)?;
			}

			// fields like `type_` no longer need this extra suffix
			let method_field_name = field_name.strip_suffix('_').unwrap_or(field_name);

			// basic setter
			if let PropertyRequired::Required{ is_default: _ } = required {
				writeln!(fields,
					include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_set.rs")),
					vis = vis,
					method_field_name = method_field_name,
					field_name = field_name,
					field_type_name = field_type_name,
					field_inner_type_name = field_inner_type_name,
				)?;
			} else {
				writeln!(fields,
					include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_opt_set.rs")),
					vis = vis,
					method_field_name = method_field_name,
					field_name = field_name,
					field_type_name = field_type_name,
				)?;
				if *field_type_impl_default {
					writeln!(fields,
						include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_opt.rs")),
						vis = vis,
						field_name = field_name,
						field_inner_type_name = field_inner_type_name,
					)?;
				}
			}

			if let PropertyRequired::Required{ is_default: _ } = required {
				writeln!(fields,
					include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_set_with.rs")),
					vis = vis,
					method_field_name = method_field_name,
					field_name = field_name,
					field_type_name = field_type_name,
				)?;
			} 
			else if *field_type_impl_default {
				writeln!(fields,
					include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_opt_set_with.rs")),
					vis = vis,
					method_field_name = method_field_name,
					field_name = field_name,
					field_inner_type_name = field_inner_type_name ,
				)?;
			}

			// {field}_insert_with for collections - insert new default, then let `func` modify it
			if collection_item_impls_default == &Some(true) {
				if let PropertyRequired::Required{ is_default: _ } = required {
					match collection_item_type {
						Some(CollectionInfo { type_: CollectionType::Vec, items_type, .. }) => {
							writeln!(fields,
								include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_push_with.rs")),
								vis = vis,
								method_field_name = method_field_name,
								field_name = field_name,
								type_name = items_type,
							)?;
						},
						Some(CollectionInfo { type_:CollectionType::Map, items_type: type_name, .. }) => {
							writeln!(fields,
								include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_insert_with.rs")),
								vis = vis,
								method_field_name = method_field_name,
								field_name = field_name,
								type_name = type_name,
							)?;
						}
						_ => {},
					}
				} else {
					match collection_item_type {
						Some(CollectionInfo { type_: CollectionType::Vec, items_type, .. }) => {
							writeln!(fields,
								include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_opt_push_with.rs")),
								vis = vis,
								method_field_name = method_field_name,
								field_name = field_name,
								type_name = items_type,
							)?;
						},
						Some(CollectionInfo { type_:CollectionType::Map, items_type, .. }) => {
							writeln!(fields,
								include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_opt_insert_with.rs")),
								vis = vis,
								method_field_name = method_field_name,
								field_name = field_name,
								type_name = items_type,
							)?;
						}
						_ => {},
					}
				}
			}

				if let PropertyRequired::Required{ is_default: _ } = required {
					match collection_item_type {
						Some(CollectionInfo { type_: CollectionType::Vec, items_type, .. }) => {
							writeln!(fields,
								include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_append_from.rs")),
								vis = vis,
								method_field_name = method_field_name,
								field_name = field_name,
								type_name = items_type,
							)?;
						},
						Some(CollectionInfo { type_:CollectionType::Map, items_type, .. }) => {
							writeln!(fields,
								include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_insert_from.rs")),
								vis = vis,
								method_field_name = method_field_name,
								field_name = field_name,
								type_name = items_type,
							)?;
						}
						_ => {},
					}
				} else {
					match collection_item_type {
						Some(CollectionInfo { type_: CollectionType::Vec, items_type, .. }) => {
							writeln!(fields,
								include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_opt_append_from.rs")),
								vis = vis,
								method_field_name = method_field_name,
								field_name = field_name,
								type_name = items_type,
							)?;
						},
						Some(CollectionInfo { type_:CollectionType::Map, items_type, .. }) => {
							writeln!(fields,
								include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/dsl_field_opt_insert_from.rs")),
								vis = vis,
								method_field_name = method_field_name,
								field_name = field_name,
								type_name = items_type,
							)?;
						}
						_ => {},
					}
				}

			Ok(fields)
		})?
	};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/struct.rs")),
		type_name = type_name,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		fields = fields_body,
		methods = methods_body,
	)?;

	Ok(())
}
