// Generated from definition io.k8s.apimachinery.pkg.util.intstr.IntOrString

/// IntOrString is a type that can hold an int32 or a string.  When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type.  This allows you to have, for example, a JSON field that can accept a name or number.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntOrString {
    Int(i32),
    String(std::string::String),
}

impl Default for IntOrString {
    fn default() -> Self {
        IntOrString::Int(0)
    }
}

impl crate::DeepMerge for IntOrString {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}

impl<'de> crate::serde::Deserialize<'de> for IntOrString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl crate::serde::de::Visitor<'_> for Visitor {
            type Value = IntOrString;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("IntOrString")
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                Ok(IntOrString::Int(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                let v = v.try_into().map_err(|_| crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Signed(v), &"a 32-bit integer"))?;
                Ok(IntOrString::Int(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                let v = v.try_into().map_err(|_| crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Unsigned(v), &"a 32-bit integer"))?;
                Ok(IntOrString::Int(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                self.visit_string(v.into())
            }

            fn visit_string<E>(self, v: std::string::String) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                Ok(IntOrString::String(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl crate::serde::Serialize for IntOrString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        match self {
            IntOrString::Int(i) => i.serialize(serializer),
            IntOrString::String(s) => s.serialize(serializer),
        }
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for IntOrString {
    fn schema_name() -> std::string::String {
        "io.k8s.apimachinery.pkg.util.intstr.IntOrString".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("IntOrString is a type that can hold an int32 or a string.  When used in JSON or YAML marshalling and unmarshalling, it produces or consumes the inner type.  This allows you to have, for example, a JSON field that can accept a name or number.".into()),
                ..Default::default()
            })),
            extensions: [
                ("x-kubernetes-int-or-string".into(), crate::serde_json::Value::Bool(true)),
            ].into(),
            ..Default::default()
        })
    }
}
