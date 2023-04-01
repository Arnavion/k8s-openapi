use crate::swagger20;

pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	generics: super::Generics<'_>,
	definition_path: &swagger20::DefinitionPath,
	definition: &swagger20::Schema,
	schema_feature: Option<&str>,
	map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
	let local = crate::map_namespace_local_to_string(map_namespace)?;

	let type_generics_impl = generics.type_part.map(|part| format!("<{part}>")).unwrap_or_default();
	let type_generics_type = generics.type_part.map(|part| format!("<{part}>")).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {part}")).unwrap_or_default();

	let cfg = schema_feature.map_or_else(String::new, |schema_feature| format!("#[cfg(feature = {schema_feature:?})]\n"));

	let mut schema = String::new();

	gen_schema(&mut schema, definition, &local, map_namespace, 1)?;

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/impl_schema.rs")),
		local = local,
		cfg = cfg,
		type_name = type_name,
		type_generics_impl = type_generics_impl,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		definition_path = &**definition_path,
		schema = schema,
	)?;

	Ok(())
}

fn gen_schema(
	out: &mut String,
	definition: &swagger20::Schema,
	local: &str,
	map_namespace: &impl crate::MapNamespace,
	depth: usize,
) -> Result<(), crate::Error> {
	use std::fmt::Write;

	let indent = "    ".repeat(depth + 1);

	writeln!(out, "{indent}{local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{")?;

	if let Some(description) = &definition.description {
		writeln!(out, "{indent}    metadata: Some(Box::new({local}schemars::schema::Metadata {{")?;
		writeln!(out, "{indent}        description: Some({description:?}.to_owned()),")?;
		writeln!(out, "{indent}        ..Default::default()")?;
		writeln!(out, "{indent}    }})),")?;
	}

	match &definition.kind {
		swagger20::SchemaKind::Properties(properties) => {
			writeln!(out,
				"{indent}    instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Object))),")?;
			writeln!(out, "{indent}    object: Some(Box::new({local}schemars::schema::ObjectValidation {{")?;

			let mut required_props = String::new();
			let mut props = String::new();
			for (name, (schema, required)) in properties {
				if *required {
					writeln!(required_props, "{indent}            {:?}.to_owned(),", &**name)?;
				}

				match &schema.kind {
					swagger20::SchemaKind::Properties(_) => unreachable!("unexpected nested properties"),

					swagger20::SchemaKind::Ref(ref_path) => {
						writeln!(props, "{indent}            (")?;
						writeln!(props, "{indent}                {:?}.to_owned(),", &**name)?;

						let type_name = crate::get_fully_qualified_type_name(ref_path, map_namespace);

						if let Some(description) = &schema.description {
							// Override the subschema's description
							writeln!(props, "{indent}                {{")?;
							writeln!(props, "{indent}                    let mut schema_obj = __gen.subschema_for::<{type_name}>().into_object();")?;
							writeln!(props, "{indent}                    schema_obj.metadata = Some(Box::new({local}schemars::schema::Metadata {{")?;
							writeln!(props, "{indent}                        description: Some({description:?}.to_owned()),")?;
							writeln!(props, "{indent}                        ..Default::default()")?;
							writeln!(props, "{indent}                    }}));")?;
							writeln!(props, "{indent}                    {local}schemars::schema::Schema::Object(schema_obj)")?;
							writeln!(props, "{indent}                }},")?;
						}
						else {
							writeln!(props, "{indent}                __gen.subschema_for::<{type_name}>(),")?;
						}

						writeln!(props, "{indent}            ),")?;
					},

					swagger20::SchemaKind::Ty(ty) => {
						writeln!(props, "{indent}            (")?;
						writeln!(props, "{indent}                {:?}.to_owned(),", &**name)?;
						gen_type_as_schema_object(&mut props, ty, schema.description.as_deref(), local, map_namespace, depth)?;
						writeln!(props, "{indent}            ),")?;
					},
				}
			}

			if !props.is_empty() {
				writeln!(out, "{indent}        properties: [")?;
				write!(out, "{props}")?;
				writeln!(out, "{indent}        ].into(),")?;
			}

			if !required_props.is_empty() {
				writeln!(out, "{indent}        required: [")?;
				write!(out, "{required_props}")?;
				writeln!(out, "{indent}        ].into(),")?;
			}

			writeln!(out, "{indent}        ..Default::default()")?;
			writeln!(out, "{indent}    }})),")?;
		},

		swagger20::SchemaKind::Ty(ty) =>
			gen_type(out, ty, local, map_namespace, depth + 1)?,

		swagger20::SchemaKind::Ref(ref_path) => unreachable!("unexpected $ref {ref_path:?}"),
	}

	writeln!(out, "{indent}    ..Default::default()")?;
	writeln!(out, "{indent}}})")?;

	Ok(())
}

