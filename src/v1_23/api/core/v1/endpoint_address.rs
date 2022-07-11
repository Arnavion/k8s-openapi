// Generated from definition io.k8s.api.core.v1.EndpointAddress

/// EndpointAddress is a tuple that describes single IP address.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndpointAddress {
    /// The Hostname of this endpoint
    pub hostname: Option<String>,

    /// The IP of this endpoint. May not be loopback (127.0.0.0/8), link-local (169.254.0.0/16), or link-local multicast ((224.0.0.0/24). IPv6 is also accepted but not fully supported on all platforms. Also, certain kubernetes components, like kube-proxy, are not IPv6 ready.
    pub ip: String,

    /// Optional: Node hosting this endpoint. This can be used to determine endpoints local to a node.
    pub node_name: Option<String>,

    /// Reference to object providing the endpoint.
    pub target_ref: Option<crate::api::core::v1::ObjectReference>,

}

#[cfg(feature = "dsl")]
impl EndpointAddress  {
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


    /// Set [`Self::ip`]
    pub  fn ip_set(&mut self, ip: impl Into<String>) -> &mut Self {
        self.ip = ip.into(); self
    }

    pub  fn ip(&mut self) -> &mut String {
        &mut self.ip
    }

    /// Modify [`Self::ip`] with a `func`
    pub  fn ip_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.ip); self
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


}


impl<'de> crate::serde::Deserialize<'de> for EndpointAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_hostname,
            Key_ip,
            Key_node_name,
            Key_target_ref,
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
                            "hostname" => Field::Key_hostname,
                            "ip" => Field::Key_ip,
                            "nodeName" => Field::Key_node_name,
                            "targetRef" => Field::Key_target_ref,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointAddress;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EndpointAddress")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_hostname: Option<String> = None;
                let mut value_ip: Option<String> = None;
                let mut value_node_name: Option<String> = None;
                let mut value_target_ref: Option<crate::api::core::v1::ObjectReference> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_hostname => value_hostname = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip => value_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_name => value_node_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_ref => value_target_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointAddress {
                    hostname: value_hostname,
                    ip: value_ip.unwrap_or_default(),
                    node_name: value_node_name,
                    target_ref: value_target_ref,
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointAddress",
            &[
                "hostname",
                "ip",
                "nodeName",
                "targetRef",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EndpointAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointAddress",
            1 +
            self.hostname.as_ref().map_or(0, |_| 1) +
            self.node_name.as_ref().map_or(0, |_| 1) +
            self.target_ref.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.hostname {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostname", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ip", &self.ip)?;
        if let Some(value) = &self.node_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeName", value)?;
        }
        if let Some(value) = &self.target_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "targetRef", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EndpointAddress {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.EndpointAddress".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("EndpointAddress is a tuple that describes single IP address.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "hostname".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The Hostname of this endpoint".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ip".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The IP of this endpoint. May not be loopback (127.0.0.0/8), link-local (169.254.0.0/16), or link-local multicast ((224.0.0.0/24). IPv6 is also accepted but not fully supported on all platforms. Also, certain kubernetes components, like kube-proxy, are not IPv6 ready.".to_owned()),
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
                                description: Some("Optional: Node hosting this endpoint. This can be used to determine endpoints local to a node.".to_owned()),
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
                                description: Some("Reference to object providing the endpoint.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "ip".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
