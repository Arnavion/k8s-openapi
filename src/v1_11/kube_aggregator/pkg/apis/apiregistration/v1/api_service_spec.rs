// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1.APIServiceSpec

/// APIServiceSpec contains information for locating and communicating with a server. Only https is supported, though you are able to disable certificate verification.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIServiceSpec {
    /// CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate.
    pub ca_bundle: Option<crate::ByteString>,

    /// Group is the API group name this server hosts
    pub group: Option<String>,

    /// GroupPriorityMininum is the priority this group should have at least. Higher priority means that the group is preferred by clients over lower priority ones. Note that other versions of this group might specify even higher GroupPriorityMininum values such that the whole group gets a higher priority. The primary sort is based on GroupPriorityMinimum, ordered highest number to lowest (20 before 10). The secondary sort is based on the alphabetical comparison of the name of the object.  (v1.bar before v1.foo) We'd recommend something like: *.k8s.io (except extensions) at 18000 and PaaSes (OpenShift, Deis) are recommended to be in the 2000s
    pub group_priority_minimum: i32,

    /// InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server. This is strongly discouraged.  You should use the CABundle instead.
    pub insecure_skip_tls_verify: Option<bool>,

    /// Service is a reference to the service for this API server.  It must communicate on port 443 If the Service is nil, that means the handling for the API groupversion is handled locally on this server. The call will simply delegate to the normal handler chain to be fulfilled.
    pub service: crate::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference,

    /// Version is the API version this server hosts.  For example, "v1"
    pub version: Option<String>,

    /// VersionPriority controls the ordering of this API version inside of its group.  Must be greater than zero. The primary sort is based on VersionPriority, ordered highest to lowest (20 before 10). Since it's inside of a group, the number can be small, probably in the 10s. In case of equal version priorities, the version string will be used to compute the order inside a group. If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version), then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first by GA \> beta \> alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
    pub version_priority: i32,
}

impl<'de> crate::serde::Deserialize<'de> for APIServiceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ca_bundle,
            Key_group,
            Key_group_priority_minimum,
            Key_insecure_skip_tls_verify,
            Key_service,
            Key_version,
            Key_version_priority,
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
                            "caBundle" => Field::Key_ca_bundle,
                            "group" => Field::Key_group,
                            "groupPriorityMinimum" => Field::Key_group_priority_minimum,
                            "insecureSkipTLSVerify" => Field::Key_insecure_skip_tls_verify,
                            "service" => Field::Key_service,
                            "version" => Field::Key_version,
                            "versionPriority" => Field::Key_version_priority,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = APIServiceSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("APIServiceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ca_bundle: Option<crate::ByteString> = None;
                let mut value_group: Option<String> = None;
                let mut value_group_priority_minimum: Option<i32> = None;
                let mut value_insecure_skip_tls_verify: Option<bool> = None;
                let mut value_service: Option<crate::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference> = None;
                let mut value_version: Option<String> = None;
                let mut value_version_priority: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ca_bundle => value_ca_bundle = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group_priority_minimum => value_group_priority_minimum = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_insecure_skip_tls_verify => value_insecure_skip_tls_verify = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_service => value_service = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_version => value_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version_priority => value_version_priority = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIServiceSpec {
                    ca_bundle: value_ca_bundle,
                    group: value_group,
                    group_priority_minimum: value_group_priority_minimum.ok_or_else(|| crate::serde::de::Error::missing_field("groupPriorityMinimum"))?,
                    insecure_skip_tls_verify: value_insecure_skip_tls_verify,
                    service: value_service.ok_or_else(|| crate::serde::de::Error::missing_field("service"))?,
                    version: value_version,
                    version_priority: value_version_priority.ok_or_else(|| crate::serde::de::Error::missing_field("versionPriority"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "APIServiceSpec",
            &[
                "caBundle",
                "group",
                "groupPriorityMinimum",
                "insecureSkipTLSVerify",
                "service",
                "version",
                "versionPriority",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for APIServiceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIServiceSpec",
            3 +
            self.ca_bundle.as_ref().map_or(0, |_| 1) +
            self.group.as_ref().map_or(0, |_| 1) +
            self.insecure_skip_tls_verify.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ca_bundle {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "caBundle", value)?;
        }
        if let Some(value) = &self.group {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "groupPriorityMinimum", &self.group_priority_minimum)?;
        if let Some(value) = &self.insecure_skip_tls_verify {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "insecureSkipTLSVerify", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "service", &self.service)?;
        if let Some(value) = &self.version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "versionPriority", &self.version_priority)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for APIServiceSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "APIServiceSpec contains information for locating and communicating with a server. Only https is supported, though you are able to disable certificate verification.",
          "properties": {
            "caBundle": {
              "description": "CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate.",
              "format": "byte",
              "type": "string"
            },
            "group": {
              "description": "Group is the API group name this server hosts",
              "type": "string"
            },
            "groupPriorityMinimum": {
              "description": "GroupPriorityMininum is the priority this group should have at least. Higher priority means that the group is preferred by clients over lower priority ones. Note that other versions of this group might specify even higher GroupPriorityMininum values such that the whole group gets a higher priority. The primary sort is based on GroupPriorityMinimum, ordered highest number to lowest (20 before 10). The secondary sort is based on the alphabetical comparison of the name of the object.  (v1.bar before v1.foo) We'd recommend something like: *.k8s.io (except extensions) at 18000 and PaaSes (OpenShift, Deis) are recommended to be in the 2000s",
              "format": "int32",
              "type": "integer"
            },
            "insecureSkipTLSVerify": {
              "description": "InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server. This is strongly discouraged.  You should use the CABundle instead.",
              "type": "boolean"
            },
            "service": crate::schema_ref_with_values(crate::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference::schema(), serde_json::json!({"description": "Service is a reference to the service for this API server.  It must communicate on port 443 If the Service is nil, that means the handling for the API groupversion is handled locally on this server. The call will simply delegate to the normal handler chain to be fulfilled."})),
            "version": {
              "description": "Version is the API version this server hosts.  For example, \"v1\"",
              "type": "string"
            },
            "versionPriority": {
              "description": "VersionPriority controls the ordering of this API version inside of its group.  Must be greater than zero. The primary sort is based on VersionPriority, ordered highest to lowest (20 before 10). Since it's inside of a group, the number can be small, probably in the 10s. In case of equal version priorities, the version string will be used to compute the order inside a group. If the version string is \"kube-like\", it will sort above non \"kube-like\" version strings, which are ordered lexicographically. \"Kube-like\" versions start with a \"v\", then are followed by a number (the major version), then optionally the string \"alpha\" or \"beta\" and another number (the minor version). These are sorted first by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.",
              "format": "int32",
              "type": "integer"
            }
          },
          "required": [
            "groupPriorityMinimum",
            "service",
            "versionPriority"
          ],
          "type": "object"
        })
    }
}
