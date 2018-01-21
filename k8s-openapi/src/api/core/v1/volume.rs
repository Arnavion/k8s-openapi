// Generated from definition io.k8s.api.core.v1.Volume

/// Volume represents a named volume in a pod that may be accessed by any container in the pod.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Volume {
    /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(rename = "awsElasticBlockStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<::api::core::v1::AWSElasticBlockStoreVolumeSource>,

    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    #[serde(rename = "azureDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<::api::core::v1::AzureDiskVolumeSource>,

    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    #[serde(rename = "azureFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<::api::core::v1::AzureFileVolumeSource>,

    /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<::api::core::v1::CephFSVolumeSource>,

    /// Cinder represents a cinder volume attached and mounted on kubelets host machine More info: https://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinder: Option<::api::core::v1::CinderVolumeSource>,

    /// ConfigMap represents a configMap that should populate this volume
    #[serde(rename = "configMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map: Option<::api::core::v1::ConfigMapVolumeSource>,

    /// DownwardAPI represents downward API about the pod that should populate this volume
    #[serde(rename = "downwardAPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<::api::core::v1::DownwardAPIVolumeSource>,

    /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(rename = "emptyDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<::api::core::v1::EmptyDirVolumeSource>,

    /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc: Option<::api::core::v1::FCVolumeSource>,

    /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
    #[serde(rename = "flexVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<::api::core::v1::FlexVolumeSource>,

    /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flocker: Option<::api::core::v1::FlockerVolumeSource>,

    /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(rename = "gcePersistentDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<::api::core::v1::GCEPersistentDiskVolumeSource>,

    /// GitRepo represents a git repository at a particular revision.
    #[serde(rename = "gitRepo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<::api::core::v1::GitRepoVolumeSource>,

    /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<::api::core::v1::GlusterfsVolumeSource>,

    /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    #[serde(rename = "hostPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<::api::core::v1::HostPathVolumeSource>,

    /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://releases.k8s.io/HEAD/examples/volumes/iscsi/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<::api::core::v1::ISCSIVolumeSource>,

    /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,

    /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs: Option<::api::core::v1::NFSVolumeSource>,

    /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    #[serde(rename = "persistentVolumeClaim")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim: Option<::api::core::v1::PersistentVolumeClaimVolumeSource>,

    /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    #[serde(rename = "photonPersistentDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<::api::core::v1::PhotonPersistentDiskVolumeSource>,

    /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
    #[serde(rename = "portworxVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<::api::core::v1::PortworxVolumeSource>,

    /// Items for all in one resources secrets, configmaps, and downward API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected: Option<::api::core::v1::ProjectedVolumeSource>,

    /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<::api::core::v1::QuobyteVolumeSource>,

    /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbd: Option<::api::core::v1::RBDVolumeSource>,

    /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    #[serde(rename = "scaleIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<::api::core::v1::ScaleIOVolumeSource>,

    /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<::api::core::v1::SecretVolumeSource>,

    /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageos: Option<::api::core::v1::StorageOSVolumeSource>,

    /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    #[serde(rename = "vsphereVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<::api::core::v1::VsphereVirtualDiskVolumeSource>,
}
