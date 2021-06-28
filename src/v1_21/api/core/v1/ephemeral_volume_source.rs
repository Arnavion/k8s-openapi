// Generated from definition io.k8s.api.core.v1.EphemeralVolumeSource

/// Represents an ephemeral volume that is handled by a normal storage driver.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EphemeralVolumeSource {
    /// Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `\<pod name\>-\<volume name\>` where `\<volume name\>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long).
    ///
    /// An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster.
    ///
    /// This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created.
    ///
    /// Required, must not be nil.
    pub volume_claim_template: Option<crate::api::core::v1::PersistentVolumeClaimTemplate>,
}

impl<'de> crate::serde::Deserialize<'de> for EphemeralVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_volume_claim_template,
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
                            "volumeClaimTemplate" => Field::Key_volume_claim_template,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EphemeralVolumeSource;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EphemeralVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_volume_claim_template: Option<crate::api::core::v1::PersistentVolumeClaimTemplate> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_volume_claim_template => value_volume_claim_template = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EphemeralVolumeSource {
                    volume_claim_template: value_volume_claim_template,
                })
            }
        }

        deserializer.deserialize_struct(
            "EphemeralVolumeSource",
            &[
                "volumeClaimTemplate",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EphemeralVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EphemeralVolumeSource",
            self.volume_claim_template.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.volume_claim_template {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeClaimTemplate", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for EphemeralVolumeSource {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Represents an ephemeral volume that is handled by a normal storage driver.",
          "properties": {
            "volumeClaimTemplate": crate::schema_ref_with_description(crate::api::core::v1::PersistentVolumeClaimTemplate::schema(), "Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long).\n\nAn existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster.\n\nThis field is read-only and no changes will be made by Kubernetes to the PVC after it has been created.\n\nRequired, must not be nil.")
          },
          "type": "object"
        })
    }
}
