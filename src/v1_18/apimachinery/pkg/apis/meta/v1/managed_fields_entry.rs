// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry

/// ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ManagedFieldsEntry {
    /// APIVersion defines the version of this resource that this field set applies to. The format is "group/version" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted.
    pub api_version: Option<String>,

    /// FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: "FieldsV1"
    pub fields_type: Option<String>,

    /// FieldsV1 holds the first JSON version format as described in the "FieldsV1" type.
    pub fields_v1: Option<crate::apimachinery::pkg::apis::meta::v1::FieldsV1>,

    /// Manager is an identifier of the workflow managing these fields.
    pub manager: Option<String>,

    /// Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'.
    pub operation: Option<String>,

    /// Time is timestamp of when these fields were set. It should always be empty if Operation is 'Apply'
    pub time: Option<crate::apimachinery::pkg::apis::meta::v1::Time>,

}

#[cfg(feature = "dsl")]
impl ManagedFieldsEntry  {
    /// Set [`Self::api_version`]
    pub  fn api_version_set(&mut self, api_version: impl Into<Option<String>>) -> &mut Self {
        self.api_version = api_version.into(); self
    }

    pub  fn api_version(&mut self) -> &mut String {
        if self.api_version.is_none() { self.api_version = Some(Default::default()) }
        self.api_version.as_mut().unwrap()
    }

    /// Modify [`Self::api_version`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn api_version_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.api_version.is_none() { self.api_version = Some(Default::default()) };
        func(self.api_version.as_mut().unwrap()); self
    }


    /// Set [`Self::fields_type`]
    pub  fn fields_type_set(&mut self, fields_type: impl Into<Option<String>>) -> &mut Self {
        self.fields_type = fields_type.into(); self
    }

    pub  fn fields_type(&mut self) -> &mut String {
        if self.fields_type.is_none() { self.fields_type = Some(Default::default()) }
        self.fields_type.as_mut().unwrap()
    }

    /// Modify [`Self::fields_type`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn fields_type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.fields_type.is_none() { self.fields_type = Some(Default::default()) };
        func(self.fields_type.as_mut().unwrap()); self
    }


    /// Set [`Self::fields_v1`]
    pub  fn fields_v1_set(&mut self, fields_v1: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::FieldsV1>>) -> &mut Self {
        self.fields_v1 = fields_v1.into(); self
    }

    pub  fn fields_v1(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::FieldsV1 {
        if self.fields_v1.is_none() { self.fields_v1 = Some(Default::default()) }
        self.fields_v1.as_mut().unwrap()
    }

    /// Modify [`Self::fields_v1`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn fields_v1_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::FieldsV1)) -> &mut Self {
        if self.fields_v1.is_none() { self.fields_v1 = Some(Default::default()) };
        func(self.fields_v1.as_mut().unwrap()); self
    }


    /// Set [`Self::manager`]
    pub  fn manager_set(&mut self, manager: impl Into<Option<String>>) -> &mut Self {
        self.manager = manager.into(); self
    }

    pub  fn manager(&mut self) -> &mut String {
        if self.manager.is_none() { self.manager = Some(Default::default()) }
        self.manager.as_mut().unwrap()
    }

    /// Modify [`Self::manager`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn manager_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.manager.is_none() { self.manager = Some(Default::default()) };
        func(self.manager.as_mut().unwrap()); self
    }


    /// Set [`Self::operation`]
    pub  fn operation_set(&mut self, operation: impl Into<Option<String>>) -> &mut Self {
        self.operation = operation.into(); self
    }

    pub  fn operation(&mut self) -> &mut String {
        if self.operation.is_none() { self.operation = Some(Default::default()) }
        self.operation.as_mut().unwrap()
    }

    /// Modify [`Self::operation`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn operation_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.operation.is_none() { self.operation = Some(Default::default()) };
        func(self.operation.as_mut().unwrap()); self
    }


    /// Set [`Self::time`]
    pub  fn time_set(&mut self, time: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::Time>>) -> &mut Self {
        self.time = time.into(); self
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
            Key_time,
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
                            "fieldsType" => Field::Key_fields_type,
                            "fieldsV1" => Field::Key_fields_v1,
                            "manager" => Field::Key_manager,
                            "operation" => Field::Key_operation,
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

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ManagedFieldsEntry")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_fields_type: Option<String> = None;
                let mut value_fields_v1: Option<crate::apimachinery::pkg::apis::meta::v1::FieldsV1> = None;
                let mut value_manager: Option<String> = None;
                let mut value_operation: Option<String> = None;
                let mut value_time: Option<crate::apimachinery::pkg::apis::meta::v1::Time> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fields_type => value_fields_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fields_v1 => value_fields_v1 = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_manager => value_manager = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operation => value_operation = crate::serde::de::MapAccess::next_value(&mut map)?,
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
        if let Some(value) = &self.time {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "time", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ManagedFieldsEntry {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.ManagedFieldsEntry".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIVersion defines the version of this resource that this field set applies to. The format is \"group/version\" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fieldsType".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: \"FieldsV1\"".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "fieldsV1".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::FieldsV1>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FieldsV1 holds the first JSON version format as described in the \"FieldsV1\" type.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "manager".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Manager is an identifier of the workflow managing these fields.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "operation".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "time".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::Time>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Time is timestamp of when these fields were set. It should always be empty if Operation is 'Apply'".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
