// Generated from definition io.k8s.api.admissionregistration.v1alpha1.Rule

/// Rule is a tuple of APIGroups, APIVersion, and Resources.It is recommended to make sure that all the tuple expansions are valid.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Rule {
    /// APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
    pub api_groups: Option<Vec<String>>,

    /// APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
    pub api_versions: Option<Vec<String>>,

    /// Resources is a list of resources this rule applies to.
    ///
    /// For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/*' means all resources and their subresources.
    ///
    /// If wildcard is present, the validation rule will ensure resources do not overlap with each other.
    ///
    /// Depending on the enclosing object, subresources might not be allowed. Required.
    pub resources: Option<Vec<String>>,
}

impl<'de> ::serde::Deserialize<'de> for Rule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_groups,
            Key_api_versions,
            Key_resources,
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
                            "apiGroups" => Field::Key_api_groups,
                            "apiVersions" => Field::Key_api_versions,
                            "resources" => Field::Key_resources,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Rule;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Rule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_groups: Option<Vec<String>> = None;
                let mut value_api_versions: Option<Vec<String>> = None;
                let mut value_resources: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_groups => value_api_groups = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_api_versions => value_api_versions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Rule {
                    api_groups: value_api_groups,
                    api_versions: value_api_versions,
                    resources: value_resources,
                })
            }
        }

        deserializer.deserialize_struct(
            "Rule",
            &[
                "apiGroups",
                "apiVersions",
                "resources",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Rule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Rule",
            0 +
            self.api_groups.as_ref().map_or(0, |_| 1) +
            self.api_versions.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_groups {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiGroups", value)?;
        }
        if let Some(value) = &self.api_versions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersions", value)?;
        }
        if let Some(value) = &self.resources {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
