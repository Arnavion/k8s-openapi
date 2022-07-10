    /// Set [`Self::{field_name}`]
    {vis} fn {method_field_name}_set(&mut self, {field_name}: impl Into<{field_type_name}>) -> &mut Self {{
        self.{field_name} = {field_name}.into(); self
    }}

    {vis} fn {field_name}(&mut self) -> &mut {field_inner_type_name} {{
        &mut self.{field_name}
    }}
