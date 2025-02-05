// Generated from definition io.k8s.api.core.v1.Volume

/// Volume represents a named volume in a pod that may be accessed by any container in the pod.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Volume {
    /// awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub aws_elastic_block_store: Option<crate::api::core::v1::AWSElasticBlockStoreVolumeSource>,

    /// azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    pub azure_disk: Option<crate::api::core::v1::AzureDiskVolumeSource>,

    /// azureFile represents an Azure File Service mount on the host and bind mount to the pod.
    pub azure_file: Option<crate::api::core::v1::AzureFileVolumeSource>,

    /// cephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    pub cephfs: Option<crate::api::core::v1::CephFSVolumeSource>,

    /// cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    pub cinder: Option<crate::api::core::v1::CinderVolumeSource>,

    /// configMap represents a configMap that should populate this volume
    pub config_map: Option<crate::api::core::v1::ConfigMapVolumeSource>,

    /// csi (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
    pub csi: Option<crate::api::core::v1::CSIVolumeSource>,

    /// downwardAPI represents downward API about the pod that should populate this volume
    pub downward_api: Option<crate::api::core::v1::DownwardAPIVolumeSource>,

    /// emptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    pub empty_dir: Option<crate::api::core::v1::EmptyDirVolumeSource>,

    /// ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed.
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
    pub ephemeral: Option<crate::api::core::v1::EphemeralVolumeSource>,

    /// fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    pub fc: Option<crate::api::core::v1::FCVolumeSource>,

    /// flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
    pub flex_volume: Option<crate::api::core::v1::FlexVolumeSource>,

    /// flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
    pub flocker: Option<crate::api::core::v1::FlockerVolumeSource>,

    /// gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub gce_persistent_disk: Option<crate::api::core::v1::GCEPersistentDiskVolumeSource>,

    /// gitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
    pub git_repo: Option<crate::api::core::v1::GitRepoVolumeSource>,

    /// glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
    pub glusterfs: Option<crate::api::core::v1::GlusterfsVolumeSource>,

    /// hostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    pub host_path: Option<crate::api::core::v1::HostPathVolumeSource>,

    /// image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine. The volume is resolved at pod startup depending on which PullPolicy value is provided:
    ///
    /// - Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails. - Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present. - IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails.
    ///
    /// The volume gets re-resolved if the pod gets deleted and recreated, which means that new remote content will become available on pod recreation. A failure to resolve or pull the image during pod startup will block containers from starting and may add significant latency. Failures will be retried using normal volume backoff and will be reported on the pod reason and message. The types of objects that may be mounted by this volume are defined by the container runtime implementation on a host machine and at minimum must include all valid types supported by the container image field. The OCI object gets mounted in a single directory (spec.containers\[*\].volumeMounts.mountPath) by merging the manifest layers in the same way as for container images. The volume will be mounted read-only (ro) and non-executable files (noexec). Sub path mounts for containers are not supported (spec.containers\[*\].volumeMounts.subpath). The field spec.securityContext.fsGroupChangePolicy has no effect on this volume type.
    pub image: Option<crate::api::core::v1::ImageVolumeSource>,

    /// iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
    pub iscsi: Option<crate::api::core::v1::ISCSIVolumeSource>,

    /// name of the volume. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: std::string::String,

    /// nfs represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub nfs: Option<crate::api::core::v1::NFSVolumeSource>,

    /// persistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    pub persistent_volume_claim: Option<crate::api::core::v1::PersistentVolumeClaimVolumeSource>,

    /// photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    pub photon_persistent_disk: Option<crate::api::core::v1::PhotonPersistentDiskVolumeSource>,

    /// portworxVolume represents a portworx volume attached and mounted on kubelets host machine
    pub portworx_volume: Option<crate::api::core::v1::PortworxVolumeSource>,

    /// projected items for all in one resources secrets, configmaps, and downward API
    pub projected: Option<crate::api::core::v1::ProjectedVolumeSource>,

    /// quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    pub quobyte: Option<crate::api::core::v1::QuobyteVolumeSource>,

    /// rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
    pub rbd: Option<crate::api::core::v1::RBDVolumeSource>,

    /// scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    pub scale_io: Option<crate::api::core::v1::ScaleIOVolumeSource>,

    /// secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    pub secret: Option<crate::api::core::v1::SecretVolumeSource>,

    /// storageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
    pub storageos: Option<crate::api::core::v1::StorageOSVolumeSource>,

    /// vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    pub vsphere_volume: Option<crate::api::core::v1::VsphereVirtualDiskVolumeSource>,
}

