    /// Modify [`Self::{field_name}`] with a `func`
    {vis} fn {method_field_name}_with(&mut self, func: impl FnOnce(&mut {field_type_name})) -> &mut Self {{
        func(&mut self.{field_name}); self
    }}
