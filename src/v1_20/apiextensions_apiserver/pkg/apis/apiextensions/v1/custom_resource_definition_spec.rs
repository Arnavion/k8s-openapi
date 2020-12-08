// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceDefinitionSpec

/// CustomResourceDefinitionSpec describes how a user wants their resource to appear
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionSpec {
    /// conversion defines conversion settings for the CRD.
    pub conversion: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion>,

    /// group is the API group of the defined custom resource. The custom resources are served under `/apis/\<group\>/...`. Must match the name of the CustomResourceDefinition (in the form `\<names.plural\>.\<group\>`).
    pub group: String,

    /// names specify the resource and kind names for the custom resource.
    pub names: crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,

    /// preserveUnknownFields indicates that object fields which are not specified in the OpenAPI schema should be preserved when persisting to storage. apiVersion, kind, metadata and known fields inside metadata are always preserved. This field is deprecated in favor of setting `x-preserve-unknown-fields` to true in `spec.versions\[*\].schema.openAPIV3Schema`. See https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions/#pruning-versus-preserving-unknown-fields for details.
    pub preserve_unknown_fields: Option<bool>,

    /// scope indicates whether the defined custom resource is cluster- or namespace-scoped. Allowed values are `Cluster` and `Namespaced`.
    pub scope: String,

    /// versions is the list of all API versions of the defined custom resource. Version names are used to compute the order in which served versions are listed in API discovery. If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version), then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first by GA \> beta \> alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
    pub versions: Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion>,
}

impl<'de> serde::Deserialize<'de> for CustomResourceDefinitionSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conversion,
            Key_group,
            Key_names,
            Key_preserve_unknown_fields,
            Key_scope,
            Key_versions,
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
                            "conversion" => Field::Key_conversion,
                            "group" => Field::Key_group,
                            "names" => Field::Key_names,
                            "preserveUnknownFields" => Field::Key_preserve_unknown_fields,
                            "scope" => Field::Key_scope,
                            "versions" => Field::Key_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceDefinitionSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_conversion: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion> = None;
                let mut value_group: Option<String> = None;
                let mut value_names: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames> = None;
                let mut value_preserve_unknown_fields: Option<bool> = None;
                let mut value_scope: Option<String> = None;
                let mut value_versions: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conversion => value_conversion = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_names => value_names = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_preserve_unknown_fields => value_preserve_unknown_fields = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scope => value_scope = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_versions => value_versions = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionSpec {
                    conversion: value_conversion,
                    group: value_group.ok_or_else(|| serde::de::Error::missing_field("group"))?,
                    names: value_names.ok_or_else(|| serde::de::Error::missing_field("names"))?,
                    preserve_unknown_fields: value_preserve_unknown_fields,
                    scope: value_scope.ok_or_else(|| serde::de::Error::missing_field("scope"))?,
                    versions: value_versions.ok_or_else(|| serde::de::Error::missing_field("versions"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionSpec",
            &[
                "conversion",
                "group",
                "names",
                "preserveUnknownFields",
                "scope",
                "versions",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for CustomResourceDefinitionSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionSpec",
            4 +
            self.conversion.as_ref().map_or(0, |_| 1) +
            self.preserve_unknown_fields.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.conversion {
            serde::ser::SerializeStruct::serialize_field(&mut state, "conversion", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "group", &self.group)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "names", &self.names)?;
        if let Some(value) = &self.preserve_unknown_fields {
            serde::ser::SerializeStruct::serialize_field(&mut state, "preserveUnknownFields", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "scope", &self.scope)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "versions", &self.versions)?;
        serde::ser::SerializeStruct::end(state)
    }
}
