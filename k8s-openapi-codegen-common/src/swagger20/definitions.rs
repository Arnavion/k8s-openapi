/// A definition path.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DefinitionPath(pub String);

impl std::borrow::Borrow<str> for DefinitionPath {
	fn borrow(&self) -> &str {
		&self.0
	}
}

impl std::ops::Deref for DefinitionPath {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
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

/// The value of an `x-kubernetes-list-type` annotation on a property.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum KubernetesListType {
	#[default]
	#[cfg_attr(feature = "serde", serde(rename = "atomic"))]
	Atomic,

	#[cfg_attr(feature = "serde", serde(rename = "map"))]
	Map,

	#[cfg_attr(feature = "serde", serde(rename = "set"))]
	Set,
}

/// The value of an `x-kubernetes-map-type` annotation on a property.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum KubernetesMapType {
	#[cfg_attr(feature = "serde", serde(rename = "atomic"))]
	Atomic,

	#[default]
	#[cfg_attr(feature = "serde", serde(rename = "granular"))]
	Granular,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MergeType {
	Default,

	List {
		strategy: KubernetesListType,
		keys: Vec<String>,
		item_merge_type: Box<MergeType>,
	},

	Map {
		strategy: KubernetesMapType,
		value_merge_type: Box<MergeType>,
	},
}

/// A number format. This corresponds to the `"format"` property of a `"number"` schema type.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NumberFormat {
	Double,
}

/// The name of a property of a schema type with a `"properties"` map.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PropertyName(pub String);

impl std::borrow::Borrow<str> for PropertyName {
	fn borrow(&self) -> &str {
		&self.0
	}
}

impl std::ops::Deref for PropertyName {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
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

		let ref_path = parts.next().ok_or_else(|| serde::de::Error::invalid_value(serde::de::Unexpected::Str(&path), &"path like `#/definitions/$definitionName`"))?;

		if parts.next().is_some() {
			return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&path), &"path like `#/definitions/$definitionName`"));
		}

		Ok(RefPath {
			path: ref_path.to_string(),
			can_be_default: None,
		})
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

	pub merge_type: MergeType,

	/// Used to store the definition path of the corresponding list type, if any.
	pub list_kind: Option<String>,

	/// Used to enable or disable the auto-generated impl of `k8s_openapi::DeepMerge` on the generated type.
	pub impl_deep_merge: bool,
}

#[cfg(feature = "serde")]
#[allow(clippy::use_self)]
impl<'de> serde::Deserialize<'de> for Schema {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		#[derive(Debug, serde::Deserialize)]
		struct InnerSchema {
			#[serde(rename = "additionalProperties")]
			additional_properties: Option<Box<Schema>>,

			description: Option<String>,

			format: Option<String>,

			items: Option<Box<Schema>>,

			#[serde(default, rename = "x-kubernetes-group-version-kind")]
			kubernetes_group_kind_versions: Vec<super::KubernetesGroupKindVersion>,

			#[serde(default, rename = "x-kubernetes-list-map-keys")]
			kubernetes_list_map_keys: Vec<String>,

			#[serde(default, rename = "x-kubernetes-list-type")]
			kubernetes_list_type: KubernetesListType,

			#[serde(default, rename = "x-kubernetes-map-type")]
			kubernetes_map_type: KubernetesMapType,

			#[serde(default, rename = "x-kubernetes-patch-merge-key")]
			kubernetes_patch_merge_key: Option<String>,

			/// Comma-separated list of strategy tags.
			/// Ref: https://github.com/kubernetes/community/blob/master/contributors/devel/sig-api-machinery/strategic-merge-patch.md
			#[serde(default, rename = "x-kubernetes-patch-strategy")]
			kubernetes_patch_strategy: String,

			properties: Option<std::collections::BTreeMap<PropertyName, Schema>>,

			#[serde(rename = "$ref")]
			ref_path: Option<RefPath>,

			#[serde(default)]
			required: Vec<PropertyName>,

			#[serde(rename = "type")]
			ty: Option<String>,
		}

		let mut value: InnerSchema = serde::Deserialize::deserialize(deserializer)?;

		let kind =
			if let Some(ref_path) = value.ref_path {
				SchemaKind::Ref(ref_path)
			}
			else if let Some(properties) = value.properties.take() {
				if value.ty.as_deref() != Some("object") {
					return Err(serde::de::Error::custom(format!("schema has properties but not type=object {value:?}")));
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

		if let Some(key) = value.kubernetes_patch_merge_key {
			value.kubernetes_list_map_keys = vec![key];
		}
		if value.kubernetes_patch_strategy.split(',').any(|x| x == "merge") {
			value.kubernetes_list_type =
				if value.kubernetes_list_map_keys.is_empty() {
					KubernetesListType::Set
				}
				else {
					KubernetesListType::Map
				};
		}

		let merge_type = match &kind {
			SchemaKind::Ty(Type::Array { items }) => MergeType::List {
				strategy: value.kubernetes_list_type,
				keys: value.kubernetes_list_map_keys,
				item_merge_type: Box::new(items.merge_type.clone()),
			},

			SchemaKind::Ty(Type::Object { additional_properties }) => MergeType::Map {
				strategy: value.kubernetes_map_type,
				value_merge_type: Box::new(additional_properties.merge_type.clone()),
			},

			_ => MergeType::Default,
		};

		Ok(Schema {
			description: value.description,
			kind,
			kubernetes_group_kind_versions: value.kubernetes_group_kind_versions,
			list_kind: None,
			merge_type,
			impl_deep_merge: true,
		})
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
	JsonSchemaPropsOr(&'static str, JsonSchemaPropsOr),

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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum JsonSchemaPropsOr {
	Array,
	Bool,
	StringArray,
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
