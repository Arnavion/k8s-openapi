// Generated from definition io.k8s.api.resource.v1.DeviceConstraint

/// DeviceConstraint must have exactly one field set besides Requests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceConstraint {
    /// DistinctAttribute requires that all devices in question have this attribute and that its type and value are unique across those devices.
    ///
    /// This acts as the inverse of MatchAttribute.
    ///
    /// This constraint is used to avoid allocating multiple requests to the same device by ensuring attribute-level differentiation.
    ///
    /// This is useful for scenarios where resource requests must be fulfilled by separate physical devices. For example, a container requests two network interfaces that must be allocated from two different physical NICs.
    pub distinct_attribute: Option<std::string::String>,

    /// MatchAttribute requires that all devices in question have this attribute and that its type and value are the same across those devices.
    ///
    /// For example, if you specified "dra.example.com/numa" (a hypothetical example!), then only devices in the same NUMA node will be chosen. A device which does not have that attribute will not be chosen. All devices should use a value of the same type for this attribute because that is part of its specification, but if one device doesn't, then it also will not be chosen.
    ///
    /// Must include the domain qualifier.
    pub match_attribute: Option<std::string::String>,

    /// Requests is a list of the one or more requests in this claim which must co-satisfy this constraint. If a request is fulfilled by multiple devices, then all of the devices must satisfy the constraint. If this is not specified, this constraint applies to all requests in this claim.
    ///
    /// References to subrequests must include the name of the main request and may include the subrequest using the format \<main request\>\[/\<subrequest\>\]. If just the main request is given, the constraint applies to all subrequests.
    pub requests: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for DeviceConstraint {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.distinct_attribute, other.distinct_attribute);
        crate::DeepMerge::merge_from(&mut self.match_attribute, other.match_attribute);
        crate::merge_strategies::list::atomic(&mut self.requests, other.requests);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceConstraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_distinct_attribute,
            Key_match_attribute,
            Key_requests,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "distinctAttribute" => Field::Key_distinct_attribute,
                            "matchAttribute" => Field::Key_match_attribute,
                            "requests" => Field::Key_requests,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceConstraint;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceConstraint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_distinct_attribute: Option<std::string::String> = None;
                let mut value_match_attribute: Option<std::string::String> = None;
                let mut value_requests: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_distinct_attribute => value_distinct_attribute = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_attribute => value_match_attribute = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_requests => value_requests = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceConstraint {
                    distinct_attribute: value_distinct_attribute,
                    match_attribute: value_match_attribute,
                    requests: value_requests,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceConstraint",
            &[
                "distinctAttribute",
                "matchAttribute",
                "requests",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceConstraint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceConstraint",
            self.distinct_attribute.as_ref().map_or(0, |_| 1) +
            self.match_attribute.as_ref().map_or(0, |_| 1) +
            self.requests.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.distinct_attribute {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "distinctAttribute", value)?;
        }
        if let Some(value) = &self.match_attribute {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchAttribute", value)?;
        }
        if let Some(value) = &self.requests {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "requests", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceConstraint {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1.DeviceConstraint".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceConstraint must have exactly one field set besides Requests.",
            "type": "object",
            "properties": {
                "distinctAttribute": {
                    "description": "DistinctAttribute requires that all devices in question have this attribute and that its type and value are unique across those devices.\n\nThis acts as the inverse of MatchAttribute.\n\nThis constraint is used to avoid allocating multiple requests to the same device by ensuring attribute-level differentiation.\n\nThis is useful for scenarios where resource requests must be fulfilled by separate physical devices. For example, a container requests two network interfaces that must be allocated from two different physical NICs.",
                    "type": "string",
                },
                "matchAttribute": {
                    "description": "MatchAttribute requires that all devices in question have this attribute and that its type and value are the same across those devices.\n\nFor example, if you specified \"dra.example.com/numa\" (a hypothetical example!), then only devices in the same NUMA node will be chosen. A device which does not have that attribute will not be chosen. All devices should use a value of the same type for this attribute because that is part of its specification, but if one device doesn't, then it also will not be chosen.\n\nMust include the domain qualifier.",
                    "type": "string",
                },
                "requests": {
                    "description": "Requests is a list of the one or more requests in this claim which must co-satisfy this constraint. If a request is fulfilled by multiple devices, then all of the devices must satisfy the constraint. If this is not specified, this constraint applies to all requests in this claim.\n\nReferences to subrequests must include the name of the main request and may include the subrequest using the format <main request>[/<subrequest>]. If just the main request is given, the constraint applies to all subrequests.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
