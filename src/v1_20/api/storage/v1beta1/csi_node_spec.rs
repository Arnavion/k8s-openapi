// Generated from definition io.k8s.api.storage.v1beta1.CSINodeSpec

/// CSINodeSpec holds information about the specification of all CSI drivers installed on a node
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CSINodeSpec {
    /// drivers is a list of information of all CSI Drivers existing on a node. If all drivers in the list are uninstalled, this can become empty.
    pub drivers: Vec<crate::api::storage::v1beta1::CSINodeDriver>,

}

#[cfg(feature = "dsl")]
impl CSINodeSpec  {
    /// Set [`Self::drivers`]
    pub  fn drivers_set(&mut self, drivers: impl Into<Vec<crate::api::storage::v1beta1::CSINodeDriver>>) -> &mut Self {
        self.drivers = drivers.into(); self
    }

    pub  fn drivers(&mut self) -> &mut Vec<crate::api::storage::v1beta1::CSINodeDriver> {
        &mut self.drivers
    }

    /// Modify [`Self::drivers`] with a `func`
    pub  fn drivers_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::storage::v1beta1::CSINodeDriver>)) -> &mut Self {
        func(&mut self.drivers); self
    }

    /// Push new element to [`Self::drivers`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn drivers_push_with(&mut self, func: impl FnOnce(&mut crate::api::storage::v1beta1::CSINodeDriver)) -> &mut Self {
      let mut new = Default::default();
      func(&mut new);
      self.drivers.push(new);
      self
    }

    /// Append all elements from `other` into [`Self::drivers`]
    pub  fn drivers_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::storage::v1beta1::CSINodeDriver]>) -> &mut Self {
         for item in other.borrow() {
             self.drivers.push(item.to_owned());
         }
         self
    }


}


impl<'de> crate::serde::Deserialize<'de> for CSINodeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_drivers,
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
                            "drivers" => Field::Key_drivers,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = CSINodeSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("CSINodeSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_drivers: Option<Vec<crate::api::storage::v1beta1::CSINodeDriver>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_drivers => value_drivers = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(CSINodeSpec {
                    drivers: value_drivers.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "CSINodeSpec",
            &[
                "drivers",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for CSINodeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "CSINodeSpec",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "drivers", &self.drivers)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for CSINodeSpec {
    fn schema_name() -> String {
        "io.k8s.api.storage.v1beta1.CSINodeSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("CSINodeSpec holds information about the specification of all CSI drivers installed on a node".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "drivers".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("drivers is a list of information of all CSI Drivers existing on a node. If all drivers in the list are uninstalled, this can become empty.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::storage::v1beta1::CSINodeDriver>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "drivers".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
