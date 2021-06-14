// Generated from definition io.k8s.api.extensions.v1beta1.DeploymentRollback

/// DEPRECATED. DeploymentRollback stores the information required to rollback a deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentRollback {
    /// Required: This must match the Name of a deployment.
    pub name: String,

    /// The config of this deployment rollback.
    pub rollback_to: crate::api::extensions::v1beta1::RollbackConfig,

    /// The annotations to be updated to a deployment
    pub updated_annotations: std::collections::BTreeMap<String, String>,
}

// Begin extensions/v1beta1/DeploymentRollback

// Generated from operation createExtensionsV1beta1NamespacedDeploymentRollback

impl DeploymentRollback {
    /// create rollback of a Deployment
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::CreateResponse`]`<Self>>` constructor, or [`crate::CreateResponse`]`<Self>` directly, to parse the HTTP response.
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
    #[cfg(feature = "api")]
    pub fn create_namespaced_deployment_rollback(
        name: &str,
        namespace: &str,
        body: &crate::api::extensions::v1beta1::DeploymentRollback,
        optional: crate::CreateOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::CreateResponse<Self>>), crate::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/deployments/{name}/rollback?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// End extensions/v1beta1/DeploymentRollback

impl crate::Resource for DeploymentRollback {
    const API_VERSION: &'static str = "extensions/v1beta1";
    const GROUP: &'static str = "extensions";
    const KIND: &'static str = "DeploymentRollback";
    const VERSION: &'static str = "v1beta1";
    const URL_PATH_SEGMENT: &'static str = "rollback";
    type Scope = crate::SubResourceScope;
}

impl<'de> crate::serde::Deserialize<'de> for DeploymentRollback {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_name,
            Key_rollback_to,
            Key_updated_annotations,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentRollback;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_rollback_to: Option<crate::api::extensions::v1beta1::RollbackConfig> = None;
                let mut value_updated_annotations: Option<std::collections::BTreeMap<String, String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_rollback_to => value_rollback_to = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_updated_annotations => value_updated_annotations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentRollback {
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    rollback_to: value_rollback_to.ok_or_else(|| crate::serde::de::Error::missing_field("rollbackTo"))?,
                    updated_annotations: value_updated_annotations.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
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

impl crate::serde::Serialize for DeploymentRollback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            4 +
            usize::from(!self.updated_annotations.is_empty()),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rollbackTo", &self.rollback_to)?;
        if !self.updated_annotations.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "updatedAnnotations", &self.updated_annotations)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
