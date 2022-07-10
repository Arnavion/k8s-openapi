    /// Insert all elements from `other` into [`Self::{field_name}`]
    {vis} fn {method_field_name}_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, {type_name}>>) -> &mut Self {{
         if self.{field_name}.is_none() {{ self.{field_name} = Some(std::collections::BTreeMap::new()); }}
         let {field_name} = &mut self.{field_name}.as_mut().unwrap();
         for (name, value) in other.borrow() {{
             {field_name}.insert(name.to_owned(), value.to_owned());
         }}
         self
    }}
