// Generated from definition io.k8s.api.core.v1.ContainerStatus

/// ContainerStatus contains details for the current status of this container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerStatus {
    /// Container's ID in the format '\<type\>://\<container_id\>'.
    pub container_id: Option<String>,

    /// The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images.
    pub image: String,

    /// ImageID of the container's image.
    pub image_id: String,

    /// Details about the container's last termination condition.
    pub last_state: Option<crate::api::core::v1::ContainerState>,

    /// This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated.
    pub name: String,

    /// Specifies whether the container has passed its readiness probe.
    pub ready: bool,

    /// The number of times the container has been restarted.
    pub restart_count: i32,

    /// Specifies whether the container has passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. Is always true when no startupProbe is defined.
    pub started: Option<bool>,

    /// Details about the container's current condition.
    pub state: Option<crate::api::core::v1::ContainerState>,

}

#[cfg(feature = "dsl")]
impl ContainerStatus  {
    /// Set [`Self::container_id`]
    pub  fn container_id_set(&mut self, container_id: impl Into<Option<String>>) -> &mut Self {
        self.container_id = container_id.into(); self
    }

    pub  fn container_id(&mut self) -> &mut String {
        if self.container_id.is_none() { self.container_id = Some(Default::default()) }
        self.container_id.as_mut().unwrap()
    }

