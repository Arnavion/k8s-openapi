// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceSubresourceScale

/// CustomResourceSubresourceScale defines how to serve the scale subresource for CustomResources.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceSubresourceScale {
    /// LabelSelectorPath defines the JSON path inside of a CustomResource that corresponds to Scale.Status.Selector. Only JSON paths without the array notation are allowed. Must be a JSON Path under .status. Must be set to work with HPA. If there is no value under the given path in the CustomResource, the status label selector value in the /scale subresource will default to the empty string.
    pub label_selector_path: Option<String>,

    /// SpecReplicasPath defines the JSON path inside of a CustomResource that corresponds to Scale.Spec.Replicas. Only JSON paths without the array notation are allowed. Must be a JSON Path under .spec. If there is no value under the given path in the CustomResource, the /scale subresource will return an error on GET.
    pub spec_replicas_path: String,

    /// StatusReplicasPath defines the JSON path inside of a CustomResource that corresponds to Scale.Status.Replicas. Only JSON paths without the array notation are allowed. Must be a JSON Path under .status. If there is no value under the given path in the CustomResource, the status replica value in the /scale subresource will default to 0.
    pub status_replicas_path: String,
}

impl<'de> ::serde::Deserialize<'de> for CustomResourceSubresourceScale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_label_selector_path,
            Key_spec_replicas_path,
            Key_status_replicas_path,
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
                            "labelSelectorPath" => Field::Key_label_selector_path,
                            "specReplicasPath" => Field::Key_spec_replicas_path,
                            "statusReplicasPath" => Field::Key_status_replicas_path,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceSubresourceScale;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CustomResourceSubresourceScale")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_label_selector_path: Option<String> = None;
                let mut value_spec_replicas_path: Option<String> = None;
                let mut value_status_replicas_path: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_label_selector_path => value_label_selector_path = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec_replicas_path => value_spec_replicas_path = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_status_replicas_path => value_status_replicas_path = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceSubresourceScale {
                    label_selector_path: value_label_selector_path,
                    spec_replicas_path: value_spec_replicas_path.ok_or_else(|| ::serde::de::Error::missing_field("specReplicasPath"))?,
                    status_replicas_path: value_status_replicas_path.ok_or_else(|| ::serde::de::Error::missing_field("statusReplicasPath"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceSubresourceScale",
            &[
                "labelSelectorPath",
                "specReplicasPath",
                "statusReplicasPath",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for CustomResourceSubresourceScale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceSubresourceScale",
            0 +
            self.label_selector_path.as_ref().map_or(0, |_| 1) +
            1 +
            1,
        )?;
        if let Some(value) = &self.label_selector_path {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "labelSelectorPath", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "specReplicasPath", &self.spec_replicas_path)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "statusReplicasPath", &self.status_replicas_path)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
