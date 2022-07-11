// Generated from definition io.k8s.api.core.v1.Volume

/// Volume represents a named volume in a pod that may be accessed by any container in the pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Volume {
    /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub aws_elastic_block_store: Option<crate::api::core::v1::AWSElasticBlockStoreVolumeSource>,

    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    pub azure_disk: Option<crate::api::core::v1::AzureDiskVolumeSource>,

    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    pub azure_file: Option<crate::api::core::v1::AzureFileVolumeSource>,

    /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    pub cephfs: Option<crate::api::core::v1::CephFSVolumeSource>,

    /// Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    pub cinder: Option<crate::api::core::v1::CinderVolumeSource>,

    /// ConfigMap represents a configMap that should populate this volume
    pub config_map: Option<crate::api::core::v1::ConfigMapVolumeSource>,

    /// CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
    pub csi: Option<crate::api::core::v1::CSIVolumeSource>,

    /// DownwardAPI represents downward API about the pod that should populate this volume
    pub downward_api: Option<crate::api::core::v1::DownwardAPIVolumeSource>,

    /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    pub empty_dir: Option<crate::api::core::v1::EmptyDirVolumeSource>,

    /// Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed.
    ///
    /// Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity
    ///    tracking are needed,
    /// c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through
    ///    a PersistentVolumeClaim (see EphemeralVolumeSource for more
    ///    information on the connection between this volume type
    ///    and PersistentVolumeClaim).
    ///
    /// Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod.
    ///
    /// Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information.
    ///
    /// A pod can use both types of ephemeral volumes and persistent volumes at the same time.
    ///
    /// This is a beta feature and only available when the GenericEphemeralVolume feature gate is enabled.
    pub ephemeral: Option<crate::api::core::v1::EphemeralVolumeSource>,

    /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    pub fc: Option<crate::api::core::v1::FCVolumeSource>,

    /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
    pub flex_volume: Option<crate::api::core::v1::FlexVolumeSource>,

    /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
    pub flocker: Option<crate::api::core::v1::FlockerVolumeSource>,

    /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub gce_persistent_disk: Option<crate::api::core::v1::GCEPersistentDiskVolumeSource>,

    /// GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
    pub git_repo: Option<crate::api::core::v1::GitRepoVolumeSource>,

    /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
    pub glusterfs: Option<crate::api::core::v1::GlusterfsVolumeSource>,

    /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    pub host_path: Option<crate::api::core::v1::HostPathVolumeSource>,

    /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
    pub iscsi: Option<crate::api::core::v1::ISCSIVolumeSource>,

    /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,

    /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub nfs: Option<crate::api::core::v1::NFSVolumeSource>,

    /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    pub persistent_volume_claim: Option<crate::api::core::v1::PersistentVolumeClaimVolumeSource>,

    /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    pub photon_persistent_disk: Option<crate::api::core::v1::PhotonPersistentDiskVolumeSource>,

    /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
    pub portworx_volume: Option<crate::api::core::v1::PortworxVolumeSource>,

    /// Items for all in one resources secrets, configmaps, and downward API
    pub projected: Option<crate::api::core::v1::ProjectedVolumeSource>,

    /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    pub quobyte: Option<crate::api::core::v1::QuobyteVolumeSource>,

    /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
    pub rbd: Option<crate::api::core::v1::RBDVolumeSource>,

    /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    pub scale_io: Option<crate::api::core::v1::ScaleIOVolumeSource>,

    /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    pub secret: Option<crate::api::core::v1::SecretVolumeSource>,

    /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
    pub storageos: Option<crate::api::core::v1::StorageOSVolumeSource>,

    /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    pub vsphere_volume: Option<crate::api::core::v1::VsphereVirtualDiskVolumeSource>,

}

#[cfg(feature = "dsl")]
impl Volume  {
    /// Set [`Self::aws_elastic_block_store`]
    pub  fn aws_elastic_block_store_set(&mut self, aws_elastic_block_store: impl Into<Option<crate::api::core::v1::AWSElasticBlockStoreVolumeSource>>) -> &mut Self {
        self.aws_elastic_block_store = aws_elastic_block_store.into(); self
    }

    pub  fn aws_elastic_block_store(&mut self) -> &mut crate::api::core::v1::AWSElasticBlockStoreVolumeSource {
        if self.aws_elastic_block_store.is_none() { self.aws_elastic_block_store = Some(Default::default()) }
        self.aws_elastic_block_store.as_mut().unwrap()
    }

