// Generated from definition io.k8s.api.core.v1.SecretVolumeSource

/// Adapts a Secret into a volume.
///
/// The contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names. Secret volumes support ownership management and SELinux relabeling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecretVolumeSource {
    /// defaultMode is Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    pub default_mode: Option<i32>,

    /// items If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    pub items: Option<Vec<crate::api::core::v1::KeyToPath>>,

    /// optional field specify whether the Secret or its keys must be defined
    pub optional: Option<bool>,

    /// secretName is the name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    pub secret_name: Option<String>,

}

#[cfg(feature = "dsl")]
impl SecretVolumeSource  {
    /// Set [`Self::default_mode`]
    pub  fn default_mode_set(&mut self, default_mode: impl Into<Option<i32>>) -> &mut Self {
        self.default_mode = default_mode.into(); self
    }

    pub  fn default_mode(&mut self) -> &mut i32 {
        if self.default_mode.is_none() { self.default_mode = Some(Default::default()) }
        self.default_mode.as_mut().unwrap()
    }

    /// Modify [`Self::default_mode`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn default_mode_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.default_mode.is_none() { self.default_mode = Some(Default::default()) };
        func(self.default_mode.as_mut().unwrap()); self
    }


    /// Set [`Self::items`]
    pub  fn items_set(&mut self, items: impl Into<Option<Vec<crate::api::core::v1::KeyToPath>>>) -> &mut Self {
        self.items = items.into(); self
    }

    pub  fn items(&mut self) -> &mut Vec<crate::api::core::v1::KeyToPath> {
        if self.items.is_none() { self.items = Some(Default::default()) }
        self.items.as_mut().unwrap()
    }

    /// Modify [`Self::items`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn items_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::KeyToPath>)) -> &mut Self {
        if self.items.is_none() { self.items = Some(Default::default()) };
        func(self.items.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::items`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn items_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::KeyToPath)) -> &mut Self {
        if self.items.is_none() {
            self.items = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.items.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::items`]
    pub  fn items_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::KeyToPath]>) -> &mut Self {
         if self.items.is_none() { self.items = Some(Vec::new()); }
         let items = &mut self.items.as_mut().unwrap();
         for item in other.borrow() {
             items.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::optional`]
    pub  fn optional_set(&mut self, optional: impl Into<Option<bool>>) -> &mut Self {
        self.optional = optional.into(); self
    }

    pub  fn optional(&mut self) -> &mut bool {
        if self.optional.is_none() { self.optional = Some(Default::default()) }
        self.optional.as_mut().unwrap()
    }

    /// Modify [`Self::optional`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn optional_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.optional.is_none() { self.optional = Some(Default::default()) };
        func(self.optional.as_mut().unwrap()); self
    }


    /// Set [`Self::secret_name`]
    pub  fn secret_name_set(&mut self, secret_name: impl Into<Option<String>>) -> &mut Self {
        self.secret_name = secret_name.into(); self
    }

    pub  fn secret_name(&mut self) -> &mut String {
        if self.secret_name.is_none() { self.secret_name = Some(Default::default()) }
        self.secret_name.as_mut().unwrap()
    }

    /// Modify [`Self::secret_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn secret_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.secret_name.is_none() { self.secret_name = Some(Default::default()) };
        func(self.secret_name.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for SecretVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default_mode,
            Key_items,
            Key_optional,
            Key_secret_name,
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
                            "defaultMode" => Field::Key_default_mode,
                            "items" => Field::Key_items,
                            "optional" => Field::Key_optional,
                            "secretName" => Field::Key_secret_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SecretVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SecretVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_default_mode: Option<i32> = None;
                let mut value_items: Option<Vec<crate::api::core::v1::KeyToPath>> = None;
                let mut value_optional: Option<bool> = None;
                let mut value_secret_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default_mode => value_default_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_items => value_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional => value_optional = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret_name => value_secret_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SecretVolumeSource {
                    default_mode: value_default_mode,
                    items: value_items,
                    optional: value_optional,
                    secret_name: value_secret_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "SecretVolumeSource",
            &[
                "defaultMode",
                "items",
                "optional",
                "secretName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SecretVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SecretVolumeSource",
            self.default_mode.as_ref().map_or(0, |_| 1) +
            self.items.as_ref().map_or(0, |_| 1) +
            self.optional.as_ref().map_or(0, |_| 1) +
            self.secret_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.default_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultMode", value)?;
        }
        if let Some(value) = &self.items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "items", value)?;
        }
        if let Some(value) = &self.optional {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "optional", value)?;
        }
        if let Some(value) = &self.secret_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secretName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SecretVolumeSource {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.SecretVolumeSource".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Adapts a Secret into a volume.\n\nThe contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names. Secret volumes support ownership management and SELinux relabeling.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "defaultMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("defaultMode is Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "items".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("items If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::KeyToPath>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "optional".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("optional field specify whether the Secret or its keys must be defined".to_owned()),
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
                                description: Some("secretName is the name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret".to_owned()),
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
