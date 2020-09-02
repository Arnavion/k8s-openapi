// Generated from definition io.k8s.api.core.v1.EphemeralContainer

/// An EphemeralContainer is a container that may be added temporarily to an existing pod for user-initiated activities such as debugging. Ephemeral containers have no resource or scheduling guarantees, and they will not be restarted when they exit or when a pod is removed or restarted. If an ephemeral container causes a pod to exceed its resource allocation, the pod may be evicted. Ephemeral containers may not be added by directly updating the pod spec. They must be added via the pod's ephemeralcontainers subresource, and they will appear in the pod spec once added. This is an alpha feature enabled by the EphemeralContainers feature flag.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EphemeralContainer {
    /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    pub args: Option<Vec<String>>,

    /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    pub command: Option<Vec<String>>,

    /// List of environment variables to set in the container. Cannot be updated.
    pub env: Option<Vec<crate::api::core::v1::EnvVar>>,

    /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
    pub env_from: Option<Vec<crate::api::core::v1::EnvFromSource>>,

    /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images
    pub image: Option<String>,

    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    pub image_pull_policy: Option<String>,

    /// Lifecycle is not allowed for ephemeral containers.
    pub lifecycle: Option<crate::api::core::v1::Lifecycle>,

    /// Probes are not allowed for ephemeral containers.
    pub liveness_probe: Option<crate::api::core::v1::Probe>,

    /// Name of the ephemeral container specified as a DNS_LABEL. This name must be unique among all containers, init containers and ephemeral containers.
    pub name: String,

    /// Ports are not allowed for ephemeral containers.
    pub ports: Option<Vec<crate::api::core::v1::ContainerPort>>,

    /// Probes are not allowed for ephemeral containers.
    pub readiness_probe: Option<crate::api::core::v1::Probe>,

    /// Resources are not allowed for ephemeral containers. Ephemeral containers use spare resources already allocated to the pod.
    pub resources: Option<crate::api::core::v1::ResourceRequirements>,

    /// SecurityContext is not allowed for ephemeral containers.
    pub security_context: Option<crate::api::core::v1::SecurityContext>,

    /// Probes are not allowed for ephemeral containers.
    pub startup_probe: Option<crate::api::core::v1::Probe>,

    /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
    pub stdin: Option<bool>,

    /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
    pub stdin_once: Option<bool>,

    /// If set, the name of the container from PodSpec that this ephemeral container targets. The ephemeral container will be run in the namespaces (IPC, PID, etc) of this container. If not set then the ephemeral container is run in whatever namespaces are shared for the pod. Note that the container runtime must support this feature.
    pub target_container_name: Option<String>,

    /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    pub termination_message_path: Option<String>,

    /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
    pub termination_message_policy: Option<String>,

    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
    pub tty: Option<bool>,

    /// volumeDevices is the list of block devices to be used by the container.
    pub volume_devices: Option<Vec<crate::api::core::v1::VolumeDevice>>,

    /// Pod volumes to mount into the container's filesystem. Cannot be updated.
    pub volume_mounts: Option<Vec<crate::api::core::v1::VolumeMount>>,

    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
    pub working_dir: Option<String>,
}

