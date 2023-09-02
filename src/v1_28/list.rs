// Generated from definition io.k8s.List

/// List is a list of resources.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct List<T> where T: crate::ListableResource {
    /// List of objects.
    pub items: Vec<T>,

    /// Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ListMeta,
}

impl<T> crate::Resource for List<T> where T: crate::ListableResource {
    const API_VERSION: &'static str = <T as crate::Resource>::API_VERSION;
    const GROUP: &'static str = <T as crate::Resource>::GROUP;
    const KIND: &'static str = <T as crate::ListableResource>::LIST_KIND;
    const VERSION: &'static str = <T as crate::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = "";
    type Scope = <T as crate::Resource>::Scope;
}

impl<T> crate::Metadata for List<T> where T: crate::ListableResource {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ListMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<T> crate::DeepMerge for List<T> where T: crate::DeepMerge + crate::Metadata<Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> + crate::ListableResource {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::map(
            &mut self.items,
            other.items,
            &[|lhs, rhs| lhs.metadata().namespace == rhs.metadata().namespace, |lhs, rhs| lhs.metadata().name == rhs.metadata().name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
    }
}

impl<'de, T> crate::serde::Deserialize<'de> for List<T> where T: crate::serde::Deserialize<'de> + crate::ListableResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_items,
            Key_metadata,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "items" => Field::Key_items,
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> crate::serde::de::Visitor<'de> for Visitor<T> where T: crate::serde::Deserialize<'de> + crate::ListableResource {
            type Value = List<T>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_items: Option<Vec<T>> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ListMeta> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_items => value_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(List {
                    items: value_items.unwrap_or_default(),
                    metadata: value_metadata.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "items",
                "metadata",
            ],
            Visitor(Default::default()),
        )
    }
}

impl<T> crate::serde::Serialize for List<T> where T: crate::serde::Serialize + crate::ListableResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            4,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "items", &self.items)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}
