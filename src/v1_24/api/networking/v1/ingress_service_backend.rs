// Generated from definition io.k8s.api.networking.v1.IngressServiceBackend

/// IngressServiceBackend references a Kubernetes Service as a Backend.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressServiceBackend {
    /// Name is the referenced service. The service must exist in the same namespace as the Ingress object.
    pub name: String,

    /// Port of the referenced service. A port name or port number is required for a IngressServiceBackend.
    pub port: Option<crate::api::networking::v1::ServiceBackendPort>,

}

#[cfg(feature = "dsl")]
impl IngressServiceBackend  {
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


    /// Set [`Self::port`]
    pub  fn port_set(&mut self, port: impl Into<Option<crate::api::networking::v1::ServiceBackendPort>>) -> &mut Self {
        self.port = port.into(); self
    }

    pub  fn port(&mut self) -> &mut crate::api::networking::v1::ServiceBackendPort {
        if self.port.is_none() { self.port = Some(Default::default()) }
        self.port.as_mut().unwrap()
    }

    /// Modify [`Self::port`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn port_with(&mut self, func: impl FnOnce(&mut crate::api::networking::v1::ServiceBackendPort)) -> &mut Self {
        if self.port.is_none() { self.port = Some(Default::default()) };
        func(self.port.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for IngressServiceBackend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_port,
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
                            "name" => Field::Key_name,
                            "port" => Field::Key_port,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IngressServiceBackend;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IngressServiceBackend")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_port: Option<crate::api::networking::v1::ServiceBackendPort> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port => value_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressServiceBackend {
                    name: value_name.unwrap_or_default(),
                    port: value_port,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressServiceBackend",
            &[
                "name",
                "port",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IngressServiceBackend {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressServiceBackend",
            1 +
            self.port.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "port", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for IngressServiceBackend {
    fn schema_name() -> String {
        "io.k8s.api.networking.v1.IngressServiceBackend".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("IngressServiceBackend references a Kubernetes Service as a Backend.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name is the referenced service. The service must exist in the same namespace as the Ingress object.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "port".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::networking::v1::ServiceBackendPort>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Port of the referenced service. A port name or port number is required for a IngressServiceBackend.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "name".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
