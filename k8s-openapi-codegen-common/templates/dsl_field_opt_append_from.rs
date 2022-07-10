    /// Append all elements from `other` into [`Self::{field_name}`]
    {vis} fn {method_field_name}_append_from(&mut self, other: impl std::borrow::Borrow<[{type_name}]>) -> &mut Self {{
         if self.{field_name}.is_none() {{ self.{field_name} = Some(Vec::new()); }}
         let {field_name} = &mut self.{field_name}.as_mut().unwrap();
         for item in other.borrow() {{
             {field_name}.push(item.to_owned());
         }}
         self
    }}
