pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	generics: super::Generics<'_>,
	fields: &[super::Property<'_>],
	crate_root: String,
	resource_metadata: Option<&super::ResourceMetadata<'_>>,
) -> Result<(), crate::Error> {
	use std::fmt::Write;

	let type_generics_impl = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_type = generics.type_part.map(|part| format!("<{}>", part)).unwrap_or_default();
	let type_generics_where = generics.where_part.map(|part| format!(" where {}", part)).unwrap_or_default();

	let mut fields_string = String::new();
	let mut required_fields_num = 0_usize;
	let mut fields_num = vec![];
	let mut has_flattened_field = false;

	if resource_metadata.is_some() {
		writeln!(fields_string, r#"        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as {}::Resource>::API_VERSION)?;"#, crate_root)?;
		writeln!(fields_string, r#"        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as {}::Resource>::KIND)?;"#, crate_root)?;

		required_fields_num += 2;
	}

	for super::Property { name, field_name, required, is_flattened, .. } in fields {
		if *is_flattened {
			writeln!(fields_string, "        serde::Serialize::serialize(&self.{}, SerializerWrapper(&mut state))?;", field_name)?;

			has_flattened_field = true;
		}
		else if *required {
			writeln!(fields_string, "        serde::ser::SerializeStruct::serialize_field(&mut state, {:?}, &self.{})?;", name, field_name)?;

			required_fields_num += 1;
		}
		else {
			writeln!(fields_string, "        if let Some(value) = &self.{} {{", field_name)?;
			writeln!(fields_string, "            serde::ser::SerializeStruct::serialize_field(&mut state, {:?}, value)?;", name)?;
			writeln!(fields_string, "        }}")?;

			fields_num.push(format!("self.{}.as_ref().map_or(0, |_| 1)", field_name));
		}
	}

	let fields_num: std::borrow::Cow<'_, str> = match (required_fields_num, fields_num.is_empty()) {
		(0, true) => "            0".into(),

		(0, false) => {
			let mut fields_num_str = String::new();
			let mut first = true;
			for field_num in fields_num {
				if first {
					first = false;
				}
				else {
					writeln!(fields_num_str, " +")?;
				}

				write!(fields_num_str, "            {}", field_num)?;
			}

			fields_num_str.into()
		},

		(required_fields_num, true) => format!("            {}", required_fields_num).into(),

		(required_fields_num, false) => {
			let mut fields_num_str = format!("            {}", required_fields_num);
			for field_num in fields_num {
				writeln!(fields_num_str, " +")?;
				write!(fields_num_str, "            {}", field_num)?;
			}

			fields_num_str.into()
		},
	};

	let serialize_type_name =
		if resource_metadata.is_some() {
			format!("<Self as {}::Resource>::KIND", crate_root)
		}
		else {
			format!("{:?}", type_name)
		};

	let struct_serializer: std::borrow::Cow<'_, str> =
		if has_flattened_field {
			let mut out = String::new();

			writeln!(out, "        struct SerializerWrapper<'a, S>(&'a mut S);")?;
			writeln!(out)?;
			writeln!(out, "        impl<'a, S> serde::Serializer for SerializerWrapper<'a, S> where S: serde::ser::SerializeStruct {{")?;
			writeln!(out, "            type Ok = ();")?;
			writeln!(out, "            type Error = <S as serde::ser::SerializeStruct>::Error;")?;
			writeln!(out)?;
			writeln!(out, "            type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;")?;
			writeln!(out, "            type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;")?;
			writeln!(out, "            type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;")?;
			writeln!(out, "            type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;")?;
			writeln!(out, "            type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;")?;
			writeln!(out, "            type SerializeStruct = SerializerStructWrapper<'a, S>;")?;
			writeln!(out, "            type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("bool is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("i8 is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom(" is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("i32 is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("i64 is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("u8 is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("u16 is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("u32 is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("u64 is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("f32 is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("f64 is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("char is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("str is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("bytes is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_none(self) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("none is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: serde::Serialize + ?Sized {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("some is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("unit is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("unit struct is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("unit variant is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_newtype_struct<T>(")?;
			writeln!(out, "                self,")?;
			writeln!(out, "                _: &'static str,")?;
			writeln!(out, "                _: &T,")?;
			writeln!(out, "            ) -> Result<Self::Ok, Self::Error> where T: serde::Serialize + ?Sized {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("newtype struct is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_newtype_variant<T>(")?;
			writeln!(out, "                self,")?;
			writeln!(out, "                _: &'static str,")?;
			writeln!(out, "                _: u32,")?;
			writeln!(out, "                _: &'static str,")?;
			writeln!(out, "                _: &T,")?;
			writeln!(out, "            ) -> Result<Self::Ok, Self::Error> where T: serde::Serialize + ?Sized {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("newtype variant is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("seq is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("tuple is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("tuple struct is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_tuple_variant(")?;
			writeln!(out, "                self,")?;
			writeln!(out, "                _: &'static str,")?;
			writeln!(out, "                _: u32,")?;
			writeln!(out, "                _: &'static str,")?;
			writeln!(out, "                _: usize,")?;
			writeln!(out, "            ) -> Result<Self::SerializeTupleVariant, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("tuple variant is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("map is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {{")?;
			writeln!(out, "                Ok(SerializerStructWrapper(self.0))")?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_struct_variant(")?;
			writeln!(out, "                self,")?;
			writeln!(out, "                _: &'static str,")?;
			writeln!(out, "                _: u32,")?;
			writeln!(out, "                _: &'static str,")?;
			writeln!(out, "                _: usize,")?;
			writeln!(out, "            ) -> Result<Self::SerializeStructVariant, Self::Error> {{")?;
			writeln!(out, r#"                Err(serde::ser::Error::custom("struct variant is not supported"))"#)?;
			writeln!(out, "            }}")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;
			writeln!(out, "        struct SerializerStructWrapper<'a, S>(&'a mut S);")?;
			writeln!(out)?;
			writeln!(out, "        impl<'a, S> serde::ser::SerializeStruct for SerializerStructWrapper<'a, S> where S: serde::ser::SerializeStruct {{")?;
			writeln!(out, "            type Ok = ();")?;
			writeln!(out, "            type Error = <S as serde::ser::SerializeStruct>::Error;")?;
			writeln!(out)?;
			writeln!(out, "            fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: serde::Serialize + ?Sized {{")?;
			writeln!(out, r#"                self.0.serialize_field(key, value)"#)?;
			writeln!(out, "            }}")?;
			writeln!(out)?;
			writeln!(out, "            fn end(self) -> Result<Self::Ok, Self::Error> {{")?;
			writeln!(out, r#"                Ok(())"#)?;
			writeln!(out, "            }}")?;
			writeln!(out, "        }}")?;
			writeln!(out)?;

			out.into()
		}
		else {
			"".into()
		};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/impl_serialize.rs")),
		type_name = type_name,
		type_generics_impl = type_generics_impl,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		struct_serializer = struct_serializer,
		fields_num = fields_num,
		fields = fields_string,
		serialize_type_name = serialize_type_name,
	)?;

	Ok(())
}
