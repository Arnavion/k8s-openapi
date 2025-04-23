// Generated from definition io.k8s.api.networking.v1.IPAddressSpec

/// IPAddressSpec describe the attributes in an IP Address.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IPAddressSpec {
    /// ParentRef references the resource that an IPAddress is attached to. An IPAddress must reference a parent object.
    pub parent_ref: crate::api::networking::v1::ParentReference,
}

impl crate::DeepMerge for IPAddressSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.parent_ref, other.parent_ref);
    }
}

impl<'de> crate::serde::Deserialize<'de> for IPAddressSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_parent_ref,
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
                            "parentRef" => Field::Key_parent_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IPAddressSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("IPAddressSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_parent_ref: Option<crate::api::networking::v1::ParentReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_parent_ref => value_parent_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IPAddressSpec {
                    parent_ref: value_parent_ref.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "IPAddressSpec",
            &[
                "parentRef",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IPAddressSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IPAddressSpec",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parentRef", &self.parent_ref)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for IPAddressSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.networking.v1.IPAddressSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("IPAddressSpec describe the attributes in an IP Address.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "parentRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::networking::v1::ParentReference>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ParentRef references the resource that an IPAddress is attached to. An IPAddress must reference a parent object.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "parentRef".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