    /// Modify [`Self::aws_elastic_block_store`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn aws_elastic_block_store_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::AWSElasticBlockStoreVolumeSource)) -> &mut Self {
        if self.aws_elastic_block_store.is_none() { self.aws_elastic_block_store = Some(Default::default()) };
        func(self.aws_elastic_block_store.as_mut().unwrap()); self
    }


    /// Set [`Self::azure_disk`]
    pub  fn azure_disk_set(&mut self, azure_disk: impl Into<Option<crate::api::core::v1::AzureDiskVolumeSource>>) -> &mut Self {
        self.azure_disk = azure_disk.into(); self
    }

    pub  fn azure_disk(&mut self) -> &mut crate::api::core::v1::AzureDiskVolumeSource {
        if self.azure_disk.is_none() { self.azure_disk = Some(Default::default()) }
        self.azure_disk.as_mut().unwrap()
    }

    /// Modify [`Self::azure_disk`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn azure_disk_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::AzureDiskVolumeSource)) -> &mut Self {
        if self.azure_disk.is_none() { self.azure_disk = Some(Default::default()) };
        func(self.azure_disk.as_mut().unwrap()); self
    }


    /// Set [`Self::azure_file`]
    pub  fn azure_file_set(&mut self, azure_file: impl Into<Option<crate::api::core::v1::AzureFileVolumeSource>>) -> &mut Self {
        self.azure_file = azure_file.into(); self
    }

    pub  fn azure_file(&mut self) -> &mut crate::api::core::v1::AzureFileVolumeSource {
        if self.azure_file.is_none() { self.azure_file = Some(Default::default()) }
        self.azure_file.as_mut().unwrap()
    }

    /// Modify [`Self::azure_file`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn azure_file_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::AzureFileVolumeSource)) -> &mut Self {
        if self.azure_file.is_none() { self.azure_file = Some(Default::default()) };
        func(self.azure_file.as_mut().unwrap()); self
    }


    /// Set [`Self::cephfs`]
    pub  fn cephfs_set(&mut self, cephfs: impl Into<Option<crate::api::core::v1::CephFSVolumeSource>>) -> &mut Self {
        self.cephfs = cephfs.into(); self
    }

    pub  fn cephfs(&mut self) -> &mut crate::api::core::v1::CephFSVolumeSource {
        if self.cephfs.is_none() { self.cephfs = Some(Default::default()) }
        self.cephfs.as_mut().unwrap()
    }

    /// Modify [`Self::cephfs`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn cephfs_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::CephFSVolumeSource)) -> &mut Self {
        if self.cephfs.is_none() { self.cephfs = Some(Default::default()) };
        func(self.cephfs.as_mut().unwrap()); self
    }


    /// Set [`Self::cinder`]
    pub  fn cinder_set(&mut self, cinder: impl Into<Option<crate::api::core::v1::CinderVolumeSource>>) -> &mut Self {
        self.cinder = cinder.into(); self
    }

    pub  fn cinder(&mut self) -> &mut crate::api::core::v1::CinderVolumeSource {
        if self.cinder.is_none() { self.cinder = Some(Default::default()) }
        self.cinder.as_mut().unwrap()
    }

    /// Modify [`Self::cinder`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn cinder_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::CinderVolumeSource)) -> &mut Self {
        if self.cinder.is_none() { self.cinder = Some(Default::default()) };
        func(self.cinder.as_mut().unwrap()); self
    }


    /// Set [`Self::config_map`]
    pub  fn config_map_set(&mut self, config_map: impl Into<Option<crate::api::core::v1::ConfigMapVolumeSource>>) -> &mut Self {
        self.config_map = config_map.into(); self
    }

    pub  fn config_map(&mut self) -> &mut crate::api::core::v1::ConfigMapVolumeSource {
        if self.config_map.is_none() { self.config_map = Some(Default::default()) }
        self.config_map.as_mut().unwrap()
    }

    /// Modify [`Self::config_map`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn config_map_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ConfigMapVolumeSource)) -> &mut Self {
        if self.config_map.is_none() { self.config_map = Some(Default::default()) };
        func(self.config_map.as_mut().unwrap()); self
    }


    /// Set [`Self::csi`]
    pub  fn csi_set(&mut self, csi: impl Into<Option<crate::api::core::v1::CSIVolumeSource>>) -> &mut Self {
        self.csi = csi.into(); self
    }

    pub  fn csi(&mut self) -> &mut crate::api::core::v1::CSIVolumeSource {
        if self.csi.is_none() { self.csi = Some(Default::default()) }
        self.csi.as_mut().unwrap()
    }

    /// Modify [`Self::csi`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn csi_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::CSIVolumeSource)) -> &mut Self {
        if self.csi.is_none() { self.csi = Some(Default::default()) };
        func(self.csi.as_mut().unwrap()); self
    }


    /// Set [`Self::downward_api`]
    pub  fn downward_api_set(&mut self, downward_api: impl Into<Option<crate::api::core::v1::DownwardAPIVolumeSource>>) -> &mut Self {
        self.downward_api = downward_api.into(); self
    }

    pub  fn downward_api(&mut self) -> &mut crate::api::core::v1::DownwardAPIVolumeSource {
        if self.downward_api.is_none() { self.downward_api = Some(Default::default()) }
        self.downward_api.as_mut().unwrap()
    }

    /// Modify [`Self::downward_api`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn downward_api_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::DownwardAPIVolumeSource)) -> &mut Self {
        if self.downward_api.is_none() { self.downward_api = Some(Default::default()) };
        func(self.downward_api.as_mut().unwrap()); self
    }


    /// Set [`Self::empty_dir`]
    pub  fn empty_dir_set(&mut self, empty_dir: impl Into<Option<crate::api::core::v1::EmptyDirVolumeSource>>) -> &mut Self {
        self.empty_dir = empty_dir.into(); self
    }

    pub  fn empty_dir(&mut self) -> &mut crate::api::core::v1::EmptyDirVolumeSource {
        if self.empty_dir.is_none() { self.empty_dir = Some(Default::default()) }
        self.empty_dir.as_mut().unwrap()
    }

    /// Modify [`Self::empty_dir`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn empty_dir_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::EmptyDirVolumeSource)) -> &mut Self {
        if self.empty_dir.is_none() { self.empty_dir = Some(Default::default()) };
        func(self.empty_dir.as_mut().unwrap()); self
    }


    /// Set [`Self::ephemeral`]
    pub  fn ephemeral_set(&mut self, ephemeral: impl Into<Option<crate::api::core::v1::EphemeralVolumeSource>>) -> &mut Self {
        self.ephemeral = ephemeral.into(); self
    }

    pub  fn ephemeral(&mut self) -> &mut crate::api::core::v1::EphemeralVolumeSource {
        if self.ephemeral.is_none() { self.ephemeral = Some(Default::default()) }
        self.ephemeral.as_mut().unwrap()
    }

    /// Modify [`Self::ephemeral`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ephemeral_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::EphemeralVolumeSource)) -> &mut Self {
        if self.ephemeral.is_none() { self.ephemeral = Some(Default::default()) };
        func(self.ephemeral.as_mut().unwrap()); self
    }


    /// Set [`Self::fc`]
    pub  fn fc_set(&mut self, fc: impl Into<Option<crate::api::core::v1::FCVolumeSource>>) -> &mut Self {
        self.fc = fc.into(); self
    }

    pub  fn fc(&mut self) -> &mut crate::api::core::v1::FCVolumeSource {
        if self.fc.is_none() { self.fc = Some(Default::default()) }
        self.fc.as_mut().unwrap()
    }

    /// Modify [`Self::fc`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn fc_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::FCVolumeSource)) -> &mut Self {
        if self.fc.is_none() { self.fc = Some(Default::default()) };
        func(self.fc.as_mut().unwrap()); self
    }


    /// Set [`Self::flex_volume`]
    pub  fn flex_volume_set(&mut self, flex_volume: impl Into<Option<crate::api::core::v1::FlexVolumeSource>>) -> &mut Self {
        self.flex_volume = flex_volume.into(); self
    }

    pub  fn flex_volume(&mut self) -> &mut crate::api::core::v1::FlexVolumeSource {
        if self.flex_volume.is_none() { self.flex_volume = Some(Default::default()) }
        self.flex_volume.as_mut().unwrap()
    }

    /// Modify [`Self::flex_volume`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn flex_volume_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::FlexVolumeSource)) -> &mut Self {
        if self.flex_volume.is_none() { self.flex_volume = Some(Default::default()) };
        func(self.flex_volume.as_mut().unwrap()); self
    }


    /// Set [`Self::flocker`]
    pub  fn flocker_set(&mut self, flocker: impl Into<Option<crate::api::core::v1::FlockerVolumeSource>>) -> &mut Self {
        self.flocker = flocker.into(); self
    }

    pub  fn flocker(&mut self) -> &mut crate::api::core::v1::FlockerVolumeSource {
        if self.flocker.is_none() { self.flocker = Some(Default::default()) }
        self.flocker.as_mut().unwrap()
    }

    /// Modify [`Self::flocker`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn flocker_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::FlockerVolumeSource)) -> &mut Self {
        if self.flocker.is_none() { self.flocker = Some(Default::default()) };
        func(self.flocker.as_mut().unwrap()); self
    }


    /// Set [`Self::gce_persistent_disk`]
    pub  fn gce_persistent_disk_set(&mut self, gce_persistent_disk: impl Into<Option<crate::api::core::v1::GCEPersistentDiskVolumeSource>>) -> &mut Self {
        self.gce_persistent_disk = gce_persistent_disk.into(); self
    }

    pub  fn gce_persistent_disk(&mut self) -> &mut crate::api::core::v1::GCEPersistentDiskVolumeSource {
        if self.gce_persistent_disk.is_none() { self.gce_persistent_disk = Some(Default::default()) }
        self.gce_persistent_disk.as_mut().unwrap()
    }

    /// Modify [`Self::gce_persistent_disk`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn gce_persistent_disk_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::GCEPersistentDiskVolumeSource)) -> &mut Self {
        if self.gce_persistent_disk.is_none() { self.gce_persistent_disk = Some(Default::default()) };
        func(self.gce_persistent_disk.as_mut().unwrap()); self
    }


    /// Set [`Self::git_repo`]
    pub  fn git_repo_set(&mut self, git_repo: impl Into<Option<crate::api::core::v1::GitRepoVolumeSource>>) -> &mut Self {
        self.git_repo = git_repo.into(); self
    }

    pub  fn git_repo(&mut self) -> &mut crate::api::core::v1::GitRepoVolumeSource {
        if self.git_repo.is_none() { self.git_repo = Some(Default::default()) }
        self.git_repo.as_mut().unwrap()
    }

    /// Modify [`Self::git_repo`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn git_repo_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::GitRepoVolumeSource)) -> &mut Self {
        if self.git_repo.is_none() { self.git_repo = Some(Default::default()) };
        func(self.git_repo.as_mut().unwrap()); self
    }


    /// Set [`Self::glusterfs`]
    pub  fn glusterfs_set(&mut self, glusterfs: impl Into<Option<crate::api::core::v1::GlusterfsVolumeSource>>) -> &mut Self {
        self.glusterfs = glusterfs.into(); self
    }

    pub  fn glusterfs(&mut self) -> &mut crate::api::core::v1::GlusterfsVolumeSource {
        if self.glusterfs.is_none() { self.glusterfs = Some(Default::default()) }
        self.glusterfs.as_mut().unwrap()
    }

    /// Modify [`Self::glusterfs`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn glusterfs_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::GlusterfsVolumeSource)) -> &mut Self {
        if self.glusterfs.is_none() { self.glusterfs = Some(Default::default()) };
        func(self.glusterfs.as_mut().unwrap()); self
    }


    /// Set [`Self::host_path`]
    pub  fn host_path_set(&mut self, host_path: impl Into<Option<crate::api::core::v1::HostPathVolumeSource>>) -> &mut Self {
        self.host_path = host_path.into(); self
    }

    pub  fn host_path(&mut self) -> &mut crate::api::core::v1::HostPathVolumeSource {
        if self.host_path.is_none() { self.host_path = Some(Default::default()) }
        self.host_path.as_mut().unwrap()
    }

    /// Modify [`Self::host_path`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn host_path_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::HostPathVolumeSource)) -> &mut Self {
        if self.host_path.is_none() { self.host_path = Some(Default::default()) };
        func(self.host_path.as_mut().unwrap()); self
    }


    /// Set [`Self::iscsi`]
    pub  fn iscsi_set(&mut self, iscsi: impl Into<Option<crate::api::core::v1::ISCSIVolumeSource>>) -> &mut Self {
        self.iscsi = iscsi.into(); self
    }

    pub  fn iscsi(&mut self) -> &mut crate::api::core::v1::ISCSIVolumeSource {
        if self.iscsi.is_none() { self.iscsi = Some(Default::default()) }
        self.iscsi.as_mut().unwrap()
    }

    /// Modify [`Self::iscsi`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn iscsi_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ISCSIVolumeSource)) -> &mut Self {
        if self.iscsi.is_none() { self.iscsi = Some(Default::default()) };
        func(self.iscsi.as_mut().unwrap()); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        &mut self.name
    }

    /// Modify [`Self::name`] with a `func`
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.name); self
    }


    /// Set [`Self::nfs`]
    pub  fn nfs_set(&mut self, nfs: impl Into<Option<crate::api::core::v1::NFSVolumeSource>>) -> &mut Self {
        self.nfs = nfs.into(); self
    }

    pub  fn nfs(&mut self) -> &mut crate::api::core::v1::NFSVolumeSource {
        if self.nfs.is_none() { self.nfs = Some(Default::default()) }
        self.nfs.as_mut().unwrap()
    }

    /// Modify [`Self::nfs`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn nfs_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::NFSVolumeSource)) -> &mut Self {
        if self.nfs.is_none() { self.nfs = Some(Default::default()) };
        func(self.nfs.as_mut().unwrap()); self
    }


    /// Set [`Self::persistent_volume_claim`]
    pub  fn persistent_volume_claim_set(&mut self, persistent_volume_claim: impl Into<Option<crate::api::core::v1::PersistentVolumeClaimVolumeSource>>) -> &mut Self {
        self.persistent_volume_claim = persistent_volume_claim.into(); self
    }

    pub  fn persistent_volume_claim(&mut self) -> &mut crate::api::core::v1::PersistentVolumeClaimVolumeSource {
        if self.persistent_volume_claim.is_none() { self.persistent_volume_claim = Some(Default::default()) }
        self.persistent_volume_claim.as_mut().unwrap()
    }

    /// Modify [`Self::persistent_volume_claim`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn persistent_volume_claim_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PersistentVolumeClaimVolumeSource)) -> &mut Self {
        if self.persistent_volume_claim.is_none() { self.persistent_volume_claim = Some(Default::default()) };
        func(self.persistent_volume_claim.as_mut().unwrap()); self
    }


    /// Set [`Self::photon_persistent_disk`]
    pub  fn photon_persistent_disk_set(&mut self, photon_persistent_disk: impl Into<Option<crate::api::core::v1::PhotonPersistentDiskVolumeSource>>) -> &mut Self {
        self.photon_persistent_disk = photon_persistent_disk.into(); self
    }

    pub  fn photon_persistent_disk(&mut self) -> &mut crate::api::core::v1::PhotonPersistentDiskVolumeSource {
        if self.photon_persistent_disk.is_none() { self.photon_persistent_disk = Some(Default::default()) }
        self.photon_persistent_disk.as_mut().unwrap()
    }

    /// Modify [`Self::photon_persistent_disk`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn photon_persistent_disk_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PhotonPersistentDiskVolumeSource)) -> &mut Self {
        if self.photon_persistent_disk.is_none() { self.photon_persistent_disk = Some(Default::default()) };
        func(self.photon_persistent_disk.as_mut().unwrap()); self
    }


    /// Set [`Self::portworx_volume`]
    pub  fn portworx_volume_set(&mut self, portworx_volume: impl Into<Option<crate::api::core::v1::PortworxVolumeSource>>) -> &mut Self {
        self.portworx_volume = portworx_volume.into(); self
    }

    pub  fn portworx_volume(&mut self) -> &mut crate::api::core::v1::PortworxVolumeSource {
        if self.portworx_volume.is_none() { self.portworx_volume = Some(Default::default()) }
        self.portworx_volume.as_mut().unwrap()
    }

    /// Modify [`Self::portworx_volume`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn portworx_volume_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::PortworxVolumeSource)) -> &mut Self {
        if self.portworx_volume.is_none() { self.portworx_volume = Some(Default::default()) };
        func(self.portworx_volume.as_mut().unwrap()); self
    }


    /// Set [`Self::projected`]
    pub  fn projected_set(&mut self, projected: impl Into<Option<crate::api::core::v1::ProjectedVolumeSource>>) -> &mut Self {
        self.projected = projected.into(); self
    }

    pub  fn projected(&mut self) -> &mut crate::api::core::v1::ProjectedVolumeSource {
        if self.projected.is_none() { self.projected = Some(Default::default()) }
        self.projected.as_mut().unwrap()
    }

    /// Modify [`Self::projected`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn projected_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ProjectedVolumeSource)) -> &mut Self {
        if self.projected.is_none() { self.projected = Some(Default::default()) };
        func(self.projected.as_mut().unwrap()); self
    }


    /// Set [`Self::quobyte`]
    pub  fn quobyte_set(&mut self, quobyte: impl Into<Option<crate::api::core::v1::QuobyteVolumeSource>>) -> &mut Self {
        self.quobyte = quobyte.into(); self
    }

    pub  fn quobyte(&mut self) -> &mut crate::api::core::v1::QuobyteVolumeSource {
        if self.quobyte.is_none() { self.quobyte = Some(Default::default()) }
        self.quobyte.as_mut().unwrap()
    }

    /// Modify [`Self::quobyte`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn quobyte_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::QuobyteVolumeSource)) -> &mut Self {
        if self.quobyte.is_none() { self.quobyte = Some(Default::default()) };
        func(self.quobyte.as_mut().unwrap()); self
    }


    /// Set [`Self::rbd`]
    pub  fn rbd_set(&mut self, rbd: impl Into<Option<crate::api::core::v1::RBDVolumeSource>>) -> &mut Self {
        self.rbd = rbd.into(); self
    }

    pub  fn rbd(&mut self) -> &mut crate::api::core::v1::RBDVolumeSource {
        if self.rbd.is_none() { self.rbd = Some(Default::default()) }
        self.rbd.as_mut().unwrap()
    }

    /// Modify [`Self::rbd`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn rbd_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::RBDVolumeSource)) -> &mut Self {
        if self.rbd.is_none() { self.rbd = Some(Default::default()) };
        func(self.rbd.as_mut().unwrap()); self
    }


    /// Set [`Self::scale_io`]
    pub  fn scale_io_set(&mut self, scale_io: impl Into<Option<crate::api::core::v1::ScaleIOVolumeSource>>) -> &mut Self {
        self.scale_io = scale_io.into(); self
    }

    pub  fn scale_io(&mut self) -> &mut crate::api::core::v1::ScaleIOVolumeSource {
        if self.scale_io.is_none() { self.scale_io = Some(Default::default()) }
        self.scale_io.as_mut().unwrap()
    }

    /// Modify [`Self::scale_io`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn scale_io_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ScaleIOVolumeSource)) -> &mut Self {
        if self.scale_io.is_none() { self.scale_io = Some(Default::default()) };
        func(self.scale_io.as_mut().unwrap()); self
    }


    /// Set [`Self::secret`]
    pub  fn secret_set(&mut self, secret: impl Into<Option<crate::api::core::v1::SecretVolumeSource>>) -> &mut Self {
        self.secret = secret.into(); self
    }

    pub  fn secret(&mut self) -> &mut crate::api::core::v1::SecretVolumeSource {
        if self.secret.is_none() { self.secret = Some(Default::default()) }
        self.secret.as_mut().unwrap()
    }

    /// Modify [`Self::secret`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn secret_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::SecretVolumeSource)) -> &mut Self {
        if self.secret.is_none() { self.secret = Some(Default::default()) };
        func(self.secret.as_mut().unwrap()); self
    }


    /// Set [`Self::storageos`]
    pub  fn storageos_set(&mut self, storageos: impl Into<Option<crate::api::core::v1::StorageOSVolumeSource>>) -> &mut Self {
        self.storageos = storageos.into(); self
    }

    pub  fn storageos(&mut self) -> &mut crate::api::core::v1::StorageOSVolumeSource {
        if self.storageos.is_none() { self.storageos = Some(Default::default()) }
        self.storageos.as_mut().unwrap()
    }

    /// Modify [`Self::storageos`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn storageos_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::StorageOSVolumeSource)) -> &mut Self {
        if self.storageos.is_none() { self.storageos = Some(Default::default()) };
        func(self.storageos.as_mut().unwrap()); self
    }


    /// Set [`Self::vsphere_volume`]
    pub  fn vsphere_volume_set(&mut self, vsphere_volume: impl Into<Option<crate::api::core::v1::VsphereVirtualDiskVolumeSource>>) -> &mut Self {
        self.vsphere_volume = vsphere_volume.into(); self
    }

    pub  fn vsphere_volume(&mut self) -> &mut crate::api::core::v1::VsphereVirtualDiskVolumeSource {
        if self.vsphere_volume.is_none() { self.vsphere_volume = Some(Default::default()) }
        self.vsphere_volume.as_mut().unwrap()
    }

    /// Modify [`Self::vsphere_volume`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn vsphere_volume_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::VsphereVirtualDiskVolumeSource)) -> &mut Self {
        if self.vsphere_volume.is_none() { self.vsphere_volume = Some(Default::default()) };
        func(self.vsphere_volume.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for Volume {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_aws_elastic_block_store,
            Key_azure_disk,
            Key_azure_file,
            Key_cephfs,
            Key_cinder,
            Key_config_map,
            Key_csi,
            Key_downward_api,
            Key_empty_dir,
            Key_ephemeral,
            Key_fc,
            Key_flex_volume,
            Key_flocker,
            Key_gce_persistent_disk,
            Key_git_repo,
            Key_glusterfs,
            Key_host_path,
            Key_iscsi,
            Key_name,
            Key_nfs,
            Key_persistent_volume_claim,
            Key_photon_persistent_disk,
            Key_portworx_volume,
            Key_projected,
            Key_quobyte,
            Key_rbd,
            Key_scale_io,
            Key_secret,
            Key_storageos,
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
                            "awsElasticBlockStore" => Field::Key_aws_elastic_block_store,
                            "azureDisk" => Field::Key_azure_disk,
                            "azureFile" => Field::Key_azure_file,
                            "cephfs" => Field::Key_cephfs,
                            "cinder" => Field::Key_cinder,
                            "configMap" => Field::Key_config_map,
                            "csi" => Field::Key_csi,
                            "downwardAPI" => Field::Key_downward_api,
                            "emptyDir" => Field::Key_empty_dir,
                            "ephemeral" => Field::Key_ephemeral,
                            "fc" => Field::Key_fc,
                            "flexVolume" => Field::Key_flex_volume,
                            "flocker" => Field::Key_flocker,
                            "gcePersistentDisk" => Field::Key_gce_persistent_disk,
                            "gitRepo" => Field::Key_git_repo,
                            "glusterfs" => Field::Key_glusterfs,
                            "hostPath" => Field::Key_host_path,
                            "iscsi" => Field::Key_iscsi,
                            "name" => Field::Key_name,
                            "nfs" => Field::Key_nfs,
                            "persistentVolumeClaim" => Field::Key_persistent_volume_claim,
                            "photonPersistentDisk" => Field::Key_photon_persistent_disk,
                            "portworxVolume" => Field::Key_portworx_volume,
                            "projected" => Field::Key_projected,
                            "quobyte" => Field::Key_quobyte,
                            "rbd" => Field::Key_rbd,
                            "scaleIO" => Field::Key_scale_io,
                            "secret" => Field::Key_secret,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Volume;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Volume")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_aws_elastic_block_store: Option<crate::api::core::v1::AWSElasticBlockStoreVolumeSource> = None;
                let mut value_azure_disk: Option<crate::api::core::v1::AzureDiskVolumeSource> = None;
                let mut value_azure_file: Option<crate::api::core::v1::AzureFileVolumeSource> = None;
                let mut value_cephfs: Option<crate::api::core::v1::CephFSVolumeSource> = None;
                let mut value_cinder: Option<crate::api::core::v1::CinderVolumeSource> = None;
                let mut value_config_map: Option<crate::api::core::v1::ConfigMapVolumeSource> = None;
                let mut value_csi: Option<crate::api::core::v1::CSIVolumeSource> = None;
                let mut value_downward_api: Option<crate::api::core::v1::DownwardAPIVolumeSource> = None;
                let mut value_empty_dir: Option<crate::api::core::v1::EmptyDirVolumeSource> = None;
                let mut value_ephemeral: Option<crate::api::core::v1::EphemeralVolumeSource> = None;
                let mut value_fc: Option<crate::api::core::v1::FCVolumeSource> = None;
                let mut value_flex_volume: Option<crate::api::core::v1::FlexVolumeSource> = None;
                let mut value_flocker: Option<crate::api::core::v1::FlockerVolumeSource> = None;
                let mut value_gce_persistent_disk: Option<crate::api::core::v1::GCEPersistentDiskVolumeSource> = None;
                let mut value_git_repo: Option<crate::api::core::v1::GitRepoVolumeSource> = None;
                let mut value_glusterfs: Option<crate::api::core::v1::GlusterfsVolumeSource> = None;
                let mut value_host_path: Option<crate::api::core::v1::HostPathVolumeSource> = None;
                let mut value_iscsi: Option<crate::api::core::v1::ISCSIVolumeSource> = None;
                let mut value_name: Option<String> = None;
                let mut value_nfs: Option<crate::api::core::v1::NFSVolumeSource> = None;
                let mut value_persistent_volume_claim: Option<crate::api::core::v1::PersistentVolumeClaimVolumeSource> = None;
                let mut value_photon_persistent_disk: Option<crate::api::core::v1::PhotonPersistentDiskVolumeSource> = None;
                let mut value_portworx_volume: Option<crate::api::core::v1::PortworxVolumeSource> = None;
                let mut value_projected: Option<crate::api::core::v1::ProjectedVolumeSource> = None;
                let mut value_quobyte: Option<crate::api::core::v1::QuobyteVolumeSource> = None;
                let mut value_rbd: Option<crate::api::core::v1::RBDVolumeSource> = None;
                let mut value_scale_io: Option<crate::api::core::v1::ScaleIOVolumeSource> = None;
                let mut value_secret: Option<crate::api::core::v1::SecretVolumeSource> = None;
                let mut value_storageos: Option<crate::api::core::v1::StorageOSVolumeSource> = None;
                let mut value_vsphere_volume: Option<crate::api::core::v1::VsphereVirtualDiskVolumeSource> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_aws_elastic_block_store => value_aws_elastic_block_store = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_azure_disk => value_azure_disk = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_azure_file => value_azure_file = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cephfs => value_cephfs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cinder => value_cinder = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_config_map => value_config_map = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_csi => value_csi = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_downward_api => value_downward_api = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_empty_dir => value_empty_dir = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ephemeral => value_ephemeral = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fc => value_fc = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_flex_volume => value_flex_volume = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_flocker => value_flocker = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_gce_persistent_disk => value_gce_persistent_disk = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_git_repo => value_git_repo = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_glusterfs => value_glusterfs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_path => value_host_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_iscsi => value_iscsi = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nfs => value_nfs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_persistent_volume_claim => value_persistent_volume_claim = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_photon_persistent_disk => value_photon_persistent_disk = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_portworx_volume => value_portworx_volume = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_projected => value_projected = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_quobyte => value_quobyte = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rbd => value_rbd = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_scale_io => value_scale_io = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_secret => value_secret = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_storageos => value_storageos = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_vsphere_volume => value_vsphere_volume = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Volume {
                    aws_elastic_block_store: value_aws_elastic_block_store,
                    azure_disk: value_azure_disk,
                    azure_file: value_azure_file,
                    cephfs: value_cephfs,
                    cinder: value_cinder,
                    config_map: value_config_map,
                    csi: value_csi,
                    downward_api: value_downward_api,
                    empty_dir: value_empty_dir,
                    ephemeral: value_ephemeral,
                    fc: value_fc,
                    flex_volume: value_flex_volume,
                    flocker: value_flocker,
                    gce_persistent_disk: value_gce_persistent_disk,
                    git_repo: value_git_repo,
                    glusterfs: value_glusterfs,
                    host_path: value_host_path,
                    iscsi: value_iscsi,
                    name: value_name.unwrap_or_default(),
                    nfs: value_nfs,
                    persistent_volume_claim: value_persistent_volume_claim,
                    photon_persistent_disk: value_photon_persistent_disk,
                    portworx_volume: value_portworx_volume,
                    projected: value_projected,
                    quobyte: value_quobyte,
                    rbd: value_rbd,
                    scale_io: value_scale_io,
                    secret: value_secret,
                    storageos: value_storageos,
                    vsphere_volume: value_vsphere_volume,
                })
            }
        }

        deserializer.deserialize_struct(
            "Volume",
            &[
                "awsElasticBlockStore",
                "azureDisk",
                "azureFile",
                "cephfs",
                "cinder",
                "configMap",
                "csi",
                "downwardAPI",
                "emptyDir",
                "ephemeral",
                "fc",
                "flexVolume",
                "flocker",
                "gcePersistentDisk",
                "gitRepo",
                "glusterfs",
                "hostPath",
                "iscsi",
                "name",
                "nfs",
                "persistentVolumeClaim",
                "photonPersistentDisk",
                "portworxVolume",
                "projected",
                "quobyte",
                "rbd",
                "scaleIO",
                "secret",
                "storageos",
                "vsphereVolume",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Volume {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Volume",
            1 +
            self.aws_elastic_block_store.as_ref().map_or(0, |_| 1) +
            self.azure_disk.as_ref().map_or(0, |_| 1) +
            self.azure_file.as_ref().map_or(0, |_| 1) +
            self.cephfs.as_ref().map_or(0, |_| 1) +
            self.cinder.as_ref().map_or(0, |_| 1) +
            self.config_map.as_ref().map_or(0, |_| 1) +
            self.csi.as_ref().map_or(0, |_| 1) +
            self.downward_api.as_ref().map_or(0, |_| 1) +
            self.empty_dir.as_ref().map_or(0, |_| 1) +
            self.ephemeral.as_ref().map_or(0, |_| 1) +
            self.fc.as_ref().map_or(0, |_| 1) +
            self.flex_volume.as_ref().map_or(0, |_| 1) +
            self.flocker.as_ref().map_or(0, |_| 1) +
            self.gce_persistent_disk.as_ref().map_or(0, |_| 1) +
            self.git_repo.as_ref().map_or(0, |_| 1) +
            self.glusterfs.as_ref().map_or(0, |_| 1) +
            self.host_path.as_ref().map_or(0, |_| 1) +
            self.iscsi.as_ref().map_or(0, |_| 1) +
            self.nfs.as_ref().map_or(0, |_| 1) +
            self.persistent_volume_claim.as_ref().map_or(0, |_| 1) +
            self.photon_persistent_disk.as_ref().map_or(0, |_| 1) +
            self.portworx_volume.as_ref().map_or(0, |_| 1) +
            self.projected.as_ref().map_or(0, |_| 1) +
            self.quobyte.as_ref().map_or(0, |_| 1) +
            self.rbd.as_ref().map_or(0, |_| 1) +
            self.scale_io.as_ref().map_or(0, |_| 1) +
            self.secret.as_ref().map_or(0, |_| 1) +
            self.storageos.as_ref().map_or(0, |_| 1) +
            self.vsphere_volume.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.aws_elastic_block_store {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "awsElasticBlockStore", value)?;
        }
        if let Some(value) = &self.azure_disk {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "azureDisk", value)?;
        }
        if let Some(value) = &self.azure_file {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "azureFile", value)?;
        }
        if let Some(value) = &self.cephfs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cephfs", value)?;
        }
        if let Some(value) = &self.cinder {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cinder", value)?;
        }
        if let Some(value) = &self.config_map {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "configMap", value)?;
        }
        if let Some(value) = &self.csi {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "csi", value)?;
        }
        if let Some(value) = &self.downward_api {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "downwardAPI", value)?;
        }
        if let Some(value) = &self.empty_dir {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "emptyDir", value)?;
        }
        if let Some(value) = &self.ephemeral {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ephemeral", value)?;
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
        if let Some(value) = &self.git_repo {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "gitRepo", value)?;
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
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.nfs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nfs", value)?;
        }
        if let Some(value) = &self.persistent_volume_claim {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "persistentVolumeClaim", value)?;
        }
        if let Some(value) = &self.photon_persistent_disk {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "photonPersistentDisk", value)?;
        }
        if let Some(value) = &self.portworx_volume {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "portworxVolume", value)?;
        }
        if let Some(value) = &self.projected {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "projected", value)?;
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
        if let Some(value) = &self.secret {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "secret", value)?;
        }
        if let Some(value) = &self.storageos {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "storageos", value)?;
        }
        if let Some(value) = &self.vsphere_volume {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "vsphereVolume", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Volume {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.Volume".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Volume represents a named volume in a pod that may be accessed by any container in the pod.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "awsElasticBlockStore".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AWSElasticBlockStoreVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore".to_owned()),
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
                                description: Some("AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "azureFile".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AzureFileVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AzureFile represents an Azure File Service mount on the host and bind mount to the pod.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "cephfs".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::CephFSVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("CephFS represents a Ceph FS mount on the host that shares a pod's lifetime".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "cinder".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::CinderVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "configMap".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ConfigMapVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ConfigMap represents a configMap that should populate this volume".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "csi".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::CSIVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "downwardAPI".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::DownwardAPIVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("DownwardAPI represents downward API about the pod that should populate this volume".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "emptyDir".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::EmptyDirVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "ephemeral".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::EphemeralVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed.\n\nUse this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity\n   tracking are needed,\nc) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through\n   a PersistentVolumeClaim (see EphemeralVolumeSource for more\n   information on the connection between this volume type\n   and PersistentVolumeClaim).\n\nUse PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod.\n\nUse CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information.\n\nA pod can use both types of ephemeral volumes and persistent volumes at the same time.\n\nThis is a beta feature and only available when the GenericEphemeralVolume feature gate is enabled.".to_owned()),
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
                                description: Some("FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "flexVolume".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::FlexVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.".to_owned()),
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
                                description: Some("Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running".to_owned()),
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
                                description: Some("GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "gitRepo".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::GitRepoVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "glusterfs".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::GlusterfsVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md".to_owned()),
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
                                description: Some("HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "iscsi".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ISCSIVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nfs".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NFSVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "persistentVolumeClaim".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PersistentVolumeClaimVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "photonPersistentDisk".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PhotonPersistentDiskVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine".to_owned()),
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
                                description: Some("PortworxVolume represents a portworx volume attached and mounted on kubelets host machine".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "projected".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ProjectedVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Items for all in one resources secrets, configmaps, and downward API".to_owned()),
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
                                description: Some("Quobyte represents a Quobyte mount on the host that shares a pod's lifetime".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "rbd".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::RBDVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "scaleIO".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ScaleIOVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "secret".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "storageos".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::StorageOSVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "vsphereVolume".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::VsphereVirtualDiskVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "name".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
