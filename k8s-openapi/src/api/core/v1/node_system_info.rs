// Generated from definition io.k8s.api.core.v1.NodeSystemInfo

/// NodeSystemInfo is a set of ids/uuids to uniquely identify the node.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NodeSystemInfo {
    /// The Architecture reported by the node
    pub architecture: String,

    /// Boot ID reported by the node.
    #[serde(rename = "bootID")]
    pub boot_id: String,

    /// ContainerRuntime Version reported by the node through runtime remote API (e.g. docker://1.5.0).
    #[serde(rename = "containerRuntimeVersion")]
    pub container_runtime_version: String,

    /// Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).
    #[serde(rename = "kernelVersion")]
    pub kernel_version: String,

    /// KubeProxy Version reported by the node.
    #[serde(rename = "kubeProxyVersion")]
    pub kube_proxy_version: String,

    /// Kubelet Version reported by the node.
    #[serde(rename = "kubeletVersion")]
    pub kubelet_version: String,

    /// MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: http://man7.org/linux/man-pages/man5/machine-id.5.html
    #[serde(rename = "machineID")]
    pub machine_id: String,

    /// The Operating System reported by the node
    #[serde(rename = "operatingSystem")]
    pub operating_system: String,

    /// OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).
    #[serde(rename = "osImage")]
    pub os_image: String,

    /// SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts https://access.redhat.com/documentation/en-US/Red_Hat_Subscription_Management/1/html/RHSM/getting-system-uuid.html
    #[serde(rename = "systemUUID")]
    pub system_uuid: String,
}
