// Generated from definition io.k8s.api.core.v1.PersistentVolumeSpec

/// PersistentVolumeSpec is the specification of a persistent volume.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeSpec {
    /// AccessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes
    pub access_modes: Option<Vec<String>>,

    /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub aws_elastic_block_store: Option<::v1_8::api::core::v1::AWSElasticBlockStoreVolumeSource>,

    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    pub azure_disk: Option<::v1_8::api::core::v1::AzureDiskVolumeSource>,

    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    pub azure_file: Option<::v1_8::api::core::v1::AzureFilePersistentVolumeSource>,

    /// A description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
    pub capacity: Option<::std::collections::BTreeMap<String, ::v1_8::apimachinery::pkg::api::resource::Quantity>>,

    /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    pub cephfs: Option<::v1_8::api::core::v1::CephFSPersistentVolumeSource>,

    /// Cinder represents a cinder volume attached and mounted on kubelets host machine More info: https://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
    pub cinder: Option<::v1_8::api::core::v1::CinderVolumeSource>,

    /// ClaimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding
    pub claim_ref: Option<::v1_8::api::core::v1::ObjectReference>,

    /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    pub fc: Option<::v1_8::api::core::v1::FCVolumeSource>,

    /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin. This is an alpha feature and may change in future.
    pub flex_volume: Option<::v1_8::api::core::v1::FlexVolumeSource>,

    /// Flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running
    pub flocker: Option<::v1_8::api::core::v1::FlockerVolumeSource>,

    /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub gce_persistent_disk: Option<::v1_8::api::core::v1::GCEPersistentDiskVolumeSource>,

    /// Glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md
    pub glusterfs: Option<::v1_8::api::core::v1::GlusterfsVolumeSource>,

    /// HostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    pub host_path: Option<::v1_8::api::core::v1::HostPathVolumeSource>,

    /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.
    pub iscsi: Option<::v1_8::api::core::v1::ISCSIVolumeSource>,

    /// Local represents directly-attached storage with node affinity
    pub local: Option<::v1_8::api::core::v1::LocalVolumeSource>,

    /// A list of mount options, e.g. \["ro", "soft"\]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options
    pub mount_options: Option<Vec<String>>,

    /// NFS represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub nfs: Option<::v1_8::api::core::v1::NFSVolumeSource>,

    /// What happens to a persistent volume when released from its claim. Valid options are Retain (default) and Recycle. Recycling must be supported by the volume plugin underlying this persistent volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming
    pub persistent_volume_reclaim_policy: Option<String>,

    /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    pub photon_persistent_disk: Option<::v1_8::api::core::v1::PhotonPersistentDiskVolumeSource>,

    /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
    pub portworx_volume: Option<::v1_8::api::core::v1::PortworxVolumeSource>,

    /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    pub quobyte: Option<::v1_8::api::core::v1::QuobyteVolumeSource>,

    /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md
    pub rbd: Option<::v1_8::api::core::v1::RBDVolumeSource>,

    /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    pub scale_io: Option<::v1_8::api::core::v1::ScaleIOPersistentVolumeSource>,

    /// Name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.
    pub storage_class_name: Option<String>,

    /// StorageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod More info: https://releases.k8s.io/HEAD/examples/volumes/storageos/README.md
    pub storageos: Option<::v1_8::api::core::v1::StorageOSPersistentVolumeSource>,

    /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    pub vsphere_volume: Option<::v1_8::api::core::v1::VsphereVirtualDiskVolumeSource>,
}

