// Generated from definition io.k8s.api.discovery.v1.EndpointHints

/// EndpointHints provides hints describing how an endpoint should be consumed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointHints {
    /// forZones indicates the zone(s) this endpoint should be consumed by to enable topology aware routing.
    pub for_zones: Option<Vec<crate::api::discovery::v1::ForZone>>,

}

#[cfg(feature = "dsl")]
impl EndpointHints  {
    /// Set [`Self::for_zones`]
    pub  fn for_zones_set(&mut self, for_zones: impl Into<Option<Vec<crate::api::discovery::v1::ForZone>>>) -> &mut Self {
        self.for_zones = for_zones.into(); self
    }

    pub  fn for_zones(&mut self) -> &mut Vec<crate::api::discovery::v1::ForZone> {
        if self.for_zones.is_none() { self.for_zones = Some(Default::default()) }
        self.for_zones.as_mut().unwrap()
    }

    /// Modify [`Self::for_zones`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn for_zones_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::discovery::v1::ForZone>)) -> &mut Self {
        if self.for_zones.is_none() { self.for_zones = Some(Default::default()) };
        func(self.for_zones.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::for_zones`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn for_zones_push_with(&mut self, func: impl FnOnce(&mut crate::api::discovery::v1::ForZone)) -> &mut Self {
        if self.for_zones.is_none() {
            self.for_zones = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.for_zones.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::for_zones`]
    pub  fn for_zones_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::discovery::v1::ForZone]>) -> &mut Self {
         if self.for_zones.is_none() { self.for_zones = Some(Vec::new()); }
         let for_zones = &mut self.for_zones.as_mut().unwrap();
         for item in other.borrow() {
             for_zones.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for EndpointHints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_for_zones,
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
                            "forZones" => Field::Key_for_zones,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointHints;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EndpointHints")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_for_zones: Option<Vec<crate::api::discovery::v1::ForZone>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_for_zones => value_for_zones = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointHints {
                    for_zones: value_for_zones,
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointHints",
            &[
                "forZones",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EndpointHints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointHints",
            self.for_zones.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.for_zones {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "forZones", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EndpointHints {
    fn schema_name() -> String {
        "io.k8s.api.discovery.v1.EndpointHints".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("EndpointHints provides hints describing how an endpoint should be consumed.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "forZones".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("forZones indicates the zone(s) this endpoint should be consumed by to enable topology aware routing.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::discovery::v1::ForZone>()))),
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
