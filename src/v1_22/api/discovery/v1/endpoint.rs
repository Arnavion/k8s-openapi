// Generated from definition io.k8s.api.discovery.v1.Endpoint

/// Endpoint represents a single logical "backend" implementing a service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Endpoint {
    /// addresses of this endpoint. The contents of this field are interpreted according to the corresponding EndpointSlice addressType field. Consumers must handle different types of addresses in the context of their own capabilities. This must contain at least one address but no more than 100.
    pub addresses: Vec<String>,

    /// conditions contains information about the current status of the endpoint.
    pub conditions: Option<crate::api::discovery::v1::EndpointConditions>,

    /// deprecatedTopology contains topology information part of the v1beta1 API. This field is deprecated, and will be removed when the v1beta1 API is removed (no sooner than kubernetes v1.24).  While this field can hold values, it is not writable through the v1 API, and any attempts to write to it will be silently ignored. Topology information can be found in the zone and nodeName fields instead.
    pub deprecated_topology: Option<std::collections::BTreeMap<String, String>>,

    /// hints contains information associated with how an endpoint should be consumed.
    pub hints: Option<crate::api::discovery::v1::EndpointHints>,

    /// hostname of this endpoint. This field may be used by consumers of endpoints to distinguish endpoints from each other (e.g. in DNS names). Multiple endpoints which use the same hostname should be considered fungible (e.g. multiple A values in DNS). Must be lowercase and pass DNS Label (RFC 1123) validation.
    pub hostname: Option<String>,

    /// nodeName represents the name of the Node hosting this endpoint. This can be used to determine endpoints local to a Node. This field can be enabled with the EndpointSliceNodeName feature gate.
    pub node_name: Option<String>,

    /// targetRef is a reference to a Kubernetes object that represents this endpoint.
    pub target_ref: Option<crate::api::core::v1::ObjectReference>,

    /// zone is the name of the Zone this endpoint exists in.
    pub zone: Option<String>,

}

#[cfg(feature = "dsl")]
impl Endpoint  {
    /// Set [`Self::addresses`]
    pub  fn addresses_set(&mut self, addresses: impl Into<Vec<String>>) -> &mut Self {
        self.addresses = addresses.into(); self
    }

    pub  fn addresses(&mut self) -> &mut Vec<String> {
        &mut self.addresses
    }

