// Generated from definition io.k8s.api.networking.v1.IngressSpec

/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressSpec {
    /// DefaultBackend is the backend that should handle requests that don't match any rule. If Rules are not specified, DefaultBackend must be specified. If DefaultBackend is not set, the handling of requests that do not match any of the rules will be up to the Ingress controller.
    pub default_backend: Option<crate::api::networking::v1::IngressBackend>,

    /// IngressClassName is the name of the IngressClass cluster resource. The associated IngressClass defines which controller will implement the resource. This replaces the deprecated `kubernetes.io/ingress.class` annotation. For backwards compatibility, when that annotation is set, it must be given precedence over this field. The controller may emit a warning if the field and annotation have different values. Implementations of this API should ignore Ingresses without a class specified. An IngressClass resource may be marked as default, which can be used to set a default value for this field. For more information, refer to the IngressClass documentation.
    pub ingress_class_name: Option<String>,

    /// A list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend.
    pub rules: Vec<crate::api::networking::v1::IngressRule>,

    /// TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI.
    pub tls: Vec<crate::api::networking::v1::IngressTLS>,
}

impl<'de> crate::serde::Deserialize<'de> for IngressSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_default_backend,
            Key_ingress_class_name,
            Key_rules,
            Key_tls,
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
                            "defaultBackend" => Field::Key_default_backend,
                            "ingressClassName" => Field::Key_ingress_class_name,
                            "rules" => Field::Key_rules,
                            "tls" => Field::Key_tls,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IngressSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IngressSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_default_backend: Option<crate::api::networking::v1::IngressBackend> = None;
                let mut value_ingress_class_name: Option<String> = None;
                let mut value_rules: Option<Vec<crate::api::networking::v1::IngressRule>> = None;
                let mut value_tls: Option<Vec<crate::api::networking::v1::IngressTLS>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_default_backend => value_default_backend = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ingress_class_name => value_ingress_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tls => value_tls = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressSpec {
                    default_backend: value_default_backend,
                    ingress_class_name: value_ingress_class_name,
                    rules: value_rules.unwrap_or_default(),
                    tls: value_tls.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressSpec",
            &[
                "defaultBackend",
                "ingressClassName",
                "rules",
                "tls",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IngressSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressSpec",
            self.default_backend.as_ref().map_or(0, |_| 1) +
            self.ingress_class_name.as_ref().map_or(0, |_| 1) +
            usize::from(!self.rules.is_empty()) +
            usize::from(!self.tls.is_empty()),
        )?;
        if let Some(value) = &self.default_backend {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "defaultBackend", value)?;
        }
        if let Some(value) = &self.ingress_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ingressClassName", value)?;
        }
        if !self.rules.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", &self.rules)?;
        }
        if !self.tls.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tls", &self.tls)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for IngressSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "IngressSpec describes the Ingress the user wishes to exist.",
          "properties": {
            "defaultBackend": crate::schema_ref_with_description(crate::api::networking::v1::IngressBackend::schema(), "DefaultBackend is the backend that should handle requests that don't match any rule. If Rules are not specified, DefaultBackend must be specified. If DefaultBackend is not set, the handling of requests that do not match any of the rules will be up to the Ingress controller."),
            "ingressClassName": {
              "description": "IngressClassName is the name of the IngressClass cluster resource. The associated IngressClass defines which controller will implement the resource. This replaces the deprecated `kubernetes.io/ingress.class` annotation. For backwards compatibility, when that annotation is set, it must be given precedence over this field. The controller may emit a warning if the field and annotation have different values. Implementations of this API should ignore Ingresses without a class specified. An IngressClass resource may be marked as default, which can be used to set a default value for this field. For more information, refer to the IngressClass documentation.",
              "type": "string"
            },
            "rules": {
              "description": "A list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend.",
              "items": crate::api::networking::v1::IngressRule::schema(),
              "type": "array"
            },
            "tls": {
              "description": "TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI.",
              "items": crate::api::networking::v1::IngressTLS::schema(),
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
