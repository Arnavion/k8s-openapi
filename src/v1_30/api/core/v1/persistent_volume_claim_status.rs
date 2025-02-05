// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimStatus

/// PersistentVolumeClaimStatus is the current status of a persistent volume claim.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaimStatus {
    /// accessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    pub access_modes: Option<std::vec::Vec<std::string::String>>,

    /// allocatedResourceStatuses stores status of resource being resized for the given PVC. Key names follow standard Kubernetes label syntax. Valid values are either:
    ///     * Un-prefixed keys:
    ///         - storage - the capacity of the volume.
    ///     * Custom resources must use implementation-defined prefixed names such as "example.com/my-custom-resource"
    /// Apart from above values - keys that are unprefixed or have kubernetes.io prefix are considered reserved and hence may not be used.
    ///
    /// ClaimResourceStatus can be in any of following states:
    ///     - ControllerResizeInProgress:
    ///         State set when resize controller starts resizing the volume in control-plane.
    ///     - ControllerResizeFailed:
    ///         State set when resize has failed in resize controller with a terminal error.
    ///     - NodeResizePending:
    ///         State set when resize controller has finished resizing the volume but further resizing of
    ///         volume is needed on the node.
    ///     - NodeResizeInProgress:
    ///         State set when kubelet starts resizing the volume.
    ///     - NodeResizeFailed:
    ///         State set when resizing has failed in kubelet with a terminal error. Transient errors don't set
    ///         NodeResizeFailed.
    /// For example: if expanding a PVC for more capacity - this field can be one of the following states:
    ///     - pvc.status.allocatedResourceStatus\['storage'\] = "ControllerResizeInProgress"
    ///      - pvc.status.allocatedResourceStatus\['storage'\] = "ControllerResizeFailed"
    ///      - pvc.status.allocatedResourceStatus\['storage'\] = "NodeResizePending"
    ///      - pvc.status.allocatedResourceStatus\['storage'\] = "NodeResizeInProgress"
    ///      - pvc.status.allocatedResourceStatus\['storage'\] = "NodeResizeFailed"
    /// When this field is not set, it means that no resize operation is in progress for the given PVC.
    ///
    /// A controller that receives PVC update with previously unknown resourceName or ClaimResourceStatus should ignore the update for the purpose it was designed. For example - a controller that only is responsible for resizing capacity of the volume, should ignore PVC updates that change other valid resources associated with PVC.
    ///
    /// This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
    pub allocated_resource_statuses: Option<std::collections::BTreeMap<std::string::String, std::string::String>>,

    /// allocatedResources tracks the resources allocated to a PVC including its capacity. Key names follow standard Kubernetes label syntax. Valid values are either:
    ///     * Un-prefixed keys:
    ///         - storage - the capacity of the volume.
    ///     * Custom resources must use implementation-defined prefixed names such as "example.com/my-custom-resource"
    /// Apart from above values - keys that are unprefixed or have kubernetes.io prefix are considered reserved and hence may not be used.
    ///
    /// Capacity reported here may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity.
    ///
    /// A controller that receives PVC update with previously unknown resourceName should ignore the update for the purpose it was designed. For example - a controller that only is responsible for resizing capacity of the volume, should ignore PVC updates that change other valid resources associated with PVC.
    ///
    /// This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
    pub allocated_resources: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// capacity represents the actual resources of the underlying volume.
    pub capacity: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// conditions is the current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'Resizing'.
    pub conditions: Option<std::vec::Vec<crate::api::core::v1::PersistentVolumeClaimCondition>>,

    /// currentVolumeAttributesClassName is the current name of the VolumeAttributesClass the PVC is using. When unset, there is no VolumeAttributeClass applied to this PersistentVolumeClaim This is an alpha field and requires enabling VolumeAttributesClass feature.
    pub current_volume_attributes_class_name: Option<std::string::String>,

    /// ModifyVolumeStatus represents the status object of ControllerModifyVolume operation. When this is unset, there is no ModifyVolume operation being attempted. This is an alpha field and requires enabling VolumeAttributesClass feature.
    pub modify_volume_status: Option<crate::api::core::v1::ModifyVolumeStatus>,

    /// phase represents the current phase of PersistentVolumeClaim.
    pub phase: Option<std::string::String>,
}

