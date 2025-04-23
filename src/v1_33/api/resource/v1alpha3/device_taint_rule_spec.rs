// Generated from definition io.k8s.api.resource.v1alpha3.DeviceTaintRuleSpec

/// DeviceTaintRuleSpec specifies the selector and one taint.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceTaintRuleSpec {
    /// DeviceSelector defines which device(s) the taint is applied to. All selector criteria must be satified for a device to match. The empty selector matches all devices. Without a selector, no devices are matches.
    pub device_selector: Option<crate::api::resource::v1alpha3::DeviceTaintSelector>,

    /// The taint that gets applied to matching devices.
    pub taint: crate::api::resource::v1alpha3::DeviceTaint,
}

impl crate::DeepMerge for DeviceTaintRuleSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.device_selector, other.device_selector);
        crate::DeepMerge::merge_from(&mut self.taint, other.taint);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceTaintRuleSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_device_selector,
            Key_taint,
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
                            "deviceSelector" => Field::Key_device_selector,
                            "taint" => Field::Key_taint,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceTaintRuleSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceTaintRuleSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_device_selector: Option<crate::api::resource::v1alpha3::DeviceTaintSelector> = None;
                let mut value_taint: Option<crate::api::resource::v1alpha3::DeviceTaint> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_device_selector => value_device_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_taint => value_taint = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceTaintRuleSpec {
                    device_selector: value_device_selector,
                    taint: value_taint.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceTaintRuleSpec",
            &[
                "deviceSelector",
                "taint",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceTaintRuleSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceTaintRuleSpec",
            1 +
            self.device_selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.device_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deviceSelector", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "taint", &self.taint)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceTaintRuleSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1alpha3.DeviceTaintRuleSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceTaintRuleSpec specifies the selector and one taint.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "deviceSelector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha3::DeviceTaintSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("DeviceSelector defines which device(s) the taint is applied to. All selector criteria must be satified for a device to match. The empty selector matches all devices. Without a selector, no devices are matches.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "taint".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::resource::v1alpha3::DeviceTaint>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The taint that gets applied to matching devices.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "taint".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
