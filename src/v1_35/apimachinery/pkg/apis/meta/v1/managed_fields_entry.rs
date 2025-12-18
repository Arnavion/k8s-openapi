// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry

/// ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ManagedFieldsEntry {
    /// APIVersion defines the version of this resource that this field set applies to. The format is "group/version" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted.
    pub api_version: Option<std::string::String>,

    /// FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: "FieldsV1"
    pub fields_type: Option<std::string::String>,

    /// FieldsV1 holds the first JSON version format as described in the "FieldsV1" type.
    pub fields_v1: Option<crate::apimachinery::pkg::apis::meta::v1::FieldsV1>,

    /// Manager is an identifier of the workflow managing these fields.
    pub manager: Option<std::string::String>,

    /// Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'.
    pub operation: Option<std::string::String>,

    /// Subresource is the name of the subresource used to update that object, or empty string if the object was updated through the main resource. The value of this field is used to distinguish between managers, even if they share the same name. For example, a status update will be distinct from a regular update using the same manager name. Note that the APIVersion field is not related to the Subresource field and it always corresponds to the version of the main resource.
    pub subresource: Option<std::string::String>,

    /// Time is the timestamp of when the ManagedFields entry was added. The timestamp will also be updated if a field is added, the manager changes any of the owned fields value or removes a field. The timestamp does not update when a field is removed from the entry because another manager took it over.
    pub time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,
}

impl crate::DeepMerge for ManagedFieldsEntry {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        crate::DeepMerge::merge_from(&mut self.fields_type, other.fields_type);
        crate::DeepMerge::merge_from(&mut self.fields_v1, other.fields_v1);
        crate::DeepMerge::merge_from(&mut self.manager, other.manager);
        crate::DeepMerge::merge_from(&mut self.operation, other.operation);
        crate::DeepMerge::merge_from(&mut self.subresource, other.subresource);
        crate::DeepMerge::merge_from(&mut self.time, other.time);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ManagedFieldsEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_fields_type,
            Key_fields_v1,
            Key_manager,
            Key_operation,
            Key_subresource,
            Key_time,
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
                            "fieldsType" => Field::Key_fields_type,
                            "fieldsV1" => Field::Key_fields_v1,
                            "manager" => Field::Key_manager,
                            "operation" => Field::Key_operation,
                            "subresource" => Field::Key_subresource,
                            "time" => Field::Key_time,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ManagedFieldsEntry;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ManagedFieldsEntry")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<std::string::String> = None;
                let mut value_fields_type: Option<std::string::String> = None;
                let mut value_fields_v1: Option<crate::apimachinery::pkg::apis::meta::v1::FieldsV1> = None;
                let mut value_manager: Option<std::string::String> = None;
                let mut value_operation: Option<std::string::String> = None;
                let mut value_subresource: Option<std::string::String> = None;
                let mut value_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fields_type => value_fields_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fields_v1 => value_fields_v1 = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_manager => value_manager = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operation => value_operation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subresource => value_subresource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_time => value_time = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ManagedFieldsEntry {
                    api_version: value_api_version,
                    fields_type: value_fields_type,
                    fields_v1: value_fields_v1,
                    manager: value_manager,
                    operation: value_operation,
                    subresource: value_subresource,
                    time: value_time,
                })
            }
        }

        deserializer.deserialize_struct(
            "ManagedFieldsEntry",
            &[
                "apiVersion",
                "fieldsType",
                "fieldsV1",
                "manager",
                "operation",
                "subresource",
                "time",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ManagedFieldsEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ManagedFieldsEntry",
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.fields_type.as_ref().map_or(0, |_| 1) +
            self.fields_v1.as_ref().map_or(0, |_| 1) +
            self.manager.as_ref().map_or(0, |_| 1) +
            self.operation.as_ref().map_or(0, |_| 1) +
            self.subresource.as_ref().map_or(0, |_| 1) +
            self.time.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.fields_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldsType", value)?;
        }
        if let Some(value) = &self.fields_v1 {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldsV1", value)?;
        }
        if let Some(value) = &self.manager {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "manager", value)?;
        }
        if let Some(value) = &self.operation {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operation", value)?;
        }
        if let Some(value) = &self.subresource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "subresource", value)?;
        }
        if let Some(value) = &self.time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "time", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ManagedFieldsEntry {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.",
            "type": "object",
            "properties": {
                "apiVersion": {
                    "description": "APIVersion defines the version of this resource that this field set applies to. The format is \"group/version\" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted.",
                    "type": "string",
                },
                "fieldsType": {
                    "description": "FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: \"FieldsV1\"",
                    "type": "string",
                },
                "fieldsV1": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::FieldsV1>();
                    schema_obj.ensure_object().insert("description".into(), "FieldsV1 holds the first JSON version format as described in the \"FieldsV1\" type.".into());
                    schema_obj
                }),
                "manager": {
                    "description": "Manager is an identifier of the workflow managing these fields.",
                    "type": "string",
                },
                "operation": {
                    "description": "Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'.",
                    "type": "string",
                },
                "subresource": {
                    "description": "Subresource is the name of the subresource used to update that object, or empty string if the object was updated through the main resource. The value of this field is used to distinguish between managers, even if they share the same name. For example, a status update will be distinct from a regular update using the same manager name. Note that the APIVersion field is not related to the Subresource field and it always corresponds to the version of the main resource.",
                    "type": "string",
                },
                "time": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>();
                    schema_obj.ensure_object().insert("description".into(), "Time is the timestamp of when the ManagedFields entry was added. The timestamp will also be updated if a field is added, the manager changes any of the owned fields value or removes a field. The timestamp does not update when a field is removed from the entry because another manager took it over.".into());
                    schema_obj
                }),
            },
        })
    }
}
