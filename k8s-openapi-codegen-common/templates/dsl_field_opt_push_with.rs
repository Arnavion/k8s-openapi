    /// Push new element to [`Self::{field_name}`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    {vis} fn {method_field_name}_push_with(&mut self, func: impl FnOnce(&mut {type_name})) -> &mut Self {{
        if self.{field_name}.is_none() {{
            self.{field_name} = Some(vec![]);
        }}
        let mut new = Default::default();
        func(&mut new);
        self.{field_name}.as_mut().unwrap().push(new);
        self
    }}
