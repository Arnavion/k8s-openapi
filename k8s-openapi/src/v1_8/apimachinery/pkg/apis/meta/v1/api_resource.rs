// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIResource

/// APIResource specifies the name of a resource and whether it is namespaced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APIResource {
    /// categories is a list of the grouped resources this resource belongs to (e.g. 'all')
    pub categories: Option<Vec<String>>,

    /// group is the preferred group of the resource.  Empty implies the group of the containing resource list. For subresources, this may have a different value, for example: Scale".
    pub group: Option<String>,

    /// kind is the kind for the resource (e.g. 'Foo' is the kind for a resource 'foo')
    pub kind: String,

    /// name is the plural name of the resource.
    pub name: String,

    /// namespaced indicates if a resource is namespaced or not.
    pub namespaced: bool,

    /// shortNames is a list of suggested short names of the resource.
    pub short_names: Option<Vec<String>>,

    /// singularName is the singular name of the resource.  This allows clients to handle plural and singular opaquely. The singularName is more correct for reporting status on a single item and both singular and plural are allowed from the kubectl CLI interface.
    pub singular_name: String,

    /// verbs is a list of supported kube verbs (this includes get, list, watch, create, update, patch, delete, deletecollection, and proxy)
    pub verbs: Vec<String>,

    /// version is the preferred version of the resource.  Empty implies the version of the containing resource list For subresources, this may have a different value, for example: v1 (while inside a v1beta1 version of the core resource's group)".
    pub version: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for APIResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_categories,
            Key_group,
            Key_kind,
            Key_name,
            Key_namespaced,
            Key_short_names,
            Key_singular_name,
            Key_verbs,
            Key_version,
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
                            "categories" => Field::Key_categories,
                            "group" => Field::Key_group,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "namespaced" => Field::Key_namespaced,
                            "shortNames" => Field::Key_short_names,
                            "singularName" => Field::Key_singular_name,
                            "verbs" => Field::Key_verbs,
                            "version" => Field::Key_version,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APIResource;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct APIResource")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_categories: Option<Vec<String>> = None;
                let mut value_group: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespaced: Option<bool> = None;
                let mut value_short_names: Option<Vec<String>> = None;
                let mut value_singular_name: Option<String> = None;
                let mut value_verbs: Option<Vec<String>> = None;
                let mut value_version: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_categories => value_categories = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_namespaced => value_namespaced = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_short_names => value_short_names = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_singular_name => value_singular_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_verbs => value_verbs = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_version => value_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(APIResource {
                    categories: value_categories,
                    group: value_group,
                    kind: value_kind.ok_or_else(|| ::serde::de::Error::missing_field("kind"))?,
                    name: value_name.ok_or_else(|| ::serde::de::Error::missing_field("name"))?,
                    namespaced: value_namespaced.ok_or_else(|| ::serde::de::Error::missing_field("namespaced"))?,
                    short_names: value_short_names,
                    singular_name: value_singular_name.ok_or_else(|| ::serde::de::Error::missing_field("singularName"))?,
                    verbs: value_verbs.ok_or_else(|| ::serde::de::Error::missing_field("verbs"))?,
                    version: value_version,
                })
            }
        }

        deserializer.deserialize_struct(
            "APIResource",
            &[
                "categories",
                "group",
                "kind",
                "name",
                "namespaced",
                "shortNames",
                "singularName",
                "verbs",
                "version",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for APIResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "APIResource",
            0 +
            self.categories.as_ref().map_or(0, |_| 1) +
            self.group.as_ref().map_or(0, |_| 1) +
            1 +
            1 +
            1 +
            self.short_names.as_ref().map_or(0, |_| 1) +
            1 +
            1 +
            self.version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.categories {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "categories", value)?;
        }
        if let Some(value) = &self.group {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaced", &self.namespaced)?;
        if let Some(value) = &self.short_names {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "shortNames", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "singularName", &self.singular_name)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "verbs", &self.verbs)?;
        if let Some(value) = &self.version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
