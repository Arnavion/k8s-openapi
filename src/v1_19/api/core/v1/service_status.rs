// Generated from definition io.k8s.api.core.v1.ServiceStatus

/// ServiceStatus represents the current status of a service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceStatus {
    /// LoadBalancer contains the current status of the load-balancer, if one is present.
    pub load_balancer: Option<crate::api::core::v1::LoadBalancerStatus>,

}

#[cfg(feature = "dsl")]
impl ServiceStatus  {
    /// Set [`Self::load_balancer`]
    pub  fn load_balancer_set(&mut self, load_balancer: impl Into<Option<crate::api::core::v1::LoadBalancerStatus>>) -> &mut Self {
        self.load_balancer = load_balancer.into(); self
    }

    pub  fn load_balancer(&mut self) -> &mut crate::api::core::v1::LoadBalancerStatus {
        if self.load_balancer.is_none() { self.load_balancer = Some(Default::default()) }
        self.load_balancer.as_mut().unwrap()
    }

    /// Modify [`Self::load_balancer`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn load_balancer_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::LoadBalancerStatus)) -> &mut Self {
        if self.load_balancer.is_none() { self.load_balancer = Some(Default::default()) };
        func(self.load_balancer.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for ServiceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_load_balancer,
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
                            "loadBalancer" => Field::Key_load_balancer,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_load_balancer: Option<crate::api::core::v1::LoadBalancerStatus> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_load_balancer => value_load_balancer = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceStatus {
                    load_balancer: value_load_balancer,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceStatus",
            &[
                "loadBalancer",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ServiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceStatus",
            self.load_balancer.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.load_balancer {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancer", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServiceStatus {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ServiceStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ServiceStatus represents the current status of a service.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "loadBalancer".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LoadBalancerStatus>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("LoadBalancer contains the current status of the load-balancer, if one is present.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
