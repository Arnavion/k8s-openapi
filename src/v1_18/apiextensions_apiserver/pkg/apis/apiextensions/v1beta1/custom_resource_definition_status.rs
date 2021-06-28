// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionStatus

/// CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionStatus {
    /// acceptedNames are the names that are actually being used to serve discovery. They may be different than the names in spec.
    pub accepted_names: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames>,

    /// conditions indicate state for particular aspects of a CustomResourceDefinition
    pub conditions: Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionCondition>,

    /// storedVersions lists all versions of CustomResources that were ever persisted. Tracking these versions allows a migration path for stored versions in etcd. The field is mutable so a migration controller can finish a migration to another version (ensuring no old objects are left in storage), and then remove the rest of the versions from this list. Versions may not be removed from `spec.versions` while they exist in this list.
    pub stored_versions: Vec<String>,
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceDefinitionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_accepted_names,
            Key_conditions,
            Key_stored_versions,
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
                            "acceptedNames" => Field::Key_accepted_names,
                            "conditions" => Field::Key_conditions,
                            "storedVersions" => Field::Key_stored_versions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CustomResourceDefinitionStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_accepted_names: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames> = None;
                let mut value_conditions: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionCondition>> = None;
                let mut value_stored_versions: Option<Vec<String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_accepted_names => value_accepted_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stored_versions => value_stored_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionStatus {
                    accepted_names: value_accepted_names,
                    conditions: value_conditions.unwrap_or_default(),
                    stored_versions: value_stored_versions.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionStatus",
            &[
                "acceptedNames",
                "conditions",
                "storedVersions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CustomResourceDefinitionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionStatus",
            self.accepted_names.as_ref().map_or(0, |_| 1) +
            usize::from(!self.conditions.is_empty()) +
            usize::from(!self.stored_versions.is_empty()),
        )?;
        if let Some(value) = &self.accepted_names {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "acceptedNames", value)?;
        }
        if !self.conditions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", &self.conditions)?;
        }
        if !self.stored_versions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storedVersions", &self.stored_versions)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for CustomResourceDefinitionStatus {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition",
          "properties": {
            "acceptedNames": crate::schema_ref_with_description(crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames::schema(), "acceptedNames are the names that are actually being used to serve discovery. They may be different than the names in spec."),
            "conditions": {
              "description": "conditions indicate state for particular aspects of a CustomResourceDefinition",
              "items": crate::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionCondition::schema(),
              "type": "array"
            },
            "storedVersions": {
              "description": "storedVersions lists all versions of CustomResources that were ever persisted. Tracking these versions allows a migration path for stored versions in etcd. The field is mutable so a migration controller can finish a migration to another version (ensuring no old objects are left in storage), and then remove the rest of the versions from this list. Versions may not be removed from `spec.versions` while they exist in this list.",
              "items": {
                "type": "string"
              },
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
