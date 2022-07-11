// Generated from definition io.k8s.api.authorization.v1.SelfSubjectAccessReviewSpec

/// SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelfSubjectAccessReviewSpec {
    /// NonResourceAttributes describes information for a non-resource access request
    pub non_resource_attributes: Option<crate::api::authorization::v1::NonResourceAttributes>,

    /// ResourceAuthorizationAttributes describes information for a resource access request
    pub resource_attributes: Option<crate::api::authorization::v1::ResourceAttributes>,

}

#[cfg(feature = "dsl")]
impl SelfSubjectAccessReviewSpec  {
    /// Set [`Self::non_resource_attributes`]
    pub  fn non_resource_attributes_set(&mut self, non_resource_attributes: impl Into<Option<crate::api::authorization::v1::NonResourceAttributes>>) -> &mut Self {
        self.non_resource_attributes = non_resource_attributes.into(); self
    }

    pub  fn non_resource_attributes(&mut self) -> &mut crate::api::authorization::v1::NonResourceAttributes {
        if self.non_resource_attributes.is_none() { self.non_resource_attributes = Some(Default::default()) }
        self.non_resource_attributes.as_mut().unwrap()
    }

    /// Modify [`Self::non_resource_attributes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn non_resource_attributes_with(&mut self, func: impl FnOnce(&mut crate::api::authorization::v1::NonResourceAttributes)) -> &mut Self {
        if self.non_resource_attributes.is_none() { self.non_resource_attributes = Some(Default::default()) };
        func(self.non_resource_attributes.as_mut().unwrap()); self
    }


    /// Set [`Self::resource_attributes`]
    pub  fn resource_attributes_set(&mut self, resource_attributes: impl Into<Option<crate::api::authorization::v1::ResourceAttributes>>) -> &mut Self {
        self.resource_attributes = resource_attributes.into(); self
    }

    pub  fn resource_attributes(&mut self) -> &mut crate::api::authorization::v1::ResourceAttributes {
        if self.resource_attributes.is_none() { self.resource_attributes = Some(Default::default()) }
        self.resource_attributes.as_mut().unwrap()
    }

    /// Modify [`Self::resource_attributes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn resource_attributes_with(&mut self, func: impl FnOnce(&mut crate::api::authorization::v1::ResourceAttributes)) -> &mut Self {
        if self.resource_attributes.is_none() { self.resource_attributes = Some(Default::default()) };
        func(self.resource_attributes.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for SelfSubjectAccessReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_non_resource_attributes,
            Key_resource_attributes,
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
                            "nonResourceAttributes" => Field::Key_non_resource_attributes,
                            "resourceAttributes" => Field::Key_resource_attributes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SelfSubjectAccessReviewSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("SelfSubjectAccessReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_non_resource_attributes: Option<crate::api::authorization::v1::NonResourceAttributes> = None;
                let mut value_resource_attributes: Option<crate::api::authorization::v1::ResourceAttributes> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_non_resource_attributes => value_non_resource_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_attributes => value_resource_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SelfSubjectAccessReviewSpec {
                    non_resource_attributes: value_non_resource_attributes,
                    resource_attributes: value_resource_attributes,
                })
            }
        }

        deserializer.deserialize_struct(
            "SelfSubjectAccessReviewSpec",
            &[
                "nonResourceAttributes",
                "resourceAttributes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SelfSubjectAccessReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SelfSubjectAccessReviewSpec",
            self.non_resource_attributes.as_ref().map_or(0, |_| 1) +
            self.resource_attributes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.non_resource_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceAttributes", value)?;
        }
        if let Some(value) = &self.resource_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceAttributes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SelfSubjectAccessReviewSpec {
    fn schema_name() -> String {
        "io.k8s.api.authorization.v1.SelfSubjectAccessReviewSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "nonResourceAttributes".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::authorization::v1::NonResourceAttributes>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NonResourceAttributes describes information for a non-resource access request".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resourceAttributes".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::authorization::v1::ResourceAttributes>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceAuthorizationAttributes describes information for a resource access request".to_owned()),
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
