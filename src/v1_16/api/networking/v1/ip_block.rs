// Generated from definition io.k8s.api.networking.v1.IPBlock

/// IPBlock describes a particular CIDR (Ex. "192.168.1.1/24") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IPBlock {
    /// CIDR is a string representing the IP Block Valid examples are "192.168.1.1/24"
    pub cidr: String,

    /// Except is a slice of CIDRs that should not be included within an IP Block Valid examples are "192.168.1.1/24" Except values will be rejected if they are outside the CIDR range
    pub except: Vec<String>,
}

impl<'de> crate::serde::Deserialize<'de> for IPBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cidr,
            Key_except,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "cidr" => Field::Key_cidr,
                            "except" => Field::Key_except,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IPBlock;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IPBlock")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_cidr: Option<String> = None;
                let mut value_except: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cidr => value_cidr = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_except => value_except = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IPBlock {
                    cidr: value_cidr.ok_or_else(|| crate::serde::de::Error::missing_field("cidr"))?,
                    except: value_except.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "IPBlock",
            &[
                "cidr",
                "except",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IPBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IPBlock",
            1 +
            usize::from(!self.except.is_empty()),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cidr", &self.cidr)?;
        if !self.except.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "except", &self.except)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for IPBlock {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "IPBlock describes a particular CIDR (Ex. \"192.168.1.1/24\") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.",
          "properties": {
            "cidr": {
              "description": "CIDR is a string representing the IP Block Valid examples are \"192.168.1.1/24\"",
              "type": "string"
            },
            "except": {
              "description": "Except is a slice of CIDRs that should not be included within an IP Block Valid examples are \"192.168.1.1/24\" Except values will be rejected if they are outside the CIDR range",
              "items": {
                "type": "string"
              },
              "type": "array"
            }
          },
          "required": [
            "cidr"
          ],
          "type": "object"
        })
    }
}
