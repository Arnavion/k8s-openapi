    /// Modify [`Self::{field_name}`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    {vis} fn {method_field_name}_with(&mut self, func: impl FnOnce(&mut {field_inner_type_name})) -> &mut Self {{
        if self.{field_name}.is_none() {{ self.{field_name} = Some(Default::default()) }};
        func(self.{field_name}.as_mut().unwrap()); self
    }}
