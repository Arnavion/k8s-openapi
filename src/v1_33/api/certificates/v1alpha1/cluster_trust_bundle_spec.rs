// Generated from definition io.k8s.api.certificates.v1alpha1.ClusterTrustBundleSpec

/// ClusterTrustBundleSpec contains the signer and trust anchors.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterTrustBundleSpec {
    /// signerName indicates the associated signer, if any.
    ///
    /// In order to create or update a ClusterTrustBundle that sets signerName, you must have the following cluster-scoped permission: group=certificates.k8s.io resource=signers resourceName=\<the signer name\> verb=attest.
    ///
    /// If signerName is not empty, then the ClusterTrustBundle object must be named with the signer name as a prefix (translating slashes to colons). For example, for the signer name `example.com/foo`, valid ClusterTrustBundle object names include `example.com:foo:abc` and `example.com:foo:v1`.
    ///
    /// If signerName is empty, then the ClusterTrustBundle object's name must not have such a prefix.
    ///
    /// List/watch requests for ClusterTrustBundles can filter on this field using a `spec.signerName=NAME` field selector.
    pub signer_name: Option<std::string::String>,

    /// trustBundle contains the individual X.509 trust anchors for this bundle, as PEM bundle of PEM-wrapped, DER-formatted X.509 certificates.
    ///
    /// The data must consist only of PEM certificate blocks that parse as valid X.509 certificates.  Each certificate must include a basic constraints extension with the CA bit set.  The API server will reject objects that contain duplicate certificates, or that use PEM block headers.
    ///
    /// Users of ClusterTrustBundles, including Kubelet, are free to reorder and deduplicate certificate blocks in this file according to their own logic, as well as to drop PEM block headers and inter-block data.
    pub trust_bundle: std::string::String,
}

impl crate::DeepMerge for ClusterTrustBundleSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.signer_name, other.signer_name);
        crate::DeepMerge::merge_from(&mut self.trust_bundle, other.trust_bundle);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ClusterTrustBundleSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_signer_name,
            Key_trust_bundle,
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
                            "signerName" => Field::Key_signer_name,
                            "trustBundle" => Field::Key_trust_bundle,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterTrustBundleSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ClusterTrustBundleSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_signer_name: Option<std::string::String> = None;
                let mut value_trust_bundle: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_signer_name => value_signer_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_trust_bundle => value_trust_bundle = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterTrustBundleSpec {
                    signer_name: value_signer_name,
                    trust_bundle: value_trust_bundle.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterTrustBundleSpec",
            &[
                "signerName",
                "trustBundle",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ClusterTrustBundleSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterTrustBundleSpec",
            1 +
            self.signer_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.signer_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "signerName", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "trustBundle", &self.trust_bundle)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ClusterTrustBundleSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.certificates.v1alpha1.ClusterTrustBundleSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ClusterTrustBundleSpec contains the signer and trust anchors.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "signerName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("signerName indicates the associated signer, if any.\n\nIn order to create or update a ClusterTrustBundle that sets signerName, you must have the following cluster-scoped permission: group=certificates.k8s.io resource=signers resourceName=<the signer name> verb=attest.\n\nIf signerName is not empty, then the ClusterTrustBundle object must be named with the signer name as a prefix (translating slashes to colons). For example, for the signer name `example.com/foo`, valid ClusterTrustBundle object names include `example.com:foo:abc` and `example.com:foo:v1`.\n\nIf signerName is empty, then the ClusterTrustBundle object's name must not have such a prefix.\n\nList/watch requests for ClusterTrustBundles can filter on this field using a `spec.signerName=NAME` field selector.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "trustBundle".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("trustBundle contains the individual X.509 trust anchors for this bundle, as PEM bundle of PEM-wrapped, DER-formatted X.509 certificates.\n\nThe data must consist only of PEM certificate blocks that parse as valid X.509 certificates.  Each certificate must include a basic constraints extension with the CA bit set.  The API server will reject objects that contain duplicate certificates, or that use PEM block headers.\n\nUsers of ClusterTrustBundles, including Kubelet, are free to reorder and deduplicate certificate blocks in this file according to their own logic, as well as to drop PEM block headers and inter-block data.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "trustBundle".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
