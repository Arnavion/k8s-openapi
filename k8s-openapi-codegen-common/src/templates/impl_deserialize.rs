pub(crate) fn generate(
    mut writer: impl std::io::Write,
    type_name: &str,
    generics: super::Generics<'_>,
    fields: &[super::Property<'_>],
    map_namespace: &impl crate::MapNamespace,
    resource_metadata: Option<&super::ResourceMetadata<'_>>,
) -> Result<(), crate::Error> {
    use std::fmt::Write;

    let local = crate::map_namespace_local_to_string(map_namespace)?;

    let type_generics_impl: std::borrow::Cow<'_, str> = match generics.type_part {
        Some(part) => format!("<'de, {part}>").into(),
        None => "<'de>".into(),
    };
    let type_generics_type = generics.type_part.map(|part| format!("<{part}>")).unwrap_or_default();
    let type_generics_where = generics.where_part.map(|part| format!(" where {part}")).unwrap_or_default();

    let (visitor_field, visitor_create_field) =
        if type_generics_type.is_empty() {
            (String::new(), "")
        }
        else {
            (format!("(core::marker::PhantomData{type_generics_type})"), "(Default::default())")
        };

    let mut fields_string = String::new();
    let mut str_to_field_match_arms = String::new();
    let mut field_value_defs = String::new();
    let mut field_value_match_arms = String::new();
    let mut field_name_list = String::new();
    let mut field_value_assignment = String::new();

    let mut flattened_field = None;

    if resource_metadata.is_some() {
        writeln!(fields_string, "            Key_api_version,")?;
        writeln!(fields_string, "            Key_kind,")?;

        writeln!(str_to_field_match_arms, r#"                            "apiVersion" => Field::Key_api_version,"#)?;
        writeln!(str_to_field_match_arms, r#"                            "kind" => Field::Key_kind,"#)?;

        writeln!(field_value_match_arms, r#"                        Field::Key_api_version => {{"#)?;
        writeln!(field_value_match_arms, r#"                            let value_api_version: std::string::String = {local}serde::de::MapAccess::next_value(&mut map)?;"#)?;
        writeln!(field_value_match_arms, r#"                            if value_api_version != <Self::Value as {local}Resource>::API_VERSION {{"#)?;
        writeln!(field_value_match_arms,
            r#"                                return Err({local}serde::de::Error::invalid_value({local}serde::de::Unexpected::Str(&value_api_version), &<Self::Value as {local}Resource>::API_VERSION));"#)?;
        writeln!(field_value_match_arms, r#"                            }}"#)?;
        writeln!(field_value_match_arms, r#"                        }},"#)?;

        writeln!(field_value_match_arms, r#"                        Field::Key_kind => {{"#)?;
        writeln!(field_value_match_arms, r#"                            let value_kind: std::string::String = {local}serde::de::MapAccess::next_value(&mut map)?;"#)?;
        writeln!(field_value_match_arms, r#"                            if value_kind != <Self::Value as {local}Resource>::KIND {{"#)?;
        writeln!(field_value_match_arms,
            r#"                                return Err({local}serde::de::Error::invalid_value({local}serde::de::Unexpected::Str(&value_kind), &<Self::Value as {local}Resource>::KIND));"#)?;
        writeln!(field_value_match_arms, r#"                            }}"#)?;
        writeln!(field_value_match_arms, r#"                        }},"#)?;

        writeln!(field_name_list, r#"                "apiVersion","#)?;
        writeln!(field_name_list, r#"                "kind","#)?;
    }
    for super::Property { name, field_name, field_type_name, required, is_flattened, .. } in fields {
        if *is_flattened {
            if flattened_field.is_some() {
                return Err(format!("{type_name} has two flattened fields").into());
            }

            flattened_field = Some((field_name, field_type_name));

            writeln!(field_value_defs,
                r#"                let mut value_{field_name}: {local}serde_json::Map<std::string::String, {local}serde_json::Value> = Default::default();"#)?;
        }
        else {
            writeln!(fields_string, "            Key_{field_name},")?;

            writeln!(str_to_field_match_arms, r#"                            {name:?} => Field::Key_{field_name},"#)?;

            match required {
                super::PropertyRequired::Required { is_default } => {
                    writeln!(field_value_defs, r#"                let mut value_{field_name}: Option<{field_type_name}> = None;"#)?;

                    // The API server doesn't always validate required fields when a resource is created,
                    // so consequently required fields aren't always present when a resource is fetched.
                    //
                    // An example is in https://github.com/Arnavion/k8s-openapi/issues/110 which says it's possible
                    // to create a DaemonSet with a PodSpec that doesn't have its required fields.
                    //
                    // Since the Deserialize impl only matters for parsing the API server response, there's no point being stricter
                    // about the response than the API server is. So we can just pretend that the missing field is the default value
                    // if the field is Default-able. But if the field isn't Default-able, then there's nothing we can do but fail as usual.
                    if *is_default {
                        writeln!(field_value_match_arms,
                            r#"                        Field::Key_{field_name} => value_{field_name} = {local}serde::de::MapAccess::next_value(&mut map)?,"#)?;

                        writeln!(field_value_assignment, "                    {field_name}: value_{field_name}.unwrap_or_default(),")?;
                    }
                    else {
                        writeln!(field_value_match_arms,
                            r#"                        Field::Key_{field_name} => value_{field_name} = Some({local}serde::de::MapAccess::next_value(&mut map)?),"#)?;

                        writeln!(field_value_assignment,
                            "                    {field_name}: value_{field_name}.ok_or_else(|| {local}serde::de::Error::missing_field({name:?}))?,")?;
                    }
                },

                super::PropertyRequired::Optional => {
                    writeln!(field_value_defs, r#"                let mut value_{field_name}: {field_type_name} = None;"#)?;

                    writeln!(field_value_match_arms,
                        r#"                        Field::Key_{field_name} => value_{field_name} = {local}serde::de::MapAccess::next_value(&mut map)?,"#)?;

                    writeln!(field_value_assignment, "                    {field_name}: value_{field_name},")?;
                },

                super::PropertyRequired::OptionalDefault => {
                    writeln!(field_value_defs, r#"                let mut value_{field_name}: Option<{field_type_name}> = None;"#)?;

                    writeln!(field_value_match_arms,
                        r#"                        Field::Key_{field_name} => value_{field_name} = {local}serde::de::MapAccess::next_value(&mut map)?,"#)?;

                    writeln!(field_value_assignment, "                    {field_name}: value_{field_name}.unwrap_or_default(),")?;
                },
            }

            writeln!(field_name_list, r#"                {name:?},"#)?;
        }
    }

    if let Some((field_name, _)) = flattened_field {
        writeln!(fields_string, "            Other(String),")?;

        writeln!(str_to_field_match_arms, "                            v => Field::Other(v.into()),")?;

        writeln!(field_value_match_arms,
            "                        Field::Other(key) => {{ value_{field_name}.insert(key, {local}serde::de::MapAccess::next_value(&mut map)?); }},")?;

        writeln!(field_value_assignment,
            "                    {field_name}: {{")?;
        writeln!(field_value_assignment,
            "                        let value_{field_name} = {local}serde::Deserialize::deserialize(value_{field_name}).map_err({local}serde::de::Error::custom)?;")?;
        writeln!(field_value_assignment,
            "                        value_{field_name}")?;
        writeln!(field_value_assignment,
            "                    }},")?;
    }
    else {
        writeln!(fields_string, "            Other,")?;

        writeln!(str_to_field_match_arms, "                            _ => Field::Other,")?;

        writeln!(field_value_match_arms,
            "                        Field::Other => {{ let _: {local}serde::de::IgnoredAny = {local}serde::de::MapAccess::next_value(&mut map)?; }},")?;
    }

    let deserialize_type_name =
        if resource_metadata.is_some() {
            format!("<Self as {local}Resource>::KIND")
        }
        else {
            format!("{type_name:?}")
        };

    let visitor_expecting_type_name =
        if resource_metadata.is_some() {
            format!("<Self::Value as {local}Resource>::KIND")
        }
        else {
            format!("{type_name:?}")
        };

    writeln!(
        writer,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/impl_deserialize.rs")),
        local = local,
        type_name = type_name,
        type_generics_impl = type_generics_impl,
        type_generics_type = type_generics_type,
        type_generics_where = type_generics_where,
        fields = fields_string,
        str_to_field_match_arms = str_to_field_match_arms,
        field_value_defs = field_value_defs,
        field_value_match_arms = field_value_match_arms,
        field_value_assignment = field_value_assignment,
        field_name_list = field_name_list,
        visitor_field = visitor_field,
        visitor_create_field = visitor_create_field,
        deserialize_type_name = deserialize_type_name,
        visitor_expecting_type_name = visitor_expecting_type_name,
    )?;

    Ok(())
}
