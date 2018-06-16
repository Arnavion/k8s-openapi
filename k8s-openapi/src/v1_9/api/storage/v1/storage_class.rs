// Generated from definition io.k8s.api.storage.v1.StorageClass

/// StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
///
/// StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageClass {
    /// AllowVolumeExpansion shows whether the storage class allow volume expand
    pub allow_volume_expansion: Option<bool>,

    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Dynamically provisioned PersistentVolumes of this storage class are created with these mountOptions, e.g. \["ro", "soft"\]. Not validated - mount of the PVs will simply fail if one is invalid.
    pub mount_options: Option<Vec<String>>,

    /// Parameters holds the parameters for the provisioner that should create volumes of this storage class.
    pub parameters: Option<::std::collections::BTreeMap<String, String>>,

    /// Provisioner indicates the type of the provisioner.
    pub provisioner: String,

    /// Dynamically provisioned PersistentVolumes of this storage class are created with this reclaimPolicy. Defaults to Delete.
    pub reclaim_policy: Option<String>,

    /// VolumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.  When unset, VolumeBindingImmediate is used. This field is alpha-level and is only honored by servers that enable the VolumeScheduling feature.
    pub volume_binding_mode: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for StorageClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allow_volume_expansion,
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_mount_options,
            Key_parameters,
            Key_provisioner,
            Key_reclaim_policy,
            Key_volume_binding_mode,
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
                            "allowVolumeExpansion" => Field::Key_allow_volume_expansion,
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "mountOptions" => Field::Key_mount_options,
                            "parameters" => Field::Key_parameters,
                            "provisioner" => Field::Key_provisioner,
                            "reclaimPolicy" => Field::Key_reclaim_policy,
                            "volumeBindingMode" => Field::Key_volume_binding_mode,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StorageClass;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct StorageClass")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_allow_volume_expansion: Option<bool> = None;
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_mount_options: Option<Vec<String>> = None;
                let mut value_parameters: Option<::std::collections::BTreeMap<String, String>> = None;
                let mut value_provisioner: Option<String> = None;
                let mut value_reclaim_policy: Option<String> = None;
                let mut value_volume_binding_mode: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allow_volume_expansion => value_allow_volume_expansion = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mount_options => value_mount_options = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameters => value_parameters = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provisioner => value_provisioner = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_reclaim_policy => value_reclaim_policy = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_binding_mode => value_volume_binding_mode = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageClass {
                    allow_volume_expansion: value_allow_volume_expansion,
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    mount_options: value_mount_options,
                    parameters: value_parameters,
                    provisioner: value_provisioner.ok_or_else(|| ::serde::de::Error::missing_field("provisioner"))?,
                    reclaim_policy: value_reclaim_policy,
                    volume_binding_mode: value_volume_binding_mode,
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageClass",
            &[
                "allowVolumeExpansion",
                "apiVersion",
                "kind",
                "metadata",
                "mountOptions",
                "parameters",
                "provisioner",
                "reclaimPolicy",
                "volumeBindingMode",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for StorageClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageClass",
            0 +
            self.allow_volume_expansion.as_ref().map_or(0, |_| 1) +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.mount_options.as_ref().map_or(0, |_| 1) +
            self.parameters.as_ref().map_or(0, |_| 1) +
            1 +
            self.reclaim_policy.as_ref().map_or(0, |_| 1) +
            self.volume_binding_mode.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allow_volume_expansion {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "allowVolumeExpansion", value)?;
        }
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.mount_options {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "mountOptions", value)?;
        }
        if let Some(value) = &self.parameters {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "parameters", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "provisioner", &self.provisioner)?;
        if let Some(value) = &self.reclaim_policy {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "reclaimPolicy", value)?;
        }
        if let Some(value) = &self.volume_binding_mode {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeBindingMode", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
