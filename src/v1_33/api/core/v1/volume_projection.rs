// Generated from definition io.k8s.api.core.v1.VolumeProjection

/// Projection that may be projected along with other supported volume types. Exactly one of these fields must be set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VolumeProjection {
    /// ClusterTrustBundle allows a pod to access the `.spec.trustBundle` field of ClusterTrustBundle objects in an auto-updating file.
    ///
    /// Alpha, gated by the ClusterTrustBundleProjection feature gate.
    ///
    /// ClusterTrustBundle objects can either be selected by name, or by the combination of signer name and a label selector.
    ///
    /// Kubelet performs aggressive normalization of the PEM contents written into the pod filesystem.  Esoteric PEM features such as inter-block comments and block headers are stripped.  Certificates are deduplicated. The ordering of certificates within the file is arbitrary, and Kubelet may change the order over time.
    pub cluster_trust_bundle: Option<crate::api::core::v1::ClusterTrustBundleProjection>,

    /// configMap information about the configMap data to project
    pub config_map: Option<crate::api::core::v1::ConfigMapProjection>,

    /// downwardAPI information about the downwardAPI data to project
    pub downward_api: Option<crate::api::core::v1::DownwardAPIProjection>,

    /// secret information about the secret data to project
    pub secret: Option<crate::api::core::v1::SecretProjection>,

    /// serviceAccountToken is information about the serviceAccountToken data to project
    pub service_account_token: Option<crate::api::core::v1::ServiceAccountTokenProjection>,
}

impl crate::DeepMerge for VolumeProjection {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.cluster_trust_bundle, other.cluster_trust_bundle);
        crate::DeepMerge::merge_from(&mut self.config_map, other.config_map);
        crate::DeepMerge::merge_from(&mut self.downward_api, other.downward_api);
        crate::DeepMerge::merge_from(&mut self.secret, other.secret);
        crate::DeepMerge::merge_from(&mut self.service_account_token, other.service_account_token);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VolumeProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cluster_trust_bundle,
            Key_config_map,
            Key_downward_api,
            Key_secret,
            Key_service_account_token,
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
                            "clusterTrustBundle" => Field::Key_cluster_trust_bundle,
                            "configMap" => Field::Key_config_map,
                            "downwardAPI" => Field::Key_downward_api,
                            "secret" => Field::Key_secret,
                            "serviceAccountToken" => Field::Key_service_account_token,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeProjection;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("VolumeProjection")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_cluster_trust_bundle: Option<crate::api::core::v1::ClusterTrustBundleProjection> = None;
                let mut value_config_map: Option<crate::api::core::v1::ConfigMapProjection> = None;
                let mut value_downward_api: Option<crate::api::core::v1::DownwardAPIProjection> = None;
                let mut value_secret: Option<crate::api::core::v1::SecretProjection> = None;
                let mut value_service_account_token: Option<crate::api::core::v1::ServiceAccountTokenProjection> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cluster_trust_bundle => value_cluster_trust_bundle = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_config_map => value_config_map = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_downward_api => value_downward_api = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret => value_secret = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account_token => value_service_account_token = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeProjection {
                    cluster_trust_bundle: value_cluster_trust_bundle,
                    config_map: value_config_map,
                    downward_api: value_downward_api,
                    secret: value_secret,
                    service_account_token: value_service_account_token,
                })
            }
        }

        deserializer.deserialize_struct(
            "VolumeProjection",
            &[
                "clusterTrustBundle",
                "configMap",
                "downwardAPI",
                "secret",
                "serviceAccountToken",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VolumeProjection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VolumeProjection",
            self.cluster_trust_bundle.as_ref().map_or(0, |_| 1) +
            self.config_map.as_ref().map_or(0, |_| 1) +
            self.downward_api.as_ref().map_or(0, |_| 1) +
            self.secret.as_ref().map_or(0, |_| 1) +
            self.service_account_token.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.cluster_trust_bundle {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterTrustBundle", value)?;
        }
        if let Some(value) = &self.config_map {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "configMap", value)?;
        }
        if let Some(value) = &self.downward_api {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "downwardAPI", value)?;
        }
        if let Some(value) = &self.secret {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secret", value)?;
        }
        if let Some(value) = &self.service_account_token {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "serviceAccountToken", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VolumeProjection {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.VolumeProjection".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Projection that may be projected along with other supported volume types. Exactly one of these fields must be set.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "clusterTrustBundle".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ClusterTrustBundleProjection>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ClusterTrustBundle allows a pod to access the `.spec.trustBundle` field of ClusterTrustBundle objects in an auto-updating file.\n\nAlpha, gated by the ClusterTrustBundleProjection feature gate.\n\nClusterTrustBundle objects can either be selected by name, or by the combination of signer name and a label selector.\n\nKubelet performs aggressive normalization of the PEM contents written into the pod filesystem.  Esoteric PEM features such as inter-block comments and block headers are stripped.  Certificates are deduplicated. The ordering of certificates within the file is arbitrary, and Kubelet may change the order over time.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "configMap".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ConfigMapProjection>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("configMap information about the configMap data to project".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "downwardAPI".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::DownwardAPIProjection>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("downwardAPI information about the downwardAPI data to project".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "secret".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretProjection>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("secret information about the secret data to project".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "serviceAccountToken".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ServiceAccountTokenProjection>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("serviceAccountToken is information about the serviceAccountToken data to project".into()),
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
