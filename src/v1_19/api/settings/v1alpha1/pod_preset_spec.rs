// Generated from definition io.k8s.api.settings.v1alpha1.PodPresetSpec

/// PodPresetSpec is a description of a pod preset.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodPresetSpec {
    /// Env defines the collection of EnvVar to inject into containers.
    pub env: Option<Vec<crate::api::core::v1::EnvVar>>,

    /// EnvFrom defines the collection of EnvFromSource to inject into containers.
    pub env_from: Option<Vec<crate::api::core::v1::EnvFromSource>>,

    /// Selector is a label query over a set of resources, in this case pods. Required.
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// VolumeMounts defines the collection of VolumeMount to inject into containers.
    pub volume_mounts: Option<Vec<crate::api::core::v1::VolumeMount>>,

    /// Volumes defines the collection of Volume to inject into the pod.
    pub volumes: Option<Vec<crate::api::core::v1::Volume>>,

}

#[cfg(feature = "dsl")]
impl PodPresetSpec  {
    /// Set [`Self::env`]
    pub  fn env_set(&mut self, env: impl Into<Option<Vec<crate::api::core::v1::EnvVar>>>) -> &mut Self {
        self.env = env.into(); self
    }

    pub  fn env(&mut self) -> &mut Vec<crate::api::core::v1::EnvVar> {
        if self.env.is_none() { self.env = Some(Default::default()) }
        self.env.as_mut().unwrap()
    }

