#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct DefinitionPath(pub String);

impl ::std::ops::Deref for DefinitionPath {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.0.deref()
	}
}

impl ::std::fmt::Display for DefinitionPath {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		self.0.fmt(f)
	}
}

#[derive(Clone, Copy, Debug)]
pub enum IntegerFormat {
	Int32,
	Int64,
}

#[derive(Clone, Copy, Debug)]
pub enum NumberFormat {
	Double,
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct PropertyName(pub String);

impl ::std::ops::Deref for PropertyName {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.0.deref()
	}
}

impl ::std::fmt::Display for PropertyName {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		self.0.fmt(f)
	}
}

#[derive(Debug)]
pub struct RefPath(pub String);

impl ::std::ops::Deref for RefPath {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.0.deref()
	}
}

impl ::std::fmt::Display for RefPath {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		self.0.fmt(f)
	}
}

impl<'de> ::serde::Deserialize<'de> for RefPath {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
		let path: String = ::serde::Deserialize::deserialize(deserializer)?;
		let mut parts = path.split('/');

		if parts.next() != Some("#") {
			return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&path), &"path like `#/definitions/$definitionName`"));
		}

		if parts.next() != Some("definitions") {
			return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&path), &"path like `#/definitions/$definitionName`"));
		}

		let ref_path = if let Some(ref_path) = parts.next() {
			ref_path.to_string()
		}
		else {
			return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&path), &"path like `#/definitions/$definitionName`"));
		};

		if parts.next().is_some() {
			return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&path), &"path like `#/definitions/$definitionName`"));
		}

		Ok(RefPath(ref_path))
	}
}

#[derive(Debug)]
pub struct Schema {
	pub description: Option<String>,
	pub kind: SchemaKind,
}

#[cfg_attr(feature = "cargo-clippy", allow(use_self))]
impl<'de> ::serde::Deserialize<'de> for Schema {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
		#[derive(Debug, Deserialize)]
		struct InnerSchema {
			#[serde(rename = "additionalProperties")]
			additional_properties: Option<Box<Schema>>,

			description: Option<String>,

			format: Option<String>,

			items: Option<Box<Schema>>,

			#[serde(default)]
			properties: ::std::collections::BTreeMap<PropertyName, Schema>,

			#[serde(rename = "$ref")]
			ref_path: Option<RefPath>,

			#[serde(default)]
			required: Vec<PropertyName>,

			#[serde(rename = "type")]
			ty: Option<String>,
		}

		let value: InnerSchema = ::serde::Deserialize::deserialize(deserializer)?;

		let description = value.description;

		let kind =
			if let Some(ref_path) = value.ref_path {
				SchemaKind::Ref(ref_path)
			}
			else if let Some(ty) = value.ty {
				SchemaKind::Ty(Type::parse::<D>(
					&ty,
					value.additional_properties,
					value.format.as_ref().map(String::as_str),
					value.items,
				)?)
			}
			else {
				let required: ::std::collections::HashSet<_> = value.required.into_iter().collect();
				SchemaKind::Properties(value.properties.into_iter().map(|(name, schema)| {
					let required = required.contains(&name);
					(name, (schema, required))
				}).collect())
			};

		Ok(Schema {
			description,
			kind,
		})
	}
}

#[derive(Debug)]
pub enum SchemaKind {
	Properties(::std::collections::BTreeMap<PropertyName, (Schema, bool)>),
	Ref(RefPath),
	Ty(Type),
}

#[derive(Clone, Copy, Debug)]
pub enum StringFormat {
	Byte,
	DateTime,
	IntOrString,
}

#[derive(Debug)]
pub enum Type {
	Array { items: Box<Schema> },
	Boolean,
	Integer { format: IntegerFormat },
	Number { format: NumberFormat },
	Object { additional_properties: Box<Schema> },
	String { format: Option<StringFormat> },
}

impl Type {
	pub(crate) fn parse<'de, D>(
		ty: &str,
		additional_properties: Option<Box<Schema>>,
		format: Option<&str>,
		items: Option<Box<Schema>>,
	) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
		match ty {
			"array" => Ok(Type::Array {
				items: items.ok_or_else(|| ::serde::de::Error::missing_field("items"))?,
			}),

			"boolean" => Ok(Type::Boolean),

			"integer" => {
				let format = match format {
					Some("int32") => IntegerFormat::Int32,
					Some("int64") | None => IntegerFormat::Int64,
					Some(format) => return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(format), &"one of int32, int64")),
				};
				Ok(Type::Integer { format })
			},

			"number" => {
				let format = format.ok_or_else(|| ::serde::de::Error::missing_field("format"))?;
				let format = match format {
					"double" => NumberFormat::Double,
					format => return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(format), &"one of double")),
				};
				Ok(Type::Number { format })
			},

			"object" => {
				let additional_properties = additional_properties.ok_or_else(|| ::serde::de::Error::missing_field("additionalProperties"))?;
				Ok(Type::Object { additional_properties })
			},

			"string" => {
				let format = match format {
					Some("byte") => Some(StringFormat::Byte),
					Some("date-time") => Some(StringFormat::DateTime),
					Some("int-or-string") => Some(StringFormat::IntOrString),
					Some(format) => return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(format), &"one of byte, date-time, int-or-string")),
					None => None,
				};
				Ok(Type::String { format })
			},

			s => Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(s), &"one of array, boolean, integer, number, object, string")),
		}
	}
}
