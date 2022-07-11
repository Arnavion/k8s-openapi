// Generated from definition io.k8s.api.core.v1.LoadBalancerStatus

/// LoadBalancerStatus represents the status of a load-balancer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoadBalancerStatus {
    /// Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
    pub ingress: Option<Vec<crate::api::core::v1::LoadBalancerIngress>>,

}

#[cfg(feature = "dsl")]
impl LoadBalancerStatus  {
    /// Set [`Self::ingress`]
    pub  fn ingress_set(&mut self, ingress: impl Into<Option<Vec<crate::api::core::v1::LoadBalancerIngress>>>) -> &mut Self {
        self.ingress = ingress.into(); self
    }

    pub  fn ingress(&mut self) -> &mut Vec<crate::api::core::v1::LoadBalancerIngress> {
        if self.ingress.is_none() { self.ingress = Some(Default::default()) }
        self.ingress.as_mut().unwrap()
    }

    /// Modify [`Self::ingress`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ingress_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::LoadBalancerIngress>)) -> &mut Self {
        if self.ingress.is_none() { self.ingress = Some(Default::default()) };
        func(self.ingress.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::ingress`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn ingress_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::LoadBalancerIngress)) -> &mut Self {
        if self.ingress.is_none() {
            self.ingress = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.ingress.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::ingress`]
    pub  fn ingress_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::LoadBalancerIngress]>) -> &mut Self {
         if self.ingress.is_none() { self.ingress = Some(Vec::new()); }
         let ingress = &mut self.ingress.as_mut().unwrap();
         for item in other.borrow() {
             ingress.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for LoadBalancerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ingress,
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
                            "ingress" => Field::Key_ingress,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancerStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LoadBalancerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ingress: Option<Vec<crate::api::core::v1::LoadBalancerIngress>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ingress => value_ingress = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LoadBalancerStatus {
                    ingress: value_ingress,
                })
            }
        }

        deserializer.deserialize_struct(
            "LoadBalancerStatus",
            &[
                "ingress",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LoadBalancerStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LoadBalancerStatus",
            self.ingress.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ingress {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ingress", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LoadBalancerStatus {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.LoadBalancerStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("LoadBalancerStatus represents the status of a load-balancer.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "ingress".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::LoadBalancerIngress>()))),
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