impl<'de> ::serde::Deserialize<'de> for PersistentVolumeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_access_modes,
            Key_aws_elastic_block_store,
            Key_azure_disk,
            Key_azure_file,
            Key_capacity,
            Key_cephfs,
            Key_cinder,
            Key_claim_ref,
            Key_fc,
            Key_flex_volume,
            Key_flocker,
            Key_gce_persistent_disk,
            Key_glusterfs,
            Key_host_path,
            Key_iscsi,
            Key_local,
            Key_mount_options,
            Key_nfs,
            Key_persistent_volume_reclaim_policy,
            Key_photon_persistent_disk,
            Key_portworx_volume,
            Key_quobyte,
            Key_rbd,
            Key_scale_io,
            Key_storage_class_name,
            Key_storageos,
            Key_vsphere_volume,
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
                            "accessModes" => Field::Key_access_modes,
                            "awsElasticBlockStore" => Field::Key_aws_elastic_block_store,
                            "azureDisk" => Field::Key_azure_disk,
                            "azureFile" => Field::Key_azure_file,
                            "capacity" => Field::Key_capacity,
                            "cephfs" => Field::Key_cephfs,
                            "cinder" => Field::Key_cinder,
                            "claimRef" => Field::Key_claim_ref,
                            "fc" => Field::Key_fc,
                            "flexVolume" => Field::Key_flex_volume,
                            "flocker" => Field::Key_flocker,
                            "gcePersistentDisk" => Field::Key_gce_persistent_disk,
                            "glusterfs" => Field::Key_glusterfs,
                            "hostPath" => Field::Key_host_path,
                            "iscsi" => Field::Key_iscsi,
                            "local" => Field::Key_local,
                            "mountOptions" => Field::Key_mount_options,
                            "nfs" => Field::Key_nfs,
                            "persistentVolumeReclaimPolicy" => Field::Key_persistent_volume_reclaim_policy,
                            "photonPersistentDisk" => Field::Key_photon_persistent_disk,
                            "portworxVolume" => Field::Key_portworx_volume,
                            "quobyte" => Field::Key_quobyte,
                            "rbd" => Field::Key_rbd,
                            "scaleIO" => Field::Key_scale_io,
                            "storageClassName" => Field::Key_storage_class_name,
                            "storageos" => Field::Key_storageos,
                            "vsphereVolume" => Field::Key_vsphere_volume,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PersistentVolumeSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_access_modes: Option<Vec<String>> = None;
                let mut value_aws_elastic_block_store: Option<::v1_8::api::core::v1::AWSElasticBlockStoreVolumeSource> = None;
                let mut value_azure_disk: Option<::v1_8::api::core::v1::AzureDiskVolumeSource> = None;
                let mut value_azure_file: Option<::v1_8::api::core::v1::AzureFilePersistentVolumeSource> = None;
                let mut value_capacity: Option<::std::collections::BTreeMap<String, ::v1_8::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_cephfs: Option<::v1_8::api::core::v1::CephFSPersistentVolumeSource> = None;
                let mut value_cinder: Option<::v1_8::api::core::v1::CinderVolumeSource> = None;
                let mut value_claim_ref: Option<::v1_8::api::core::v1::ObjectReference> = None;
                let mut value_fc: Option<::v1_8::api::core::v1::FCVolumeSource> = None;
                let mut value_flex_volume: Option<::v1_8::api::core::v1::FlexVolumeSource> = None;
                let mut value_flocker: Option<::v1_8::api::core::v1::FlockerVolumeSource> = None;
                let mut value_gce_persistent_disk: Option<::v1_8::api::core::v1::GCEPersistentDiskVolumeSource> = None;
                let mut value_glusterfs: Option<::v1_8::api::core::v1::GlusterfsVolumeSource> = None;
                let mut value_host_path: Option<::v1_8::api::core::v1::HostPathVolumeSource> = None;
                let mut value_iscsi: Option<::v1_8::api::core::v1::ISCSIVolumeSource> = None;
                let mut value_local: Option<::v1_8::api::core::v1::LocalVolumeSource> = None;
                let mut value_mount_options: Option<Vec<String>> = None;
                let mut value_nfs: Option<::v1_8::api::core::v1::NFSVolumeSource> = None;
                let mut value_persistent_volume_reclaim_policy: Option<String> = None;
                let mut value_photon_persistent_disk: Option<::v1_8::api::core::v1::PhotonPersistentDiskVolumeSource> = None;
                let mut value_portworx_volume: Option<::v1_8::api::core::v1::PortworxVolumeSource> = None;
                let mut value_quobyte: Option<::v1_8::api::core::v1::QuobyteVolumeSource> = None;
                let mut value_rbd: Option<::v1_8::api::core::v1::RBDVolumeSource> = None;
                let mut value_scale_io: Option<::v1_8::api::core::v1::ScaleIOPersistentVolumeSource> = None;
                let mut value_storage_class_name: Option<String> = None;
                let mut value_storageos: Option<::v1_8::api::core::v1::StorageOSPersistentVolumeSource> = None;
                let mut value_vsphere_volume: Option<::v1_8::api::core::v1::VsphereVirtualDiskVolumeSource> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_modes => value_access_modes = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_aws_elastic_block_store => value_aws_elastic_block_store = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_azure_disk => value_azure_disk = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_azure_file => value_azure_file = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cephfs => value_cephfs = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cinder => value_cinder = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_claim_ref => value_claim_ref = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fc => value_fc = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_flex_volume => value_flex_volume = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_flocker => value_flocker = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gce_persistent_disk => value_gce_persistent_disk = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_glusterfs => value_glusterfs = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_path => value_host_path = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_iscsi => value_iscsi = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_local => value_local = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mount_options => value_mount_options = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nfs => value_nfs = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_persistent_volume_reclaim_policy => value_persistent_volume_reclaim_policy = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_photon_persistent_disk => value_photon_persistent_disk = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_portworx_volume => value_portworx_volume = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_quobyte => value_quobyte = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rbd => value_rbd = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scale_io => value_scale_io = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_class_name => value_storage_class_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storageos => value_storageos = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vsphere_volume => value_vsphere_volume = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeSpec {
                    access_modes: value_access_modes,
                    aws_elastic_block_store: value_aws_elastic_block_store,
                    azure_disk: value_azure_disk,
                    azure_file: value_azure_file,
                    capacity: value_capacity,
                    cephfs: value_cephfs,
                    cinder: value_cinder,
                    claim_ref: value_claim_ref,
                    fc: value_fc,
                    flex_volume: value_flex_volume,
                    flocker: value_flocker,
                    gce_persistent_disk: value_gce_persistent_disk,
                    glusterfs: value_glusterfs,
                    host_path: value_host_path,
                    iscsi: value_iscsi,
                    local: value_local,
                    mount_options: value_mount_options,
                    nfs: value_nfs,
                    persistent_volume_reclaim_policy: value_persistent_volume_reclaim_policy,
                    photon_persistent_disk: value_photon_persistent_disk,
                    portworx_volume: value_portworx_volume,
                    quobyte: value_quobyte,
                    rbd: value_rbd,
                    scale_io: value_scale_io,
                    storage_class_name: value_storage_class_name,
                    storageos: value_storageos,
                    vsphere_volume: value_vsphere_volume,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeSpec",
            &[
                "accessModes",
                "awsElasticBlockStore",
                "azureDisk",
                "azureFile",
                "capacity",
                "cephfs",
                "cinder",
                "claimRef",
                "fc",
                "flexVolume",
                "flocker",
                "gcePersistentDisk",
                "glusterfs",
                "hostPath",
                "iscsi",
                "local",
                "mountOptions",
                "nfs",
                "persistentVolumeReclaimPolicy",
                "photonPersistentDisk",
                "portworxVolume",
                "quobyte",
                "rbd",
                "scaleIO",
                "storageClassName",
                "storageos",
                "vsphereVolume",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PersistentVolumeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeSpec",
            0 +
            (if self.access_modes.is_some() { 1 } else { 0 }) +
            (if self.aws_elastic_block_store.is_some() { 1 } else { 0 }) +
            (if self.azure_disk.is_some() { 1 } else { 0 }) +
            (if self.azure_file.is_some() { 1 } else { 0 }) +
            (if self.capacity.is_some() { 1 } else { 0 }) +
            (if self.cephfs.is_some() { 1 } else { 0 }) +
            (if self.cinder.is_some() { 1 } else { 0 }) +
            (if self.claim_ref.is_some() { 1 } else { 0 }) +
            (if self.fc.is_some() { 1 } else { 0 }) +
            (if self.flex_volume.is_some() { 1 } else { 0 }) +
            (if self.flocker.is_some() { 1 } else { 0 }) +
            (if self.gce_persistent_disk.is_some() { 1 } else { 0 }) +
            (if self.glusterfs.is_some() { 1 } else { 0 }) +
            (if self.host_path.is_some() { 1 } else { 0 }) +
            (if self.iscsi.is_some() { 1 } else { 0 }) +
            (if self.local.is_some() { 1 } else { 0 }) +
            (if self.mount_options.is_some() { 1 } else { 0 }) +
            (if self.nfs.is_some() { 1 } else { 0 }) +
            (if self.persistent_volume_reclaim_policy.is_some() { 1 } else { 0 }) +
            (if self.photon_persistent_disk.is_some() { 1 } else { 0 }) +
            (if self.portworx_volume.is_some() { 1 } else { 0 }) +
            (if self.quobyte.is_some() { 1 } else { 0 }) +
            (if self.rbd.is_some() { 1 } else { 0 }) +
            (if self.scale_io.is_some() { 1 } else { 0 }) +
            (if self.storage_class_name.is_some() { 1 } else { 0 }) +
            (if self.storageos.is_some() { 1 } else { 0 }) +
            (if self.vsphere_volume.is_some() { 1 } else { 0 }),
        )?;
        if let Some(value) = &self.access_modes {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "accessModes", value)?;
        }
        if let Some(value) = &self.aws_elastic_block_store {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "awsElasticBlockStore", value)?;
        }
        if let Some(value) = &self.azure_disk {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "azureDisk", value)?;
        }
        if let Some(value) = &self.azure_file {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "azureFile", value)?;
        }
        if let Some(value) = &self.capacity {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", value)?;
        }
        if let Some(value) = &self.cephfs {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "cephfs", value)?;
        }
        if let Some(value) = &self.cinder {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "cinder", value)?;
        }
        if let Some(value) = &self.claim_ref {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "claimRef", value)?;
        }
        if let Some(value) = &self.fc {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "fc", value)?;
        }
        if let Some(value) = &self.flex_volume {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "flexVolume", value)?;
        }
        if let Some(value) = &self.flocker {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "flocker", value)?;
        }
        if let Some(value) = &self.gce_persistent_disk {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "gcePersistentDisk", value)?;
        }
        if let Some(value) = &self.glusterfs {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "glusterfs", value)?;
        }
        if let Some(value) = &self.host_path {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "hostPath", value)?;
        }
        if let Some(value) = &self.iscsi {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "iscsi", value)?;
        }
        if let Some(value) = &self.local {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "local", value)?;
        }
        if let Some(value) = &self.mount_options {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "mountOptions", value)?;
        }
        if let Some(value) = &self.nfs {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "nfs", value)?;
        }
        if let Some(value) = &self.persistent_volume_reclaim_policy {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "persistentVolumeReclaimPolicy", value)?;
        }
        if let Some(value) = &self.photon_persistent_disk {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "photonPersistentDisk", value)?;
        }
        if let Some(value) = &self.portworx_volume {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "portworxVolume", value)?;
        }
        if let Some(value) = &self.quobyte {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "quobyte", value)?;
        }
        if let Some(value) = &self.rbd {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "rbd", value)?;
        }
        if let Some(value) = &self.scale_io {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "scaleIO", value)?;
        }
        if let Some(value) = &self.storage_class_name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "storageClassName", value)?;
        }
        if let Some(value) = &self.storageos {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "storageos", value)?;
        }
        if let Some(value) = &self.vsphere_volume {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "vsphereVolume", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
