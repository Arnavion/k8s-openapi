// Generated from definition io.k8s.api.core.v1.WindowsSecurityContextOptions

/// WindowsSecurityContextOptions contain Windows-specific options and credentials.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowsSecurityContextOptions {
    /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
    pub gmsa_credential_spec: Option<std::string::String>,

    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    pub gmsa_credential_spec_name: Option<std::string::String>,

    /// HostProcess determines if a container should be run as a 'Host Process' container. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers). In addition, if HostProcess is true then HostNetwork must also be set to true.
    pub host_process: Option<bool>,

    /// The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    pub run_as_user_name: Option<std::string::String>,
}

impl crate::DeepMerge for WindowsSecurityContextOptions {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.gmsa_credential_spec, other.gmsa_credential_spec);
        crate::DeepMerge::merge_from(&mut self.gmsa_credential_spec_name, other.gmsa_credential_spec_name);
        crate::DeepMerge::merge_from(&mut self.host_process, other.host_process);
        crate::DeepMerge::merge_from(&mut self.run_as_user_name, other.run_as_user_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for WindowsSecurityContextOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_gmsa_credential_spec,
            Key_gmsa_credential_spec_name,
            Key_host_process,
            Key_run_as_user_name,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "gmsaCredentialSpec" => Field::Key_gmsa_credential_spec,
                            "gmsaCredentialSpecName" => Field::Key_gmsa_credential_spec_name,
                            "hostProcess" => Field::Key_host_process,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("WindowsSecurityContextOptions")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_gmsa_credential_spec: Option<std::string::String> = None;
                let mut value_gmsa_credential_spec_name: Option<std::string::String> = None;
                let mut value_host_process: Option<bool> = None;
                let mut value_run_as_user_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_gmsa_credential_spec => value_gmsa_credential_spec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gmsa_credential_spec_name => value_gmsa_credential_spec_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_process => value_host_process = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_user_name => value_run_as_user_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WindowsSecurityContextOptions {
                    gmsa_credential_spec: value_gmsa_credential_spec,
                    gmsa_credential_spec_name: value_gmsa_credential_spec_name,
                    host_process: value_host_process,
                    run_as_user_name: value_run_as_user_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "WindowsSecurityContextOptions",
            &[
                "gmsaCredentialSpec",
                "gmsaCredentialSpecName",
                "hostProcess",
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
            self.host_process.as_ref().map_or(0, |_| 1) +
            self.run_as_user_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.gmsa_credential_spec {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gmsaCredentialSpec", value)?;
        }
        if let Some(value) = &self.gmsa_credential_spec_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gmsaCredentialSpecName", value)?;
        }
        if let Some(value) = &self.host_process {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostProcess", value)?;
        }
        if let Some(value) = &self.run_as_user_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "runAsUserName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for WindowsSecurityContextOptions {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.WindowsSecurityContextOptions".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("WindowsSecurityContextOptions contain Windows-specific options and credentials.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "gmsaCredentialSpec".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "gmsaCredentialSpecName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("GMSACredentialSpecName is the name of the GMSA credential spec to use.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "hostProcess".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("HostProcess determines if a container should be run as a 'Host Process' container. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers). In addition, if HostProcess is true then HostNetwork must also be set to true.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "runAsUserName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
