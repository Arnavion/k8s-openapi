// Generated from definition io.k8s.api.core.v1.EndpointSubset

/// EndpointSubset is a group of addresses with a common set of ports. The expanded set of endpoints is the Cartesian product of Addresses x Ports. For example, given:
///   {
///     Addresses: \[{"ip": "10.10.1.1"}, {"ip": "10.10.2.2"}\],
///     Ports:     \[{"name": "a", "port": 8675}, {"name": "b", "port": 309}\]
///   }
/// The resulting set of endpoints can be viewed as:
///     a: \[ 10.10.1.1:8675, 10.10.2.2:8675 \],
///     b: \[ 10.10.1.1:309, 10.10.2.2:309 \]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointSubset {
    /// IP addresses which offer the related ports that are marked as ready. These endpoints should be considered safe for load balancers and clients to utilize.
    pub addresses: Option<Vec<crate::api::core::v1::EndpointAddress>>,

    /// IP addresses which offer the related ports but are not currently marked as ready because they have not yet finished starting, have recently failed a readiness check, or have recently failed a liveness check.
    pub not_ready_addresses: Option<Vec<crate::api::core::v1::EndpointAddress>>,

    /// Port numbers available on the related IP addresses.
    pub ports: Option<Vec<crate::api::core::v1::EndpointPort>>,

}

#[cfg(feature = "dsl")]
impl EndpointSubset  {
    /// Set [`Self::addresses`]
    pub  fn addresses_set(&mut self, addresses: impl Into<Option<Vec<crate::api::core::v1::EndpointAddress>>>) -> &mut Self {
        self.addresses = addresses.into(); self
    }

    pub  fn addresses(&mut self) -> &mut Vec<crate::api::core::v1::EndpointAddress> {
        if self.addresses.is_none() { self.addresses = Some(Default::default()) }
        self.addresses.as_mut().unwrap()
    }

    /// Modify [`Self::addresses`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn addresses_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::EndpointAddress>)) -> &mut Self {
        if self.addresses.is_none() { self.addresses = Some(Default::default()) };
        func(self.addresses.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::addresses`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn addresses_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::EndpointAddress)) -> &mut Self {
        if self.addresses.is_none() {
            self.addresses = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.addresses.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::addresses`]
    pub  fn addresses_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::EndpointAddress]>) -> &mut Self {
         if self.addresses.is_none() { self.addresses = Some(Vec::new()); }
         let addresses = &mut self.addresses.as_mut().unwrap();
         for item in other.borrow() {
             addresses.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::not_ready_addresses`]
    pub  fn not_ready_addresses_set(&mut self, not_ready_addresses: impl Into<Option<Vec<crate::api::core::v1::EndpointAddress>>>) -> &mut Self {
        self.not_ready_addresses = not_ready_addresses.into(); self
    }

    pub  fn not_ready_addresses(&mut self) -> &mut Vec<crate::api::core::v1::EndpointAddress> {
        if self.not_ready_addresses.is_none() { self.not_ready_addresses = Some(Default::default()) }
        self.not_ready_addresses.as_mut().unwrap()
    }

    /// Modify [`Self::not_ready_addresses`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn not_ready_addresses_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::EndpointAddress>)) -> &mut Self {
        if self.not_ready_addresses.is_none() { self.not_ready_addresses = Some(Default::default()) };
        func(self.not_ready_addresses.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::not_ready_addresses`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn not_ready_addresses_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::EndpointAddress)) -> &mut Self {
        if self.not_ready_addresses.is_none() {
            self.not_ready_addresses = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.not_ready_addresses.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::not_ready_addresses`]
    pub  fn not_ready_addresses_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::EndpointAddress]>) -> &mut Self {
         if self.not_ready_addresses.is_none() { self.not_ready_addresses = Some(Vec::new()); }
         let not_ready_addresses = &mut self.not_ready_addresses.as_mut().unwrap();
         for item in other.borrow() {
             not_ready_addresses.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::ports`]
    pub  fn ports_set(&mut self, ports: impl Into<Option<Vec<crate::api::core::v1::EndpointPort>>>) -> &mut Self {
        self.ports = ports.into(); self
    }

    pub  fn ports(&mut self) -> &mut Vec<crate::api::core::v1::EndpointPort> {
        if self.ports.is_none() { self.ports = Some(Default::default()) }
        self.ports.as_mut().unwrap()
    }

    /// Modify [`Self::ports`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ports_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::EndpointPort>)) -> &mut Self {
        if self.ports.is_none() { self.ports = Some(Default::default()) };
        func(self.ports.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::ports`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn ports_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::EndpointPort)) -> &mut Self {
        if self.ports.is_none() {
            self.ports = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.ports.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::ports`]
    pub  fn ports_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::EndpointPort]>) -> &mut Self {
         if self.ports.is_none() { self.ports = Some(Vec::new()); }
         let ports = &mut self.ports.as_mut().unwrap();
         for item in other.borrow() {
             ports.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for EndpointSubset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_addresses,
            Key_not_ready_addresses,
            Key_ports,
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
                            "addresses" => Field::Key_addresses,
                            "notReadyAddresses" => Field::Key_not_ready_addresses,
                            "ports" => Field::Key_ports,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointSubset;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EndpointSubset")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_addresses: Option<Vec<crate::api::core::v1::EndpointAddress>> = None;
                let mut value_not_ready_addresses: Option<Vec<crate::api::core::v1::EndpointAddress>> = None;
                let mut value_ports: Option<Vec<crate::api::core::v1::EndpointPort>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_addresses => value_addresses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_not_ready_addresses => value_not_ready_addresses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointSubset {
                    addresses: value_addresses,
                    not_ready_addresses: value_not_ready_addresses,
                    ports: value_ports,
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointSubset",
            &[
                "addresses",
                "notReadyAddresses",
                "ports",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EndpointSubset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointSubset",
            self.addresses.as_ref().map_or(0, |_| 1) +
            self.not_ready_addresses.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.addresses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "addresses", value)?;
        }
        if let Some(value) = &self.not_ready_addresses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "notReadyAddresses", value)?;
        }
        if let Some(value) = &self.ports {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EndpointSubset {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.EndpointSubset".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("EndpointSubset is a group of addresses with a common set of ports. The expanded set of endpoints is the Cartesian product of Addresses x Ports. For example, given:\n  {\n    Addresses: [{\"ip\": \"10.10.1.1\"}, {\"ip\": \"10.10.2.2\"}],\n    Ports:     [{\"name\": \"a\", \"port\": 8675}, {\"name\": \"b\", \"port\": 309}]\n  }\nThe resulting set of endpoints can be viewed as:\n    a: [ 10.10.1.1:8675, 10.10.2.2:8675 ],\n    b: [ 10.10.1.1:309, 10.10.2.2:309 ]".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "addresses".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IP addresses which offer the related ports that are marked as ready. These endpoints should be considered safe for load balancers and clients to utilize.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::EndpointAddress>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "notReadyAddresses".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IP addresses which offer the related ports but are not currently marked as ready because they have not yet finished starting, have recently failed a readiness check, or have recently failed a liveness check.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::EndpointAddress>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ports".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Port numbers available on the related IP addresses.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::EndpointPort>()))),
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
