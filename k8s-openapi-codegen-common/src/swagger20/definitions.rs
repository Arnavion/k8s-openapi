/// A definition path.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DefinitionPath(pub String);

impl std::ops::Deref for DefinitionPath {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&*self.0
	}
}

impl std::fmt::Display for DefinitionPath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

/// An integer format. This corresponds to the `"format"` property of an `"integer"` schema type.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IntegerFormat {
	Int32,
	Int64,
}

/// A number format. This corresponds to the `"format"` property of a `"number"` schema type.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NumberFormat {
	Double,
}

/// The name of a property of a schema type with a `"properties"` map.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PropertyName(pub String);

impl std::ops::Deref for PropertyName {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&*self.0
	}
}

impl std::fmt::Display for PropertyName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

/// The path specified by a `"$ref"` property.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RefPath {
	pub path: String,
	pub can_be_default: Option<bool>,
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RefPath {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		let path: String = serde::Deserialize::deserialize(deserializer)?;
		let mut parts = path.split('/');

		if parts.next() != Some("#") {
			return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&path), &"path like `#/definitions/$definitionName`"));
		}

		if parts.next() != Some("definitions") {
			return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&path), &"path like `#/definitions/$definitionName`"));
		}

		let ref_path = if let Some(ref_path) = parts.next() {
			ref_path.to_string()
		}
		else {
			return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&path), &"path like `#/definitions/$definitionName`"));
		};

		if parts.next().is_some() {
			return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&path), &"path like `#/definitions/$definitionName`"));
		}

		Ok(RefPath {
			path: ref_path,
			can_be_default: None,
		})
	}
}

#[cfg(all(feature = "serde", feature = "schema"))]
impl serde::Serialize for RefPath {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		format_args!("#/definitions/{}", &self.path).serialize(serializer)
	}
}

impl RefPath {
	pub(crate) fn references_scope(&self, map_namespace: &impl crate::MapNamespace) -> bool {
		let path_parts: Vec<_> = self.path.split('.').collect();
		map_namespace.map_namespace(&path_parts[..(path_parts.len() - 1)]).is_none()
	}
}

/// The schema of a definition or operation parameter.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Schema {
	pub description: Option<String>,
	pub kind: SchemaKind,
	pub kubernetes_group_kind_versions: Vec<super::KubernetesGroupKindVersion>,

	/// Used to store the definition path of the corresponding list type, if any.
	pub list_kind: Option<String>,
}

#[cfg(feature = "serde")]
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
struct InnerSchema {
	// This must be before description to make schema inlining pattern easier.
	#[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
	ref_path: Option<RefPath>,

	#[serde(rename = "additionalProperties", skip_serializing_if = "Option::is_none")]
	additional_properties: Option<Box<Schema>>,

	#[serde(skip_serializing_if = "Option::is_none")]
	description: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	format: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	items: Option<Box<Schema>>,

	#[serde(default, rename = "x-kubernetes-group-version-kind", skip_serializing_if = "Vec::is_empty")]
	kubernetes_group_kind_versions: Vec<super::KubernetesGroupKindVersion>,

	#[serde(skip_serializing_if = "Option::is_none")]
	properties: Option<std::collections::BTreeMap<PropertyName, Schema>>,

	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	required: Vec<PropertyName>,

	#[serde(rename = "type", skip_serializing_if = "Option::is_none")]
	ty: Option<String>,
}

#[cfg(feature = "serde")]
#[allow(clippy::use_self)]
impl<'de> serde::Deserialize<'de> for Schema {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		let mut value: InnerSchema = serde::Deserialize::deserialize(deserializer)?;

		let kind =
			if let Some(ref_path) = value.ref_path {
				SchemaKind::Ref(ref_path)
			}
			else if let Some(properties) = value.properties.take() {
				// Starting from 1.14, the spec sets type=object for all types with properties.
				// Earlier specs did not set it at all.
				if let Some(ty) = value.ty.take() {
					if ty != "object" {
						return Err(serde::de::Error::custom(format!("schema has properties but not type=object {:?}", value)));
					}
				}

				let required: std::collections::BTreeSet<_> = value.required.into_iter().collect();
				SchemaKind::Properties(properties.into_iter().map(|(name, schema)| {
					let required = required.contains(&name);
					(name, (schema, required))
				}).collect())
			}
			else if let Some(ty) = value.ty {
				SchemaKind::Ty(Type::parse::<D>(
					&ty,
					value.additional_properties,
					value.format.as_deref(),
					value.items,
				)?)
			}
			else {
				SchemaKind::Ty(Type::Any)
			};

		Ok(Schema {
			description: value.description,
			kind,
			kubernetes_group_kind_versions: value.kubernetes_group_kind_versions,
			list_kind: None,
		})
	}
}

