// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionStatus

/// CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionStatus {
    /// AcceptedNames are the names that are actually being used to serve discovery They may be different than the names in spec.
    pub accepted_names: ::v1_12::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames,

    /// Conditions indicate state for particular aspects of a CustomResourceDefinition
    pub conditions: Option<Vec<::v1_12::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionCondition>>,

    /// StoredVersions are all versions of CustomResources that were ever persisted. Tracking these versions allows a migration path for stored versions in etcd. The field is mutable so the migration controller can first finish a migration to another version (i.e. that no old objects are left in the storage), and then remove the rest of the versions from this list. None of the versions in this list can be removed from the spec.Versions field.
    pub stored_versions: Vec<String>,
}

impl<'de> ::serde::Deserialize<'de> for CustomResourceDefinitionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_accepted_names,
            Key_conditions,
            Key_stored_versions,
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

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CustomResourceDefinitionStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_accepted_names: Option<::v1_12::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames> = None;
                let mut value_conditions: Option<Vec<::v1_12::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionCondition>> = None;
                let mut value_stored_versions: Option<Vec<String>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_accepted_names => value_accepted_names = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_conditions => value_conditions = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stored_versions => value_stored_versions = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionStatus {
                    accepted_names: value_accepted_names.ok_or_else(|| ::serde::de::Error::missing_field("acceptedNames"))?,
                    conditions: value_conditions,
                    stored_versions: value_stored_versions.ok_or_else(|| ::serde::de::Error::missing_field("storedVersions"))?,
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

impl ::serde::Serialize for CustomResourceDefinitionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionStatus",
            0 +
            1 +
            self.conditions.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "acceptedNames", &self.accepted_names)?;
        if let Some(value) = &self.conditions {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "storedVersions", &self.stored_versions)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
