// Generated from definition io.k8s.api.core.v1.AzureFileVolumeSource

/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AzureFileVolumeSource {
    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    pub read_only: Option<bool>,

    /// secretName is the  name of secret that contains Azure Storage Account Name and Key
    pub secret_name: String,

    /// shareName is the azure share Name
    pub share_name: String,

}

#[cfg(feature = "dsl")]
impl AzureFileVolumeSource  {
    /// Set [`Self::read_only`]
    pub  fn read_only_set(&mut self, read_only: impl Into<Option<bool>>) -> &mut Self {
        self.read_only = read_only.into(); self
    }

    pub  fn read_only(&mut self) -> &mut bool {
        if self.read_only.is_none() { self.read_only = Some(Default::default()) }
        self.read_only.as_mut().unwrap()
    }

    /// Modify [`Self::read_only`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn read_only_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.read_only.is_none() { self.read_only = Some(Default::default()) };
        func(self.read_only.as_mut().unwrap()); self
    }


    /// Set [`Self::secret_name`]
    pub  fn secret_name_set(&mut self, secret_name: impl Into<String>) -> &mut Self {
        self.secret_name = secret_name.into(); self
    }

    pub  fn secret_name(&mut self) -> &mut String {
        &mut self.secret_name
    }

    /// Modify [`Self::secret_name`] with a `func`
    pub  fn secret_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.secret_name); self
    }


    /// Set [`Self::share_name`]
    pub  fn share_name_set(&mut self, share_name: impl Into<String>) -> &mut Self {
        self.share_name = share_name.into(); self
    }

    pub  fn share_name(&mut self) -> &mut String {
        &mut self.share_name
    }

    /// Modify [`Self::share_name`] with a `func`
    pub  fn share_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.share_name); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for AzureFileVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_read_only,
            Key_secret_name,
            Key_share_name,
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
                            "readOnly" => Field::Key_read_only,
                            "secretName" => Field::Key_secret_name,
                            "shareName" => Field::Key_share_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = AzureFileVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AzureFileVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_read_only: Option<bool> = None;
                let mut value_secret_name: Option<String> = None;
                let mut value_share_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_read_only => value_read_only = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_name => value_secret_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_share_name => value_share_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AzureFileVolumeSource {
                    read_only: value_read_only,
                    secret_name: value_secret_name.unwrap_or_default(),
                    share_name: value_share_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "AzureFileVolumeSource",
            &[
                "readOnly",
                "secretName",
                "shareName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AzureFileVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AzureFileVolumeSource",
            2 +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.read_only {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretName", &self.secret_name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "shareName", &self.share_name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AzureFileVolumeSource {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.AzureFileVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("AzureFile represents an Azure File Service mount on the host and bind mount to the pod.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "readOnly".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "secretName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("secretName is the  name of secret that contains Azure Storage Account Name and Key".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "shareName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("shareName is the azure share Name".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "secretName".to_owned(),
                    "shareName".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
