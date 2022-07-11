// Generated from definition io.k8s.api.core.v1.PodDNSConfig

/// PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodDNSConfig {
    /// A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed.
    pub nameservers: Option<Vec<String>>,

    /// A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy.
    pub options: Option<Vec<crate::api::core::v1::PodDNSConfigOption>>,

    /// A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed.
    pub searches: Option<Vec<String>>,

}

#[cfg(feature = "dsl")]
impl PodDNSConfig  {
    /// Set [`Self::nameservers`]
    pub  fn nameservers_set(&mut self, nameservers: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.nameservers = nameservers.into(); self
    }

    pub  fn nameservers(&mut self) -> &mut Vec<String> {
        if self.nameservers.is_none() { self.nameservers = Some(Default::default()) }
        self.nameservers.as_mut().unwrap()
    }

    /// Modify [`Self::nameservers`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn nameservers_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.nameservers.is_none() { self.nameservers = Some(Default::default()) };
        func(self.nameservers.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::nameservers`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn nameservers_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.nameservers.is_none() {
            self.nameservers = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.nameservers.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::nameservers`]
    pub  fn nameservers_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.nameservers.is_none() { self.nameservers = Some(Vec::new()); }
         let nameservers = &mut self.nameservers.as_mut().unwrap();
         for item in other.borrow() {
             nameservers.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::options`]
    pub  fn options_set(&mut self, options: impl Into<Option<Vec<crate::api::core::v1::PodDNSConfigOption>>>) -> &mut Self {
        self.options = options.into(); self
    }

    pub  fn options(&mut self) -> &mut Vec<crate::api::core::v1::PodDNSConfigOption> {
        if self.options.is_none() { self.options = Some(Default::default()) }
        self.options.as_mut().unwrap()
    }

    /// Modify [`Self::options`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn options_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::PodDNSConfigOption>)) -> &mut Self {
        if self.options.is_none() { self.options = Some(Default::default()) };
        func(self.options.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::options`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn options_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PodDNSConfigOption)) -> &mut Self {
        if self.options.is_none() {
            self.options = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.options.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::options`]
    pub  fn options_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::PodDNSConfigOption]>) -> &mut Self {
         if self.options.is_none() { self.options = Some(Vec::new()); }
         let options = &mut self.options.as_mut().unwrap();
         for item in other.borrow() {
             options.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::searches`]
    pub  fn searches_set(&mut self, searches: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.searches = searches.into(); self
    }

    pub  fn searches(&mut self) -> &mut Vec<String> {
        if self.searches.is_none() { self.searches = Some(Default::default()) }
        self.searches.as_mut().unwrap()
    }

    /// Modify [`Self::searches`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn searches_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.searches.is_none() { self.searches = Some(Default::default()) };
        func(self.searches.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::searches`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn searches_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.searches.is_none() {
            self.searches = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.searches.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::searches`]
    pub  fn searches_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.searches.is_none() { self.searches = Some(Vec::new()); }
         let searches = &mut self.searches.as_mut().unwrap();
         for item in other.borrow() {
             searches.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for PodDNSConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_nameservers,
            Key_options,
            Key_searches,
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
                            "nameservers" => Field::Key_nameservers,
                            "options" => Field::Key_options,
                            "searches" => Field::Key_searches,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PodDNSConfig;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodDNSConfig")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_nameservers: Option<Vec<String>> = None;
                let mut value_options: Option<Vec<crate::api::core::v1::PodDNSConfigOption>> = None;
                let mut value_searches: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_nameservers => value_nameservers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_options => value_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_searches => value_searches = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodDNSConfig {
                    nameservers: value_nameservers,
                    options: value_options,
                    searches: value_searches,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodDNSConfig",
            &[
                "nameservers",
                "options",
                "searches",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PodDNSConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodDNSConfig",
            self.nameservers.as_ref().map_or(0, |_| 1) +
            self.options.as_ref().map_or(0, |_| 1) +
            self.searches.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.nameservers {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nameservers", value)?;
        }
        if let Some(value) = &self.options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "options", value)?;
        }
        if let Some(value) = &self.searches {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "searches", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PodDNSConfig {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.PodDNSConfig".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "nameservers".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed.".to_owned()),
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
                        "options".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::PodDNSConfigOption>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "searches".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed.".to_owned()),
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
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