impl<'de> serde::Deserialize<'de> for EphemeralContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_args,
            Key_command,
            Key_env,
            Key_env_from,
            Key_image,
            Key_image_pull_policy,
            Key_lifecycle,
            Key_liveness_probe,
            Key_name,
            Key_ports,
            Key_readiness_probe,
            Key_resources,
            Key_security_context,
            Key_startup_probe,
            Key_stdin,
            Key_stdin_once,
            Key_target_container_name,
            Key_termination_message_path,
            Key_termination_message_policy,
            Key_tty,
            Key_volume_devices,
            Key_volume_mounts,
            Key_working_dir,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "args" => Field::Key_args,
                            "command" => Field::Key_command,
                            "env" => Field::Key_env,
                            "envFrom" => Field::Key_env_from,
                            "image" => Field::Key_image,
                            "imagePullPolicy" => Field::Key_image_pull_policy,
                            "lifecycle" => Field::Key_lifecycle,
                            "livenessProbe" => Field::Key_liveness_probe,
                            "name" => Field::Key_name,
                            "ports" => Field::Key_ports,
                            "readinessProbe" => Field::Key_readiness_probe,
                            "resources" => Field::Key_resources,
                            "securityContext" => Field::Key_security_context,
                            "startupProbe" => Field::Key_startup_probe,
                            "stdin" => Field::Key_stdin,
                            "stdinOnce" => Field::Key_stdin_once,
                            "targetContainerName" => Field::Key_target_container_name,
                            "terminationMessagePath" => Field::Key_termination_message_path,
                            "terminationMessagePolicy" => Field::Key_termination_message_policy,
                            "tty" => Field::Key_tty,
                            "volumeDevices" => Field::Key_volume_devices,
                            "volumeMounts" => Field::Key_volume_mounts,
                            "workingDir" => Field::Key_working_dir,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EphemeralContainer;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EphemeralContainer")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_args: Option<Vec<String>> = None;
                let mut value_command: Option<Vec<String>> = None;
                let mut value_env: Option<Vec<crate::api::core::v1::EnvVar>> = None;
                let mut value_env_from: Option<Vec<crate::api::core::v1::EnvFromSource>> = None;
                let mut value_image: Option<String> = None;
                let mut value_image_pull_policy: Option<String> = None;
                let mut value_lifecycle: Option<crate::api::core::v1::Lifecycle> = None;
                let mut value_liveness_probe: Option<crate::api::core::v1::Probe> = None;
                let mut value_name: Option<String> = None;
                let mut value_ports: Option<Vec<crate::api::core::v1::ContainerPort>> = None;
                let mut value_readiness_probe: Option<crate::api::core::v1::Probe> = None;
                let mut value_resources: Option<crate::api::core::v1::ResourceRequirements> = None;
                let mut value_security_context: Option<crate::api::core::v1::SecurityContext> = None;
                let mut value_startup_probe: Option<crate::api::core::v1::Probe> = None;
                let mut value_stdin: Option<bool> = None;
                let mut value_stdin_once: Option<bool> = None;
                let mut value_target_container_name: Option<String> = None;
                let mut value_termination_message_path: Option<String> = None;
                let mut value_termination_message_policy: Option<String> = None;
                let mut value_tty: Option<bool> = None;
                let mut value_volume_devices: Option<Vec<crate::api::core::v1::VolumeDevice>> = None;
                let mut value_volume_mounts: Option<Vec<crate::api::core::v1::VolumeMount>> = None;
                let mut value_working_dir: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_args => value_args = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_command => value_command = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env => value_env = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env_from => value_env_from = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image => value_image = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_pull_policy => value_image_pull_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lifecycle => value_lifecycle = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_liveness_probe => value_liveness_probe = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_ports => value_ports = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_readiness_probe => value_readiness_probe = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_security_context => value_security_context = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_startup_probe => value_startup_probe = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stdin => value_stdin = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stdin_once => value_stdin_once = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_target_container_name => value_target_container_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination_message_path => value_termination_message_path = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination_message_policy => value_termination_message_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tty => value_tty = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_devices => value_volume_devices = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mounts => value_volume_mounts = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_working_dir => value_working_dir = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EphemeralContainer {
                    args: value_args,
                    command: value_command,
                    env: value_env,
                    env_from: value_env_from,
                    image: value_image,
                    image_pull_policy: value_image_pull_policy,
                    lifecycle: value_lifecycle,
                    liveness_probe: value_liveness_probe,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    ports: value_ports,
                    readiness_probe: value_readiness_probe,
                    resources: value_resources,
                    security_context: value_security_context,
                    startup_probe: value_startup_probe,
                    stdin: value_stdin,
                    stdin_once: value_stdin_once,
                    target_container_name: value_target_container_name,
                    termination_message_path: value_termination_message_path,
                    termination_message_policy: value_termination_message_policy,
                    tty: value_tty,
                    volume_devices: value_volume_devices,
                    volume_mounts: value_volume_mounts,
                    working_dir: value_working_dir,
                })
            }
        }

        deserializer.deserialize_struct(
            "EphemeralContainer",
            &[
                "args",
                "command",
                "env",
                "envFrom",
                "image",
                "imagePullPolicy",
                "lifecycle",
                "livenessProbe",
                "name",
                "ports",
                "readinessProbe",
                "resources",
                "securityContext",
                "startupProbe",
                "stdin",
                "stdinOnce",
                "targetContainerName",
                "terminationMessagePath",
                "terminationMessagePolicy",
                "tty",
                "volumeDevices",
                "volumeMounts",
                "workingDir",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for EphemeralContainer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EphemeralContainer",
            1 +
            self.args.as_ref().map_or(0, |_| 1) +
            self.command.as_ref().map_or(0, |_| 1) +
            self.env.as_ref().map_or(0, |_| 1) +
            self.env_from.as_ref().map_or(0, |_| 1) +
            self.image.as_ref().map_or(0, |_| 1) +
            self.image_pull_policy.as_ref().map_or(0, |_| 1) +
            self.lifecycle.as_ref().map_or(0, |_| 1) +
            self.liveness_probe.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1) +
            self.readiness_probe.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.security_context.as_ref().map_or(0, |_| 1) +
            self.startup_probe.as_ref().map_or(0, |_| 1) +
            self.stdin.as_ref().map_or(0, |_| 1) +
            self.stdin_once.as_ref().map_or(0, |_| 1) +
            self.target_container_name.as_ref().map_or(0, |_| 1) +
            self.termination_message_path.as_ref().map_or(0, |_| 1) +
            self.termination_message_policy.as_ref().map_or(0, |_| 1) +
            self.tty.as_ref().map_or(0, |_| 1) +
            self.volume_devices.as_ref().map_or(0, |_| 1) +
            self.volume_mounts.as_ref().map_or(0, |_| 1) +
            self.working_dir.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.args {
            serde::ser::SerializeStruct::serialize_field(&mut state, "args", value)?;
        }
        if let Some(value) = &self.command {
            serde::ser::SerializeStruct::serialize_field(&mut state, "command", value)?;
        }
        if let Some(value) = &self.env {
            serde::ser::SerializeStruct::serialize_field(&mut state, "env", value)?;
        }
        if let Some(value) = &self.env_from {
            serde::ser::SerializeStruct::serialize_field(&mut state, "envFrom", value)?;
        }
        if let Some(value) = &self.image {
            serde::ser::SerializeStruct::serialize_field(&mut state, "image", value)?;
        }
        if let Some(value) = &self.image_pull_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "imagePullPolicy", value)?;
        }
        if let Some(value) = &self.lifecycle {
            serde::ser::SerializeStruct::serialize_field(&mut state, "lifecycle", value)?;
        }
        if let Some(value) = &self.liveness_probe {
            serde::ser::SerializeStruct::serialize_field(&mut state, "livenessProbe", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.ports {
            serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        if let Some(value) = &self.readiness_probe {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readinessProbe", value)?;
        }
        if let Some(value) = &self.resources {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        if let Some(value) = &self.security_context {
            serde::ser::SerializeStruct::serialize_field(&mut state, "securityContext", value)?;
        }
        if let Some(value) = &self.startup_probe {
            serde::ser::SerializeStruct::serialize_field(&mut state, "startupProbe", value)?;
        }
        if let Some(value) = &self.stdin {
            serde::ser::SerializeStruct::serialize_field(&mut state, "stdin", value)?;
        }
        if let Some(value) = &self.stdin_once {
            serde::ser::SerializeStruct::serialize_field(&mut state, "stdinOnce", value)?;
        }
        if let Some(value) = &self.target_container_name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "targetContainerName", value)?;
        }
        if let Some(value) = &self.termination_message_path {
            serde::ser::SerializeStruct::serialize_field(&mut state, "terminationMessagePath", value)?;
        }
        if let Some(value) = &self.termination_message_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "terminationMessagePolicy", value)?;
        }
        if let Some(value) = &self.tty {
            serde::ser::SerializeStruct::serialize_field(&mut state, "tty", value)?;
        }
        if let Some(value) = &self.volume_devices {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumeDevices", value)?;
        }
        if let Some(value) = &self.volume_mounts {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumeMounts", value)?;
        }
        if let Some(value) = &self.working_dir {
            serde::ser::SerializeStruct::serialize_field(&mut state, "workingDir", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
