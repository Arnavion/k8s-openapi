// Generated from definition io.k8s.api.core.v1.WindowsSecurityContextOptions

/// WindowsSecurityContextOptions contain Windows-specific options and credentials.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowsSecurityContextOptions {
    /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
    pub gmsa_credential_spec: Option<String>,

    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    pub gmsa_credential_spec_name: Option<String>,

    /// The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_user_name: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for WindowsSecurityContextOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_gmsa_credential_spec,
            Key_gmsa_credential_spec_name,
            Key_run_as_user_name,
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
                            "gmsaCredentialSpec" => Field::Key_gmsa_credential_spec,
                            "gmsaCredentialSpecName" => Field::Key_gmsa_credential_spec_name,
                            "runAsUserName" => Field::Key_run_as_user_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = WindowsSecurityContextOptions;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("WindowsSecurityContextOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_gmsa_credential_spec: Option<String> = None;
                let mut value_gmsa_credential_spec_name: Option<String> = None;
                let mut value_run_as_user_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_gmsa_credential_spec => value_gmsa_credential_spec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gmsa_credential_spec_name => value_gmsa_credential_spec_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_user_name => value_run_as_user_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WindowsSecurityContextOptions {
                    gmsa_credential_spec: value_gmsa_credential_spec,
                    gmsa_credential_spec_name: value_gmsa_credential_spec_name,
                    run_as_user_name: value_run_as_user_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "WindowsSecurityContextOptions",
            &[
                "gmsaCredentialSpec",
                "gmsaCredentialSpecName",
                "runAsUserName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for WindowsSecurityContextOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WindowsSecurityContextOptions",
            self.gmsa_credential_spec.as_ref().map_or(0, |_| 1) +
            self.gmsa_credential_spec_name.as_ref().map_or(0, |_| 1) +
            self.run_as_user_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.gmsa_credential_spec {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gmsaCredentialSpec", value)?;
        }
        if let Some(value) = &self.gmsa_credential_spec_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gmsaCredentialSpecName", value)?;
        }
        if let Some(value) = &self.run_as_user_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runAsUserName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WindowsSecurityContextOptions {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.WindowsSecurityContextOptions".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("WindowsSecurityContextOptions contain Windows-specific options and credentials.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "gmsaCredentialSpec".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gmsaCredentialSpecName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("GMSACredentialSpecName is the name of the GMSA credential spec to use.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsUserName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
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
