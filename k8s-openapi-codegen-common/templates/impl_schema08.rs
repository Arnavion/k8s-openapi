
{cfg}impl{type_generics_impl} {local}schemars08::JsonSchema for {type_name}{type_generics_type}{type_generics_where} {{
    fn schema_name() -> std::string::String {{
        {definition_path:?}.into()
    }}

    fn json_schema(__gen: &mut {local}schemars08::gen::SchemaGenerator) -> {local}schemars08::schema::Schema {{
{schema}    }}
}}