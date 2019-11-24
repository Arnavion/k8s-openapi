
impl{type_generics_impl} serde::Serialize for {type_name}{type_generics_type}{type_generics_where} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{
        let mut state = serializer.serialize_struct(
            {serialize_type_name},
{fields_num},
        )?;
{fields}        serde::ser::SerializeStruct::end(state)
    }}
}}