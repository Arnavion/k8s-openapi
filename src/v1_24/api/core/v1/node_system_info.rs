// Generated from definition io.k8s.api.core.v1.NodeSystemInfo

/// NodeSystemInfo is a set of ids/uuids to uniquely identify the node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSystemInfo {
    /// The Architecture reported by the node
    pub architecture: String,

    /// Boot ID reported by the node.
    pub boot_id: String,

    /// ContainerRuntime Version reported by the node through runtime remote API (e.g. containerd://1.4.2).
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
                        Field::Key_architecture => value_architecture = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_boot_id => value_boot_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_container_runtime_version => value_container_runtime_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kernel_version => value_kernel_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kube_proxy_version => value_kube_proxy_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kubelet_version => value_kubelet_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_machine_id => value_machine_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_operating_system => value_operating_system = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_os_image => value_os_image = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_system_uuid => value_system_uuid = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSystemInfo {
                    architecture: value_architecture.unwrap_or_default(),
                    boot_id: value_boot_id.unwrap_or_default(),
                    container_runtime_version: value_container_runtime_version.unwrap_or_default(),
                    kernel_version: value_kernel_version.unwrap_or_default(),
                    kube_proxy_version: value_kube_proxy_version.unwrap_or_default(),
                    kubelet_version: value_kubelet_version.unwrap_or_default(),
                    machine_id: value_machine_id.unwrap_or_default(),
                    operating_system: value_operating_system.unwrap_or_default(),
                    os_image: value_os_image.unwrap_or_default(),
                    system_uuid: value_system_uuid.unwrap_or_default(),
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

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for NodeSystemInfo {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.NodeSystemInfo".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("NodeSystemInfo is a set of ids/uuids to uniquely identify the node.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "architecture".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The Architecture reported by the node".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "bootID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Boot ID reported by the node.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "containerRuntimeVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ContainerRuntime Version reported by the node through runtime remote API (e.g. containerd://1.4.2).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kernelVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kubeProxyVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("KubeProxy Version reported by the node.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kubeletVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kubelet Version reported by the node.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "machineID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: http://man7.org/linux/man-pages/man5/machine-id.5.html".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "operatingSystem".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The Operating System reported by the node".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "osImage".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "systemUUID".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "architecture".to_owned(),
                    "bootID".to_owned(),
                    "containerRuntimeVersion".to_owned(),
                    "kernelVersion".to_owned(),
                    "kubeProxyVersion".to_owned(),
                    "kubeletVersion".to_owned(),
                    "machineID".to_owned(),
                    "operatingSystem".to_owned(),
                    "osImage".to_owned(),
                    "systemUUID".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
