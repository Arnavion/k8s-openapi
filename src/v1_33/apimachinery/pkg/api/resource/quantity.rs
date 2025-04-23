// Generated from definition io.k8s.apimachinery.pkg.api.resource.Quantity

/// Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.
///
/// The serialization format is:
///
///  \<quantity\>        ::= \<signedNumber\>\<suffix\>
///
///   (Note that \<suffix\> may be empty, from the "" case in \<decimalSI\>.)
///
/// \<digit\>           ::= 0 | 1 | ... | 9 \<digits\>          ::= \<digit\> | \<digit\>\<digits\> \<number\>          ::= \<digits\> | \<digits\>.\<digits\> | \<digits\>. | .\<digits\> \<sign\>            ::= "+" | "-" \<signedNumber\>    ::= \<number\> | \<sign\>\<number\> \<suffix\>          ::= \<binarySI\> | \<decimalExponent\> | \<decimalSI\> \<binarySI\>        ::= Ki | Mi | Gi | Ti | Pi | Ei
///
///   (International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)
///
/// \<decimalSI\>       ::= m | "" | k | M | G | T | P | E
///
///   (Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)
///
/// \<decimalExponent\> ::= "e" \<signedNumber\> | "E" \<signedNumber\>
///
/// No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.
///
/// When a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.
///
/// Before serializing, Quantity will be put in "canonical form". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:
///
/// - No precision is lost - No fractional digits will be emitted - The exponent (or suffix) is as large as possible.
///
/// The sign will be omitted unless the number is negative.
///
/// Examples:
///
/// - 1.5 will be serialized as "1500m" - 1.5Gi will be serialized as "1536Mi"
///
/// Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.
///
/// Non-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)
///
/// This format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Quantity(pub std::string::String);

impl crate::DeepMerge for Quantity {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Quantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Quantity;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Quantity")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: crate::serde::Deserializer<'de> {
                Ok(Quantity(crate::serde::Deserialize::deserialize(deserializer)?))
            }
        }

        deserializer.deserialize_newtype_struct("Quantity", Visitor)
    }
}

impl crate::serde::Serialize for Quantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        serializer.serialize_newtype_struct("Quantity", &self.0)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Quantity {
    fn schema_name() -> std::string::String {
        "io.k8s.apimachinery.pkg.api.resource.Quantity".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.\n\nThe serialization format is:\n\n``` <quantity>        ::= <signedNumber><suffix>\n\n\t(Note that <suffix> may be empty, from the \"\" case in <decimalSI>.)\n\n<digit>           ::= 0 | 1 | ... | 9 <digits>          ::= <digit> | <digit><digits> <number>          ::= <digits> | <digits>.<digits> | <digits>. | .<digits> <sign>            ::= \"+\" | \"-\" <signedNumber>    ::= <number> | <sign><number> <suffix>          ::= <binarySI> | <decimalExponent> | <decimalSI> <binarySI>        ::= Ki | Mi | Gi | Ti | Pi | Ei\n\n\t(International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)\n\n<decimalSI>       ::= m | \"\" | k | M | G | T | P | E\n\n\t(Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)\n\n<decimalExponent> ::= \"e\" <signedNumber> | \"E\" <signedNumber> ```\n\nNo matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.\n\nWhen a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.\n\nBefore serializing, Quantity will be put in \"canonical form\". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:\n\n- No precision is lost - No fractional digits will be emitted - The exponent (or suffix) is as large as possible.\n\nThe sign will be omitted unless the number is negative.\n\nExamples:\n\n- 1.5 will be serialized as \"1500m\" - 1.5Gi will be serialized as \"1536Mi\"\n\nNote that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.\n\nNon-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)\n\nThis format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
            ..Default::default()
        })
    }
}
