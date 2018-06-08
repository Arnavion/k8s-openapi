// Generated from definition io.k8s.apimachinery.pkg.runtime.RawExtension

/// RawExtension is used to hold extensions in external versions.
///
/// To use this, make a field which has RawExtension as its type in your external, versioned struct, and Object in your internal struct. You also need to register your various plugin types.
///
/// // Internal package: type MyAPIObject struct {
/// 	runtime.TypeMeta `json:",inline"`
/// 	MyPlugin runtime.Object `json:"myPlugin"`
/// } type PluginA struct {
/// 	AOption string `json:"aOption"`
/// }
///
/// // External package: type MyAPIObject struct {
/// 	runtime.TypeMeta `json:",inline"`
/// 	MyPlugin runtime.RawExtension `json:"myPlugin"`
/// } type PluginA struct {
/// 	AOption string `json:"aOption"`
/// }
///
/// // On the wire, the JSON will look something like this: {
/// 	"kind":"MyAPIObject",
/// 	"apiVersion":"v1",
/// 	"myPlugin": {
/// 		"kind":"PluginA",
/// 		"aOption":"foo",
/// 	},
/// }
///
/// So what happens? Decode first uses json or yaml to unmarshal the serialized data into your external MyAPIObject. That causes the raw JSON to be stored, but not unpacked. The next step is to copy (using pkg/conversion) into the internal struct. The runtime package's DefaultScheme has conversion functions installed which will unpack the JSON stored in RawExtension, turning it into the correct object type, and storing it in the Object. (TODO: In the case where the object is of an unknown type, a runtime.Unknown object will be created and stored.)
#[derive(Debug, Default)]
pub struct RawExtension {
    /// Raw is the underlying serialization of this object.
    pub raw: ::ByteString,
}

impl<'de> ::serde::Deserialize<'de> for RawExtension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_raw,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "Raw" => Field::Key_raw,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RawExtension;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct RawExtension")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_raw: Option<::ByteString> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_raw => value_raw = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RawExtension {
                    raw: value_raw.ok_or_else(|| ::serde::de::Error::missing_field("Raw"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "RawExtension",
            &[
                "Raw",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for RawExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RawExtension",
            0 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "Raw", &self.raw)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