#[cfg(all(feature = "serde", feature = "schema"))]
impl serde::Serialize for Schema {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut inner = InnerSchema {
			description: self.description.clone(),
			kubernetes_group_kind_versions: self.kubernetes_group_kind_versions.clone(),
			..InnerSchema::default()
		};
		match &self.kind {
			SchemaKind::Ref(ref_path) => {
				inner.ref_path = Some(ref_path.clone());
			},

			SchemaKind::Properties(properties) => {
				let mut required: Vec<PropertyName> = Vec::new();
				inner.ty = Some("object".to_owned());
				inner.properties = Some(properties.clone().into_iter().map(|(name, (schema, req))| {
					if req {
						required.push(name.clone());
					}
					(name, schema)
				}).collect());
				inner.required = required;
			}

			SchemaKind::Ty(ty) => {
				match ty {
					Type::Any => {
						inner.ty = Some("object".to_owned());
					}
					// TODO
					Type::JsonSchemaPropsOrArray(_) |
					Type::JsonSchemaPropsOrBool(_) |
					Type::JsonSchemaPropsOrStringArray(_) => {
						inner.ty = Some("object".to_owned());
					}

					Type::Array { items } => {
						inner.ty = Some("array".to_owned());
						inner.items = Some(items.clone());
					}

					Type::Boolean => {
						inner.ty = Some("boolean".to_owned());
					}

					Type::Integer { format } => {
						inner.ty = Some("integer".to_owned());
						match format {
							IntegerFormat::Int32 => {
								inner.format = Some("int32".to_owned());
							}
							IntegerFormat::Int64 => {
								inner.format = Some("int64".to_owned());
							}
						}
					}

					Type::Number { format } => {
						inner.ty = Some("number".to_owned());
						match format {
							NumberFormat::Double => {
								inner.format = Some("double".to_owned());
							}
						}
					}

					Type::Object { additional_properties } => {
						inner.ty = Some("object".to_owned());
						inner.additional_properties = Some(additional_properties.clone());
					}

					Type::String { format } => {
						inner.ty = Some("string".to_owned());
						inner.format = match format {
							Some(StringFormat::Byte) => Some("byte".to_owned()),
							Some(StringFormat::DateTime) => Some("date-time".to_owned()),
							None => None,
						};
					}

					Type::IntOrString => {
						inner.ty = Some("string".to_owned());
						inner.format = Some("int-or-string".to_owned());
					}

					_ => unreachable!("serialize {:?}", self.kind),
				}
			},
		}

		inner.serialize(serializer)
	}
}

/// The kind of a [`Schema`]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SchemaKind {
	Properties(std::collections::BTreeMap<PropertyName, (Schema, bool)>),
	Ref(RefPath),
	Ty(Type),
}

/// A string format. This corresponds to the `"format"` property of an `"string"` schema type.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StringFormat {
	Byte,
	DateTime,
}

/// A type definition.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
	Any,
	Array { items: Box<Schema> },
	Boolean,
	Integer { format: IntegerFormat },
	Number { format: NumberFormat },
	Object { additional_properties: Box<Schema> },
	String { format: Option<StringFormat> },

	// Special type for the `subresources` field of custom resources.
	CustomResourceSubresources(String),

	// Special types that need alterative codegen
	IntOrString,
	JsonSchemaPropsOrArray(&'static str),
	JsonSchemaPropsOrBool(&'static str),
	JsonSchemaPropsOrStringArray(&'static str),
	Patch,
	WatchEvent(RefPath),

	// Special type for lists
	ListDef { metadata: Box<SchemaKind> }, // The definition of the List type
	ListRef { items: Box<SchemaKind> }, // A reference to a specialization of the List type for a particular resource type, eg List<Pod> for PodList

	// Special types for common parameters of some API operations
	CreateOptional(std::collections::BTreeMap<PropertyName, Schema>),
	DeleteOptional(std::collections::BTreeMap<PropertyName, Schema>),
	ListOptional(std::collections::BTreeMap<PropertyName, Schema>),
	PatchOptional(std::collections::BTreeMap<PropertyName, Schema>),
	ReplaceOptional(std::collections::BTreeMap<PropertyName, Schema>),
	WatchOptional(std::collections::BTreeMap<PropertyName, Schema>),

	// Special types for responses of some API operations
	CreateResponse,
	DeleteResponse,
	ListResponse,
	PatchResponse,
	ReplaceResponse,
	WatchResponse,
}

impl Type {
	#[cfg(feature = "serde")]
	pub(crate) fn parse<'de, D>(
		ty: &str,
		additional_properties: Option<Box<Schema>>,
		format: Option<&str>,
		items: Option<Box<Schema>>,
	) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		match ty {
			"array" => Ok(Type::Array {
				items: items.ok_or_else(|| serde::de::Error::missing_field("items"))?,
			}),

			"boolean" => Ok(Type::Boolean),

			"integer" => {
				let format = match format {
					Some("int32") => IntegerFormat::Int32,
					Some("int64") | None => IntegerFormat::Int64,
					Some(format) => return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(format), &"one of int32, int64")),
				};
				Ok(Type::Integer { format })
			},

			"number" => {
				let format = format.ok_or_else(|| serde::de::Error::missing_field("format"))?;
				let format = match format {
					"double" => NumberFormat::Double,
					format => return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(format), &"one of double")),
				};
				Ok(Type::Number { format })
			},

			"object" => match additional_properties {
				Some(additional_properties) => Ok(Type::Object { additional_properties }),
				None => Ok(Type::Any),
			},

			"string" => {
				let format = match format {
					Some("byte") => Some(StringFormat::Byte),
					Some("date-time") => Some(StringFormat::DateTime),
					Some("int-or-string") => return Ok(Type::IntOrString),
					Some(format) => return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(format), &"one of byte, date-time, int-or-string")),
					None => None,
				};
				Ok(Type::String { format })
			},

			s => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(s), &"one of array, boolean, integer, number, object, string")),
		}
	}
}
