    /// Insert all elements from `other` into [`Self::{field_name}`]
    {vis} fn {method_field_name}_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, {type_name}>>) -> &mut Self {{
         for (name, value) in other.borrow() {{
             self.{field_name}.insert(name.to_owned(), value.to_owned());
         }}
         self
    }}