fn gen_type_as_schema_object(
	out: &mut String,
	ty: &swagger20::Type,
	description_override: Option<&str>,
	local: &str,
	map_namespace: &impl crate::MapNamespace,
	depth: usize,
) -> Result<(), crate::Error> {
	use std::fmt::Write;

	let indent = "    ".repeat(depth + 1);

	if let swagger20::Type::CustomResourceSubresources(v) = ty {
		let json_schema_props_type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
			path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{v}.CustomResourceSubresources"),
			can_be_default: None,
		}, map_namespace);
		writeln!(out, "{indent}                __gen.subschema_for::<{json_schema_props_type_name}>(),")?;
	}
	else {
		writeln!(out, "{indent}                {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{")?;
		if let Some(description) = description_override {
			writeln!(out, "{indent}                    metadata: Some(Box::new({local}schemars::schema::Metadata {{")?;
			writeln!(out, "{indent}                        description: Some({description:?}.to_owned()),")?;
			writeln!(out, "{indent}                        ..Default::default()")?;
			writeln!(out, "{indent}                    }})),")?;
		}

		gen_type(out, ty, local, map_namespace, depth + 5)?;

		writeln!(out, "{indent}                    ..Default::default()")?;
		writeln!(out, "{indent}                }}),")?;
	}

	Ok(())
}

