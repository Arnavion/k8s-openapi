// Generated from definition io.k8s.api.core.v1.ServiceAccount

/// ServiceAccount binds together: * a name, understood by users, and perhaps by peripheral systems, for an identity * a principal that can be authenticated and authorized * a set of secrets
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceAccount {
    /// AutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted. Can be overridden at the pod level.
    pub automount_service_account_token: Option<bool>,

    /// ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images in pods that reference this ServiceAccount. ImagePullSecrets are distinct from Secrets because Secrets can be mounted in the pod, but ImagePullSecrets are only accessed by the kubelet. More info: https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod
    pub image_pull_secrets: Option<std::vec::Vec<crate::api::core::v1::LocalObjectReference>>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Secrets is a list of the secrets in the same namespace that pods running using this ServiceAccount are allowed to use. Pods are only limited to this list if this service account has a "kubernetes.io/enforce-mountable-secrets" annotation set to "true". The "kubernetes.io/enforce-mountable-secrets" annotation is deprecated since v1.32. Prefer separate namespaces to isolate access to mounted secrets. This field should not be used to find auto-generated service account token secrets for use outside of pods. Instead, tokens can be requested directly using the TokenRequest API, or service account token secrets can be manually created. More info: https://kubernetes.io/docs/concepts/configuration/secret
    pub secrets: Option<std::vec::Vec<crate::api::core::v1::ObjectReference>>,
}

impl crate::Resource for ServiceAccount {
    const API_VERSION: &'static str = "v1";
    const GROUP: &'static str = "";
    const KIND: &'static str = "ServiceAccount";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "serviceaccounts";
    type Scope = crate::NamespaceResourceScope;
}

impl crate::ListableResource for ServiceAccount {
    const LIST_KIND: &'static str = "ServiceAccountList";
}

impl crate::Metadata for ServiceAccount {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl crate::DeepMerge for ServiceAccount {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.automount_service_account_token, other.automount_service_account_token);
        crate::merge_strategies::list::atomic(&mut self.image_pull_secrets, other.image_pull_secrets);
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::merge_strategies::list::map(
            &mut self.secrets,
            other.secrets,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for ServiceAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_automount_service_account_token,
            Key_image_pull_secrets,
            Key_metadata,
            Key_secrets,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "automountServiceAccountToken" => Field::Key_automount_service_account_token,
                            "imagePullSecrets" => Field::Key_image_pull_secrets,
                            "metadata" => Field::Key_metadata,
                            "secrets" => Field::Key_secrets,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceAccount;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_automount_service_account_token: Option<bool> = None;
                let mut value_image_pull_secrets: Option<std::vec::Vec<crate::api::core::v1::LocalObjectReference>> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_secrets: Option<std::vec::Vec<crate::api::core::v1::ObjectReference>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_automount_service_account_token => value_automount_service_account_token = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_pull_secrets => value_image_pull_secrets = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secrets => value_secrets = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceAccount {
                    automount_service_account_token: value_automount_service_account_token,
                    image_pull_secrets: value_image_pull_secrets,
                    metadata: value_metadata.unwrap_or_default(),
                    secrets: value_secrets,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "automountServiceAccountToken",
                "imagePullSecrets",
                "metadata",
                "secrets",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ServiceAccount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            3 +
            self.automount_service_account_token.as_ref().map_or(0, |_| 1) +
            self.image_pull_secrets.as_ref().map_or(0, |_| 1) +
            self.secrets.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        if let Some(value) = &self.automount_service_account_token {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "automountServiceAccountToken", value)?;
        }
        if let Some(value) = &self.image_pull_secrets {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "imagePullSecrets", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.secrets {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secrets", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServiceAccount {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.core.v1.ServiceAccount".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ServiceAccount binds together: * a name, understood by users, and perhaps by peripheral systems, for an identity * a principal that can be authenticated and authorized * a set of secrets",
            "type": "object",
            "properties": {
                "apiVersion": {
                    "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources",
                    "type": "string",
                },
                "automountServiceAccountToken": {
                    "description": "AutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted. Can be overridden at the pod level.",
                    "type": "boolean",
                },
                "imagePullSecrets": {
                    "description": "ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images in pods that reference this ServiceAccount. ImagePullSecrets are distinct from Secrets because Secrets can be mounted in the pod, but ImagePullSecrets are only accessed by the kubelet. More info: https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::LocalObjectReference>()),
                },
                "kind": {
                    "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds",
                    "type": "string",
                },
                "metadata": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>();
                    schema_obj.ensure_object().insert("description".into(), "Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".into());
                    schema_obj
                }),
                "secrets": {
                    "description": "Secrets is a list of the secrets in the same namespace that pods running using this ServiceAccount are allowed to use. Pods are only limited to this list if this service account has a \"kubernetes.io/enforce-mountable-secrets\" annotation set to \"true\". The \"kubernetes.io/enforce-mountable-secrets\" annotation is deprecated since v1.32. Prefer separate namespaces to isolate access to mounted secrets. This field should not be used to find auto-generated service account token secrets for use outside of pods. Instead, tokens can be requested directly using the TokenRequest API, or service account token secrets can be manually created. More info: https://kubernetes.io/docs/concepts/configuration/secret",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::core::v1::ObjectReference>()),
                },
            },
            "required": [
                "metadata",
            ],
        })
    }
}
