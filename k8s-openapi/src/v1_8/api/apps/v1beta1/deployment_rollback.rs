// Generated from definition io.k8s.api.apps.v1beta1.DeploymentRollback

/// DEPRECATED. DeploymentRollback stores the information required to rollback a deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentRollback {
    /// Required: This must match the Name of a deployment.
    pub name: String,

    /// The config of this deployment rollback.
    pub rollback_to: crate::v1_8::api::apps::v1beta1::RollbackConfig,

    /// The annotations to be updated to a deployment
    pub updated_annotations: Option<std::collections::BTreeMap<String, String>>,
}

// Begin apps/v1beta1/DeploymentRollback

// Generated from operation createAppsV1beta1NamespacedDeploymentRollback

impl DeploymentRollback {
    /// create rollback of a Deployment
    ///
    /// Use [`CreateNamespacedDeploymentRollbackResponse`](./enum.CreateNamespacedDeploymentRollbackResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the DeploymentRollback
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    pub fn create_namespaced_deployment_rollback(
        name: &str,
        namespace: &str,
        body: &crate::v1_8::api::apps::v1beta1::DeploymentRollback,
        optional: CreateNamespacedDeploymentRollbackOptional<'_>,
    ) -> Result<http::Request<Vec<u8>>, crate::RequestError> {
        let CreateNamespacedDeploymentRollbackOptional {
            pretty,
        } = optional;
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/rollback?", name = name, namespace = namespace);
        let mut __query_pairs = url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = http::Request::post(__url);
        let __body = serde_json::to_vec(&body).map_err(crate::RequestError::Json)?;
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`DeploymentRollback::create_namespaced_deployment_rollback`](./struct.DeploymentRollback.html#method.create_namespaced_deployment_rollback)
#[derive(Clone, Copy, Debug, Default)]
pub struct CreateNamespacedDeploymentRollbackOptional<'a> {
    /// If 'true', then the output is pretty printed.
    pub pretty: Option<&'a str>,
}

/// Parses the HTTP response of [`DeploymentRollback::create_namespaced_deployment_rollback`](./struct.DeploymentRollback.html#method.create_namespaced_deployment_rollback)
#[derive(Debug)]
pub enum CreateNamespacedDeploymentRollbackResponse {
    Ok(crate::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl crate::Response for CreateNamespacedDeploymentRollbackResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedDeploymentRollbackResponse::Ok(result), buf.len()))
            },
            http::StatusCode::UNAUTHORIZED => Ok((CreateNamespacedDeploymentRollbackResponse::Unauthorized, 0)),
            _ => Ok((CreateNamespacedDeploymentRollbackResponse::Other, 0)),
        }
    }
}

// End apps/v1beta1/DeploymentRollback

impl crate::Resource for DeploymentRollback {
    fn api_version() -> &'static str {
        "apps/v1beta1"
    }

    fn group() -> &'static str {
        "apps"
    }

    fn kind() -> &'static str {
        "DeploymentRollback"
    }

    fn version() -> &'static str {
        "v1beta1"
    }
}

impl<'de> serde::Deserialize<'de> for DeploymentRollback {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_name,
            Key_rollback_to,
            Key_updated_annotations,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentRollback;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct DeploymentRollback")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_rollback_to: Option<crate::v1_8::api::apps::v1beta1::RollbackConfig> = None;
                let mut value_updated_annotations: Option<std::collections::BTreeMap<String, String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_rollback_to => value_rollback_to = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_updated_annotations => value_updated_annotations = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentRollback {
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    rollback_to: value_rollback_to.ok_or_else(|| serde::de::Error::missing_field("rollbackTo"))?,
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

impl serde::Serialize for DeploymentRollback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeploymentRollback",
            4 +
            self.updated_annotations.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "rollbackTo", &self.rollback_to)?;
        if let Some(value) = &self.updated_annotations {
            serde::ser::SerializeStruct::serialize_field(&mut state, "updatedAnnotations", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