    /// Modify [`Self::addresses`] with a `func`
    pub  fn addresses_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        func(&mut self.addresses); self
    }

    /// Push new element to [`Self::addresses`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn addresses_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
      let mut new = Default::default();
      func(&mut new);
      self.addresses.push(new);
      self
    }

    /// Append all elements from `other` into [`Self::addresses`]
    pub  fn addresses_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         for item in other.borrow() {
             self.addresses.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::conditions`]
    pub  fn conditions_set(&mut self, conditions: impl Into<Option<crate::api::discovery::v1::EndpointConditions>>) -> &mut Self {
        self.conditions = conditions.into(); self
    }

    pub  fn conditions(&mut self) -> &mut crate::api::discovery::v1::EndpointConditions {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) }
        self.conditions.as_mut().unwrap()
    }

    /// Modify [`Self::conditions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conditions_with(&mut self, func: impl FnOnce(&mut crate::api::discovery::v1::EndpointConditions)) -> &mut Self {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) };
        func(self.conditions.as_mut().unwrap()); self
    }


    /// Set [`Self::deprecated_topology`]
    pub  fn deprecated_topology_set(&mut self, deprecated_topology: impl Into<Option<std::collections::BTreeMap<String, String>>>) -> &mut Self {
        self.deprecated_topology = deprecated_topology.into(); self
    }

    pub  fn deprecated_topology(&mut self) -> &mut std::collections::BTreeMap<String, String> {
        if self.deprecated_topology.is_none() { self.deprecated_topology = Some(Default::default()) }
        self.deprecated_topology.as_mut().unwrap()
    }

    /// Modify [`Self::deprecated_topology`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn deprecated_topology_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, String>)) -> &mut Self {
        if self.deprecated_topology.is_none() { self.deprecated_topology = Some(Default::default()) };
        func(self.deprecated_topology.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::deprecated_topology`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn deprecated_topology_insert_with(&mut self, name: &str, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.deprecated_topology.is_none() {
            self.deprecated_topology = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.deprecated_topology.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::deprecated_topology`]
    pub  fn deprecated_topology_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, String>>) -> &mut Self {
         if self.deprecated_topology.is_none() { self.deprecated_topology = Some(std::collections::BTreeMap::new()); }
         let deprecated_topology = &mut self.deprecated_topology.as_mut().unwrap();
         for (name, value) in other.borrow() {
             deprecated_topology.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::hints`]
    pub  fn hints_set(&mut self, hints: impl Into<Option<crate::api::discovery::v1::EndpointHints>>) -> &mut Self {
        self.hints = hints.into(); self
    }

    pub  fn hints(&mut self) -> &mut crate::api::discovery::v1::EndpointHints {
        if self.hints.is_none() { self.hints = Some(Default::default()) }
        self.hints.as_mut().unwrap()
    }

    /// Modify [`Self::hints`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn hints_with(&mut self, func: impl FnOnce(&mut crate::api::discovery::v1::EndpointHints)) -> &mut Self {
        if self.hints.is_none() { self.hints = Some(Default::default()) };
        func(self.hints.as_mut().unwrap()); self
    }


    /// Set [`Self::hostname`]
    pub  fn hostname_set(&mut self, hostname: impl Into<Option<String>>) -> &mut Self {
        self.hostname = hostname.into(); self
    }

    pub  fn hostname(&mut self) -> &mut String {
        if self.hostname.is_none() { self.hostname = Some(Default::default()) }
        self.hostname.as_mut().unwrap()
    }

    /// Modify [`Self::hostname`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn hostname_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.hostname.is_none() { self.hostname = Some(Default::default()) };
        func(self.hostname.as_mut().unwrap()); self
    }


    /// Set [`Self::node_name`]
    pub  fn node_name_set(&mut self, node_name: impl Into<Option<String>>) -> &mut Self {
        self.node_name = node_name.into(); self
    }

    pub  fn node_name(&mut self) -> &mut String {
        if self.node_name.is_none() { self.node_name = Some(Default::default()) }
        self.node_name.as_mut().unwrap()
    }

    /// Modify [`Self::node_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn node_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.node_name.is_none() { self.node_name = Some(Default::default()) };
        func(self.node_name.as_mut().unwrap()); self
    }


    /// Set [`Self::target_ref`]
    pub  fn target_ref_set(&mut self, target_ref: impl Into<Option<crate::api::core::v1::ObjectReference>>) -> &mut Self {
        self.target_ref = target_ref.into(); self
    }

    pub  fn target_ref(&mut self) -> &mut crate::api::core::v1::ObjectReference {
        if self.target_ref.is_none() { self.target_ref = Some(Default::default()) }
        self.target_ref.as_mut().unwrap()
    }

    /// Modify [`Self::target_ref`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn target_ref_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ObjectReference)) -> &mut Self {
        if self.target_ref.is_none() { self.target_ref = Some(Default::default()) };
        func(self.target_ref.as_mut().unwrap()); self
    }


    /// Set [`Self::zone`]
    pub  fn zone_set(&mut self, zone: impl Into<Option<String>>) -> &mut Self {
        self.zone = zone.into(); self
    }

    pub  fn zone(&mut self) -> &mut String {
        if self.zone.is_none() { self.zone = Some(Default::default()) }
        self.zone.as_mut().unwrap()
    }

    /// Modify [`Self::zone`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn zone_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.zone.is_none() { self.zone = Some(Default::default()) };
        func(self.zone.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for Endpoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_addresses,
            Key_conditions,
            Key_deprecated_topology,
            Key_hints,
            Key_hostname,
            Key_node_name,
            Key_target_ref,
            Key_zone,
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
                            "conditions" => Field::Key_conditions,
                            "deprecatedTopology" => Field::Key_deprecated_topology,
                            "hints" => Field::Key_hints,
                            "hostname" => Field::Key_hostname,
                            "nodeName" => Field::Key_node_name,
                            "targetRef" => Field::Key_target_ref,
                            "zone" => Field::Key_zone,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Endpoint;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Endpoint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_addresses: Option<Vec<String>> = None;
                let mut value_conditions: Option<crate::api::discovery::v1::EndpointConditions> = None;
                let mut value_deprecated_topology: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_hints: Option<crate::api::discovery::v1::EndpointHints> = None;
                let mut value_hostname: Option<String> = None;
                let mut value_node_name: Option<String> = None;
                let mut value_target_ref: Option<crate::api::core::v1::ObjectReference> = None;
                let mut value_zone: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_addresses => value_addresses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_deprecated_topology => value_deprecated_topology = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_hints => value_hints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_hostname => value_hostname = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_ref => value_target_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_zone => value_zone = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Endpoint {
                    addresses: value_addresses.unwrap_or_default(),
                    conditions: value_conditions,
                    deprecated_topology: value_deprecated_topology,
                    hints: value_hints,
                    hostname: value_hostname,
                    node_name: value_node_name,
                    target_ref: value_target_ref,
                    zone: value_zone,
                })
            }
        }

        deserializer.deserialize_struct(
            "Endpoint",
            &[
                "addresses",
                "conditions",
                "deprecatedTopology",
                "hints",
                "hostname",
                "nodeName",
                "targetRef",
                "zone",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Endpoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Endpoint",
            1 +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.deprecated_topology.as_ref().map_or(0, |_| 1) +
            self.hints.as_ref().map_or(0, |_| 1) +
            self.hostname.as_ref().map_or(0, |_| 1) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.target_ref.as_ref().map_or(0, |_| 1) +
            self.zone.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "addresses", &self.addresses)?;
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.deprecated_topology {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "deprecatedTopology", value)?;
        }
        if let Some(value) = &self.hints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hints", value)?;
        }
        if let Some(value) = &self.hostname {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostname", value)?;
        }
        if let Some(value) = &self.node_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        if let Some(value) = &self.target_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetRef", value)?;
        }
        if let Some(value) = &self.zone {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "zone", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Endpoint {
    fn schema_name() -> String {
        "io.k8s.api.discovery.v1.Endpoint".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Endpoint represents a single logical \"backend\" implementing a service.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "addresses".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("addresses of this endpoint. The contents of this field are interpreted according to the corresponding EndpointSlice addressType field. Consumers must handle different types of addresses in the context of their own capabilities. This must contain at least one address but no more than 100.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "conditions".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::discovery::v1::EndpointConditions>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("conditions contains information about the current status of the endpoint.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "deprecatedTopology".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("deprecatedTopology contains topology information part of the v1beta1 API. This field is deprecated, and will be removed when the v1beta1 API is removed (no sooner than kubernetes v1.24).  While this field can hold values, it is not writable through the v1 API, and any attempts to write to it will be silently ignored. Topology information can be found in the zone and nodeName fields instead.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                )),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hints".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::discovery::v1::EndpointHints>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("hints contains information associated with how an endpoint should be consumed.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "hostname".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("hostname of this endpoint. This field may be used by consumers of endpoints to distinguish endpoints from each other (e.g. in DNS names). Multiple endpoints which use the same hostname should be considered fungible (e.g. multiple A values in DNS). Must be lowercase and pass DNS Label (RFC 1123) validation.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nodeName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("nodeName represents the name of the Node hosting this endpoint. This can be used to determine endpoints local to a Node. This field can be enabled with the EndpointSliceNodeName feature gate.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "targetRef".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("targetRef is a reference to a Kubernetes object that represents this endpoint.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "zone".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("zone is the name of the Zone this endpoint exists in.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "addresses".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
