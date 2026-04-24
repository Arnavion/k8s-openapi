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

    /// Projects an auto-rotating credential bundle (private key and certificate chain) that the pod can use either as a TLS client or server.
    ///
    /// Kubelet generates a private key and uses it to send a PodCertificateRequest to the named signer.  Once the signer approves the request and issues a certificate chain, Kubelet writes the key and certificate chain to the pod filesystem.  The pod does not start until certificates have been issued for each podCertificate projected volume source in its spec.
    ///
    /// Kubelet will begin trying to rotate the certificate at the time indicated by the signer using the PodCertificateRequest.Status.BeginRefreshAt timestamp.
    ///
    /// Kubelet can write a single file, indicated by the credentialBundlePath field, or separate files, indicated by the keyPath and certificateChainPath fields.
    ///
    /// The credential bundle is a single file in PEM format.  The first PEM entry is the private key (in PKCS#8 format), and the remaining PEM entries are the certificate chain issued by the signer (typically, signers will return their certificate chain in leaf-to-root order).
    ///
    /// Prefer using the credential bundle format, since your application code can read it atomically.  If you use keyPath and certificateChainPath, your application must make two separate file reads. If these coincide with a certificate rotation, it is possible that the private key and leaf certificate you read may not correspond to each other.  Your application will need to check for this condition, and re-read until they are consistent.
    ///
    /// The named signer controls chooses the format of the certificate it issues; consult the signer implementation's documentation to learn how to use the certificates it issues.
    pub pod_certificate: Option<crate::api::core::v1::PodCertificateProjection>,

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
        crate::DeepMerge::merge_from(&mut self.pod_certificate, other.pod_certificate);
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
            Key_pod_certificate,
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
                            "podCertificate" => Field::Key_pod_certificate,
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
                let mut value_pod_certificate: Option<crate::api::core::v1::PodCertificateProjection> = None;
                let mut value_secret: Option<crate::api::core::v1::SecretProjection> = None;
                let mut value_service_account_token: Option<crate::api::core::v1::ServiceAccountTokenProjection> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cluster_trust_bundle => value_cluster_trust_bundle = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_config_map => value_config_map = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_downward_api => value_downward_api = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_certificate => value_pod_certificate = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret => value_secret = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service_account_token => value_service_account_token = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VolumeProjection {
                    cluster_trust_bundle: value_cluster_trust_bundle,
                    config_map: value_config_map,
                    downward_api: value_downward_api,
                    pod_certificate: value_pod_certificate,
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
                "podCertificate",
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
            self.pod_certificate.as_ref().map_or(0, |_| 1) +
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
        if let Some(value) = &self.pod_certificate {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "podCertificate", value)?;
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
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.VolumeProjection".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Projection that may be projected along with other supported volume types. Exactly one of these fields must be set.",
            "type": "object",
            "properties": {
                "clusterTrustBundle": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ClusterTrustBundleProjection>();
                    schema_obj.ensure_object().insert("description".into(), "ClusterTrustBundle allows a pod to access the `.spec.trustBundle` field of ClusterTrustBundle objects in an auto-updating file.\n\nAlpha, gated by the ClusterTrustBundleProjection feature gate.\n\nClusterTrustBundle objects can either be selected by name, or by the combination of signer name and a label selector.\n\nKubelet performs aggressive normalization of the PEM contents written into the pod filesystem.  Esoteric PEM features such as inter-block comments and block headers are stripped.  Certificates are deduplicated. The ordering of certificates within the file is arbitrary, and Kubelet may change the order over time.".into());
                    schema_obj
                }),
                "configMap": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ConfigMapProjection>();
                    schema_obj.ensure_object().insert("description".into(), "configMap information about the configMap data to project".into());
                    schema_obj
                }),
                "downwardAPI": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::DownwardAPIProjection>();
                    schema_obj.ensure_object().insert("description".into(), "downwardAPI information about the downwardAPI data to project".into());
                    schema_obj
                }),
                "podCertificate": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodCertificateProjection>();
                    schema_obj.ensure_object().insert("description".into(), "Projects an auto-rotating credential bundle (private key and certificate chain) that the pod can use either as a TLS client or server.\n\nKubelet generates a private key and uses it to send a PodCertificateRequest to the named signer.  Once the signer approves the request and issues a certificate chain, Kubelet writes the key and certificate chain to the pod filesystem.  The pod does not start until certificates have been issued for each podCertificate projected volume source in its spec.\n\nKubelet will begin trying to rotate the certificate at the time indicated by the signer using the PodCertificateRequest.Status.BeginRefreshAt timestamp.\n\nKubelet can write a single file, indicated by the credentialBundlePath field, or separate files, indicated by the keyPath and certificateChainPath fields.\n\nThe credential bundle is a single file in PEM format.  The first PEM entry is the private key (in PKCS#8 format), and the remaining PEM entries are the certificate chain issued by the signer (typically, signers will return their certificate chain in leaf-to-root order).\n\nPrefer using the credential bundle format, since your application code can read it atomically.  If you use keyPath and certificateChainPath, your application must make two separate file reads. If these coincide with a certificate rotation, it is possible that the private key and leaf certificate you read may not correspond to each other.  Your application will need to check for this condition, and re-read until they are consistent.\n\nThe named signer controls chooses the format of the certificate it issues; consult the signer implementation's documentation to learn how to use the certificates it issues.".into());
                    schema_obj
                }),
                "secret": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretProjection>();
                    schema_obj.ensure_object().insert("description".into(), "secret information about the secret data to project".into());
                    schema_obj
                }),
                "serviceAccountToken": ({
                    let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ServiceAccountTokenProjection>();
                    schema_obj.ensure_object().insert("description".into(), "serviceAccountToken is information about the serviceAccountToken data to project".into());
                    schema_obj
                }),
            },
        })
    }
}
