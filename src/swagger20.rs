#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, ::serde_derive::Deserialize)]
pub struct DefinitionPath(pub String);

impl ::std::fmt::Display for DefinitionPath {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		self.0.fmt(f)
	}
}

impl ::std::ops::Deref for DefinitionPath {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.0.deref()
	}
}

#[derive(Debug, ::serde_derive::Deserialize)]
pub struct Info {
	pub title: String,
	pub version: String,
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

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, ::serde_derive::Deserialize)]
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
		#[derive(Debug, ::serde_derive::Deserialize)]
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

		let kind = if let Some(ref_path) = value.ref_path {
			SchemaKind::Ref(ref_path)
		}
		else {
			match value.ty.as_ref().map(|s| s.as_str()) {
				Some("array") => {
					SchemaKind::Ty(Type::Array {
						items: value.items.ok_or_else(|| ::serde::de::Error::missing_field("items"))?,
					})
				},

				Some("boolean") => SchemaKind::Ty(Type::Boolean),

				Some("integer") => {
					let format = value.format.ok_or_else(|| ::serde::de::Error::missing_field("format"))?;
					let format = match &*format {
						"int32" => IntegerFormat::Int32,
						"int64" => IntegerFormat::Int64,
						format => return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(format), &"one of int32, int64")),
					};
					SchemaKind::Ty(Type::Integer { format })
				},

				Some("number") => {
					let format = value.format.ok_or_else(|| ::serde::de::Error::missing_field("format"))?;
					let format = match &*format {
						"double" => NumberFormat::Double,
						format => return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(format), &"one of double")),
					};
					SchemaKind::Ty(Type::Number { format })
				},

				Some("object") => {
					let additional_properties = value.additional_properties.ok_or_else(|| ::serde::de::Error::missing_field("additionalProperties"))?;
					SchemaKind::Ty(Type::Object { additional_properties })
				},

				Some("string") => {
					let format = match value.format.as_ref().map(|s| s.as_str()) {
						Some("byte") => Some(StringFormat::Byte),
						Some("date-time") => Some(StringFormat::DateTime),
						Some("int-or-string") => Some(StringFormat::IntOrString),
						Some(format) => return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(format), &"one of byte, date-time, int-or-string")),
						None => None,
					};
					SchemaKind::Ty(Type::String { format })
				},

				Some(s) => return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(s), &"one of array, boolean, integer, number, object, string")),

				None => {
					let required: ::std::collections::HashSet<_> = value.required.into_iter().collect();
					SchemaKind::Properties(value.properties.into_iter().map(|(name, schema)| {
						let required = required.contains(&name);
						(name, (schema, required))
					}).collect())
				},
			}
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

#[derive(Debug)]
pub struct Spec {
	pub info: Info,
	pub definitions: ::std::collections::BTreeMap<DefinitionPath, Schema>,
}

#[cfg_attr(feature = "cargo-clippy", allow(use_self))]
impl<'de> ::serde::Deserialize<'de> for Spec {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
		#[derive(Debug, ::serde_derive::Deserialize)]
		pub struct InnerSpec {
			swagger: String,
			info: Info,
			definitions: ::std::collections::BTreeMap<DefinitionPath, Schema>,
		}

		let result: InnerSpec = ::serde::Deserialize::deserialize(deserializer)?;

		if result.swagger != "2.0" {
			return Err(::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(&result.swagger), &"2.0"));
		}

		Ok(Spec {
			info: result.info,
			definitions: result.definitions,
		})
	}
}
