// Generated from definition io.k8s.api.core.v1.PersistentVolumeSpec

/// PersistentVolumeSpec is the specification of a persistent volume.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeSpec {
    /// AccessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes
    pub access_modes: Vec<String>,

    /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub aws_elastic_block_store: Option<crate::api::core::v1::AWSElasticBlockStoreVolumeSource>,

    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    pub azure_disk: Option<crate::api::core::v1::AzureDiskVolumeSource>,

    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    pub azure_file: Option<crate::api::core::v1::AzureFilePersistentVolumeSource>,

    /// A description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
    pub capacity: std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>,

    /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    pub cephfs: Option<crate::api::core::v1::CephFSPersistentVolumeSource>,

    /// Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    pub cinder: Option<crate::api::core::v1::CinderPersistentVolumeSource>,

    /// ClaimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding
    pub claim_ref: Option<crate::api::core::v1::ObjectReference>,

    /// CSI represents storage that is handled by an external CSI driver (Beta feature).
    pub csi: Option<crate::api::core::v1::CSIPersistentVolumeSource>,

    /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    pub fc: Option<crate::api::core::v1::FCVolumeSource>,

    /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
    pub flex_volume: Option<crate::api::core::v1::FlexPersistentVolumeSource>,

    /// Flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running
    pub flocker: Option<crate::api::core::v1::FlockerVolumeSource>,

    /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub gce_persistent_disk: Option<crate::api::core::v1::GCEPersistentDiskVolumeSource>,

    /// Glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. More info: https://examples.k8s.io/volumes/glusterfs/README.md
    pub glusterfs: Option<crate::api::core::v1::GlusterfsPersistentVolumeSource>,

    /// HostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    pub host_path: Option<crate::api::core::v1::HostPathVolumeSource>,

    /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.
    pub iscsi: Option<crate::api::core::v1::ISCSIPersistentVolumeSource>,

    /// Local represents directly-attached storage with node affinity
    pub local: Option<crate::api::core::v1::LocalVolumeSource>,

    /// A list of mount options, e.g. \["ro", "soft"\]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options
    pub mount_options: Vec<String>,

    /// NFS represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub nfs: Option<crate::api::core::v1::NFSVolumeSource>,

    /// NodeAffinity defines constraints that limit what nodes this volume can be accessed from. This field influences the scheduling of pods that use this volume.
    pub node_affinity: Option<crate::api::core::v1::VolumeNodeAffinity>,

    /// What happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming
    pub persistent_volume_reclaim_policy: Option<String>,

    /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    pub photon_persistent_disk: Option<crate::api::core::v1::PhotonPersistentDiskVolumeSource>,

    /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
    pub portworx_volume: Option<crate::api::core::v1::PortworxVolumeSource>,

    /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    pub quobyte: Option<crate::api::core::v1::QuobyteVolumeSource>,

    /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
    pub rbd: Option<crate::api::core::v1::RBDPersistentVolumeSource>,

    /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    pub scale_io: Option<crate::api::core::v1::ScaleIOPersistentVolumeSource>,

    /// Name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.
    pub storage_class_name: Option<String>,

    /// StorageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod More info: https://examples.k8s.io/volumes/storageos/README.md
    pub storageos: Option<crate::api::core::v1::StorageOSPersistentVolumeSource>,

    /// volumeMode defines if a volume is intended to be used with a formatted filesystem or to remain in raw block state. Value of Filesystem is implied when not included in spec.
    pub volume_mode: Option<String>,

    /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    pub vsphere_volume: Option<crate::api::core::v1::VsphereVirtualDiskVolumeSource>,
}

