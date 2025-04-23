// Generated from definition io.k8s.api.resource.v1beta1.CELDeviceSelector

/// CELDeviceSelector contains a CEL expression for selecting a device.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CELDeviceSelector {
    /// Expression is a CEL expression which evaluates a single device. It must evaluate to true when the device under consideration satisfies the desired criteria, and false when it does not. Any other result is an error and causes allocation of devices to abort.
    ///
    /// The expression's input is an object named "device", which carries the following properties:
    ///  - driver (string): the name of the driver which defines this device.
    ///  - attributes (map\[string\]object): the device's attributes, grouped by prefix
    ///    (e.g. device.attributes\["dra.example.com"\] evaluates to an object with all
    ///    of the attributes which were prefixed by "dra.example.com".
    ///  - capacity (map\[string\]object): the device's capacities, grouped by prefix.
    ///
    /// Example: Consider a device with driver="dra.example.com", which exposes two attributes named "model" and "ext.example.com/family" and which exposes one capacity named "modules". This input to this expression would have the following fields:
    ///
    ///   device.driver
    ///     device.attributes\["dra.example.com"\].model
    ///     device.attributes\["ext.example.com"\].family
    ///     device.capacity\["dra.example.com"\].modules
    ///
    /// The device.driver field can be used to check for a specific driver, either as a high-level precondition (i.e. you only want to consider devices from this driver) or as part of a multi-clause expression that is meant to consider devices from different drivers.
    ///
    /// The value type of each attribute is defined by the device definition, and users who write these expressions must consult the documentation for their specific drivers. The value type of each capacity is Quantity.
    ///
    /// If an unknown prefix is used as a lookup in either device.attributes or device.capacity, an empty map will be returned. Any reference to an unknown field will cause an evaluation error and allocation to abort.
    ///
    /// A robust expression should check for the existence of attributes before referencing them.
    ///
    /// For ease of use, the cel.bind() function is enabled, and can be used to simplify expressions that access multiple attributes with the same domain. For example:
    ///
    ///   cel.bind(dra, device.attributes\["dra.example.com"\], dra.someBool && dra.anotherBool)
    ///
    /// The length of the expression must be smaller or equal to 10 Ki. The cost of evaluating it is also limited based on the estimated number of logical steps.
    pub expression: std::string::String,
}

impl crate::DeepMerge for CELDeviceSelector {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.expression, other.expression);
    }
}

impl<'de> crate::serde::Deserialize<'de> for CELDeviceSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_expression,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "expression" => Field::Key_expression,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CELDeviceSelector;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("CELDeviceSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_expression: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_expression => value_expression = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CELDeviceSelector {
                    expression: value_expression.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CELDeviceSelector",
            &[
                "expression",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CELDeviceSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CELDeviceSelector",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "expression", &self.expression)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CELDeviceSelector {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta1.CELDeviceSelector".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("CELDeviceSelector contains a CEL expression for selecting a device.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "expression".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Expression is a CEL expression which evaluates a single device. It must evaluate to true when the device under consideration satisfies the desired criteria, and false when it does not. Any other result is an error and causes allocation of devices to abort.\n\nThe expression's input is an object named \"device\", which carries the following properties:\n - driver (string): the name of the driver which defines this device.\n - attributes (map[string]object): the device's attributes, grouped by prefix\n   (e.g. device.attributes[\"dra.example.com\"] evaluates to an object with all\n   of the attributes which were prefixed by \"dra.example.com\".\n - capacity (map[string]object): the device's capacities, grouped by prefix.\n\nExample: Consider a device with driver=\"dra.example.com\", which exposes two attributes named \"model\" and \"ext.example.com/family\" and which exposes one capacity named \"modules\". This input to this expression would have the following fields:\n\n    device.driver\n    device.attributes[\"dra.example.com\"].model\n    device.attributes[\"ext.example.com\"].family\n    device.capacity[\"dra.example.com\"].modules\n\nThe device.driver field can be used to check for a specific driver, either as a high-level precondition (i.e. you only want to consider devices from this driver) or as part of a multi-clause expression that is meant to consider devices from different drivers.\n\nThe value type of each attribute is defined by the device definition, and users who write these expressions must consult the documentation for their specific drivers. The value type of each capacity is Quantity.\n\nIf an unknown prefix is used as a lookup in either device.attributes or device.capacity, an empty map will be returned. Any reference to an unknown field will cause an evaluation error and allocation to abort.\n\nA robust expression should check for the existence of attributes before referencing them.\n\nFor ease of use, the cel.bind() function is enabled, and can be used to simplify expressions that access multiple attributes with the same domain. For example:\n\n    cel.bind(dra, device.attributes[\"dra.example.com\"], dra.someBool && dra.anotherBool)\n\nThe length of the expression must be smaller or equal to 10 Ki. The cost of evaluating it is also limited based on the estimated number of logical steps.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "expression".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
