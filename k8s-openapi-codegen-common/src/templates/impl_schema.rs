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

    gen_schema(&mut schema, definition, &local, map_namespace, 2)?;

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

    let indent = "    ".repeat(depth);

    writeln!(out, "{{")?;

    if let Some(description) = &definition.description {
        writeln!(out, r#"{indent}    "description": {description:?},"#)?;
    }

    match &definition.kind {
        swagger20::SchemaKind::Properties(properties) => {
            writeln!(out, r#"{indent}    "type": "object","#)?;
            writeln!(out, r#"{indent}    "properties": {{"#)?;

            let mut required_props = String::new();
            let mut props = String::new();
            for (name, (schema, required)) in properties {
                if *required {
                    writeln!(required_props, "{indent}        {:?},", &**name)?;
                }

                match &schema.kind {
                    swagger20::SchemaKind::Properties(_) => unreachable!("unexpected nested properties"),

                    swagger20::SchemaKind::Ref(ref_path) => {
                        write!(props, "{indent}        {:?}: ", &**name)?;

                        let type_name = crate::get_fully_qualified_type_name(ref_path, map_namespace);

                        if let Some(description) = &schema.description {
                            // Override the subschema's description
                            writeln!(props, "({{")?;
                            writeln!(props, "{indent}            let mut schema_obj = __gen.subschema_for::<{type_name}>();")?;
                            writeln!(props, r#"{indent}            schema_obj.ensure_object().insert("description".into(), {description:?}.into());"#)?;
                            writeln!(props, "{indent}            schema_obj")?;
                            writeln!(props, "{indent}        }}),")?;
                        }
                        else {
                            writeln!(props, "__gen.subschema_for::<{type_name}>(),")?;
                        }
                    },

                    swagger20::SchemaKind::Ty(ty) => {
                        write!(props, "{indent}        {:?}: ", &**name)?;
                        gen_type_as_schema_object(&mut props, ty, schema.description.as_deref(), local, map_namespace, depth + 2)?;
                    },
                }
            }

            write!(out, "{props}")?;

            writeln!(out, "{indent}    }},")?;

            if !required_props.is_empty() {
                writeln!(out, r#"{indent}    "required": ["#)?;
                write!(out, "{required_props}")?;
                writeln!(out, "{indent}    ],")?;
            }
        },

        swagger20::SchemaKind::Ty(ty) =>
            gen_type(out, ty, local, map_namespace, depth + 1)?,

        swagger20::SchemaKind::Ref(ref_path) => unreachable!("unexpected $ref {ref_path:?}"),
    }

    write!(out, "{indent}}}")?;

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

    let indent = "    ".repeat(depth);

    if let swagger20::Type::CustomResourceSubresources(v) = ty {
        let json_schema_props_type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
            path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{v}.CustomResourceSubresources"),
            can_be_default: None,
        }, map_namespace);
        writeln!(out, "(__gen.subschema_for::<{json_schema_props_type_name}>()),")?;
    }
    else {
        writeln!(out, "{{")?;
        if let Some(description) = description_override {
            writeln!(out, r#"{indent}    "description": {description:?},"#)?;
        }

        gen_type(out, ty, local, map_namespace, depth + 1)?;

        writeln!(out, "{indent}}},")?;
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

    let indent = "    ".repeat(depth);

    match ty {
        swagger20::Type::Any =>
            writeln!(out,
                r#"{indent}"type": "object","#)?,

        swagger20::Type::JsonSchemaPropsOr(v, swagger20::JsonSchemaPropsOr::Array) => {
            let json_schema_props_type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
                path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{v}.JSONSchemaProps"),
                can_be_default: None,
            }, map_namespace);

            writeln!(out, r#"{indent}"oneOf": ["#)?;
            writeln!(out, "{indent}    (__gen.subschema_for::<{json_schema_props_type_name}>()),")?;
            writeln!(out, "{indent}    {{")?;
            writeln!(out, r#"{indent}        "type": "array","#)?;
            writeln!(out, r#"{indent}        "items": (__gen.subschema_for::<{json_schema_props_type_name}>()),"#)?;
            writeln!(out, "{indent}    }},")?;

            writeln!(out, "{indent}],")?;
        }

        swagger20::Type::JsonSchemaPropsOr(v, swagger20::JsonSchemaPropsOr::Bool) => {
            let json_schema_props_type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
                path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{v}.JSONSchemaProps"),
                can_be_default: None,
            }, map_namespace);

            writeln!(out, r#"{indent}"oneOf": ["#)?;
            writeln!(out, "{indent}    (__gen.subschema_for::<{json_schema_props_type_name}>()),")?;
            writeln!(out, "{indent}    {{")?;
            writeln!(out, r#"{indent}        "type": "boolean","#)?;
            writeln!(out, "{indent}    }},")?;

            writeln!(out, "{indent}],")?;
        }

        swagger20::Type::JsonSchemaPropsOr(v, swagger20::JsonSchemaPropsOr::StringArray) => {
            let json_schema_props_type_name = crate::get_fully_qualified_type_name(&swagger20::RefPath {
                path: format!("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.{v}.JSONSchemaProps"),
                can_be_default: None,
            }, map_namespace);

            writeln!(out, r#"{indent}"oneOf": ["#)?;
            writeln!(out, "{indent}    (__gen.subschema_for::<{json_schema_props_type_name}>()),")?;
            writeln!(out, "{indent}    {{")?;
            writeln!(out, r#"{indent}        "type": "array","#)?;
            writeln!(out, r#"{indent}        "items": {{"#)?;
            writeln!(out, r#"{indent}            "type": "string","#)?;
            writeln!(out, "{indent}        }},")?;
            writeln!(out, "{indent}    }},")?;
            writeln!(out, "{indent}],")?;
        }

        swagger20::Type::Array { items } => {
            writeln!(out, r#"{indent}"type": "array","#)?;
            if let swagger20::SchemaKind::Ref(ref_path) = &items.kind {
                writeln!(out,
                    r#"{indent}"items": (__gen.subschema_for::<{}>()),"#,
                    crate::get_fully_qualified_type_name(ref_path, map_namespace))?;
            }
            else {
                write!(out, r#"{indent}"items": "#)?;
                gen_schema(out, items, local, map_namespace, depth)?;
                writeln!(out, ",")?;
            }
        }

        swagger20::Type::Boolean => writeln!(out, r#"{indent}"type": "boolean","#)?,

        swagger20::Type::Integer { format } => {
            writeln!(out, r#"{indent}"type": "integer","#)?;
            let format = match format {
                swagger20::IntegerFormat::Int32 => {
                    "int32"
                }
                swagger20::IntegerFormat::Int64 => {
                    "int64"
                }
            };
            writeln!(out, r#"{indent}"format": {format:?},"#)?;
        }

        swagger20::Type::Number { format } => {
            writeln!(out, r#"{indent}"type": "number","#)?;

            match format {
                swagger20::NumberFormat::Double => writeln!(out, r#"{indent}"format": "double","#)?,
            }
        }

        swagger20::Type::Object { additional_properties } => {
            writeln!(out, r#"{indent}"type": "object","#)?;
            if let swagger20::SchemaKind::Ref(ref_path) = &additional_properties.kind {
                writeln!(out,
                    r#"{indent}"additionalProperties": (__gen.subschema_for::<{}>()),"#,
                    crate::get_fully_qualified_type_name(ref_path, map_namespace))?;
            }
            else {
                write!(out, r#"{indent}"additionalProperties": "#)?;
                gen_schema(out, additional_properties, local, map_namespace, depth)?;
                writeln!(out, ",")?;
            }
        }

        swagger20::Type::String { format } => {
            writeln!(out, r#"{indent}"type": "string","#)?;
            match format {
                Some(swagger20::StringFormat::Byte) => writeln!(out, r#"{indent}"format": "byte","#)?,
                Some(swagger20::StringFormat::DateTime) => writeln!(out, r#"{indent}"format": "date-time","#)?,
                None => (),
            }
        }

        swagger20::Type::IntOrString => {
            writeln!(out, r#"{indent}"x-kubernetes-int-or-string": true,"#)?;
        }

        swagger20::Type::Patch => writeln!(out, r#"{indent}"type": "object","#)?,

        swagger20::Type::WatchEvent(ref_path) => {
            writeln!(out, r#"{indent}"type": "object","#)?;
            writeln!(out, r#"{indent}"properties": {{"#)?;
            writeln!(out, r#"{indent}    "object": (__gen.subschema_for::<{}>()),"#, crate::get_fully_qualified_type_name(ref_path, map_namespace))?;
            writeln!(out, r#"{indent}    "type": {{"#)?;
            writeln!(out, r#"{indent}        "type": "string","#)?;
            writeln!(out, "{indent}    }},")?;
            writeln!(out, "{indent}}},")?;

            writeln!(out, r#"{indent}"required": ["#)?;
            writeln!(out, r#"{indent}    "object","#)?;
            writeln!(out, r#"{indent}    "type","#)?;
            writeln!(out, "{indent}],")?;
        }

        _ => unreachable!("cannot generate schema for type {ty:?}"),
    }

    Ok(())
}
