// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceDefinitionStatus

/// CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionStatus {
    /// acceptedNames are the names that are actually being used to serve discovery. They may be different than the names in spec.
    pub accepted_names: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames>,

    /// conditions indicate state for particular aspects of a CustomResourceDefinition
    pub conditions: Option<std::vec::Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionCondition>>,

    /// The generation observed by the CRD controller.
    pub observed_generation: Option<i64>,

    /// storedVersions lists all versions of CustomResources that were ever persisted. Tracking these versions allows a migration path for stored versions in etcd. The field is mutable so a migration controller can finish a migration to another version (ensuring no old objects are left in storage), and then remove the rest of the versions from this list. Versions may not be removed from `spec.versions` while they exist in this list.
    pub stored_versions: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for CustomResourceDefinitionStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.accepted_names, other.accepted_names);
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.observed_generation, other.observed_generation);
        crate::merge_strategies::list::atomic(&mut self.stored_versions, other.stored_versions);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CustomResourceDefinitionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_accepted_names,
            Key_conditions,
            Key_observed_generation,
            Key_stored_versions,
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
                            "acceptedNames" => Field::Key_accepted_names,
                            "conditions" => Field::Key_conditions,
                            "observedGeneration" => Field::Key_observed_generation,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CustomResourceDefinitionStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_accepted_names: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames> = None;
                let mut value_conditions: Option<std::vec::Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionCondition>> = None;
                let mut value_observed_generation: Option<i64> = None;
                let mut value_stored_versions: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_accepted_names => value_accepted_names = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observed_generation => value_observed_generation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stored_versions => value_stored_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionStatus {
                    accepted_names: value_accepted_names,
                    conditions: value_conditions,
                    observed_generation: value_observed_generation,
                    stored_versions: value_stored_versions,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionStatus",
            &[
                "acceptedNames",
                "conditions",
                "observedGeneration",
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
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.observed_generation.as_ref().map_or(0, |_| 1) +
            self.stored_versions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.accepted_names {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "acceptedNames", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.observed_generation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observedGeneration", value)?;
        }
        if let Some(value) = &self.stored_versions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storedVersions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CustomResourceDefinitionStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.CustomResourceDefinitionStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition",
            "type": "object",
            "properties": {
                "acceptedNames": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames>();
                    schema_obj.ensure_object().insert("description".into(), "acceptedNames are the names that are actually being used to serve discovery. They may be different than the names in spec.".into());
                    schema_obj
                }),
                "conditions": {
                    "description": "conditions indicate state for particular aspects of a CustomResourceDefinition",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionCondition>()),
                },
                "observedGeneration": {
                    "description": "The generation observed by the CRD controller.",
                    "type": "integer",
                    "format": "int64",
                },
                "storedVersions": {
                    "description": "storedVersions lists all versions of CustomResources that were ever persisted. Tracking these versions allows a migration path for stored versions in etcd. The field is mutable so a migration controller can finish a migration to another version (ensuring no old objects are left in storage), and then remove the rest of the versions from this list. Versions may not be removed from `spec.versions` while they exist in this list.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
