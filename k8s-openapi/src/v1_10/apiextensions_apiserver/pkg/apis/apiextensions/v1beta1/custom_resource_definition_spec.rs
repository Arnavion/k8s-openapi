// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionSpec

/// CustomResourceDefinitionSpec describes how a user wants their resource to appear
#[derive(Debug, Default)]
pub struct CustomResourceDefinitionSpec {
    /// Group is the group this resource belongs in
    pub group: String,

    /// Names are the names used to describe this custom resource
    pub names: ::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames,

    /// Scope indicates whether this resource is cluster or namespace scoped.  Default is namespaced
    pub scope: String,

    /// Subresources describes the subresources for CustomResources This field is alpha-level and should only be sent to servers that enable subresources via the CustomResourceSubresources feature gate.
    pub subresources: Option<::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources>,

    /// Validation describes the validation methods for CustomResources
    pub validation: Option<::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation>,

    /// Version is the version this resource belongs in
    pub version: String,
}

impl<'de> ::serde::Deserialize<'de> for CustomResourceDefinitionSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_group,
            Key_names,
            Key_scope,
            Key_subresources,
            Key_validation,
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
                            "group" => Field::Key_group,
                            "names" => Field::Key_names,
                            "scope" => Field::Key_scope,
                            "subresources" => Field::Key_subresources,
                            "validation" => Field::Key_validation,
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
            type Value = CustomResourceDefinitionSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct CustomResourceDefinitionSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_group: Option<String> = None;
                let mut value_names: Option<::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceDefinitionNames> = None;
                let mut value_scope: Option<String> = None;
                let mut value_subresources: Option<::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceSubresources> = None;
                let mut value_validation: Option<::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::CustomResourceValidation> = None;
                let mut value_version: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group => value_group = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_names => value_names = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_scope => value_scope = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_subresources => value_subresources = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_validation => value_validation = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CustomResourceDefinitionSpec {
                    group: value_group.ok_or_else(|| ::serde::de::Error::missing_field("group"))?,
                    names: value_names.ok_or_else(|| ::serde::de::Error::missing_field("names"))?,
                    scope: value_scope.ok_or_else(|| ::serde::de::Error::missing_field("scope"))?,
                    subresources: value_subresources,
                    validation: value_validation,
                    version: value_version.ok_or_else(|| ::serde::de::Error::missing_field("version"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "CustomResourceDefinitionSpec",
            &[
                "group",
                "names",
                "scope",
                "subresources",
                "validation",
                "version",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for CustomResourceDefinitionSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CustomResourceDefinitionSpec",
            0 +
            1 +
            1 +
            1 +
            (if self.subresources.is_some() { 1 } else { 0 }) +
            (if self.validation.is_some() { 1 } else { 0 }) +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "group", &self.group)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "names", &self.names)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "scope", &self.scope)?;
        if let Some(value) = &self.subresources {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "subresources", value)?;
        }
        if let Some(value) = &self.validation {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "validation", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "version", &self.version)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
