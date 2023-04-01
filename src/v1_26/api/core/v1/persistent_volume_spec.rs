// Generated from definition io.k8s.api.core.v1.PersistentVolumeSpec

/// PersistentVolumeSpec is the specification of a persistent volume.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeSpec {
    /// accessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes
    pub access_modes: Option<Vec<String>>,

    /// awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub aws_elastic_block_store: Option<crate::api::core::v1::AWSElasticBlockStoreVolumeSource>,

    /// azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    pub azure_disk: Option<crate::api::core::v1::AzureDiskVolumeSource>,

    /// azureFile represents an Azure File Service mount on the host and bind mount to the pod.
    pub azure_file: Option<crate::api::core::v1::AzureFilePersistentVolumeSource>,

    /// capacity is the description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
    pub capacity: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

    /// cephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    pub cephfs: Option<crate::api::core::v1::CephFSPersistentVolumeSource>,

    /// cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    pub cinder: Option<crate::api::core::v1::CinderPersistentVolumeSource>,

    /// claimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding
    pub claim_ref: Option<crate::api::core::v1::ObjectReference>,

    /// csi represents storage that is handled by an external CSI driver (Beta feature).
    pub csi: Option<crate::api::core::v1::CSIPersistentVolumeSource>,

    /// fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    pub fc: Option<crate::api::core::v1::FCVolumeSource>,

    /// flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
    pub flex_volume: Option<crate::api::core::v1::FlexPersistentVolumeSource>,

    /// flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running
    pub flocker: Option<crate::api::core::v1::FlockerVolumeSource>,

    /// gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub gce_persistent_disk: Option<crate::api::core::v1::GCEPersistentDiskVolumeSource>,

    /// glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. More info: https://examples.k8s.io/volumes/glusterfs/README.md
    pub glusterfs: Option<crate::api::core::v1::GlusterfsPersistentVolumeSource>,

    /// hostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    pub host_path: Option<crate::api::core::v1::HostPathVolumeSource>,

    /// iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.
    pub iscsi: Option<crate::api::core::v1::ISCSIPersistentVolumeSource>,

    /// local represents directly-attached storage with node affinity
    pub local: Option<crate::api::core::v1::LocalVolumeSource>,

    /// mountOptions is the list of mount options, e.g. \["ro", "soft"\]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options
    pub mount_options: Option<Vec<String>>,

    /// nfs represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub nfs: Option<crate::api::core::v1::NFSVolumeSource>,

    /// nodeAffinity defines constraints that limit what nodes this volume can be accessed from. This field influences the scheduling of pods that use this volume.
    pub node_affinity: Option<crate::api::core::v1::VolumeNodeAffinity>,

    /// persistentVolumeReclaimPolicy defines what happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming
    ///
    pub persistent_volume_reclaim_policy: Option<String>,

    /// photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    pub photon_persistent_disk: Option<crate::api::core::v1::PhotonPersistentDiskVolumeSource>,

    /// portworxVolume represents a portworx volume attached and mounted on kubelets host machine
    pub portworx_volume: Option<crate::api::core::v1::PortworxVolumeSource>,

    /// quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    pub quobyte: Option<crate::api::core::v1::QuobyteVolumeSource>,

    /// rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
    pub rbd: Option<crate::api::core::v1::RBDPersistentVolumeSource>,

    /// scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    pub scale_io: Option<crate::api::core::v1::ScaleIOPersistentVolumeSource>,

    /// storageClassName is the name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.
    pub storage_class_name: Option<String>,

    /// storageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod More info: https://examples.k8s.io/volumes/storageos/README.md
    pub storageos: Option<crate::api::core::v1::StorageOSPersistentVolumeSource>,

    /// volumeMode defines if a volume is intended to be used with a formatted filesystem or to remain in raw block state. Value of Filesystem is implied when not included in spec.
    pub volume_mode: Option<String>,

    /// vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    pub vsphere_volume: Option<crate::api::core::v1::VsphereVirtualDiskVolumeSource>,
}

