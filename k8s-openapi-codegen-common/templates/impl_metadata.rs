
impl{type_generics_impl} {local}Metadata for {type_name}{type_generics_type}{type_generics_where} {{
    type Ty = {metadata_type_name};

    fn metadata(&self) -> &<Self as {local}Metadata>::Ty {{
        {metadata_expr}
    }}

    fn metadata_mut(&mut self) -> &mut<Self as {local}Metadata>::Ty {{
        {metadata_mut_expr}
    }}
}}