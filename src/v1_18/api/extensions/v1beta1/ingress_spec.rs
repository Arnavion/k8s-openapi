// Generated from definition io.k8s.api.extensions.v1beta1.IngressSpec

/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IngressSpec {
    /// A default backend capable of servicing requests that don't match any rule. At least one of 'backend' or 'rules' must be specified. This field is optional to allow the loadbalancer controller or defaulting logic to specify a global default.
    pub backend: Option<crate::api::extensions::v1beta1::IngressBackend>,

    /// IngressClassName is the name of the IngressClass cluster resource. The associated IngressClass defines which controller will implement the resource. This replaces the deprecated `kubernetes.io/ingress.class` annotation. For backwards compatibility, when that annotation is set, it must be given precedence over this field. The controller may emit a warning if the field and annotation have different values. Implementations of this API should ignore Ingresses without a class specified. An IngressClass resource may be marked as default, which can be used to set a default value for this field. For more information, refer to the IngressClass documentation.
    pub ingress_class_name: Option<String>,

    /// A list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend.
    pub rules: Option<Vec<crate::api::extensions::v1beta1::IngressRule>>,

    /// TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI.
    pub tls: Option<Vec<crate::api::extensions::v1beta1::IngressTLS>>,
}

impl<'de> serde::Deserialize<'de> for IngressSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_backend,
            Key_ingress_class_name,
            Key_rules,
            Key_tls,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "backend" => Field::Key_backend,
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IngressSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IngressSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_backend: Option<crate::api::extensions::v1beta1::IngressBackend> = None;
                let mut value_ingress_class_name: Option<String> = None;
                let mut value_rules: Option<Vec<crate::api::extensions::v1beta1::IngressRule>> = None;
                let mut value_tls: Option<Vec<crate::api::extensions::v1beta1::IngressTLS>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_backend => value_backend = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ingress_class_name => value_ingress_class_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tls => value_tls = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IngressSpec {
                    backend: value_backend,
                    ingress_class_name: value_ingress_class_name,
                    rules: value_rules,
                    tls: value_tls,
                })
            }
        }

        deserializer.deserialize_struct(
            "IngressSpec",
            &[
                "backend",
                "ingressClassName",
                "rules",
                "tls",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for IngressSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IngressSpec",
            self.backend.as_ref().map_or(0, |_| 1) +
            self.ingress_class_name.as_ref().map_or(0, |_| 1) +
            self.rules.as_ref().map_or(0, |_| 1) +
            self.tls.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.backend {
            serde::ser::SerializeStruct::serialize_field(&mut state, "backend", value)?;
        }
        if let Some(value) = &self.ingress_class_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "ingressClassName", value)?;
        }
        if let Some(value) = &self.rules {
            serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        if let Some(value) = &self.tls {
            serde::ser::SerializeStruct::serialize_field(&mut state, "tls", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