impl crate::DeepMerge for PersistentVolumeSpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.access_modes, other.access_modes);
        crate::DeepMerge::merge_from(&mut self.aws_elastic_block_store, other.aws_elastic_block_store);
        crate::DeepMerge::merge_from(&mut self.azure_disk, other.azure_disk);
        crate::DeepMerge::merge_from(&mut self.azure_file, other.azure_file);
        crate::merge_strategies::map::granular(&mut self.capacity, other.capacity, |current_item, other_item| {
            crate::DeepMerge::merge_from(current_item, other_item);
        });
        crate::DeepMerge::merge_from(&mut self.cephfs, other.cephfs);
        crate::DeepMerge::merge_from(&mut self.cinder, other.cinder);
        crate::DeepMerge::merge_from(&mut self.claim_ref, other.claim_ref);
        crate::DeepMerge::merge_from(&mut self.csi, other.csi);
        crate::DeepMerge::merge_from(&mut self.fc, other.fc);
        crate::DeepMerge::merge_from(&mut self.flex_volume, other.flex_volume);
        crate::DeepMerge::merge_from(&mut self.flocker, other.flocker);
        crate::DeepMerge::merge_from(&mut self.gce_persistent_disk, other.gce_persistent_disk);
        crate::DeepMerge::merge_from(&mut self.glusterfs, other.glusterfs);
        crate::DeepMerge::merge_from(&mut self.host_path, other.host_path);
        crate::DeepMerge::merge_from(&mut self.iscsi, other.iscsi);
        crate::DeepMerge::merge_from(&mut self.local, other.local);
        crate::merge_strategies::list::atomic(&mut self.mount_options, other.mount_options);
        crate::DeepMerge::merge_from(&mut self.nfs, other.nfs);
        crate::DeepMerge::merge_from(&mut self.node_affinity, other.node_affinity);
        crate::DeepMerge::merge_from(&mut self.persistent_volume_reclaim_policy, other.persistent_volume_reclaim_policy);
        crate::DeepMerge::merge_from(&mut self.photon_persistent_disk, other.photon_persistent_disk);
        crate::DeepMerge::merge_from(&mut self.portworx_volume, other.portworx_volume);
        crate::DeepMerge::merge_from(&mut self.quobyte, other.quobyte);
        crate::DeepMerge::merge_from(&mut self.rbd, other.rbd);
        crate::DeepMerge::merge_from(&mut self.scale_io, other.scale_io);
        crate::DeepMerge::merge_from(&mut self.storage_class_name, other.storage_class_name);
        crate::DeepMerge::merge_from(&mut self.storageos, other.storageos);
        crate::DeepMerge::merge_from(&mut self.volume_mode, other.volume_mode);
        crate::DeepMerge::merge_from(&mut self.vsphere_volume, other.vsphere_volume);
    }
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
                    access_modes: value_access_modes,
                    aws_elastic_block_store: value_aws_elastic_block_store,
                    azure_disk: value_azure_disk,
                    azure_file: value_azure_file,
                    capacity: value_capacity,
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
                    mount_options: value_mount_options,
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
            self.access_modes.as_ref().map_or(0, |_| 1) +
            self.aws_elastic_block_store.as_ref().map_or(0, |_| 1) +
            self.azure_disk.as_ref().map_or(0, |_| 1) +
            self.azure_file.as_ref().map_or(0, |_| 1) +
            self.capacity.as_ref().map_or(0, |_| 1) +
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
            self.mount_options.as_ref().map_or(0, |_| 1) +
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
        if let Some(value) = &self.access_modes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "accessModes", value)?;
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
        if let Some(value) = &self.capacity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "capacity", value)?;
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
        if let Some(value) = &self.mount_options {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "mountOptions", value)?;
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PersistentVolumeSpec {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.PersistentVolumeSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PersistentVolumeSpec is the specification of a persistent volume.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "accessModes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("accessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes".to_owned()),
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
                        "awsElasticBlockStore".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AWSElasticBlockStoreVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "azureDisk".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AzureDiskVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "azureFile".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AzureFilePersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("azureFile represents an Azure File Service mount on the host and bind mount to the pod.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "capacity".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("capacity is the description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity".to_owned()),
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
                        "cephfs".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::CephFSPersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("cephFS represents a Ceph FS mount on the host that shares a pod's lifetime".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "cinder".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::CinderPersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "claimRef".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ObjectReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("claimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "csi".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::CSIPersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("csi represents storage that is handled by an external CSI driver (Beta feature).".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "fc".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::FCVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "flexVolume".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::FlexPersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "flocker".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::FlockerVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "gcePersistentDisk".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::GCEPersistentDiskVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "glusterfs".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::GlusterfsPersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. More info: https://examples.k8s.io/volumes/glusterfs/README.md".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "hostPath".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::HostPathVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("hostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "iscsi".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ISCSIPersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "local".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::LocalVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("local represents directly-attached storage with node affinity".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "mountOptions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("mountOptions is the list of mount options, e.g. [\"ro\", \"soft\"]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options".to_owned()),
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
                        "nfs".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NFSVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("nfs represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "nodeAffinity".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::VolumeNodeAffinity>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("nodeAffinity defines constraints that limit what nodes this volume can be accessed from. This field influences the scheduling of pods that use this volume.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "persistentVolumeReclaimPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("persistentVolumeReclaimPolicy defines what happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming\n\n".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "photonPersistentDisk".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PhotonPersistentDiskVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "portworxVolume".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PortworxVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("portworxVolume represents a portworx volume attached and mounted on kubelets host machine".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "quobyte".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::QuobyteVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("quobyte represents a Quobyte mount on the host that shares a pod's lifetime".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "rbd".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::RBDPersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "scaleIO".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ScaleIOPersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "storageClassName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("storageClassName is the name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "storageos".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::StorageOSPersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("storageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod More info: https://examples.k8s.io/volumes/storageos/README.md".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "volumeMode".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("volumeMode defines if a volume is intended to be used with a formatted filesystem or to remain in raw block state. Value of Filesystem is implied when not included in spec.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "vsphereVolume".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::VsphereVirtualDiskVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine".to_owned()),
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
