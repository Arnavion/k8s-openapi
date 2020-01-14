// Generated from definition io.k8s.apimachinery.pkg.runtime.RawExtension

/// RawExtension is used to hold extensions in external versions.
///
/// To use this, make a field which has RawExtension as its type in your external, versioned struct, and Object in your internal struct. You also need to register your various plugin types.
///
/// // Internal package: type MyAPIObject struct {
///     runtime.TypeMeta `json:",inline"`
///     MyPlugin runtime.Object `json:"myPlugin"`
/// } type PluginA struct {
///     AOption string `json:"aOption"`
/// }
///
/// // External package: type MyAPIObject struct {
///     runtime.TypeMeta `json:",inline"`
///     MyPlugin runtime.RawExtension `json:"myPlugin"`
/// } type PluginA struct {
///     AOption string `json:"aOption"`
/// }
///
/// // On the wire, the JSON will look something like this: {
///     "kind":"MyAPIObject",
///     "apiVersion":"v1",
///     "myPlugin": {
///         "kind":"PluginA",
///         "aOption":"foo",
///     },
/// }
///
/// So what happens? Decode first uses json or yaml to unmarshal the serialized data into your external MyAPIObject. That causes the raw JSON to be stored, but not unpacked. The next step is to copy (using pkg/conversion) into the internal struct. The runtime package's DefaultScheme has conversion functions installed which will unpack the JSON stored in RawExtension, turning it into the correct object type, and storing it in the Object. (TODO: In the case where the object is of an unknown type, a runtime.Unknown object will be created and stored.)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RawExtension(pub serde_json::Value);

impl<'de> serde::Deserialize<'de> for RawExtension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RawExtension;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RawExtension")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
                Ok(RawExtension(serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("RawExtension", Visitor)
    }
}

impl serde::Serialize for RawExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_newtype_struct("RawExtension", &self.0)
    }
}