impl crate::DeepMerge for PersistentVolumeClaimStatus {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.access_modes, other.access_modes);
        crate::merge_strategies::map::granular(&mut self.allocated_resource_statuses, other.allocated_resource_statuses, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.allocated_resources, other.allocated_resources, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::map::granular(&mut self.capacity, other.capacity, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::merge_strategies::list::map(
            &mut self.conditions,
            other.conditions,
            &[|lhs, rhs| lhs.type_ == rhs.type_],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.current_volume_attributes_class_name, other.current_volume_attributes_class_name);
        crate::DeepMerge::merge_from(&mut self.modify_volume_status, other.modify_volume_status);
        crate::DeepMerge::merge_from(&mut self.phase, other.phase);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PersistentVolumeClaimStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_access_modes,
            Key_allocated_resource_statuses,
            Key_allocated_resources,
            Key_capacity,
            Key_conditions,
            Key_current_volume_attributes_class_name,
            Key_modify_volume_status,
            Key_phase,
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
                            "accessModes" => Field::Key_access_modes,
                            "allocatedResourceStatuses" => Field::Key_allocated_resource_statuses,
                            "allocatedResources" => Field::Key_allocated_resources,
                            "capacity" => Field::Key_capacity,
                            "conditions" => Field::Key_conditions,
                            "currentVolumeAttributesClassName" => Field::Key_current_volume_attributes_class_name,
                            "modifyVolumeStatus" => Field::Key_modify_volume_status,
                            "phase" => Field::Key_phase,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("PersistentVolumeClaimStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_access_modes: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_allocated_resource_statuses: Option<std::collections::BTreeMap<std::string::String, std::string::String>> = None;
                let mut value_allocated_resources: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_capacity: Option<std::collections::BTreeMap<std::string::String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_conditions: Option<std::vec::Vec<crate::api::core::v1::PersistentVolumeClaimCondition>> = None;
                let mut value_current_volume_attributes_class_name: Option<std::string::String> = None;
                let mut value_modify_volume_status: Option<crate::api::core::v1::ModifyVolumeStatus> = None;
                let mut value_phase: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_modes => value_access_modes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allocated_resource_statuses => value_allocated_resource_statuses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allocated_resources => value_allocated_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_current_volume_attributes_class_name => value_current_volume_attributes_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_modify_volume_status => value_modify_volume_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_phase => value_phase = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeClaimStatus {
                    access_modes: value_access_modes,
                    allocated_resource_statuses: value_allocated_resource_statuses,
                    allocated_resources: value_allocated_resources,
                    capacity: value_capacity,
                    conditions: value_conditions,
                    current_volume_attributes_class_name: value_current_volume_attributes_class_name,
                    modify_volume_status: value_modify_volume_status,
                    phase: value_phase,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeClaimStatus",
            &[
                "accessModes",
                "allocatedResourceStatuses",
                "allocatedResources",
                "capacity",
                "conditions",
                "currentVolumeAttributesClassName",
                "modifyVolumeStatus",
                "phase",
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
            self.allocated_resource_statuses.as_ref().map_or(0, |_| 1) +
            self.allocated_resources.as_ref().map_or(0, |_| 1) +
            self.capacity.as_ref().map_or(0, |_| 1) +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.current_volume_attributes_class_name.as_ref().map_or(0, |_| 1) +
            self.modify_volume_status.as_ref().map_or(0, |_| 1) +
            self.phase.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.access_modes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "accessModes", value)?;
        }
        if let Some(value) = &self.allocated_resource_statuses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocatedResourceStatuses", value)?;
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
        if let Some(value) = &self.current_volume_attributes_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "currentVolumeAttributesClassName", value)?;
        }
        if let Some(value) = &self.modify_volume_status {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "modifyVolumeStatus", value)?;
        }
        if let Some(value) = &self.phase {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "phase", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PersistentVolumeClaimStatus {
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.PersistentVolumeClaimStatus".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("PersistentVolumeClaimStatus is the current status of a persistent volume claim.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "accessModes".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("accessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allocatedResourceStatuses".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("allocatedResourceStatuses stores status of resource being resized for the given PVC. Key names follow standard Kubernetes label syntax. Valid values are either:\n\t* Un-prefixed keys:\n\t\t- storage - the capacity of the volume.\n\t* Custom resources must use implementation-defined prefixed names such as \"example.com/my-custom-resource\"\nApart from above values - keys that are unprefixed or have kubernetes.io prefix are considered reserved and hence may not be used.\n\nClaimResourceStatus can be in any of following states:\n\t- ControllerResizeInProgress:\n\t\tState set when resize controller starts resizing the volume in control-plane.\n\t- ControllerResizeFailed:\n\t\tState set when resize has failed in resize controller with a terminal error.\n\t- NodeResizePending:\n\t\tState set when resize controller has finished resizing the volume but further resizing of\n\t\tvolume is needed on the node.\n\t- NodeResizeInProgress:\n\t\tState set when kubelet starts resizing the volume.\n\t- NodeResizeFailed:\n\t\tState set when resizing has failed in kubelet with a terminal error. Transient errors don't set\n\t\tNodeResizeFailed.\nFor example: if expanding a PVC for more capacity - this field can be one of the following states:\n\t- pvc.status.allocatedResourceStatus['storage'] = \"ControllerResizeInProgress\"\n     - pvc.status.allocatedResourceStatus['storage'] = \"ControllerResizeFailed\"\n     - pvc.status.allocatedResourceStatus['storage'] = \"NodeResizePending\"\n     - pvc.status.allocatedResourceStatus['storage'] = \"NodeResizeInProgress\"\n     - pvc.status.allocatedResourceStatus['storage'] = \"NodeResizeFailed\"\nWhen this field is not set, it means that no resize operation is in progress for the given PVC.\n\nA controller that receives PVC update with previously unknown resourceName or ClaimResourceStatus should ignore the update for the purpose it was designed. For example - a controller that only is responsible for resizing capacity of the volume, should ignore PVC updates that change other valid resources associated with PVC.\n\nThis is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                )),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "allocatedResources".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("allocatedResources tracks the resources allocated to a PVC including its capacity. Key names follow standard Kubernetes label syntax. Valid values are either:\n\t* Un-prefixed keys:\n\t\t- storage - the capacity of the volume.\n\t* Custom resources must use implementation-defined prefixed names such as \"example.com/my-custom-resource\"\nApart from above values - keys that are unprefixed or have kubernetes.io prefix are considered reserved and hence may not be used.\n\nCapacity reported here may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity.\n\nA controller that receives PVC update with previously unknown resourceName should ignore the update for the purpose it was designed. For example - a controller that only is responsible for resizing capacity of the volume, should ignore PVC updates that change other valid resources associated with PVC.\n\nThis is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "capacity".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("capacity represents the actual resources of the underlying volume.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(std::boxed::Box::new(__gen.subschema_for::<crate::apimachinery::pkg::api::resource::Quantity>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "conditions".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("conditions is the current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'Resizing'.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::core::v1::PersistentVolumeClaimCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "currentVolumeAttributesClassName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("currentVolumeAttributesClassName is the current name of the VolumeAttributesClass the PVC is using. When unset, there is no VolumeAttributeClass applied to this PersistentVolumeClaim This is an alpha field and requires enabling VolumeAttributesClass feature.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "modifyVolumeStatus".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ModifyVolumeStatus>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ModifyVolumeStatus represents the status object of ControllerModifyVolume operation. When this is unset, there is no ModifyVolume operation being attempted. This is an alpha field and requires enabling VolumeAttributesClass feature.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "phase".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("phase represents the current phase of PersistentVolumeClaim.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
