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

	let type_generics_impl = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_type = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {}", part)).unwrap_or_default();

	let cfg = schema_feature.map_or_else(String::new, |schema_feature| format!("#[cfg(feature = \"{}\")]\n", schema_feature));

	let schema = gen_schema(definition, map_namespace, 1)?;

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
	definition: &swagger20::Schema,
	map_namespace: &impl crate::MapNamespace,
	depth: usize,
) -> Result<String, crate::Error> {
	use std::fmt::Write;

	let local = crate::map_namespace_local_to_string(map_namespace)?;
	let mut schema = String::new();
	let indent = "    ".repeat(depth + 1);
	writeln!(schema, "{indent}{local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{", indent = indent, local = local)?;
	if let Some(description) = &definition.description {
		writeln!(schema, "{indent}    metadata: Some(Box::new({local}schemars::schema::Metadata {{", indent = indent, local = local)?;
		writeln!(schema, "{indent}        description: Some({:?}.to_owned()),", description, indent = indent)?;
		writeln!(schema, "{indent}        ..Default::default()", indent = indent)?;
		writeln!(schema, "{indent}    }})),", indent = indent)?;
	}

	match &definition.kind {
		swagger20::SchemaKind::Properties(properties) => {
			writeln!(schema, "{indent}    instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Object))),", indent = indent, local = local)?;
			writeln!(schema, "{indent}    object: Some(Box::new({local}schemars::schema::ObjectValidation {{", indent = indent, local = local)?;

			let mut required_properties = String::new();
			let mut props = String::new();
			for (name, (schema, required)) in properties {
				if *required {
					writeln!(required_properties, "{indent}            {:?},", &**name, indent = indent)?;
				}

				match &schema.kind {
					swagger20::SchemaKind::Properties(_) => unreachable!("unexpected nested properties"),

					swagger20::SchemaKind::Ref(ref_path) => {
						writeln!(props, "{indent}            (", indent = indent)?;
						writeln!(props, "{indent}                {:?}.to_owned(),", &**name, indent = indent)?;
						let type_name = crate::get_fully_qualified_type_name(ref_path, map_namespace);
						if let Some(description) = &schema.description {
							// Override description
							writeln!(props, "{indent}                {{", indent = indent)?;
							writeln!(props, "{indent}                    let mut schema_obj = __gen.subschema_for::<{}>().into_object();", type_name, indent = indent)?;
							writeln!(props, "{indent}                    schema_obj.metadata = Some(Box::new({local}schemars::schema::Metadata {{", indent = indent, local = local)?;
							writeln!(props, "{indent}                        description: Some({:?}.to_owned()),", description, indent = indent)?;
							writeln!(props, "{indent}                        ..Default::default()", indent = indent)?;
							writeln!(props, "{indent}                    }}));", indent = indent)?;
							writeln!(props, "{indent}                    {local}schemars::schema::Schema::Object(schema_obj)", indent = indent, local = local)?;
							writeln!(props, "{indent}                }},", indent = indent)?;
						} else {
							writeln!(props, "{indent}                __gen.subschema_for::<{}>(),", type_name, indent = indent)?;
						}
						writeln!(props, "{indent}            ),", indent = indent)?;
					},

					swagger20::SchemaKind::Ty(ty) => {
						writeln!(props, "{indent}            (", indent = indent)?;
						writeln!(props, "{indent}                {:?}.to_owned(),", &**name, indent = indent)?;
						writeln!(props, "{indent}                {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{", indent = indent, local = local)?;
						if let Some(description) = &schema.description {
							writeln!(props, "{indent}                    metadata: Some(Box::new({local}schemars::schema::Metadata {{", indent = indent, local = local)?;
							writeln!(props, "{indent}                        description: Some({:?}.to_owned()),", description, indent = indent)?;
							writeln!(props, "{indent}                        ..Default::default()", indent = indent)?;
							writeln!(props, "{indent}                    }})),", indent = indent)?;
						}
						if let Some(s) = gen_type(ty, map_namespace, depth + 5)? {
							write!(props, "{}", s)?;
						}
						writeln!(props, "{indent}                    ..Default::default()", indent = indent)?;
						writeln!(props, "{indent}                }}),", indent = indent)?;
						writeln!(props, "{indent}            ),", indent = indent)?;
					},
				}
			}

			if !props.is_empty() {
				writeln!(schema, "{indent}        properties: std::array::IntoIter::new([", indent = indent)?;
				write!(schema, "{}", props)?;
				writeln!(schema, "{indent}        ]).collect(),", indent = indent)?;
			}

			if !required_properties.is_empty() {
				writeln!(schema, "{indent}        required: std::array::IntoIter::new([", indent = indent)?;
				write!(schema, "{}", required_properties)?;
				writeln!(schema, "{indent}        ]).map(std::borrow::ToOwned::to_owned).collect(),", indent = indent)?;
			}

			writeln!(schema, "{indent}        ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}    }})),", indent = indent)?;
		},

		swagger20::SchemaKind::Ty(ty) => {
			if let Some(s) = gen_type(ty, map_namespace, depth + 1)? {
				write!(schema, "{}", s)?;
			}
		},

		swagger20::SchemaKind::Ref(ref_path) => unreachable!("unexpected $ref {:?}", ref_path),
	}

	writeln!(schema, "{indent}    ..Default::default()", indent = indent)?;
	writeln!(schema, "{indent}}})", indent = indent)?;
	Ok(schema)
}


