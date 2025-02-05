
{cfg}impl{type_generics_impl} {local}schemars::JsonSchema for {type_name}{type_generics_type}{type_generics_where} {{
    fn schema_name() -> std::string::String {{
        {definition_path:?}.into()
    }}

    fn json_schema(__gen: &mut {local}schemars::gen::SchemaGenerator) -> {local}schemars::schema::Schema {{
{schema}    }}
}}