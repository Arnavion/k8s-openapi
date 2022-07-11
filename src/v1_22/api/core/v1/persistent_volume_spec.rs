// Generated from definition io.k8s.api.core.v1.PersistentVolumeSpec

/// PersistentVolumeSpec is the specification of a persistent volume.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeSpec {
    /// AccessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes
    pub access_modes: Option<Vec<String>>,

    /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub aws_elastic_block_store: Option<crate::api::core::v1::AWSElasticBlockStoreVolumeSource>,

    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    pub azure_disk: Option<crate::api::core::v1::AzureDiskVolumeSource>,

    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    pub azure_file: Option<crate::api::core::v1::AzureFilePersistentVolumeSource>,

    /// A description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
    pub capacity: Option<std::collections::BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity>>,

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
    pub mount_options: Option<Vec<String>>,

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

#[cfg(feature = "dsl")]
impl PersistentVolumeSpec  {
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
    pub  fn azure_file_set(&mut self, azure_file: impl Into<Option<crate::api::core::v1::AzureFilePersistentVolumeSource>>) -> &mut Self {
        self.azure_file = azure_file.into(); self
    }

    pub  fn azure_file(&mut self) -> &mut crate::api::core::v1::AzureFilePersistentVolumeSource {
        if self.azure_file.is_none() { self.azure_file = Some(Default::default()) }
        self.azure_file.as_mut().unwrap()
    }

    /// Modify [`Self::azure_file`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn azure_file_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::AzureFilePersistentVolumeSource)) -> &mut Self {
        if self.azure_file.is_none() { self.azure_file = Some(Default::default()) };
        func(self.azure_file.as_mut().unwrap()); self
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


    /// Set [`Self::cephfs`]
    pub  fn cephfs_set(&mut self, cephfs: impl Into<Option<crate::api::core::v1::CephFSPersistentVolumeSource>>) -> &mut Self {
        self.cephfs = cephfs.into(); self
    }

    pub  fn cephfs(&mut self) -> &mut crate::api::core::v1::CephFSPersistentVolumeSource {
        if self.cephfs.is_none() { self.cephfs = Some(Default::default()) }
        self.cephfs.as_mut().unwrap()
    }