fn gen_type(
	out: &mut String,
	ty: &swagger20::Type,
	local: &str,
	map_namespace: &impl crate::MapNamespace,
	depth: usize,
) -> Result<(), crate::Error> {
	use std::fmt::Write;

	let indent = "    ".repeat(depth + 1);

	match ty {
		swagger20::Type::Any =>
			writeln!(out,
				"{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Object))),")?,

		swagger20::Type::JsonSchemaPropsOr(v, swagger20::JsonSchemaPropsOr::Array) => {
			let json_schema_props_type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
				path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{v}.JSONSchemaProps"),
				can_be_default: None,
			}, map_namespace);

			writeln!(out, "{indent}subschemas: Some(Box::new({local}schemars::schema::SubschemaValidation {{")?;
			writeln!(out, "{indent}    one_of: Some(vec![")?;

			writeln!(out, "{indent}        __gen.subschema_for::<{json_schema_props_type_name}>(),")?;

			writeln!(out, "{indent}        {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{")?;
			writeln!(out,
				"{indent}            instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Array))),")?;
			writeln!(out, "{indent}            array: Some(Box::new({local}schemars::schema::ArrayValidation {{")?;
			writeln!(out,
				"{indent}                items: Some({local}schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<{json_schema_props_type_name}>()))),")?;
			writeln!(out, "{indent}                ..Default::default()")?;
			writeln!(out, "{indent}            }})),")?;
			writeln!(out, "{indent}            ..Default::default()")?;
			writeln!(out, "{indent}        }}),")?;

			writeln!(out, "{indent}    ]),")?;
			writeln!(out, "{indent}    ..Default::default()")?;
			writeln!(out, "{indent}}})),")?;
		}

		swagger20::Type::JsonSchemaPropsOr(v, swagger20::JsonSchemaPropsOr::Bool) => {
			let json_schema_props_type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
				path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{v}.JSONSchemaProps"),
				can_be_default: None,
			}, map_namespace);

			writeln!(out, "{indent}subschemas: Some(Box::new({local}schemars::schema::SubschemaValidation {{")?;
			writeln!(out, "{indent}    one_of: Some(vec![")?;

			writeln!(out, "{indent}        __gen.subschema_for::<{json_schema_props_type_name}>(),")?;

			writeln!(out, "{indent}        {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{")?;
			writeln!(out,
				"{indent}            instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Boolean))),")?;
			writeln!(out, "{indent}            ..Default::default()")?;
			writeln!(out, "{indent}        }}),")?;

			writeln!(out, "{indent}    ]),")?;
			writeln!(out, "{indent}    ..Default::default()")?;
			writeln!(out, "{indent}}})),")?;
		}

		swagger20::Type::JsonSchemaPropsOr(v, swagger20::JsonSchemaPropsOr::StringArray) => {
			let json_schema_props_type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
				path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{v}.JSONSchemaProps"),
				can_be_default: None,
			}, map_namespace);

			writeln!(out, "{indent}subschemas: Some(Box::new({local}schemars::schema::SubschemaValidation {{")?;
			writeln!(out, "{indent}    one_of: Some(vec![")?;

			writeln!(out, "{indent}        __gen.subschema_for::<{json_schema_props_type_name}>(),")?;


			writeln!(out, "{indent}        {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{")?;
			writeln!(out,
				"{indent}            instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Array))),")?;
			writeln!(out, "{indent}            array: Some(Box::new({local}schemars::schema::ArrayValidation {{")?;

			writeln!(out, "{indent}                items: Some({local}schemars::schema::SingleOrVec::Single(Box::new(")?;
			writeln!(out, "{indent}                    {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{")?;
			writeln!(out,
				"{indent}                        instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::String))),")?;
			writeln!(out, "{indent}                        ..Default::default()")?;
			writeln!(out, "{indent}                    }}),")?;
			writeln!(out, "{indent}                ))),")?;

			writeln!(out, "{indent}                ..Default::default()")?;
			writeln!(out, "{indent}            }})),")?;
			writeln!(out, "{indent}            ..Default::default()")?;
			writeln!(out, "{indent}        }}),")?;


			writeln!(out, "{indent}    ]),")?;
			writeln!(out, "{indent}    ..Default::default()")?;
			writeln!(out, "{indent}}})),")?;
		}

		swagger20::Type::Array { items } => {
			writeln!(out,
				"{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Array))),")?;
			writeln!(out, "{indent}array: Some(Box::new({local}schemars::schema::ArrayValidation {{")?;
			if let swagger20::SchemaKind::Ref(ref_path) = &items.kind {
				writeln!(out,
					"{indent}    items: Some({local}schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<{}>()))),",
					crate::get_fully_qualified_type_name(ref_path, map_namespace))?;
			}
			else {
				writeln!(out, "{indent}    items: Some({local}schemars::schema::SingleOrVec::Single(Box::new(")?;
				gen_schema(out, items, local, map_namespace, depth + 2)?;
				writeln!(out, "{indent}    ))),")?;
			}
			writeln!(out, "{indent}    ..Default::default()")?;
			writeln!(out, "{indent}}})),")?;
		}

		swagger20::Type::Boolean => {
			writeln!(out,
				"{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Boolean))),")?;
		}

		swagger20::Type::Integer { format } => {
			writeln!(out,
				"{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Integer))),")?;
			let format = match format {
				swagger20::IntegerFormat::Int32 => {
					"int32"
				}
				swagger20::IntegerFormat::Int64 => {
					"int64"
				}
			};
			writeln!(out, "{indent}format: Some({format:?}.to_owned()),")?;
		}

		swagger20::Type::Number { format } => {
			writeln!(out,
				"{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Number))),")?;

			match format {
				swagger20::NumberFormat::Double => {
					writeln!(out, r#"{indent}format: Some("double".to_owned()),"#)?;
				}
			}
		}

		swagger20::Type::Object { additional_properties } => {
			writeln!(out,
				"{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Object))),")?;
			writeln!(out, "{indent}object: Some(Box::new({local}schemars::schema::ObjectValidation {{")?;
			if let swagger20::SchemaKind::Ref(ref_path) = &additional_properties.kind {
				writeln!(out,
					"{indent}    additional_properties: Some(Box::new(__gen.subschema_for::<{}>())),",
					crate::get_fully_qualified_type_name(ref_path, map_namespace))?;
			}
			else {
				writeln!(out, "{indent}    additional_properties: Some(Box::new(")?;
				gen_schema(out, additional_properties, local, map_namespace, depth + 2)?;
				writeln!(out, "{indent}    )),")?;
			}
			writeln!(out, "{indent}    ..Default::default()")?;
			writeln!(out, "{indent}}})),")?;
		}

		swagger20::Type::String { format } => {
			writeln!(out,
				"{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::String))),")?;
			match format {
				Some(swagger20::StringFormat::Byte) => {
					writeln!(out, r#"{indent}format: Some("byte".to_owned()),"#)?;
				},
				Some(swagger20::StringFormat::DateTime) => {
					writeln!(out, r#"{indent}format: Some("date-time".to_owned()),"#)?;
				},
				None => (),
			}
		}

		swagger20::Type::IntOrString => {
			writeln!(out,
				"{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::String))),")?;
			writeln!(out, r#"{indent}format: Some("int-or-string".to_owned()),"#)?;
		}

		swagger20::Type::Patch => {
			writeln!(out,
				"{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Object))),")?;
		}

		swagger20::Type::WatchEvent(ref_path) => {
			writeln!(out,
				"{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Object))),")?;
			writeln!(out, "{indent}object: Some(Box::new({local}schemars::schema::ObjectValidation {{")?;
			writeln!(out, "{indent}    properties: [")?;
			writeln!(out, "{indent}        (")?;
			writeln!(out, r#"{indent}            "object".to_owned(),"#)?;
			writeln!(out,  "{indent}            __gen.subschema_for::<{}>(),", crate::get_fully_qualified_type_name(ref_path, map_namespace))?;
			writeln!(out, "{indent}        ),")?;

			writeln!(out, "{indent}        (")?;
			writeln!(out, r#"{indent}            "type".to_owned(),"#)?;
			writeln!(out, "{indent}            {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{")?;
			writeln!(out,
				"{indent}                instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::String))),")?;
			writeln!(out, "{indent}                ..Default::default()")?;
			writeln!(out, "{indent}            }}),")?;
			writeln!(out, "{indent}        ),")?;
			writeln!(out, "{indent}    ].into(),")?;

			writeln!(out, "{indent}    required: [")?;
			writeln!(out, r#"{indent}        "object".to_owned(),"#)?;
			writeln!(out, r#"{indent}        "type".to_owned(),"#)?;
			writeln!(out, "{indent}    ].into(),")?;

			writeln!(out, "{indent}    ..Default::default()")?;
			writeln!(out, "{indent}}})),")?;
		}

		_ => unreachable!("cannot generate schema for type {ty:?}"),
	}

	Ok(())
}
