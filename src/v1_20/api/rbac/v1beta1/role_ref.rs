// Generated from definition io.k8s.api.rbac.v1beta1.RoleRef

/// RoleRef contains information that points to the role being used
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoleRef {
    /// APIGroup is the group for the resource being referenced
    pub api_group: String,

    /// Kind is the type of resource being referenced
    pub kind: String,

    /// Name is the name of resource being referenced
    pub name: String,

}

#[cfg(feature = "dsl")]
impl RoleRef  {
    /// Set [`Self::api_group`]
    pub  fn api_group_set(&mut self, api_group: impl Into<String>) -> &mut Self {
        self.api_group = api_group.into(); self
    }

    pub  fn api_group(&mut self) -> &mut String {
        &mut self.api_group
    }

    /// Modify [`Self::api_group`] with a `func`
    pub  fn api_group_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.api_group); self
    }


    /// Set [`Self::kind`]
    pub  fn kind_set(&mut self, kind: impl Into<String>) -> &mut Self {
        self.kind = kind.into(); self
    }

    pub  fn kind(&mut self) -> &mut String {
        &mut self.kind
    }

    /// Modify [`Self::kind`] with a `func`
    pub  fn kind_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.kind); self
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


}


impl<'de> crate::serde::Deserialize<'de> for RoleRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_group,
            Key_kind,
            Key_name,
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
                            "apiGroup" => Field::Key_api_group,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = RoleRef;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("RoleRef")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_group: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_group => value_api_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(RoleRef {
                    api_group: value_api_group.unwrap_or_default(),
                    kind: value_kind.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "RoleRef",
            &[
                "apiGroup",
                "kind",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for RoleRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "RoleRef",
            3,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroup", &self.api_group)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for RoleRef {
    fn schema_name() -> String {
        "io.k8s.api.rbac.v1beta1.RoleRef".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("RoleRef contains information that points to the role being used".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiGroup".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIGroup is the group for the resource being referenced".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kind is the type of resource being referenced".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name is the name of resource being referenced".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "apiGroup".to_owned(),
                    "kind".to_owned(),
                    "name".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
