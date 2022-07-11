// Generated from definition io.k8s.api.apps.v1.DaemonSetUpdateStrategy

/// DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DaemonSetUpdateStrategy {
    /// Rolling update config params. Present only if type = "RollingUpdate".
    pub rolling_update: Option<crate::api::apps::v1::RollingUpdateDaemonSet>,

    /// Type of daemon set update. Can be "RollingUpdate" or "OnDelete". Default is RollingUpdate.
    pub type_: Option<String>,

}

#[cfg(feature = "dsl")]
impl DaemonSetUpdateStrategy  {
    /// Set [`Self::rolling_update`]
    pub  fn rolling_update_set(&mut self, rolling_update: impl Into<Option<crate::api::apps::v1::RollingUpdateDaemonSet>>) -> &mut Self {
        self.rolling_update = rolling_update.into(); self
    }

    pub  fn rolling_update(&mut self) -> &mut crate::api::apps::v1::RollingUpdateDaemonSet {
        if self.rolling_update.is_none() { self.rolling_update = Some(Default::default()) }
        self.rolling_update.as_mut().unwrap()
    }

    /// Modify [`Self::rolling_update`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn rolling_update_with(&mut self, func: impl FnOnce(&mut crate::api::apps::v1::RollingUpdateDaemonSet)) -> &mut Self {
        if self.rolling_update.is_none() { self.rolling_update = Some(Default::default()) };
        func(self.rolling_update.as_mut().unwrap()); self
    }


    /// Set [`Self::type_`]
    pub  fn type_set(&mut self, type_: impl Into<Option<String>>) -> &mut Self {
        self.type_ = type_.into(); self
    }

    pub  fn type_(&mut self) -> &mut String {
        if self.type_.is_none() { self.type_ = Some(Default::default()) }
        self.type_.as_mut().unwrap()
    }

    /// Modify [`Self::type_`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.type_.is_none() { self.type_ = Some(Default::default()) };
        func(self.type_.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for DaemonSetUpdateStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_rolling_update,
            Key_type_,
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
                            "rollingUpdate" => Field::Key_rolling_update,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DaemonSetUpdateStrategy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DaemonSetUpdateStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_rolling_update: Option<crate::api::apps::v1::RollingUpdateDaemonSet> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_rolling_update => value_rolling_update = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DaemonSetUpdateStrategy {
                    rolling_update: value_rolling_update,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "DaemonSetUpdateStrategy",
            &[
                "rollingUpdate",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DaemonSetUpdateStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DaemonSetUpdateStrategy",
            self.rolling_update.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.rolling_update {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rollingUpdate", value)?;
        }
        if let Some(value) = &self.type_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DaemonSetUpdateStrategy {
    fn schema_name() -> String {
        "io.k8s.api.apps.v1.DaemonSetUpdateStrategy".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "rollingUpdate".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::apps::v1::RollingUpdateDaemonSet>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Rolling update config params. Present only if type = \"RollingUpdate\".".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Type of daemon set update. Can be \"RollingUpdate\" or \"OnDelete\". Default is RollingUpdate.".to_owned()),
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
