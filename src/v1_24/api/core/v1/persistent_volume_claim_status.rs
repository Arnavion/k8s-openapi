// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimStatus

/// PersistentVolumeClaimStatus is the current status of a persistent volume claim.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaimStatus {
    /// accessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    pub access_modes: Option<Vec<String>>,

    /// allocatedResources is the storage resource within AllocatedResources tracks the capacity allocated to a PVC. It may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
    pub allocated_resources: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// capacity represents the actual resources of the underlying volume.
    pub capacity: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// conditions is the current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'ResizeStarted'.
    pub conditions: Option<Vec<crate::api::core::v1::PersistentVolumeClaimCondition>>,

    /// phase represents the current phase of PersistentVolumeClaim.
    ///
    pub phase: Option<String>,

    /// resizeStatus stores status of resize operation. ResizeStatus is not set by default but when expansion is complete resizeStatus is set to empty string by resize controller or kubelet. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
    pub resize_status: Option<String>,

}

#[cfg(feature = "dsl")]
impl PersistentVolumeClaimStatus  {
    /// Set [`Self::access_modes`]
    pub  fn access_modes_set(&mut self, access_modes: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.access_modes = access_modes.into(); self
    }

    pub  fn access_modes(&mut self) -> &mut Vec<String> {
        if self.access_modes.is_none() { self.access_modes = Some(Default::default()) }
        self.access_modes.as_mut().unwrap()
    }

    /// Modify [`Self::access_modes`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn access_modes_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.access_modes.is_none() { self.access_modes = Some(Default::default()) };
        func(self.access_modes.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::access_modes`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn access_modes_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.access_modes.is_none() {
            self.access_modes = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.access_modes.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::access_modes`]
    pub  fn access_modes_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.access_modes.is_none() { self.access_modes = Some(Vec::new()); }
         let access_modes = &mut self.access_modes.as_mut().unwrap();
         for item in other.borrow() {
             access_modes.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::allocated_resources`]
    pub  fn allocated_resources_set(&mut self, allocated_resources: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.allocated_resources = allocated_resources.into(); self
    }

    pub  fn allocated_resources(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.allocated_resources.is_none() { self.allocated_resources = Some(Default::default()) }
        self.allocated_resources.as_mut().unwrap()
    }

    /// Modify [`Self::allocated_resources`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allocated_resources_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.allocated_resources.is_none() { self.allocated_resources = Some(Default::default()) };
        func(self.allocated_resources.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::allocated_resources`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn allocated_resources_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.allocated_resources.is_none() {
            self.allocated_resources = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.allocated_resources.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::allocated_resources`]
    pub  fn allocated_resources_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.allocated_resources.is_none() { self.allocated_resources = Some(std::collections::BTreeMap::new()); }
         let allocated_resources = &mut self.allocated_resources.as_mut().unwrap();
         for (name, value) in other.borrow() {
             allocated_resources.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::capacity`]
    pub  fn capacity_set(&mut self, capacity: impl Into<Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>>) -> &mut Self {
        self.capacity = capacity.into(); self
    }

    pub  fn capacity(&mut self) -> &mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> {
        if self.capacity.is_none() { self.capacity = Some(Default::default()) }
        self.capacity.as_mut().unwrap()
    }

    /// Modify [`Self::capacity`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn capacity_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>)) -> &mut Self {
        if self.capacity.is_none() { self.capacity = Some(Default::default()) };
        func(self.capacity.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::capacity`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn capacity_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apimachinery::pkg::api::resource::Quantity)) -> &mut Self {
        if self.capacity.is_none() {
            self.capacity = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.capacity.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::capacity`]
    pub  fn capacity_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>) -> &mut Self {
         if self.capacity.is_none() { self.capacity = Some(std::collections::BTreeMap::new()); }
         let capacity = &mut self.capacity.as_mut().unwrap();
         for (name, value) in other.borrow() {
             capacity.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::conditions`]
    pub  fn conditions_set(&mut self, conditions: impl Into<Option<Vec<crate::api::core::v1::PersistentVolumeClaimCondition>>>) -> &mut Self {
        self.conditions = conditions.into(); self
    }

    pub  fn conditions(&mut self) -> &mut Vec<crate::api::core::v1::PersistentVolumeClaimCondition> {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) }
        self.conditions.as_mut().unwrap()
    }

    /// Modify [`Self::conditions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn conditions_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::PersistentVolumeClaimCondition>)) -> &mut Self {
        if self.conditions.is_none() { self.conditions = Some(Default::default()) };
        func(self.conditions.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::conditions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn conditions_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PersistentVolumeClaimCondition)) -> &mut Self {
        if self.conditions.is_none() {
            self.conditions = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.conditions.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::conditions`]
    pub  fn conditions_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::PersistentVolumeClaimCondition]>) -> &mut Self {
         if self.conditions.is_none() { self.conditions = Some(Vec::new()); }
         let conditions = &mut self.conditions.as_mut().unwrap();
         for item in other.borrow() {
             conditions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::phase`]
    pub  fn phase_set(&mut self, phase: impl Into<Option<String>>) -> &mut Self {
        self.phase = phase.into(); self
    }

    pub  fn phase(&mut self) -> &mut String {
        if self.phase.is_none() { self.phase = Some(Default::default()) }
        self.phase.as_mut().unwrap()
    }

    /// Modify [`Self::phase`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn phase_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.phase.is_none() { self.phase = Some(Default::default()) };
        func(self.phase.as_mut().unwrap()); self
    }


