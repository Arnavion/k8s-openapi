// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionNames

/// CustomResourceDefinitionNames indicates the names to serve this CustomResourceDefinition
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomResourceDefinitionNames {
    /// Categories is a list of grouped resources custom resources belong to (e.g. 'all')
    pub categories: Option<Vec<String>>,

    /// Kind is the serialized kind of the resource.  It is normally CamelCase and singular.
    pub kind: String,

    /// ListKind is the serialized kind of the list for this resource.  Defaults to <kind>List.
    pub list_kind: Option<String>,

    /// Plural is the plural name of the resource to serve.  It must match the name of the CustomResourceDefinition-registration too: plural.group and it must be all lowercase.
    pub plural: String,

    /// ShortNames are short names for the resource.  It must be all lowercase.
    pub short_names: Option<Vec<String>>,

    /// Singular is the singular name of the resource.  It must be all lowercase  Defaults to lowercased <kind>
    pub singular: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for CustomResourceDefinitionNames {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_categories,
            Key_kind,
            Key_list_kind,
            Key_plural,
            Key_short_names,
            Key_singular,
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
                            "kind" => Field::Key_kind,
                            "listKind" => Field::Key_list_kind,
                            "plural" => Field::Key_plural,
                            "shortNames" => Field::Key_short_names,
                            "singular" => Field::Key_singular,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceDefinitionNames;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CustomResourceDefinitionNames")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_categories: Option<Vec<String>> = None;
                let mut value_kind: Option<String> = None;
                let mut value_list_kind: Option<String> = None;
                let mut value_plural: Option<String> = None;
                let mut value_short_names: Option<Vec<String>> = None;
                let mut value_singular: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_categories => value_categories = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_list_kind => value_list_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_plural => value_plural = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_short_names => value_short_names = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_singular => value_singular = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionNames {
                    categories: value_categories,
                    kind: value_kind.ok_or_else(|| ::serde::de::Error::missing_field("kind"))?,
                    list_kind: value_list_kind,
                    plural: value_plural.ok_or_else(|| ::serde::de::Error::missing_field("plural"))?,
                    short_names: value_short_names,
                    singular: value_singular,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionNames",
            &[
                "categories",
                "kind",
                "listKind",
                "plural",
                "shortNames",
                "singular",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for CustomResourceDefinitionNames {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionNames",
            0 +
            self.categories.as_ref().map_or(0, |_| 1) +
            1 +
            self.list_kind.as_ref().map_or(0, |_| 1) +
            1 +
            self.short_names.as_ref().map_or(0, |_| 1) +
            self.singular.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.categories {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "categories", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        if let Some(value) = &self.list_kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "listKind", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "plural", &self.plural)?;
        if let Some(value) = &self.short_names {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "shortNames", value)?;
        }
        if let Some(value) = &self.singular {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "singular", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
