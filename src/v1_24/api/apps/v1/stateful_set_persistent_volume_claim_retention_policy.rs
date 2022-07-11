// Generated from definition io.k8s.api.apps.v1.StatefulSetPersistentVolumeClaimRetentionPolicy

/// StatefulSetPersistentVolumeClaimRetentionPolicy describes the policy used for PVCs created from the StatefulSet VolumeClaimTemplates.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatefulSetPersistentVolumeClaimRetentionPolicy {
    /// WhenDeleted specifies what happens to PVCs created from StatefulSet VolumeClaimTemplates when the StatefulSet is deleted. The default policy of `Retain` causes PVCs to not be affected by StatefulSet deletion. The `Delete` policy causes those PVCs to be deleted.
    pub when_deleted: Option<String>,

    /// WhenScaled specifies what happens to PVCs created from StatefulSet VolumeClaimTemplates when the StatefulSet is scaled down. The default policy of `Retain` causes PVCs to not be affected by a scaledown. The `Delete` policy causes the associated PVCs for any excess pods above the replica count to be deleted.
    pub when_scaled: Option<String>,

}

#[cfg(feature = "dsl")]
impl StatefulSetPersistentVolumeClaimRetentionPolicy  {
    /// Set [`Self::when_deleted`]
    pub  fn when_deleted_set(&mut self, when_deleted: impl Into<Option<String>>) -> &mut Self {
        self.when_deleted = when_deleted.into(); self
    }

    pub  fn when_deleted(&mut self) -> &mut String {
        if self.when_deleted.is_none() { self.when_deleted = Some(Default::default()) }
        self.when_deleted.as_mut().unwrap()
    }

    /// Modify [`Self::when_deleted`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn when_deleted_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.when_deleted.is_none() { self.when_deleted = Some(Default::default()) };
        func(self.when_deleted.as_mut().unwrap()); self
    }


    /// Set [`Self::when_scaled`]
    pub  fn when_scaled_set(&mut self, when_scaled: impl Into<Option<String>>) -> &mut Self {
        self.when_scaled = when_scaled.into(); self
    }

    pub  fn when_scaled(&mut self) -> &mut String {
        if self.when_scaled.is_none() { self.when_scaled = Some(Default::default()) }
        self.when_scaled.as_mut().unwrap()
    }

    /// Modify [`Self::when_scaled`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn when_scaled_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.when_scaled.is_none() { self.when_scaled = Some(Default::default()) };
        func(self.when_scaled.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for StatefulSetPersistentVolumeClaimRetentionPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_when_deleted,
            Key_when_scaled,
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
                            "whenDeleted" => Field::Key_when_deleted,
                            "whenScaled" => Field::Key_when_scaled,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = StatefulSetPersistentVolumeClaimRetentionPolicy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StatefulSetPersistentVolumeClaimRetentionPolicy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_when_deleted: Option<String> = None;
                let mut value_when_scaled: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_when_deleted => value_when_deleted = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_when_scaled => value_when_scaled = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatefulSetPersistentVolumeClaimRetentionPolicy {
                    when_deleted: value_when_deleted,
                    when_scaled: value_when_scaled,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatefulSetPersistentVolumeClaimRetentionPolicy",
            &[
                "whenDeleted",
                "whenScaled",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for StatefulSetPersistentVolumeClaimRetentionPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatefulSetPersistentVolumeClaimRetentionPolicy",
            self.when_deleted.as_ref().map_or(0, |_| 1) +
            self.when_scaled.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.when_deleted {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "whenDeleted", value)?;
        }
        if let Some(value) = &self.when_scaled {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "whenScaled", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for StatefulSetPersistentVolumeClaimRetentionPolicy {
    fn schema_name() -> String {
        "io.k8s.api.apps.v1.StatefulSetPersistentVolumeClaimRetentionPolicy".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("StatefulSetPersistentVolumeClaimRetentionPolicy describes the policy used for PVCs created from the StatefulSet VolumeClaimTemplates.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "whenDeleted".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("WhenDeleted specifies what happens to PVCs created from StatefulSet VolumeClaimTemplates when the StatefulSet is deleted. The default policy of `Retain` causes PVCs to not be affected by StatefulSet deletion. The `Delete` policy causes those PVCs to be deleted.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "whenScaled".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("WhenScaled specifies what happens to PVCs created from StatefulSet VolumeClaimTemplates when the StatefulSet is scaled down. The default policy of `Retain` causes PVCs to not be affected by a scaledown. The `Delete` policy causes the associated PVCs for any excess pods above the replica count to be deleted.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
