// Generated from definition io.k8s.api.settings.v1alpha1.PodPresetSpec

/// PodPresetSpec is a description of a pod preset.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodPresetSpec {
    /// Env defines the collection of EnvVar to inject into containers.
    pub env: Option<Vec<::v1_12::api::core::v1::EnvVar>>,

    /// EnvFrom defines the collection of EnvFromSource to inject into containers.
    pub env_from: Option<Vec<::v1_12::api::core::v1::EnvFromSource>>,

    /// Selector is a label query over a set of resources, in this case pods. Required.
    pub selector: Option<::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// VolumeMounts defines the collection of VolumeMount to inject into containers.
    pub volume_mounts: Option<Vec<::v1_12::api::core::v1::VolumeMount>>,

    /// Volumes defines the collection of Volume to inject into the pod.
    pub volumes: Option<Vec<::v1_12::api::core::v1::Volume>>,
}

impl<'de> ::serde::Deserialize<'de> for PodPresetSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_env,
            Key_env_from,
            Key_selector,
            Key_volume_mounts,
            Key_volumes,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "env" => Field::Key_env,
                            "envFrom" => Field::Key_env_from,
                            "selector" => Field::Key_selector,
                            "volumeMounts" => Field::Key_volume_mounts,
                            "volumes" => Field::Key_volumes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PodPresetSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PodPresetSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_env: Option<Vec<::v1_12::api::core::v1::EnvVar>> = None;
                let mut value_env_from: Option<Vec<::v1_12::api::core::v1::EnvFromSource>> = None;
                let mut value_selector: Option<::v1_12::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_volume_mounts: Option<Vec<::v1_12::api::core::v1::VolumeMount>> = None;
                let mut value_volumes: Option<Vec<::v1_12::api::core::v1::Volume>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_env => value_env = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env_from => value_env_from = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mounts => value_volume_mounts = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volumes => value_volumes = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodPresetSpec {
                    env: value_env,
                    env_from: value_env_from,
                    selector: value_selector,
                    volume_mounts: value_volume_mounts,
                    volumes: value_volumes,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodPresetSpec",
            &[
                "env",
                "envFrom",
                "selector",
                "volumeMounts",
                "volumes",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PodPresetSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodPresetSpec",
            0 +
            self.env.as_ref().map_or(0, |_| 1) +
            self.env_from.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.volume_mounts.as_ref().map_or(0, |_| 1) +
            self.volumes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.env {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "env", value)?;
        }
        if let Some(value) = &self.env_from {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "envFrom", value)?;
        }
        if let Some(value) = &self.selector {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.volume_mounts {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeMounts", value)?;
        }
        if let Some(value) = &self.volumes {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "volumes", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
