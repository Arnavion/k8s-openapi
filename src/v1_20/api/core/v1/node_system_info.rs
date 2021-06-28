// Generated from definition io.k8s.api.core.v1.NodeSystemInfo

/// NodeSystemInfo is a set of ids/uuids to uniquely identify the node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSystemInfo {
    /// The Architecture reported by the node
    pub architecture: String,

    /// Boot ID reported by the node.
    pub boot_id: String,

    /// ContainerRuntime Version reported by the node through runtime remote API (e.g. docker://1.5.0).
    pub container_runtime_version: String,

    /// Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).
    pub kernel_version: String,

    /// KubeProxy Version reported by the node.
    pub kube_proxy_version: String,

    /// Kubelet Version reported by the node.
    pub kubelet_version: String,

    /// MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: http://man7.org/linux/man-pages/man5/machine-id.5.html
    pub machine_id: String,

    /// The Operating System reported by the node
    pub operating_system: String,

    /// OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).
    pub os_image: String,

    /// SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid
    pub system_uuid: String,
}

impl<'de> crate::serde::Deserialize<'de> for NodeSystemInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_architecture,
            Key_boot_id,
            Key_container_runtime_version,
            Key_kernel_version,
            Key_kube_proxy_version,
            Key_kubelet_version,
            Key_machine_id,
            Key_operating_system,
            Key_os_image,
            Key_system_uuid,
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
                            "architecture" => Field::Key_architecture,
                            "bootID" => Field::Key_boot_id,
                            "containerRuntimeVersion" => Field::Key_container_runtime_version,
                            "kernelVersion" => Field::Key_kernel_version,
                            "kubeProxyVersion" => Field::Key_kube_proxy_version,
                            "kubeletVersion" => Field::Key_kubelet_version,
                            "machineID" => Field::Key_machine_id,
                            "operatingSystem" => Field::Key_operating_system,
                            "osImage" => Field::Key_os_image,
                            "systemUUID" => Field::Key_system_uuid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeSystemInfo;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NodeSystemInfo")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_architecture: Option<String> = None;
                let mut value_boot_id: Option<String> = None;
                let mut value_container_runtime_version: Option<String> = None;
                let mut value_kernel_version: Option<String> = None;
                let mut value_kube_proxy_version: Option<String> = None;
                let mut value_kubelet_version: Option<String> = None;
                let mut value_machine_id: Option<String> = None;
                let mut value_operating_system: Option<String> = None;
                let mut value_os_image: Option<String> = None;
                let mut value_system_uuid: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_architecture => value_architecture = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_boot_id => value_boot_id = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_container_runtime_version => value_container_runtime_version = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_kernel_version => value_kernel_version = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_kube_proxy_version => value_kube_proxy_version = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_kubelet_version => value_kubelet_version = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_machine_id => value_machine_id = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_operating_system => value_operating_system = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_os_image => value_os_image = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_system_uuid => value_system_uuid = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSystemInfo {
                    architecture: value_architecture.ok_or_else(|| crate::serde::de::Error::missing_field("architecture"))?,
                    boot_id: value_boot_id.ok_or_else(|| crate::serde::de::Error::missing_field("bootID"))?,
                    container_runtime_version: value_container_runtime_version.ok_or_else(|| crate::serde::de::Error::missing_field("containerRuntimeVersion"))?,
                    kernel_version: value_kernel_version.ok_or_else(|| crate::serde::de::Error::missing_field("kernelVersion"))?,
                    kube_proxy_version: value_kube_proxy_version.ok_or_else(|| crate::serde::de::Error::missing_field("kubeProxyVersion"))?,
                    kubelet_version: value_kubelet_version.ok_or_else(|| crate::serde::de::Error::missing_field("kubeletVersion"))?,
                    machine_id: value_machine_id.ok_or_else(|| crate::serde::de::Error::missing_field("machineID"))?,
                    operating_system: value_operating_system.ok_or_else(|| crate::serde::de::Error::missing_field("operatingSystem"))?,
                    os_image: value_os_image.ok_or_else(|| crate::serde::de::Error::missing_field("osImage"))?,
                    system_uuid: value_system_uuid.ok_or_else(|| crate::serde::de::Error::missing_field("systemUUID"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeSystemInfo",
            &[
                "architecture",
                "bootID",
                "containerRuntimeVersion",
                "kernelVersion",
                "kubeProxyVersion",
                "kubeletVersion",
                "machineID",
                "operatingSystem",
                "osImage",
                "systemUUID",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeSystemInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeSystemInfo",
            10,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "architecture", &self.architecture)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bootID", &self.boot_id)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerRuntimeVersion", &self.container_runtime_version)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kernelVersion", &self.kernel_version)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kubeProxyVersion", &self.kube_proxy_version)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kubeletVersion", &self.kubelet_version)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "machineID", &self.machine_id)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "operatingSystem", &self.operating_system)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "osImage", &self.os_image)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "systemUUID", &self.system_uuid)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for NodeSystemInfo {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "NodeSystemInfo is a set of ids/uuids to uniquely identify the node.",
          "properties": {
            "architecture": {
              "description": "The Architecture reported by the node",
              "type": "string"
            },
            "bootID": {
              "description": "Boot ID reported by the node.",
              "type": "string"
            },
            "containerRuntimeVersion": {
              "description": "ContainerRuntime Version reported by the node through runtime remote API (e.g. docker://1.5.0).",
              "type": "string"
            },
            "kernelVersion": {
              "description": "Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).",
              "type": "string"
            },
            "kubeProxyVersion": {
              "description": "KubeProxy Version reported by the node.",
              "type": "string"
            },
            "kubeletVersion": {
              "description": "Kubelet Version reported by the node.",
              "type": "string"
            },
            "machineID": {
              "description": "MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: http://man7.org/linux/man-pages/man5/machine-id.5.html",
              "type": "string"
            },
            "operatingSystem": {
              "description": "The Operating System reported by the node",
              "type": "string"
            },
            "osImage": {
              "description": "OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).",
              "type": "string"
            },
            "systemUUID": {
              "description": "SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid",
              "type": "string"
            }
          },
          "required": [
            "architecture",
            "bootID",
            "containerRuntimeVersion",
            "kernelVersion",
            "kubeProxyVersion",
            "kubeletVersion",
            "machineID",
            "operatingSystem",
            "osImage",
            "systemUUID"
          ],
          "type": "object"
        })
    }
}