impl crate::DeepMerge for Volume {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.aws_elastic_block_store, other.aws_elastic_block_store);
        crate::DeepMerge::merge_from(&mut self.azure_disk, other.azure_disk);
        crate::DeepMerge::merge_from(&mut self.azure_file, other.azure_file);
        crate::DeepMerge::merge_from(&mut self.cephfs, other.cephfs);
        crate::DeepMerge::merge_from(&mut self.cinder, other.cinder);
        crate::DeepMerge::merge_from(&mut self.config_map, other.config_map);
        crate::DeepMerge::merge_from(&mut self.csi, other.csi);
        crate::DeepMerge::merge_from(&mut self.downward_api, other.downward_api);
        crate::DeepMerge::merge_from(&mut self.empty_dir, other.empty_dir);
        crate::DeepMerge::merge_from(&mut self.ephemeral, other.ephemeral);
        crate::DeepMerge::merge_from(&mut self.fc, other.fc);
        crate::DeepMerge::merge_from(&mut self.flex_volume, other.flex_volume);
        crate::DeepMerge::merge_from(&mut self.flocker, other.flocker);
        crate::DeepMerge::merge_from(&mut self.gce_persistent_disk, other.gce_persistent_disk);
        crate::DeepMerge::merge_from(&mut self.git_repo, other.git_repo);
        crate::DeepMerge::merge_from(&mut self.glusterfs, other.glusterfs);
        crate::DeepMerge::merge_from(&mut self.host_path, other.host_path);
        crate::DeepMerge::merge_from(&mut self.image, other.image);
        crate::DeepMerge::merge_from(&mut self.iscsi, other.iscsi);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.nfs, other.nfs);
        crate::DeepMerge::merge_from(&mut self.persistent_volume_claim, other.persistent_volume_claim);
        crate::DeepMerge::merge_from(&mut self.photon_persistent_disk, other.photon_persistent_disk);
        crate::DeepMerge::merge_from(&mut self.portworx_volume, other.portworx_volume);
        crate::DeepMerge::merge_from(&mut self.projected, other.projected);
        crate::DeepMerge::merge_from(&mut self.quobyte, other.quobyte);
        crate::DeepMerge::merge_from(&mut self.rbd, other.rbd);
        crate::DeepMerge::merge_from(&mut self.scale_io, other.scale_io);
        crate::DeepMerge::merge_from(&mut self.secret, other.secret);
        crate::DeepMerge::merge_from(&mut self.storageos, other.storageos);
        crate::DeepMerge::merge_from(&mut self.vsphere_volume, other.vsphere_volume);
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
            Key_image,
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
                            "image" => Field::Key_image,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
                let mut value_image: Option<crate::api::core::v1::ImageVolumeSource> = None;
                let mut value_iscsi: Option<crate::api::core::v1::ISCSIVolumeSource> = None;
                let mut value_name: Option<std::string::String> = None;
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
                        Field::Key_image => value_image = crate::serde::de::MapAccess::next_value(&mut map)?,
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
                    image: value_image,
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
                "image",
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
            self.image.as_ref().map_or(0, |_| 1) +
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
        if let Some(value) = &self.image {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "image", value)?;
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
    fn schema_name() -> std::string::String {
        "io.k8s.api.core.v1.Volume".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Volume represents a named volume in a pod that may be accessed by any container in the pod.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "awsElasticBlockStore".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AWSElasticBlockStoreVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "azureDisk".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AzureDiskVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "azureFile".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::AzureFileVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("azureFile represents an Azure File Service mount on the host and bind mount to the pod.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "cephfs".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::CephFSVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("cephFS represents a Ceph FS mount on the host that shares a pod's lifetime".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "cinder".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::CinderVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "configMap".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ConfigMapVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("configMap represents a configMap that should populate this volume".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "csi".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::CSIVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("csi (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "downwardAPI".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::DownwardAPIVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("downwardAPI represents downward API about the pod that should populate this volume".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "emptyDir".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::EmptyDirVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("emptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "ephemeral".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::EphemeralVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed.\n\nUse this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity\n   tracking are needed,\nc) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through\n   a PersistentVolumeClaim (see EphemeralVolumeSource for more\n   information on the connection between this volume type\n   and PersistentVolumeClaim).\n\nUse PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod.\n\nUse CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information.\n\nA pod can use both types of ephemeral volumes and persistent volumes at the same time.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "fc".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::FCVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "flexVolume".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::FlexVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "flocker".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::FlockerVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "gcePersistentDisk".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::GCEPersistentDiskVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "gitRepo".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::GitRepoVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("gitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "glusterfs".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::GlusterfsVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "hostPath".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::HostPathVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("hostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "image".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ImageVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine. The volume is resolved at pod startup depending on which PullPolicy value is provided:\n\n- Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails. - Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present. - IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails.\n\nThe volume gets re-resolved if the pod gets deleted and recreated, which means that new remote content will become available on pod recreation. A failure to resolve or pull the image during pod startup will block containers from starting and may add significant latency. Failures will be retried using normal volume backoff and will be reported on the pod reason and message. The types of objects that may be mounted by this volume are defined by the container runtime implementation on a host machine and at minimum must include all valid types supported by the container image field. The OCI object gets mounted in a single directory (spec.containers[*].volumeMounts.mountPath) by merging the manifest layers in the same way as for container images. The volume will be mounted read-only (ro) and non-executable files (noexec). Sub path mounts for containers are not supported (spec.containers[*].volumeMounts.subpath). The field spec.securityContext.fsGroupChangePolicy has no effect on this volume type.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "iscsi".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ISCSIVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("name of the volume. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "nfs".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::NFSVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("nfs represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "persistentVolumeClaim".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PersistentVolumeClaimVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("persistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "photonPersistentDisk".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PhotonPersistentDiskVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "portworxVolume".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PortworxVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("portworxVolume represents a portworx volume attached and mounted on kubelets host machine".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "projected".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ProjectedVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("projected items for all in one resources secrets, configmaps, and downward API".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "quobyte".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::QuobyteVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("quobyte represents a Quobyte mount on the host that shares a pod's lifetime".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "rbd".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::RBDVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "scaleIO".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::ScaleIOVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "secret".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SecretVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "storageos".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::StorageOSVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("storageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "vsphereVolume".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::VsphereVirtualDiskVolumeSource>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "name".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