    /// Modify [`Self::container_id`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn container_id_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.container_id.is_none() { self.container_id = Some(Default::default()) };
        func(self.container_id.as_mut().unwrap()); self
    }


    /// Set [`Self::image`]
    pub  fn image_set(&mut self, image: impl Into<String>) -> &mut Self {
        self.image = image.into(); self
    }

    pub  fn image(&mut self) -> &mut String {
        &mut self.image
    }

    /// Modify [`Self::image`] with a `func`
    pub  fn image_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.image); self
    }


    /// Set [`Self::image_id`]
    pub  fn image_id_set(&mut self, image_id: impl Into<String>) -> &mut Self {
        self.image_id = image_id.into(); self
    }

    pub  fn image_id(&mut self) -> &mut String {
        &mut self.image_id
    }

    /// Modify [`Self::image_id`] with a `func`
    pub  fn image_id_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.image_id); self
    }


    /// Set [`Self::last_state`]
    pub  fn last_state_set(&mut self, last_state: impl Into<Option<crate::api::core::v1::ContainerState>>) -> &mut Self {
        self.last_state = last_state.into(); self
    }

    pub  fn last_state(&mut self) -> &mut crate::api::core::v1::ContainerState {
        if self.last_state.is_none() { self.last_state = Some(Default::default()) }
        self.last_state.as_mut().unwrap()
    }

    /// Modify [`Self::last_state`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn last_state_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ContainerState)) -> &mut Self {
        if self.last_state.is_none() { self.last_state = Some(Default::default()) };
        func(self.last_state.as_mut().unwrap()); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        &mut self.name
    }

    /// Modify [`Self::name`] with a `func`
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.name); self
    }


    /// Set [`Self::ready`]
    pub  fn ready_set(&mut self, ready: impl Into<bool>) -> &mut Self {
        self.ready = ready.into(); self
    }

    pub  fn ready(&mut self) -> &mut bool {
        &mut self.ready
    }

    /// Modify [`Self::ready`] with a `func`
    pub  fn ready_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        func(&mut self.ready); self
    }


    /// Set [`Self::restart_count`]
    pub  fn restart_count_set(&mut self, restart_count: impl Into<i32>) -> &mut Self {
        self.restart_count = restart_count.into(); self
    }

    pub  fn restart_count(&mut self) -> &mut i32 {
        &mut self.restart_count
    }

    /// Modify [`Self::restart_count`] with a `func`
    pub  fn restart_count_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        func(&mut self.restart_count); self
    }


    /// Set [`Self::started`]
    pub  fn started_set(&mut self, started: impl Into<Option<bool>>) -> &mut Self {
        self.started = started.into(); self
    }

    pub  fn started(&mut self) -> &mut bool {
        if self.started.is_none() { self.started = Some(Default::default()) }
        self.started.as_mut().unwrap()
    }

    /// Modify [`Self::started`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn started_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.started.is_none() { self.started = Some(Default::default()) };
        func(self.started.as_mut().unwrap()); self
    }


    /// Set [`Self::state`]
    pub  fn state_set(&mut self, state: impl Into<Option<crate::api::core::v1::ContainerState>>) -> &mut Self {
        self.state = state.into(); self
    }

    pub  fn state(&mut self) -> &mut crate::api::core::v1::ContainerState {
        if self.state.is_none() { self.state = Some(Default::default()) }
        self.state.as_mut().unwrap()
    }

    /// Modify [`Self::state`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn state_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ContainerState)) -> &mut Self {
        if self.state.is_none() { self.state = Some(Default::default()) };
        func(self.state.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for ContainerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_id,
            Key_image,
            Key_image_id,
            Key_last_state,
            Key_name,
            Key_ready,
            Key_restart_count,
            Key_started,
            Key_state,
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
                            "containerID" => Field::Key_container_id,
                            "image" => Field::Key_image,
                            "imageID" => Field::Key_image_id,
                            "lastState" => Field::Key_last_state,
                            "name" => Field::Key_name,
                            "ready" => Field::Key_ready,
                            "restartCount" => Field::Key_restart_count,
                            "started" => Field::Key_started,
                            "state" => Field::Key_state,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContainerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_id: Option<String> = None;
                let mut value_image: Option<String> = None;
                let mut value_image_id: Option<String> = None;
                let mut value_last_state: Option<crate::api::core::v1::ContainerState> = None;
                let mut value_name: Option<String> = None;
                let mut value_ready: Option<bool> = None;
                let mut value_restart_count: Option<i32> = None;
                let mut value_started: Option<bool> = None;
                let mut value_state: Option<crate::api::core::v1::ContainerState> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_id => value_container_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image => value_image = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_id => value_image_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_last_state => value_last_state = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ready => value_ready = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_restart_count => value_restart_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_started => value_started = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerStatus {
                    container_id: value_container_id,
                    image: value_image.unwrap_or_default(),
                    image_id: value_image_id.unwrap_or_default(),
                    last_state: value_last_state,
                    name: value_name.unwrap_or_default(),
                    ready: value_ready.unwrap_or_default(),
                    restart_count: value_restart_count.unwrap_or_default(),
                    started: value_started,
                    state: value_state,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerStatus",
            &[
                "containerID",
                "image",
                "imageID",
                "lastState",
                "name",
                "ready",
                "restartCount",
                "started",
                "state",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ContainerStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerStatus",
            5 +
            self.container_id.as_ref().map_or(0, |_| 1) +
            self.last_state.as_ref().map_or(0, |_| 1) +
            self.started.as_ref().map_or(0, |_| 1) +
            self.state.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.container_id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerID", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "image", &self.image)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "imageID", &self.image_id)?;
        if let Some(value) = &self.last_state {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lastState", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ready", &self.ready)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "restartCount", &self.restart_count)?;
        if let Some(value) = &self.started {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "started", value)?;
        }
        if let Some(value) = &self.state {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "state", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ContainerStatus {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ContainerStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ContainerStatus contains details for the current status of this container.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "containerID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Container's ID in the format '<type>://<container_id>'.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "image".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "imageID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ImageID of the container's image.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "lastState".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerState>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Details about the container's last termination condition.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ready".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies whether the container has passed its readiness probe.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "restartCount".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The number of times the container has been restarted.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "started".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specifies whether the container has passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. Is always true when no startupProbe is defined.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "state".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ContainerState>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Details about the container's current condition.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "image".to_owned(),
                    "imageID".to_owned(),
                    "name".to_owned(),
                    "ready".to_owned(),
                    "restartCount".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
