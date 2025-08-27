// Generated from definition io.k8s.api.admissionregistration.v1.MutatingWebhookConfiguration

/// MutatingWebhookConfiguration describes the configuration of and admission webhook that accept or reject and may change the object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MutatingWebhookConfiguration {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Webhooks is a list of webhooks and the affected resources and operations.
    pub webhooks: Option<std::vec::Vec<crate::api::admissionregistration::v1::MutatingWebhook>>,
}

impl crate::Resource for MutatingWebhookConfiguration {
    const API_VERSION: &'static str = "admissionregistration.k8s.io/v1";
    const GROUP: &'static str = "admissionregistration.k8s.io";
    const KIND: &'static str = "MutatingWebhookConfiguration";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "mutatingwebhookconfigurations";
    type Scope = crate::ClusterResourceScope;
}

impl crate::ListableResource for MutatingWebhookConfiguration {
    const LIST_KIND: &'static str = "MutatingWebhookConfigurationList";
}

impl crate::Metadata for MutatingWebhookConfiguration {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl crate::DeepMerge for MutatingWebhookConfiguration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::merge_strategies::list::map(
            &mut self.webhooks,
            other.webhooks,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for MutatingWebhookConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_webhooks,
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
                            "metadata" => Field::Key_metadata,
                            "webhooks" => Field::Key_webhooks,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = MutatingWebhookConfiguration;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_webhooks: Option<std::vec::Vec<crate::api::admissionregistration::v1::MutatingWebhook>> = None;

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
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_webhooks => value_webhooks = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MutatingWebhookConfiguration {
                    metadata: value_metadata.unwrap_or_default(),
                    webhooks: value_webhooks,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "metadata",
                "webhooks",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for MutatingWebhookConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            3 +
            self.webhooks.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.webhooks {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "webhooks", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for MutatingWebhookConfiguration {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.admissionregistration.v1.MutatingWebhookConfiguration".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "MutatingWebhookConfiguration describes the configuration of and admission webhook that accept or reject and may change the object.",
            "type": "object",
            "properties": {
                "apiVersion": {
                    "description": "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources",
                    "type": "string",
                },
                "kind": {
                    "description": "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds",
                    "type": "string",
                },
                "metadata": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>();
                    schema_obj.ensure_object().insert("description".into(), "Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.".into());
                    schema_obj
                }),
                "webhooks": {
                    "description": "Webhooks is a list of webhooks and the affected resources and operations.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::api::admissionregistration::v1::MutatingWebhook>()),
                },
            },
            "required": [
                "metadata",
            ],
        })
    }
}
