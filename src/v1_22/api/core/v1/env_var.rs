// Generated from definition io.k8s.api.core.v1.EnvVar

/// EnvVar represents an environment variable present in a Container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnvVar {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,

    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    pub value: Option<String>,

    /// Source for the environment variable's value. Cannot be used if value is not empty.
    pub value_from: Option<crate::api::core::v1::EnvVarSource>,

}

#[cfg(feature = "dsl")]
impl EnvVar  {
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


    /// Set [`Self::value`]
    pub  fn value_set(&mut self, value: impl Into<Option<String>>) -> &mut Self {
        self.value = value.into(); self
    }

    pub  fn value(&mut self) -> &mut String {
        if self.value.is_none() { self.value = Some(Default::default()) }
        self.value.as_mut().unwrap()
    }

    /// Modify [`Self::value`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn value_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.value.is_none() { self.value = Some(Default::default()) };
        func(self.value.as_mut().unwrap()); self
    }


    /// Set [`Self::value_from`]
    pub  fn value_from_set(&mut self, value_from: impl Into<Option<crate::api::core::v1::EnvVarSource>>) -> &mut Self {
        self.value_from = value_from.into(); self
    }

    pub  fn value_from(&mut self) -> &mut crate::api::core::v1::EnvVarSource {
        if self.value_from.is_none() { self.value_from = Some(Default::default()) }
        self.value_from.as_mut().unwrap()
    }

    /// Modify [`Self::value_from`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn value_from_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::EnvVarSource)) -> &mut Self {
        if self.value_from.is_none() { self.value_from = Some(Default::default()) };
        func(self.value_from.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for EnvVar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_value,
            Key_value_from,
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
                            "value" => Field::Key_value,
                            "valueFrom" => Field::Key_value_from,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EnvVar;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EnvVar")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_value: Option<String> = None;
                let mut value_value_from: Option<crate::api::core::v1::EnvVarSource> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value_from => value_value_from = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EnvVar {
                    name: value_name.unwrap_or_default(),
                    value: value_value,
                    value_from: value_value_from,
                })
            }
        }

        deserializer.deserialize_struct(
            "EnvVar",
            &[
                "name",
                "value",
                "valueFrom",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EnvVar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EnvVar",
            1 +
            self.value.as_ref().map_or(0, |_| 1) +
            self.value_from.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.value {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
        }
        if let Some(value) = &self.value_from {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "valueFrom", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for EnvVar {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.EnvVar".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("EnvVar represents an environment variable present in a Container.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name of the environment variable. Must be a C_IDENTIFIER.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "value".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to \"\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "valueFrom".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::EnvVarSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Source for the environment variable's value. Cannot be used if value is not empty.".to_owned()),
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