fn gen_type(
	ty: &swagger20::Type,
	map_namespace: &impl crate::MapNamespace,
	depth: usize,
) -> Result<Option<String>, crate::Error> {
	use std::fmt::Write;

	let local = crate::map_namespace_local_to_string(map_namespace)?;
	let mut schema = String::new();
	let indent = "    ".repeat(depth + 1);
	match ty {
		swagger20::Type::Any => return Ok(None),

		swagger20::Type::JsonSchemaPropsOrArray(v) => {
			let type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
				path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{}.JSONSchemaProps", v),
				can_be_default: None,
			}, map_namespace);

			writeln!(schema, "{indent}subschemas: Some(Box::new({local}schemars::schema::SubschemaValidation {{", indent = indent, local = local)?;
			writeln!(schema, "{indent}    one_of: Some(vec![", indent = indent)?;

			writeln!(schema, "{indent}        __gen.subschema_for::<{}>(),", type_name, indent = indent)?;

			writeln!(schema, "{indent}        {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{", indent = indent, local = local)?;
			writeln!(schema, "{indent}            instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Array))),", indent = indent, local = local)?;
			writeln!(schema, "{indent}            array: Some(Box::new({local}schemars::schema::ArrayValidation {{", indent = indent, local = local)?;
			writeln!(schema, "{indent}                items: Some({local}schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<{}>()))),", type_name, indent = indent, local = local)?;
			writeln!(schema, "{indent}                ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}            }})),", indent = indent)?;
			writeln!(schema, "{indent}            ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}        }}),", indent = indent)?;

			writeln!(schema, "{indent}    ]),", indent = indent)?;
			writeln!(schema, "{indent}    ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}}})),", indent = indent)?;
		}

		swagger20::Type::JsonSchemaPropsOrBool(v) => {
			let type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
				path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{}.JSONSchemaProps", v),
				can_be_default: None,
			}, map_namespace);

			writeln!(schema, "{indent}subschemas: Some(Box::new({local}schemars::schema::SubschemaValidation {{", indent = indent, local = local)?;
			writeln!(schema, "{indent}    one_of: Some(vec![", indent = indent)?;

			writeln!(schema, "{indent}        __gen.subschema_for::<{}>(),", type_name, indent = indent)?;

			writeln!(schema, "{indent}        {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{", indent = indent, local = local)?;
			writeln!(schema, "{indent}            instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Boolean))),", indent = indent, local = local)?;
			writeln!(schema, "{indent}            ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}        }}),", indent = indent)?;

			writeln!(schema, "{indent}    ]),", indent = indent)?;
			writeln!(schema, "{indent}    ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}}})),", indent = indent)?;
		}

		swagger20::Type::JsonSchemaPropsOrStringArray(v) => {
			let type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
				path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{}.JSONSchemaProps", v),
				can_be_default: None,
			}, map_namespace);

			writeln!(schema, "{indent}subschemas: Some(Box::new({local}schemars::schema::SubschemaValidation {{", indent = indent, local = local)?;
			writeln!(schema, "{indent}    one_of: Some(vec![", indent = indent)?;

			writeln!(schema, "{indent}        __gen.subschema_for::<{}>(),", type_name, indent = indent)?;


			writeln!(schema, "{indent}        {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{", indent = indent, local = local)?;
			writeln!(schema, "{indent}            instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Array))),", indent = indent, local = local)?;
			writeln!(schema, "{indent}            array: Some(Box::new({local}schemars::schema::ArrayValidation {{", indent = indent, local = local)?;

			writeln!(schema, "{indent}                items: Some({local}schemars::schema::SingleOrVec::Single(Box::new(", indent = indent, local = local)?;
			writeln!(schema, "{indent}                    {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{", indent = indent, local = local)?;
			writeln!(schema, "{indent}                        instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::String))),", indent = indent, local = local)?;
			writeln!(schema, "{indent}                        ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}                    }}),", indent = indent)?;
			writeln!(schema, "{indent}                ))),", indent = indent)?;

			writeln!(schema, "{indent}                ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}            }})),", indent = indent)?;
			writeln!(schema, "{indent}            ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}        }}),", indent = indent)?;


			writeln!(schema, "{indent}    ]),", indent = indent)?;
			writeln!(schema, "{indent}    ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}}})),", indent = indent)?;
		}

		swagger20::Type::Array { items } => {
			writeln!(schema, "{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Array))),", indent = indent, local = local)?;
			writeln!(schema, "{indent}array: Some(Box::new({local}schemars::schema::ArrayValidation {{", indent = indent, local = local)?;
			if let swagger20::SchemaKind::Ref(ref_path) = &items.kind {
				writeln!(schema, "{indent}    items: Some({local}schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<{}>()))),", crate::get_fully_qualified_type_name(ref_path, map_namespace), indent = indent, local = local)?;
			} else {
				writeln!(schema, "{indent}    items: Some({local}schemars::schema::SingleOrVec::Single(Box::new(", indent = indent, local = local)?;
				write!(schema, "{}", gen_schema(items, map_namespace, depth + 2)?)?;
				writeln!(schema, "{indent}    ))),", indent = indent)?;
			}
			writeln!(schema, "{indent}    ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}}})),", indent = indent)?;
		}

		swagger20::Type::Boolean => {
			writeln!(schema, "{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Boolean))),", indent = indent, local = local)?;
		}

		swagger20::Type::Integer { format } => {
			writeln!(schema, "{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Integer))),", indent = indent, local = local)?;
			let format = match format {
				swagger20::IntegerFormat::Int32 => {
					"int32"
				}
				swagger20::IntegerFormat::Int64 => {
					"int64"
				}
			};
			writeln!(schema, "{indent}format: Some({:?}.to_owned()),", format, indent = indent)?;
		}

		swagger20::Type::Number { format } => {
			writeln!(schema, "{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Number))),", indent = indent, local = local)?;

			match format {
				swagger20::NumberFormat::Double => {
					writeln!(schema, r#"{indent}format: Some("double".to_owned()),"#, indent = indent)?;
				}
			}
		}

		swagger20::Type::Object { additional_properties } => {
			writeln!(schema, "{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Object))),", indent = indent, local = local)?;
			writeln!(schema, "{indent}object: Some(Box::new({local}schemars::schema::ObjectValidation {{", indent = indent, local = local)?;
			if let swagger20::SchemaKind::Ref(ref_path) = &additional_properties.kind {
				writeln!(schema, "{indent}    additional_properties: Some(Box::new(__gen.subschema_for::<{}>())),", crate::get_fully_qualified_type_name(ref_path, map_namespace), indent = indent)?;
			} else {
				writeln!(schema, "{indent}    additional_properties: Some(Box::new(", indent = indent)?;
				write!(schema, "{}", gen_schema(additional_properties, map_namespace, depth + 2)?)?;
				writeln!(schema, "{indent}    )),", indent = indent)?;
			}
			writeln!(schema, "{indent}    ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}}})),", indent = indent)?;
		}

		swagger20::Type::String { format } => {
			writeln!(schema, "{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::String))),", indent = indent, local = local)?;
			match format {
				Some(swagger20::StringFormat::Byte) => {
					writeln!(schema, r#"{indent}format: Some("byte".to_owned()),"#, indent = indent)?;
				},
				Some(swagger20::StringFormat::DateTime) => {
					writeln!(schema, r#"{indent}format: Some("date-time".to_owned()),"#, indent = indent)?;
				},
				None => (),
			}
		}

		swagger20::Type::IntOrString => {
			writeln!(schema, "{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::String))),", indent = indent, local = local)?;
			writeln!(schema, r#"{indent}format: Some("int-or-string".to_owned()),"#, indent = indent)?;
		}

		swagger20::Type::Patch => {
			writeln!(schema, "{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Object))),", indent = indent, local = local)?;
		}

		swagger20::Type::WatchEvent(ref_path) => {
			writeln!(schema, "{indent}instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::Object))),", indent = indent, local = local)?;
			writeln!(schema, "{indent}object: Some(Box::new({local}schemars::schema::ObjectValidation {{", indent = indent, local = local)?;
			writeln!(schema, "{indent}    properties: std::array::IntoIter::new([", indent = indent)?;
			writeln!(schema, "{indent}        (", indent = indent)?;
			writeln!(schema, r#"{indent}            "object".to_owned(),"#, indent = indent)?;
			writeln!(schema,  "{indent}            __gen.subschema_for::<{}>(),", crate::get_fully_qualified_type_name(ref_path, map_namespace), indent = indent)?;
			writeln!(schema, "{indent}        ),", indent = indent)?;

			writeln!(schema, "{indent}        (", indent = indent)?;
			writeln!(schema, r#"{indent}            "type".to_owned(),"#, indent = indent)?;
			writeln!(schema, "{indent}            {local}schemars::schema::Schema::Object({local}schemars::schema::SchemaObject {{", indent = indent, local = local)?;
			writeln!(schema, "{indent}                instance_type: Some({local}schemars::schema::SingleOrVec::Single(Box::new({local}schemars::schema::InstanceType::String))),", indent = indent, local = local)?;
			writeln!(schema, "{indent}                ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}            }}),", indent = indent)?;
			writeln!(schema, "{indent}        ),", indent = indent)?;
			writeln!(schema, "{indent}    ]).collect(),", indent = indent)?;

			writeln!(schema, "{indent}    required: std::array::IntoIter::new([", indent = indent)?;
			writeln!(schema, r#"{indent}        "object","#, indent = indent)?;
			writeln!(schema, r#"{indent}        "type","#, indent = indent)?;
			writeln!(schema, "{indent}    ]).map(std::borrow::ToOwned::to_owned).collect(),", indent = indent)?;

			writeln!(schema, "{indent}    ..Default::default()", indent = indent)?;
			writeln!(schema, "{indent}}})),", indent = indent)?;
		}

		_ => unreachable!("{:?}", ty),
	}
	Ok(Some(schema))
}
