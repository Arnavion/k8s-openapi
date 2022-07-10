    /// Insert a new element to [`Self::{field_name}`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    {vis} fn {method_field_name}_insert_with(&mut self, name: &str, func: impl FnOnce(&mut {type_name})) -> &mut Self {{
        if self.{field_name}.is_none() {{
            self.{field_name} = Some(std::collections::BTreeMap::new());
        }}
        let mut new = Default::default();
        func(&mut new);
        self.{field_name}.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }}