impl<'de> crate::serde::Deserialize<'de> for PersistentVolumeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
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
            Key_csi,
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
            Key_node_affinity,
            Key_persistent_volume_reclaim_policy,
            Key_photon_persistent_disk,
            Key_portworx_volume,
            Key_quobyte,
            Key_rbd,
            Key_scale_io,
            Key_storage_class_name,
            Key_storageos,
            Key_volume_mode,
            Key_vsphere_volume,
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
                            "awsElasticBlockStore" => Field::Key_aws_elastic_block_store,
                            "azureDisk" => Field::Key_azure_disk,
                            "azureFile" => Field::Key_azure_file,
                            "capacity" => Field::Key_capacity,
                            "cephfs" => Field::Key_cephfs,
                            "cinder" => Field::Key_cinder,
                            "claimRef" => Field::Key_claim_ref,
                            "csi" => Field::Key_csi,
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
                            "nodeAffinity" => Field::Key_node_affinity,
                            "persistentVolumeReclaimPolicy" => Field::Key_persistent_volume_reclaim_policy,
                            "photonPersistentDisk" => Field::Key_photon_persistent_disk,
                            "portworxVolume" => Field::Key_portworx_volume,
                            "quobyte" => Field::Key_quobyte,
                            "rbd" => Field::Key_rbd,
                            "scaleIO" => Field::Key_scale_io,
                            "storageClassName" => Field::Key_storage_class_name,
                            "storageos" => Field::Key_storageos,
                            "volumeMode" => Field::Key_volume_mode,
                            "vsphereVolume" => Field::Key_vsphere_volume,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PersistentVolumeSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_access_modes: Option<Vec<String>> = None;
                let mut value_aws_elastic_block_store: Option<crate::api::core::v1::AWSElasticBlockStoreVolumeSource> = None;
                let mut value_azure_disk: Option<crate::api::core::v1::AzureDiskVolumeSource> = None;
                let mut value_azure_file: Option<crate::api::core::v1::AzureFilePersistentVolumeSource> = None;
                let mut value_capacity: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>> = None;
                let mut value_cephfs: Option<crate::api::core::v1::CephFSPersistentVolumeSource> = None;
                let mut value_cinder: Option<crate::api::core::v1::CinderPersistentVolumeSource> = None;
                let mut value_claim_ref: Option<crate::api::core::v1::ObjectReference> = None;
                let mut value_csi: Option<crate::api::core::v1::CSIPersistentVolumeSource> = None;
                let mut value_fc: Option<crate::api::core::v1::FCVolumeSource> = None;
                let mut value_flex_volume: Option<crate::api::core::v1::FlexPersistentVolumeSource> = None;
                let mut value_flocker: Option<crate::api::core::v1::FlockerVolumeSource> = None;
                let mut value_gce_persistent_disk: Option<crate::api::core::v1::GCEPersistentDiskVolumeSource> = None;
                let mut value_glusterfs: Option<crate::api::core::v1::GlusterfsPersistentVolumeSource> = None;
                let mut value_host_path: Option<crate::api::core::v1::HostPathVolumeSource> = None;
                let mut value_iscsi: Option<crate::api::core::v1::ISCSIPersistentVolumeSource> = None;
                let mut value_local: Option<crate::api::core::v1::LocalVolumeSource> = None;
                let mut value_mount_options: Option<Vec<String>> = None;
                let mut value_nfs: Option<crate::api::core::v1::NFSVolumeSource> = None;
                let mut value_node_affinity: Option<crate::api::core::v1::VolumeNodeAffinity> = None;
                let mut value_persistent_volume_reclaim_policy: Option<String> = None;
                let mut value_photon_persistent_disk: Option<crate::api::core::v1::PhotonPersistentDiskVolumeSource> = None;
                let mut value_portworx_volume: Option<crate::api::core::v1::PortworxVolumeSource> = None;
                let mut value_quobyte: Option<crate::api::core::v1::QuobyteVolumeSource> = None;
                let mut value_rbd: Option<crate::api::core::v1::RBDPersistentVolumeSource> = None;
                let mut value_scale_io: Option<crate::api::core::v1::ScaleIOPersistentVolumeSource> = None;
                let mut value_storage_class_name: Option<String> = None;
                let mut value_storageos: Option<crate::api::core::v1::StorageOSPersistentVolumeSource> = None;
                let mut value_volume_mode: Option<String> = None;
                let mut value_vsphere_volume: Option<crate::api::core::v1::VsphereVirtualDiskVolumeSource> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_access_modes => value_access_modes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_aws_elastic_block_store => value_aws_elastic_block_store = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_azure_disk => value_azure_disk = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_azure_file => value_azure_file = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_capacity => value_capacity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cephfs => value_cephfs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cinder => value_cinder = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_claim_ref => value_claim_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_csi => value_csi = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fc => value_fc = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_flex_volume => value_flex_volume = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_flocker => value_flocker = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gce_persistent_disk => value_gce_persistent_disk = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_glusterfs => value_glusterfs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_path => value_host_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_iscsi => value_iscsi = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_local => value_local = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mount_options => value_mount_options = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nfs => value_nfs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_node_affinity => value_node_affinity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_persistent_volume_reclaim_policy => value_persistent_volume_reclaim_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_photon_persistent_disk => value_photon_persistent_disk = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_portworx_volume => value_portworx_volume = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_quobyte => value_quobyte = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rbd => value_rbd = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scale_io => value_scale_io = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storage_class_name => value_storage_class_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storageos => value_storageos = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mode => value_volume_mode = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vsphere_volume => value_vsphere_volume = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeSpec {
                    access_modes: value_access_modes.unwrap_or_default(),
                    aws_elastic_block_store: value_aws_elastic_block_store,
                    azure_disk: value_azure_disk,
                    azure_file: value_azure_file,
                    capacity: value_capacity.unwrap_or_default(),
                    cephfs: value_cephfs,
                    cinder: value_cinder,
                    claim_ref: value_claim_ref,
                    csi: value_csi,
                    fc: value_fc,
                    flex_volume: value_flex_volume,
                    flocker: value_flocker,
                    gce_persistent_disk: value_gce_persistent_disk,
                    glusterfs: value_glusterfs,
                    host_path: value_host_path,
                    iscsi: value_iscsi,
                    local: value_local,
                    mount_options: value_mount_options.unwrap_or_default(),
                    nfs: value_nfs,
                    node_affinity: value_node_affinity,
                    persistent_volume_reclaim_policy: value_persistent_volume_reclaim_policy,
                    photon_persistent_disk: value_photon_persistent_disk,
                    portworx_volume: value_portworx_volume,
                    quobyte: value_quobyte,
                    rbd: value_rbd,
                    scale_io: value_scale_io,
                    storage_class_name: value_storage_class_name,
                    storageos: value_storageos,
                    volume_mode: value_volume_mode,
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
                "csi",
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
                "nodeAffinity",
                "persistentVolumeReclaimPolicy",
                "photonPersistentDisk",
                "portworxVolume",
                "quobyte",
                "rbd",
                "scaleIO",
                "storageClassName",
                "storageos",
                "volumeMode",
                "vsphereVolume",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PersistentVolumeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeSpec",
            usize::from(!self.access_modes.is_empty()) +
            self.aws_elastic_block_store.as_ref().map_or(0, |_| 1) +
            self.azure_disk.as_ref().map_or(0, |_| 1) +
            self.azure_file.as_ref().map_or(0, |_| 1) +
            usize::from(!self.capacity.is_empty()) +
            self.cephfs.as_ref().map_or(0, |_| 1) +
            self.cinder.as_ref().map_or(0, |_| 1) +
            self.claim_ref.as_ref().map_or(0, |_| 1) +
            self.csi.as_ref().map_or(0, |_| 1) +
            self.fc.as_ref().map_or(0, |_| 1) +
            self.flex_volume.as_ref().map_or(0, |_| 1) +
            self.flocker.as_ref().map_or(0, |_| 1) +
            self.gce_persistent_disk.as_ref().map_or(0, |_| 1) +
            self.glusterfs.as_ref().map_or(0, |_| 1) +
            self.host_path.as_ref().map_or(0, |_| 1) +
            self.iscsi.as_ref().map_or(0, |_| 1) +
            self.local.as_ref().map_or(0, |_| 1) +
            usize::from(!self.mount_options.is_empty()) +
            self.nfs.as_ref().map_or(0, |_| 1) +
            self.node_affinity.as_ref().map_or(0, |_| 1) +
            self.persistent_volume_reclaim_policy.as_ref().map_or(0, |_| 1) +
            self.photon_persistent_disk.as_ref().map_or(0, |_| 1) +
            self.portworx_volume.as_ref().map_or(0, |_| 1) +
            self.quobyte.as_ref().map_or(0, |_| 1) +
            self.rbd.as_ref().map_or(0, |_| 1) +
            self.scale_io.as_ref().map_or(0, |_| 1) +
            self.storage_class_name.as_ref().map_or(0, |_| 1) +
            self.storageos.as_ref().map_or(0, |_| 1) +
            self.volume_mode.as_ref().map_or(0, |_| 1) +
            self.vsphere_volume.as_ref().map_or(0, |_| 1),
        )?;
        if !self.access_modes.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "accessModes", &self.access_modes)?;
        }
        if let Some(value) = &self.aws_elastic_block_store {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "awsElasticBlockStore", value)?;
        }
        if let Some(value) = &self.azure_disk {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "azureDisk", value)?;
        }
        if let Some(value) = &self.azure_file {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "azureFile", value)?;
        }
        if !self.capacity.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", &self.capacity)?;
        }
        if let Some(value) = &self.cephfs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cephfs", value)?;
        }
        if let Some(value) = &self.cinder {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cinder", value)?;
        }
        if let Some(value) = &self.claim_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "claimRef", value)?;
        }
        if let Some(value) = &self.csi {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "csi", value)?;
        }
        if let Some(value) = &self.fc {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fc", value)?;
        }
        if let Some(value) = &self.flex_volume {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "flexVolume", value)?;
        }
        if let Some(value) = &self.flocker {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "flocker", value)?;
        }
        if let Some(value) = &self.gce_persistent_disk {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gcePersistentDisk", value)?;
        }
        if let Some(value) = &self.glusterfs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "glusterfs", value)?;
        }
        if let Some(value) = &self.host_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "hostPath", value)?;
        }
        if let Some(value) = &self.iscsi {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "iscsi", value)?;
        }
        if let Some(value) = &self.local {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "local", value)?;
        }
        if !self.mount_options.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "mountOptions", &self.mount_options)?;
        }
        if let Some(value) = &self.nfs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nfs", value)?;
        }
        if let Some(value) = &self.node_affinity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeAffinity", value)?;
        }
        if let Some(value) = &self.persistent_volume_reclaim_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "persistentVolumeReclaimPolicy", value)?;
        }
        if let Some(value) = &self.photon_persistent_disk {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "photonPersistentDisk", value)?;
        }
        if let Some(value) = &self.portworx_volume {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "portworxVolume", value)?;
        }
        if let Some(value) = &self.quobyte {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "quobyte", value)?;
        }
        if let Some(value) = &self.rbd {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rbd", value)?;
        }
        if let Some(value) = &self.scale_io {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "scaleIO", value)?;
        }
        if let Some(value) = &self.storage_class_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageClassName", value)?;
        }
        if let Some(value) = &self.storageos {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageos", value)?;
        }
        if let Some(value) = &self.volume_mode {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeMode", value)?;
        }
        if let Some(value) = &self.vsphere_volume {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "vsphereVolume", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl PersistentVolumeSpec {
    pub fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "PersistentVolumeSpec is the specification of a persistent volume.",
          "properties": {
            "accessModes": {
              "description": "AccessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "awsElasticBlockStore": crate::schema_ref_with_description(crate::api::core::v1::AWSElasticBlockStoreVolumeSource::schema(), "AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore"),
            "azureDisk": crate::schema_ref_with_description(crate::api::core::v1::AzureDiskVolumeSource::schema(), "AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod."),
            "azureFile": crate::schema_ref_with_description(crate::api::core::v1::AzureFilePersistentVolumeSource::schema(), "AzureFile represents an Azure File Service mount on the host and bind mount to the pod."),
            "capacity": {
              "additionalProperties": crate::apimachinery::pkg::api::resource::Quantity::schema(),
              "description": "A description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity",
              "type": "object"
            },
            "cephfs": crate::schema_ref_with_description(crate::api::core::v1::CephFSPersistentVolumeSource::schema(), "CephFS represents a Ceph FS mount on the host that shares a pod's lifetime"),
            "cinder": crate::schema_ref_with_description(crate::api::core::v1::CinderPersistentVolumeSource::schema(), "Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md"),
            "claimRef": crate::schema_ref_with_description(crate::api::core::v1::ObjectReference::schema(), "ClaimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding"),
            "csi": crate::schema_ref_with_description(crate::api::core::v1::CSIPersistentVolumeSource::schema(), "CSI represents storage that is handled by an external CSI driver (Beta feature)."),
            "fc": crate::schema_ref_with_description(crate::api::core::v1::FCVolumeSource::schema(), "FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod."),
            "flexVolume": crate::schema_ref_with_description(crate::api::core::v1::FlexPersistentVolumeSource::schema(), "FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin."),
            "flocker": crate::schema_ref_with_description(crate::api::core::v1::FlockerVolumeSource::schema(), "Flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running"),
            "gcePersistentDisk": crate::schema_ref_with_description(crate::api::core::v1::GCEPersistentDiskVolumeSource::schema(), "GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk"),
            "glusterfs": crate::schema_ref_with_description(crate::api::core::v1::GlusterfsPersistentVolumeSource::schema(), "Glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. More info: https://examples.k8s.io/volumes/glusterfs/README.md"),
            "hostPath": crate::schema_ref_with_description(crate::api::core::v1::HostPathVolumeSource::schema(), "HostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath"),
            "iscsi": crate::schema_ref_with_description(crate::api::core::v1::ISCSIPersistentVolumeSource::schema(), "ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin."),
            "local": crate::schema_ref_with_description(crate::api::core::v1::LocalVolumeSource::schema(), "Local represents directly-attached storage with node affinity"),
            "mountOptions": {
              "description": "A list of mount options, e.g. [\"ro\", \"soft\"]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "nfs": crate::schema_ref_with_description(crate::api::core::v1::NFSVolumeSource::schema(), "NFS represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs"),
            "nodeAffinity": crate::schema_ref_with_description(crate::api::core::v1::VolumeNodeAffinity::schema(), "NodeAffinity defines constraints that limit what nodes this volume can be accessed from. This field influences the scheduling of pods that use this volume."),
            "persistentVolumeReclaimPolicy": {
              "description": "What happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming",
              "type": "string"
            },
            "photonPersistentDisk": crate::schema_ref_with_description(crate::api::core::v1::PhotonPersistentDiskVolumeSource::schema(), "PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine"),
            "portworxVolume": crate::schema_ref_with_description(crate::api::core::v1::PortworxVolumeSource::schema(), "PortworxVolume represents a portworx volume attached and mounted on kubelets host machine"),
            "quobyte": crate::schema_ref_with_description(crate::api::core::v1::QuobyteVolumeSource::schema(), "Quobyte represents a Quobyte mount on the host that shares a pod's lifetime"),
            "rbd": crate::schema_ref_with_description(crate::api::core::v1::RBDPersistentVolumeSource::schema(), "RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md"),
            "scaleIO": crate::schema_ref_with_description(crate::api::core::v1::ScaleIOPersistentVolumeSource::schema(), "ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes."),
            "storageClassName": {
              "description": "Name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.",
              "type": "string"
            },
            "storageos": crate::schema_ref_with_description(crate::api::core::v1::StorageOSPersistentVolumeSource::schema(), "StorageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod More info: https://examples.k8s.io/volumes/storageos/README.md"),
            "volumeMode": {
              "description": "volumeMode defines if a volume is intended to be used with a formatted filesystem or to remain in raw block state. Value of Filesystem is implied when not included in spec.",
              "type": "string"
            },
            "vsphereVolume": crate::schema_ref_with_description(crate::api::core::v1::VsphereVirtualDiskVolumeSource::schema(), "VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine")
          },
          "type": "object"
        })
    }
}