    /// Modify [`Self::env`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn env_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::EnvVar>)) -> &mut Self {
        if self.env.is_none() { self.env = Some(Default::default()) };
        func(self.env.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::env`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn env_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::EnvVar)) -> &mut Self {
        if self.env.is_none() {
            self.env = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.env.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::env`]
    pub  fn env_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::EnvVar]>) -> &mut Self {
         if self.env.is_none() { self.env = Some(Vec::new()); }
         let env = &mut self.env.as_mut().unwrap();
         for item in other.borrow() {
             env.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::env_from`]
    pub  fn env_from_set(&mut self, env_from: impl Into<Option<Vec<crate::api::core::v1::EnvFromSource>>>) -> &mut Self {
        self.env_from = env_from.into(); self
    }

    pub  fn env_from(&mut self) -> &mut Vec<crate::api::core::v1::EnvFromSource> {
        if self.env_from.is_none() { self.env_from = Some(Default::default()) }
        self.env_from.as_mut().unwrap()
    }

    /// Modify [`Self::env_from`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn env_from_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::EnvFromSource>)) -> &mut Self {
        if self.env_from.is_none() { self.env_from = Some(Default::default()) };
        func(self.env_from.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::env_from`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn env_from_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::EnvFromSource)) -> &mut Self {
        if self.env_from.is_none() {
            self.env_from = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.env_from.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::env_from`]
    pub  fn env_from_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::EnvFromSource]>) -> &mut Self {
         if self.env_from.is_none() { self.env_from = Some(Vec::new()); }
         let env_from = &mut self.env_from.as_mut().unwrap();
         for item in other.borrow() {
             env_from.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::selector`]
    pub  fn selector_set(&mut self, selector: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>>) -> &mut Self {
        self.selector = selector.into(); self
    }

    pub  fn selector(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector {
        if self.selector.is_none() { self.selector = Some(Default::default()) }
        self.selector.as_mut().unwrap()
    }

    /// Modify [`Self::selector`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn selector_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector)) -> &mut Self {
        if self.selector.is_none() { self.selector = Some(Default::default()) };
        func(self.selector.as_mut().unwrap()); self
    }


    /// Set [`Self::volume_mounts`]
    pub  fn volume_mounts_set(&mut self, volume_mounts: impl Into<Option<Vec<crate::api::core::v1::VolumeMount>>>) -> &mut Self {
        self.volume_mounts = volume_mounts.into(); self
    }

    pub  fn volume_mounts(&mut self) -> &mut Vec<crate::api::core::v1::VolumeMount> {
        if self.volume_mounts.is_none() { self.volume_mounts = Some(Default::default()) }
        self.volume_mounts.as_mut().unwrap()
    }

    /// Modify [`Self::volume_mounts`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volume_mounts_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::VolumeMount>)) -> &mut Self {
        if self.volume_mounts.is_none() { self.volume_mounts = Some(Default::default()) };
        func(self.volume_mounts.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::volume_mounts`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn volume_mounts_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::VolumeMount)) -> &mut Self {
        if self.volume_mounts.is_none() {
            self.volume_mounts = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.volume_mounts.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::volume_mounts`]
    pub  fn volume_mounts_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::VolumeMount]>) -> &mut Self {
         if self.volume_mounts.is_none() { self.volume_mounts = Some(Vec::new()); }
         let volume_mounts = &mut self.volume_mounts.as_mut().unwrap();
         for item in other.borrow() {
             volume_mounts.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::volumes`]
    pub  fn volumes_set(&mut self, volumes: impl Into<Option<Vec<crate::api::core::v1::Volume>>>) -> &mut Self {
        self.volumes = volumes.into(); self
    }

    pub  fn volumes(&mut self) -> &mut Vec<crate::api::core::v1::Volume> {
        if self.volumes.is_none() { self.volumes = Some(Default::default()) }
        self.volumes.as_mut().unwrap()
    }

    /// Modify [`Self::volumes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volumes_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::Volume>)) -> &mut Self {
        if self.volumes.is_none() { self.volumes = Some(Default::default()) };
        func(self.volumes.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::volumes`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn volumes_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::Volume)) -> &mut Self {
        if self.volumes.is_none() {
            self.volumes = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.volumes.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::volumes`]
    pub  fn volumes_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::Volume]>) -> &mut Self {
         if self.volumes.is_none() { self.volumes = Some(Vec::new()); }
         let volumes = &mut self.volumes.as_mut().unwrap();
         for item in other.borrow() {
             volumes.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for PodPresetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_env,
            Key_env_from,
            Key_selector,
            Key_volume_mounts,
            Key_volumes,
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
                            "env" => Field::Key_env,
                            "envFrom" => Field::Key_env_from,
                            "selector" => Field::Key_selector,
                            "volumeMounts" => Field::Key_volume_mounts,
                            "volumes" => Field::Key_volumes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodPresetSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodPresetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_env: Option<Vec<crate::api::core::v1::EnvVar>> = None;
                let mut value_env_from: Option<Vec<crate::api::core::v1::EnvFromSource>> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_volume_mounts: Option<Vec<crate::api::core::v1::VolumeMount>> = None;
                let mut value_volumes: Option<Vec<crate::api::core::v1::Volume>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_env => value_env = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env_from => value_env_from = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mounts => value_volume_mounts = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes => value_volumes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodPresetSpec {
                    env: value_env,
                    env_from: value_env_from,
                    selector: value_selector,
                    volume_mounts: value_volume_mounts,
                    volumes: value_volumes,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodPresetSpec",
            &[
                "env",
                "envFrom",
                "selector",
                "volumeMounts",
                "volumes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodPresetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodPresetSpec",
            self.env.as_ref().map_or(0, |_| 1) +
            self.env_from.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.volume_mounts.as_ref().map_or(0, |_| 1) +
            self.volumes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.env {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "env", value)?;
        }
        if let Some(value) = &self.env_from {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "envFrom", value)?;
        }
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.volume_mounts {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeMounts", value)?;
        }
        if let Some(value) = &self.volumes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodPresetSpec {
    fn schema_name() -> String {
        "io.k8s.api.settings.v1alpha1.PodPresetSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PodPresetSpec is a description of a pod preset.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "env".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Env defines the collection of EnvVar to inject into containers.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::EnvVar>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "envFrom".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EnvFrom defines the collection of EnvFromSource to inject into containers.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::EnvFromSource>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Selector is a label query over a set of resources, in this case pods. Required.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "volumeMounts".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VolumeMounts defines the collection of VolumeMount to inject into containers.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::VolumeMount>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "volumes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Volumes defines the collection of Volume to inject into the pod.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::Volume>()))),
                                ..Default::default()
                            })),
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
