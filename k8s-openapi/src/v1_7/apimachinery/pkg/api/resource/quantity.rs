// Generated from definition io.k8s.apimachinery.pkg.api.resource.Quantity

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Quantity(pub String);

impl<'de> ::serde::Deserialize<'de> for Quantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Quantity;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "Quantity")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: ::serde::Deserializer<'de> {
                Ok(Quantity(::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("Quantity", Visitor)
    }
}

impl ::serde::Serialize for Quantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        serializer.serialize_newtype_struct("Quantity", &self.0)
    }
}
