// Generated from definition io.k8s.api.resource.v1beta1.DeviceDerivedAttribute

/// DeviceDerivedAttribute defines a derived attribute computed via CEL.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceDerivedAttribute {
    /// Expression is a CEL expression evaluated against each candidate device. The expression must evaluate to a primitive scalar (string, integer, boolean, or semver) or a list of these scalars (\[\]string, \[\]int64, \[\]bool, \[\]semver) to act as a virtual grouping key. Any other return type is an error and causes CEL evaluation for the device to fail.
    ///
    /// The expression's input is an object named "device", which carries the same properties as in a CELDeviceSelector.
    ///
    /// When pod scheduling encounters CEL runtime errors (such as looking up an attribute that isn't defined) for some devices, it will abort allocation and fail scheduling for the Pod. Surfacing evaluation errors immediately prevents silent topology matching failures that are extremely hard to detect. A robust expression should, for example, check for the existence of attributes before referencing them to avoid runtime evaluation errors.
    ///
    /// The expression gets evaluated after a device has passed the other selector expressions for the request in which this expression is used. This allows writing expressions that are tailored towards the specific devices being requested (for example, by assuming the device is from a certain vendor and skipping those checks).
    ///
    /// The length of the expression must be smaller or equal to 10 Ki. The cost of evaluating it is also limited based on the estimated number of logical steps; the combined cost of all derived attributes in a claim is capped by a shared CEL cost budget.
    pub expression: std::string::String,

    /// Name is the identifier for this derived attribute, used in constraints.
    ///
    /// It must be a DNS subdomain followed by a slash ("/") followed by a C identifier (e.g. "example.com/numaNode" or "derived/numaNode").
    ///
    /// If the chosen name matches an existing physical attribute from a driver, the derived attribute's expression will shadow the physical attribute, and its evaluated value will be used in constraints instead. When the goal is to define a derived attribute that is only used within the ResourceClaim and not meant to shadow an existing attribute, use a domain prefix that no DRA driver should be using (e.g. "derived/myAttribute").
    ///
    /// It is not valid to define a derived attribute that isn't used in at least one constraint.
    pub name: std::string::String,
}

impl crate::DeepMerge for DeviceDerivedAttribute {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.expression, other.expression);
        crate::DeepMerge::merge_from(&mut self.name, other.name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceDerivedAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_expression,
            Key_name,
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
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDerivedAttribute;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceDerivedAttribute")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_expression: Option<std::string::String> = None;
                let mut value_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_expression => value_expression = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceDerivedAttribute {
                    expression: value_expression.unwrap_or_default(),
                    name: value_name.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceDerivedAttribute",
            &[
                "expression",
                "name",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceDerivedAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceDerivedAttribute",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "expression", &self.expression)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceDerivedAttribute {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta1.DeviceDerivedAttribute".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceDerivedAttribute defines a derived attribute computed via CEL.",
            "type": "object",
            "properties": {
                "expression": {
                    "description": "Expression is a CEL expression evaluated against each candidate device. The expression must evaluate to a primitive scalar (string, integer, boolean, or semver) or a list of these scalars ([]string, []int64, []bool, []semver) to act as a virtual grouping key. Any other return type is an error and causes CEL evaluation for the device to fail.\n\nThe expression's input is an object named \"device\", which carries the same properties as in a CELDeviceSelector.\n\nWhen pod scheduling encounters CEL runtime errors (such as looking up an attribute that isn't defined) for some devices, it will abort allocation and fail scheduling for the Pod. Surfacing evaluation errors immediately prevents silent topology matching failures that are extremely hard to detect. A robust expression should, for example, check for the existence of attributes before referencing them to avoid runtime evaluation errors.\n\nThe expression gets evaluated after a device has passed the other selector expressions for the request in which this expression is used. This allows writing expressions that are tailored towards the specific devices being requested (for example, by assuming the device is from a certain vendor and skipping those checks).\n\nThe length of the expression must be smaller or equal to 10 Ki. The cost of evaluating it is also limited based on the estimated number of logical steps; the combined cost of all derived attributes in a claim is capped by a shared CEL cost budget.",
                    "type": "string",
                },
                "name": {
                    "description": "Name is the identifier for this derived attribute, used in constraints.\n\nIt must be a DNS subdomain followed by a slash (\"/\") followed by a C identifier (e.g. \"example.com/numaNode\" or \"derived/numaNode\").\n\nIf the chosen name matches an existing physical attribute from a driver, the derived attribute's expression will shadow the physical attribute, and its evaluated value will be used in constraints instead. When the goal is to define a derived attribute that is only used within the ResourceClaim and not meant to shadow an existing attribute, use a domain prefix that no DRA driver should be using (e.g. \"derived/myAttribute\").\n\nIt is not valid to define a derived attribute that isn't used in at least one constraint.",
                    "type": "string",
                },
            },
            "required": [
                "expression",
                "name",
            ],
        })
    }
}

#[cfg(feature = "schemars08")]
impl crate::schemars08::JsonSchema for DeviceDerivedAttribute {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta1.DeviceDerivedAttribute".into()
    }

    fn json_schema(__gen: &mut crate::schemars08::gen::SchemaGenerator) -> crate::schemars08::schema::Schema {
        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                description: Some("DeviceDerivedAttribute defines a derived attribute computed via CEL.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars08::schema::ObjectValidation {
                properties: [
                    (
                        "expression".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Expression is a CEL expression evaluated against each candidate device. The expression must evaluate to a primitive scalar (string, integer, boolean, or semver) or a list of these scalars ([]string, []int64, []bool, []semver) to act as a virtual grouping key. Any other return type is an error and causes CEL evaluation for the device to fail.\n\nThe expression's input is an object named \"device\", which carries the same properties as in a CELDeviceSelector.\n\nWhen pod scheduling encounters CEL runtime errors (such as looking up an attribute that isn't defined) for some devices, it will abort allocation and fail scheduling for the Pod. Surfacing evaluation errors immediately prevents silent topology matching failures that are extremely hard to detect. A robust expression should, for example, check for the existence of attributes before referencing them to avoid runtime evaluation errors.\n\nThe expression gets evaluated after a device has passed the other selector expressions for the request in which this expression is used. This allows writing expressions that are tailored towards the specific devices being requested (for example, by assuming the device is from a certain vendor and skipping those checks).\n\nThe length of the expression must be smaller or equal to 10 Ki. The cost of evaluating it is also limited based on the estimated number of logical steps; the combined cost of all derived attributes in a claim is capped by a shared CEL cost budget.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".into(),
                        crate::schemars08::schema::Schema::Object(crate::schemars08::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars08::schema::Metadata {
                                description: Some("Name is the identifier for this derived attribute, used in constraints.\n\nIt must be a DNS subdomain followed by a slash (\"/\") followed by a C identifier (e.g. \"example.com/numaNode\" or \"derived/numaNode\").\n\nIf the chosen name matches an existing physical attribute from a driver, the derived attribute's expression will shadow the physical attribute, and its evaluated value will be used in constraints instead. When the goal is to define a derived attribute that is only used within the ResourceClaim and not meant to shadow an existing attribute, use a domain prefix that no DRA driver should be using (e.g. \"derived/myAttribute\").\n\nIt is not valid to define a derived attribute that isn't used in at least one constraint.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars08::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars08::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "expression".into(),
                    "name".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
