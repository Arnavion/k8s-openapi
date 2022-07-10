    {vis} fn {field_name}(&mut self) -> &mut {field_inner_type_name} {{
        if self.{field_name}.is_none() {{ self.{field_name} = Some(Default::default()) }}
        self.{field_name}.as_mut().unwrap()
    }}
