// Generated from definition io.k8s.api.core.v1.PersistentVolumeClaimVolumeSource

/// PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace. This volume finds the bound PV and mounts that volume for the pod. A PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of volume that is owned by someone else (the system).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PersistentVolumeClaimVolumeSource {
    /// ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    pub claim_name: String,

    /// Will force the ReadOnly setting in VolumeMounts. Default false.
    pub read_only: Option<bool>,
}

impl<'de> ::serde::Deserialize<'de> for PersistentVolumeClaimVolumeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_claim_name,
            Key_read_only,
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
                            "claimName" => Field::Key_claim_name,
                            "readOnly" => Field::Key_read_only,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PersistentVolumeClaimVolumeSource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PersistentVolumeClaimVolumeSource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_claim_name: Option<String> = None;
                let mut value_read_only: Option<bool> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_claim_name => value_claim_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_read_only => value_read_only = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PersistentVolumeClaimVolumeSource {
                    claim_name: value_claim_name.ok_or_else(|| ::serde::de::Error::missing_field("claimName"))?,
                    read_only: value_read_only,
                })
            }
        }

        deserializer.deserialize_struct(
            "PersistentVolumeClaimVolumeSource",
            &[
                "claimName",
                "readOnly",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PersistentVolumeClaimVolumeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PersistentVolumeClaimVolumeSource",
            0 +
            1 +
            self.read_only.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "claimName", &self.claim_name)?;
        if let Some(value) = &self.read_only {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "readOnly", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
