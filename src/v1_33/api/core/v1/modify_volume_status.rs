// Generated from definition io.k8s.api.core.v1.ModifyVolumeStatus

/// ModifyVolumeStatus represents the status object of ControllerModifyVolume operation
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModifyVolumeStatus {
    /// status is the status of the ControllerModifyVolume operation. It can be in any of following states:
    ///  - Pending
    ///    Pending indicates that the PersistentVolumeClaim cannot be modified due to unmet requirements, such as
    ///    the specified VolumeAttributesClass not existing.
    ///  - InProgress
    ///    InProgress indicates that the volume is being modified.
    ///  - Infeasible
    ///   Infeasible indicates that the request has been rejected as invalid by the CSI driver. To
    ///       resolve the error, a valid VolumeAttributesClass needs to be specified.
    /// Note: New statuses can be added in the future. Consumers should check for unknown statuses and fail appropriately.
    pub status: std::string::String,

    /// targetVolumeAttributesClassName is the name of the VolumeAttributesClass the PVC currently being reconciled
    pub target_volume_attributes_class_name: Option<std::string::String>,
}

impl crate::DeepMerge for ModifyVolumeStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.status, other.status);
        crate::DeepMerge::merge_from(&mut self.target_volume_attributes_class_name, other.target_volume_attributes_class_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ModifyVolumeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_status,
            Key_target_volume_attributes_class_name,
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
                            "status" => Field::Key_status,
                            "targetVolumeAttributesClassName" => Field::Key_target_volume_attributes_class_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ModifyVolumeStatus;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ModifyVolumeStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_status: Option<std::string::String> = None;
                let mut value_target_volume_attributes_class_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_volume_attributes_class_name => value_target_volume_attributes_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ModifyVolumeStatus {
                    status: value_status.unwrap_or_default(),
                    target_volume_attributes_class_name: value_target_volume_attributes_class_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "ModifyVolumeStatus",
            &[
                "status",
                "targetVolumeAttributesClassName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ModifyVolumeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ModifyVolumeStatus",
            1 +
            self.target_volume_attributes_class_name.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        if let Some(value) = &self.target_volume_attributes_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetVolumeAttributesClassName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ModifyVolumeStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.ModifyVolumeStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ModifyVolumeStatus represents the status object of ControllerModifyVolume operation".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "status".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("status is the status of the ControllerModifyVolume operation. It can be in any of following states:\n - Pending\n   Pending indicates that the PersistentVolumeClaim cannot be modified due to unmet requirements, such as\n   the specified VolumeAttributesClass not existing.\n - InProgress\n   InProgress indicates that the volume is being modified.\n - Infeasible\n  Infeasible indicates that the request has been rejected as invalid by the CSI driver. To\n\t  resolve the error, a valid VolumeAttributesClass needs to be specified.\nNote: New statuses can be added in the future. Consumers should check for unknown statuses and fail appropriately.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "targetVolumeAttributesClassName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("targetVolumeAttributesClassName is the name of the VolumeAttributesClass the PVC currently being reconciled".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "status".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