    /// Set [`Self::resize_status`]
    pub  fn resize_status_set(&mut self, resize_status: impl Into<Option<String>>) -> &mut Self {
        self.resize_status = resize_status.into(); self
    }

    pub  fn resize_status(&mut self) -> &mut String {
        if self.resize_status.is_none() { self.resize_status = Some(Default::default()) }
        self.resize_status.as_mut().unwrap()
    }

    /// Modify [`Self::resize_status`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn resize_status_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.resize_status.is_none() { self.resize_status = Some(Default::default()) };
        func(self.resize_status.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for PersistentVolumeClaimStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_access_modes,
            Key_allocated_resources,
            Key_capacity,
            Key_conditions,
            Key_phase,
            Key_resize_status,
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
                            "accessModes" => Field::Key_access_modes,
                            "allocatedResources" => Field::Key_allocated_resources,
                            "capacity" => Field::Key_capacity,
                            "conditions" => Field::Key_conditions,
                            "phase" => Field::Key_phase,
                            "resizeStatus" => Field::Key_resize_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeClaimStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PersistentVolumeClaimStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_access_modes: Option<Vec<String>> = None;
                let mut value_allocated_resources: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_capacity: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_conditions: Option<Vec<crate::api::core::v1::PersistentVolumeClaimCondition>> = None;
                let mut value_phase: Option<String> = None;
                let mut value_resize_status: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_modes => value_access_modes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allocated_resources => value_allocated_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resize_status => value_resize_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeClaimStatus {
                    access_modes: value_access_modes,
                    allocated_resources: value_allocated_resources,
                    capacity: value_capacity,
                    conditions: value_conditions,
                    phase: value_phase,
                    resize_status: value_resize_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeClaimStatus",
            &[
                "accessModes",
                "allocatedResources",
                "capacity",
                "conditions",
                "phase",
                "resizeStatus",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PersistentVolumeClaimStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeClaimStatus",
            self.access_modes.as_ref().map_or(0, |_| 1) +
            self.allocated_resources.as_ref().map_or(0, |_| 1) +
            self.capacity.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1) +
            self.resize_status.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.access_modes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "accessModes", value)?;
        }
        if let Some(value) = &self.allocated_resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocatedResources", value)?;
        }
        if let Some(value) = &self.capacity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", value)?;
        }
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.phase {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        if let Some(value) = &self.resize_status {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resizeStatus", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PersistentVolumeClaimStatus {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.PersistentVolumeClaimStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PersistentVolumeClaimStatus is the current status of a persistent volume claim.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "accessModes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("accessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allocatedResources".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("allocatedResources is the storage resource within AllocatedResources tracks the capacity allocated to a PVC. It may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "capacity".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("capacity represents the actual resources of the underlying volume.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "conditions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("conditions is the current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'ResizeStarted'.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::PersistentVolumeClaimCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "phase".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("phase represents the current phase of PersistentVolumeClaim.\n\n".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "resizeStatus".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("resizeStatus stores status of resize operation. ResizeStatus is not set by default but when expansion is complete resizeStatus is set to empty string by resize controller or kubelet. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
