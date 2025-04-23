// Generated from definition io.k8s.apimachinery.pkg.runtime.RawExtension

/// RawExtension is used to hold extensions in external versions.
///
/// To use this, make a field which has RawExtension as its type in your external, versioned struct, and Object in your internal struct. You also need to register your various plugin types.
///
/// // Internal package:
///
///   type MyAPIObject struct {
///         runtime.TypeMeta `json:",inline"`
///         MyPlugin runtime.Object `json:"myPlugin"`
///     }
///
///   type PluginA struct {
///         AOption string `json:"aOption"`
///     }
///
/// // External package:
///
///   type MyAPIObject struct {
///         runtime.TypeMeta `json:",inline"`
///         MyPlugin runtime.RawExtension `json:"myPlugin"`
///     }
///
///   type PluginA struct {
///         AOption string `json:"aOption"`
///     }
///
/// // On the wire, the JSON will look something like this:
///
///   {
///         "kind":"MyAPIObject",
///         "apiVersion":"v1",
///         "myPlugin": {
///             "kind":"PluginA",
///             "aOption":"foo",
///         },
///     }
///
/// So what happens? Decode first uses json or yaml to unmarshal the serialized data into your external MyAPIObject. That causes the raw JSON to be stored, but not unpacked. The next step is to copy (using pkg/conversion) into the internal struct. The runtime package's DefaultScheme has conversion functions installed which will unpack the JSON stored in RawExtension, turning it into the correct object type, and storing it in the Object. (TODO: In the case where the object is of an unknown type, a runtime.Unknown object will be created and stored.)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RawExtension(pub crate::serde_json::Value);

impl crate::DeepMerge for RawExtension {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for RawExtension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = RawExtension;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("RawExtension")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(RawExtension(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("RawExtension", Visitor)
    }
}

impl crate::serde::Serialize for RawExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("RawExtension", &self.0)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RawExtension {
    fn schema_name() -> std::string::String {
        "io.k8s.apimachinery.pkg.runtime.RawExtension".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("RawExtension is used to hold extensions in external versions.\n\nTo use this, make a field which has RawExtension as its type in your external, versioned struct, and Object in your internal struct. You also need to register your various plugin types.\n\n// Internal package:\n\n\ttype MyAPIObject struct {\n\t\truntime.TypeMeta `json:\",inline\"`\n\t\tMyPlugin runtime.Object `json:\"myPlugin\"`\n\t}\n\n\ttype PluginA struct {\n\t\tAOption string `json:\"aOption\"`\n\t}\n\n// External package:\n\n\ttype MyAPIObject struct {\n\t\truntime.TypeMeta `json:\",inline\"`\n\t\tMyPlugin runtime.RawExtension `json:\"myPlugin\"`\n\t}\n\n\ttype PluginA struct {\n\t\tAOption string `json:\"aOption\"`\n\t}\n\n// On the wire, the JSON will look something like this:\n\n\t{\n\t\t\"kind\":\"MyAPIObject\",\n\t\t\"apiVersion\":\"v1\",\n\t\t\"myPlugin\": {\n\t\t\t\"kind\":\"PluginA\",\n\t\t\t\"aOption\":\"foo\",\n\t\t},\n\t}\n\nSo what happens? Decode first uses json or yaml to unmarshal the serialized data into your external MyAPIObject. That causes the raw JSON to be stored, but not unpacked. The next step is to copy (using pkg/conversion) into the internal struct. The runtime package's DefaultScheme has conversion functions installed which will unpack the JSON stored in RawExtension, turning it into the correct object type, and storing it in the Object. (TODO: In the case where the object is of an unknown type, a runtime.Unknown object will be created and stored.)".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            ..Default::default()
        })
    }
}
