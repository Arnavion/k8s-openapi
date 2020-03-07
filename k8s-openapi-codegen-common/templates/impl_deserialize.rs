
impl{type_generics_impl} serde::Deserialize<'de> for {type_name}{type_generics_type}{type_generics_where} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{
        #[allow(non_camel_case_types)]
        enum Field {{
{fields}        }}

        impl<'de> serde::Deserialize<'de> for Field {{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {{
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                        f.write_str("field identifier")
                    }}

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {{
                        Ok(match v {{
{str_to_field_match_arms}                        }})
                    }}
                }}

                deserializer.deserialize_identifier(Visitor)
            }}
        }}

        struct Visitor{type_generics_type}{visitor_field};

        impl{type_generics_impl} serde::de::Visitor<'de> for Visitor{type_generics_type}{type_generics_where} {{
            type Value = {type_name}{type_generics_type};

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str({visitor_expecting_type_name})
            }}

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {{
{field_value_defs}
                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {{
                    match key {{
{field_value_match_arms}                    }}
                }}

                Ok({type_name} {{
{field_value_assignment}                }})
            }}
        }}

        deserializer.deserialize_struct(
            {deserialize_type_name},
            &[
{field_name_list}            ],
            Visitor{visitor_create_field},
        )
    }}
}}