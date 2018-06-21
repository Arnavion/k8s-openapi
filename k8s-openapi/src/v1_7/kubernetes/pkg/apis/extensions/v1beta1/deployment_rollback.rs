// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.DeploymentRollback

/// DeploymentRollback stores the information required to rollback a deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentRollback {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Required: This must match the Name of a deployment.
    pub name: String,

    /// The config of this deployment rollback.
    pub rollback_to: ::v1_7::kubernetes::pkg::apis::extensions::v1beta1::RollbackConfig,

    /// The annotations to be updated to a deployment
    pub updated_annotations: Option<::std::collections::BTreeMap<String, String>>,
}

// Begin extensions/v1beta1/DeploymentRollback

// Generated from operation createExtensionsV1beta1NamespacedDeploymentRollback

#[derive(Debug)]
pub enum CreateExtensionsV1beta1NamespacedDeploymentRollbackResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DeploymentRollback),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl DeploymentRollback {
    /// create rollback of a Deployment
    pub fn create_extensions_v1beta1_namespaced_deployment_rollback<C>(
        __client: &C,
        // name of the DeploymentRollback
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DeploymentRollback,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateExtensionsV1beta1NamespacedDeploymentRollbackResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/rollback", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.post(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreateExtensionsV1beta1NamespacedDeploymentRollbackResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateExtensionsV1beta1NamespacedDeploymentRollbackResponse::Unauthorized(response),
            other => CreateExtensionsV1beta1NamespacedDeploymentRollbackResponse::Other(other, response),
        })
    }
}

// End extensions/v1beta1/DeploymentRollback

impl<'de> ::serde::Deserialize<'de> for DeploymentRollback {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_name,
            Key_rollback_to,
            Key_updated_annotations,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "rollbackTo" => Field::Key_rollback_to,
                            "updatedAnnotations" => Field::Key_updated_annotations,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentRollback;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct DeploymentRollback")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_rollback_to: Option<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::RollbackConfig> = None;
                let mut value_updated_annotations: Option<::std::collections::BTreeMap<String, String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_rollback_to => value_rollback_to = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_updated_annotations => value_updated_annotations = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentRollback {
                    api_version: value_api_version,
                    kind: value_kind,
                    name: value_name.ok_or_else(|| ::serde::de::Error::missing_field("name"))?,
                    rollback_to: value_rollback_to.ok_or_else(|| ::serde::de::Error::missing_field("rollbackTo"))?,
                    updated_annotations: value_updated_annotations,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeploymentRollback",
            &[
                "apiVersion",
                "kind",
                "name",
                "rollbackTo",
                "updatedAnnotations",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for DeploymentRollback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentRollback",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            1 +
            1 +
            self.updated_annotations.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "rollbackTo", &self.rollback_to)?;
        if let Some(value) = &self.updated_annotations {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "updatedAnnotations", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
