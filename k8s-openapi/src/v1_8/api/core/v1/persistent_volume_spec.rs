// Generated from definition io.k8s.api.core.v1.PersistentVolumeSpec

/// PersistentVolumeSpec is the specification of a persistent volume.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PersistentVolumeSpec {
    /// AccessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes
    #[serde(rename = "accessModes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_modes: Option<Vec<String>>,

    /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(rename = "awsElasticBlockStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<::v1_8::api::core::v1::AWSElasticBlockStoreVolumeSource>,

    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    #[serde(rename = "azureDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<::v1_8::api::core::v1::AzureDiskVolumeSource>,

    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    #[serde(rename = "azureFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<::v1_8::api::core::v1::AzureFilePersistentVolumeSource>,

    /// A description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<::std::collections::BTreeMap<String, ::v1_8::apimachinery::pkg::api::resource::Quantity>>,

    /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<::v1_8::api::core::v1::CephFSPersistentVolumeSource>,

    /// Cinder represents a cinder volume attached and mounted on kubelets host machine More info: https://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinder: Option<::v1_8::api::core::v1::CinderVolumeSource>,

    /// ClaimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding
    #[serde(rename = "claimRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_ref: Option<::v1_8::api::core::v1::ObjectReference>,

    /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc: Option<::v1_8::api::core::v1::FCVolumeSource>,

    /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin. This is an alpha feature and may change in future.
    #[serde(rename = "flexVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<::v1_8::api::core::v1::FlexVolumeSource>,

    /// Flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flocker: Option<::v1_8::api::core::v1::FlockerVolumeSource>,

    /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(rename = "gcePersistentDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<::v1_8::api::core::v1::GCEPersistentDiskVolumeSource>,

    /// Glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<::v1_8::api::core::v1::GlusterfsVolumeSource>,

    /// HostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    #[serde(rename = "hostPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<::v1_8::api::core::v1::HostPathVolumeSource>,

    /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<::v1_8::api::core::v1::ISCSIVolumeSource>,

    /// Local represents directly-attached storage with node affinity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local: Option<::v1_8::api::core::v1::LocalVolumeSource>,

    /// A list of mount options, e.g. ["ro", "soft"]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options
    #[serde(rename = "mountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,

    /// NFS represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs: Option<::v1_8::api::core::v1::NFSVolumeSource>,

    /// What happens to a persistent volume when released from its claim. Valid options are Retain (default) and Recycle. Recycling must be supported by the volume plugin underlying this persistent volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming
    #[serde(rename = "persistentVolumeReclaimPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_reclaim_policy: Option<String>,

    /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    #[serde(rename = "photonPersistentDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<::v1_8::api::core::v1::PhotonPersistentDiskVolumeSource>,

    /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
    #[serde(rename = "portworxVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<::v1_8::api::core::v1::PortworxVolumeSource>,

    /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<::v1_8::api::core::v1::QuobyteVolumeSource>,

    /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbd: Option<::v1_8::api::core::v1::RBDVolumeSource>,

    /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    #[serde(rename = "scaleIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<::v1_8::api::core::v1::ScaleIOPersistentVolumeSource>,

    /// Name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.
    #[serde(rename = "storageClassName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,

    /// StorageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod More info: https://releases.k8s.io/HEAD/examples/volumes/storageos/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageos: Option<::v1_8::api::core::v1::StorageOSPersistentVolumeSource>,

    /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    #[serde(rename = "vsphereVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<::v1_8::api::core::v1::VsphereVirtualDiskVolumeSource>,
}