    /// Modify [`Self::cephfs`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn cephfs_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::CephFSPersistentVolumeSource)) -> &mut Self {
        if self.cephfs.is_none() { self.cephfs = Some(Default::default()) };
        func(self.cephfs.as_mut().unwrap()); self
    }


    /// Set [`Self::cinder`]
    pub  fn cinder_set(&mut self, cinder: impl Into<Option<crate::api::core::v1::CinderPersistentVolumeSource>>) -> &mut Self {
        self.cinder = cinder.into(); self
    }

    pub  fn cinder(&mut self) -> &mut crate::api::core::v1::CinderPersistentVolumeSource {
        if self.cinder.is_none() { self.cinder = Some(Default::default()) }
        self.cinder.as_mut().unwrap()
    }

    /// Modify [`Self::cinder`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn cinder_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::CinderPersistentVolumeSource)) -> &mut Self {
        if self.cinder.is_none() { self.cinder = Some(Default::default()) };
        func(self.cinder.as_mut().unwrap()); self
    }


    /// Set [`Self::claim_ref`]
    pub  fn claim_ref_set(&mut self, claim_ref: impl Into<Option<crate::api::core::v1::ObjectReference>>) -> &mut Self {
        self.claim_ref = claim_ref.into(); self
    }

    pub  fn claim_ref(&mut self) -> &mut crate::api::core::v1::ObjectReference {
        if self.claim_ref.is_none() { self.claim_ref = Some(Default::default()) }
        self.claim_ref.as_mut().unwrap()
    }

    /// Modify [`Self::claim_ref`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn claim_ref_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ObjectReference)) -> &mut Self {
        if self.claim_ref.is_none() { self.claim_ref = Some(Default::default()) };
        func(self.claim_ref.as_mut().unwrap()); self
    }


    /// Set [`Self::csi`]
    pub  fn csi_set(&mut self, csi: impl Into<Option<crate::api::core::v1::CSIPersistentVolumeSource>>) -> &mut Self {
        self.csi = csi.into(); self
    }

    pub  fn csi(&mut self) -> &mut crate::api::core::v1::CSIPersistentVolumeSource {
        if self.csi.is_none() { self.csi = Some(Default::default()) }
        self.csi.as_mut().unwrap()
    }

    /// Modify [`Self::csi`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn csi_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::CSIPersistentVolumeSource)) -> &mut Self {
        if self.csi.is_none() { self.csi = Some(Default::default()) };
        func(self.csi.as_mut().unwrap()); self
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
    pub  fn flex_volume_set(&mut self, flex_volume: impl Into<Option<crate::api::core::v1::FlexPersistentVolumeSource>>) -> &mut Self {
        self.flex_volume = flex_volume.into(); self
    }

    pub  fn flex_volume(&mut self) -> &mut crate::api::core::v1::FlexPersistentVolumeSource {
        if self.flex_volume.is_none() { self.flex_volume = Some(Default::default()) }
        self.flex_volume.as_mut().unwrap()
    }

    /// Modify [`Self::flex_volume`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn flex_volume_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::FlexPersistentVolumeSource)) -> &mut Self {
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


    /// Set [`Self::glusterfs`]
    pub  fn glusterfs_set(&mut self, glusterfs: impl Into<Option<crate::api::core::v1::GlusterfsPersistentVolumeSource>>) -> &mut Self {
        self.glusterfs = glusterfs.into(); self
    }

    pub  fn glusterfs(&mut self) -> &mut crate::api::core::v1::GlusterfsPersistentVolumeSource {
        if self.glusterfs.is_none() { self.glusterfs = Some(Default::default()) }
        self.glusterfs.as_mut().unwrap()
    }

    /// Modify [`Self::glusterfs`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn glusterfs_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::GlusterfsPersistentVolumeSource)) -> &mut Self {
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
    pub  fn iscsi_set(&mut self, iscsi: impl Into<Option<crate::api::core::v1::ISCSIPersistentVolumeSource>>) -> &mut Self {
        self.iscsi = iscsi.into(); self
    }

    pub  fn iscsi(&mut self) -> &mut crate::api::core::v1::ISCSIPersistentVolumeSource {
        if self.iscsi.is_none() { self.iscsi = Some(Default::default()) }
        self.iscsi.as_mut().unwrap()
    }

    /// Modify [`Self::iscsi`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn iscsi_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ISCSIPersistentVolumeSource)) -> &mut Self {
        if self.iscsi.is_none() { self.iscsi = Some(Default::default()) };
        func(self.iscsi.as_mut().unwrap()); self
    }


    /// Set [`Self::local`]
    pub  fn local_set(&mut self, local: impl Into<Option<crate::api::core::v1::LocalVolumeSource>>) -> &mut Self {
        self.local = local.into(); self
    }

    pub  fn local(&mut self) -> &mut crate::api::core::v1::LocalVolumeSource {
        if self.local.is_none() { self.local = Some(Default::default()) }
        self.local.as_mut().unwrap()
    }

    /// Modify [`Self::local`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn local_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::LocalVolumeSource)) -> &mut Self {
        if self.local.is_none() { self.local = Some(Default::default()) };
        func(self.local.as_mut().unwrap()); self
    }


    /// Set [`Self::mount_options`]
    pub  fn mount_options_set(&mut self, mount_options: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.mount_options = mount_options.into(); self
    }

    pub  fn mount_options(&mut self) -> &mut Vec<String> {
        if self.mount_options.is_none() { self.mount_options = Some(Default::default()) }
        self.mount_options.as_mut().unwrap()
    }

    /// Modify [`Self::mount_options`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn mount_options_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.mount_options.is_none() { self.mount_options = Some(Default::default()) };
        func(self.mount_options.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::mount_options`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn mount_options_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.mount_options.is_none() {
            self.mount_options = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.mount_options.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::mount_options`]
    pub  fn mount_options_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.mount_options.is_none() { self.mount_options = Some(Vec::new()); }
         let mount_options = &mut self.mount_options.as_mut().unwrap();
         for item in other.borrow() {
             mount_options.push(item.to_owned());
         }
         self
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


    /// Set [`Self::node_affinity`]
    pub  fn node_affinity_set(&mut self, node_affinity: impl Into<Option<crate::api::core::v1::VolumeNodeAffinity>>) -> &mut Self {
        self.node_affinity = node_affinity.into(); self
    }

    pub  fn node_affinity(&mut self) -> &mut crate::api::core::v1::VolumeNodeAffinity {
        if self.node_affinity.is_none() { self.node_affinity = Some(Default::default()) }
        self.node_affinity.as_mut().unwrap()
    }

    /// Modify [`Self::node_affinity`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn node_affinity_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::VolumeNodeAffinity)) -> &mut Self {
        if self.node_affinity.is_none() { self.node_affinity = Some(Default::default()) };
        func(self.node_affinity.as_mut().unwrap()); self
    }


    /// Set [`Self::persistent_volume_reclaim_policy`]
    pub  fn persistent_volume_reclaim_policy_set(&mut self, persistent_volume_reclaim_policy: impl Into<Option<String>>) -> &mut Self {
        self.persistent_volume_reclaim_policy = persistent_volume_reclaim_policy.into(); self
    }

    pub  fn persistent_volume_reclaim_policy(&mut self) -> &mut String {
        if self.persistent_volume_reclaim_policy.is_none() { self.persistent_volume_reclaim_policy = Some(Default::default()) }
        self.persistent_volume_reclaim_policy.as_mut().unwrap()
    }

    /// Modify [`Self::persistent_volume_reclaim_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn persistent_volume_reclaim_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.persistent_volume_reclaim_policy.is_none() { self.persistent_volume_reclaim_policy = Some(Default::default()) };
        func(self.persistent_volume_reclaim_policy.as_mut().unwrap()); self
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
    pub  fn rbd_set(&mut self, rbd: impl Into<Option<crate::api::core::v1::RBDPersistentVolumeSource>>) -> &mut Self {
        self.rbd = rbd.into(); self
    }

    pub  fn rbd(&mut self) -> &mut crate::api::core::v1::RBDPersistentVolumeSource {
        if self.rbd.is_none() { self.rbd = Some(Default::default()) }
        self.rbd.as_mut().unwrap()
    }

    /// Modify [`Self::rbd`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn rbd_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::RBDPersistentVolumeSource)) -> &mut Self {
        if self.rbd.is_none() { self.rbd = Some(Default::default()) };
        func(self.rbd.as_mut().unwrap()); self
    }


    /// Set [`Self::scale_io`]
    pub  fn scale_io_set(&mut self, scale_io: impl Into<Option<crate::api::core::v1::ScaleIOPersistentVolumeSource>>) -> &mut Self {
        self.scale_io = scale_io.into(); self
    }

    pub  fn scale_io(&mut self) -> &mut crate::api::core::v1::ScaleIOPersistentVolumeSource {
        if self.scale_io.is_none() { self.scale_io = Some(Default::default()) }
        self.scale_io.as_mut().unwrap()
    }

    /// Modify [`Self::scale_io`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn scale_io_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ScaleIOPersistentVolumeSource)) -> &mut Self {
        if self.scale_io.is_none() { self.scale_io = Some(Default::default()) };
        func(self.scale_io.as_mut().unwrap()); self
    }


    /// Set [`Self::storage_class_name`]
    pub  fn storage_class_name_set(&mut self, storage_class_name: impl Into<Option<String>>) -> &mut Self {
        self.storage_class_name = storage_class_name.into(); self
    }

    pub  fn storage_class_name(&mut self) -> &mut String {
        if self.storage_class_name.is_none() { self.storage_class_name = Some(Default::default()) }
        self.storage_class_name.as_mut().unwrap()
    }

    /// Modify [`Self::storage_class_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn storage_class_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.storage_class_name.is_none() { self.storage_class_name = Some(Default::default()) };
        func(self.storage_class_name.as_mut().unwrap()); self
    }


    /// Set [`Self::storageos`]
    pub  fn storageos_set(&mut self, storageos: impl Into<Option<crate::api::core::v1::StorageOSPersistentVolumeSource>>) -> &mut Self {
        self.storageos = storageos.into(); self
    }

    pub  fn storageos(&mut self) -> &mut crate::api::core::v1::StorageOSPersistentVolumeSource {
        if self.storageos.is_none() { self.storageos = Some(Default::default()) }
        self.storageos.as_mut().unwrap()
    }

    /// Modify [`Self::storageos`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn storageos_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::StorageOSPersistentVolumeSource)) -> &mut Self {
        if self.storageos.is_none() { self.storageos = Some(Default::default()) };
        func(self.storageos.as_mut().unwrap()); self
    }


    /// Set [`Self::volume_mode`]
    pub  fn volume_mode_set(&mut self, volume_mode: impl Into<Option<String>>) -> &mut Self {
        self.volume_mode = volume_mode.into(); self
    }

    pub  fn volume_mode(&mut self) -> &mut String {
        if self.volume_mode.is_none() { self.volume_mode = Some(Default::default()) }
        self.volume_mode.as_mut().unwrap()
    }

    /// Modify [`Self::volume_mode`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn volume_mode_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.volume_mode.is_none() { self.volume_mode = Some(Default::default()) };
        func(self.volume_mode.as_mut().unwrap()); self
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
                                description: Some("AccessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes".to_owned()),
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
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AzureFilePersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AzureFile represents an Azure File Service mount on the host and bind mount to the pod.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "capacity".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity".to_owned()),
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
                                description: Some("CephFS represents a Ceph FS mount on the host that shares a pod's lifetime".to_owned()),
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
                                description: Some("Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md".to_owned()),
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
                                description: Some("ClaimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding".to_owned()),
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
                                description: Some("CSI represents storage that is handled by an external CSI driver (Beta feature).".to_owned()),
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
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::FlexPersistentVolumeSource>().into_object();
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
                                description: Some("Flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running".to_owned()),
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
                                description: Some("GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk".to_owned()),
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
                                description: Some("Glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. More info: https://examples.k8s.io/volumes/glusterfs/README.md".to_owned()),
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
                                description: Some("HostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath".to_owned()),
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
                                description: Some("ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.".to_owned()),
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
                                description: Some("Local represents directly-attached storage with node affinity".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "mountOptions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("A list of mount options, e.g. [\"ro\", \"soft\"]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options".to_owned()),
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
                                description: Some("NFS represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs".to_owned()),
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
                                description: Some("NodeAffinity defines constraints that limit what nodes this volume can be accessed from. This field influences the scheduling of pods that use this volume.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "persistentVolumeReclaimPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("What happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming".to_owned()),
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
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::RBDPersistentVolumeSource>().into_object();
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
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ScaleIOPersistentVolumeSource>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "storageClassName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.".to_owned()),
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
                                description: Some("StorageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod More info: https://examples.k8s.io/volumes/storageos/README.md".to_owned()),
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
                                description: Some("VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine".to_owned()),
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
