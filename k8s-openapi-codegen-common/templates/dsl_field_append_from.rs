    /// Append all elements from `other` into [`Self::{field_name}`]
    {vis} fn {method_field_name}_append_from(&mut self, other: impl std::borrow::Borrow<[{type_name}]>) -> &mut Self {{
         for item in other.borrow() {{
             self.{field_name}.push(item.to_owned());
         }}
         self
    }}
