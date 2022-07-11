// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaProps

/// JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSONSchemaProps {
    pub ref_path: Option<String>,

    pub schema: Option<String>,

    pub additional_items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool>,

    pub additional_properties: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool>,

    pub all_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub any_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    /// default is a default value for undefined object fields. Defaulting is a beta feature under the CustomResourceDefaulting feature gate. Defaulting requires spec.preserveUnknownFields to be false.
    pub default: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>,

    pub definitions: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub dependencies: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray>>,

    pub description: Option<String>,

    pub enum_: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>>,

    pub example: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>,

    pub exclusive_maximum: Option<bool>,

    pub exclusive_minimum: Option<bool>,

    pub external_docs: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation>,

    /// format is an OpenAPI v3 format string. Unknown formats are ignored. The following formats are validated:
    ///
    /// - bsonobjectid: a bson object ID, i.e. a 24 characters hex string - uri: an URI as parsed by Golang net/url.ParseRequestURI - email: an email address as parsed by Golang net/mail.ParseAddress - hostname: a valid representation for an Internet host name, as defined by RFC 1034, section 3.1 \[RFC1034\]. - ipv4: an IPv4 IP as parsed by Golang net.ParseIP - ipv6: an IPv6 IP as parsed by Golang net.ParseIP - cidr: a CIDR as parsed by Golang net.ParseCIDR - mac: a MAC address as parsed by Golang net.ParseMAC - uuid: an UUID that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?\[0-9a-f\]{4}-?\[0-9a-f\]{4}-?\[0-9a-f\]{12}$ - uuid3: an UUID3 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?3\[0-9a-f\]{3}-?\[0-9a-f\]{4}-?\[0-9a-f\]{12}$ - uuid4: an UUID4 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?4\[0-9a-f\]{3}-?\[89ab\]\[0-9a-f\]{3}-?\[0-9a-f\]{12}$ - uuid5: an UUID5 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?5\[0-9a-f\]{3}-?\[89ab\]\[0-9a-f\]{3}-?\[0-9a-f\]{12}$ - isbn: an ISBN10 or ISBN13 number string like "0321751043" or "978-0321751041" - isbn10: an ISBN10 number string like "0321751043" - isbn13: an ISBN13 number string like "978-0321751041" - creditcard: a credit card number defined by the regex ^(?:4\[0-9\]{12}(?:\[0-9\]{3})?|5\[1-5\]\[0-9\]{14}|6(?:011|5\[0-9\]\[0-9\])\[0-9\]{12}|3\[47\]\[0-9\]{13}|3(?:0\[0-5\]|\[68\]\[0-9\])\[0-9\]{11}|(?:2131|1800|35\\d{3})\\d{11})$ with any non digit characters mixed in - ssn: a U.S. social security number following the regex ^\\d{3}\[- \]?\\d{2}\[- \]?\\d{4}$ - hexcolor: an hexadecimal color code like "#FFFFFF: following the regex ^#?(\[0-9a-fA-F\]{3}|\[0-9a-fA-F\]{6})$ - rgbcolor: an RGB color code like rgb like "rgb(255,255,2559" - byte: base64 encoded binary data - password: any kind of string - date: a date string like "2006-01-02" as defined by full-date in RFC3339 - duration: a duration string like "22 ns" as parsed by Golang time.ParseDuration or compatible with Scala duration format - datetime: a date time string like "2014-12-15T19:30:20.000Z" as defined by date-time in RFC3339.
    pub format: Option<String>,

    pub id: Option<String>,

    pub items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray>,

    pub max_items: Option<i64>,

    pub max_length: Option<i64>,

    pub max_properties: Option<i64>,

    pub maximum: Option<f64>,

    pub min_items: Option<i64>,

    pub min_length: Option<i64>,

    pub min_properties: Option<i64>,

    pub minimum: Option<f64>,

    pub multiple_of: Option<f64>,

    pub not: Option<Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub nullable: Option<bool>,

    pub one_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub pattern: Option<String>,

    pub pattern_properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>,

    pub required: Option<Vec<String>>,

    pub title: Option<String>,

    pub type_: Option<String>,

    pub unique_items: Option<bool>,

    /// x-kubernetes-embedded-resource defines that the value is an embedded Kubernetes runtime.Object, with TypeMeta and ObjectMeta. The type must be object. It is allowed to further restrict the embedded object. kind, apiVersion and metadata are validated automatically. x-kubernetes-preserve-unknown-fields is allowed to be true, but does not have to be if the object is fully specified (up to kind, apiVersion, metadata).
    pub x_kubernetes_embedded_resource: Option<bool>,

    /// x-kubernetes-int-or-string specifies that this value is either an integer or a string. If this is true, an empty type is allowed and type as child of anyOf is permitted if following one of the following patterns:
    ///
    /// 1) anyOf:
    ///    - type: integer
    ///    - type: string
    /// 2) allOf:
    ///    - anyOf:
    ///      - type: integer
    ///      - type: string
    ///    - ... zero or more
    pub x_kubernetes_int_or_string: Option<bool>,

    /// x-kubernetes-list-map-keys annotates an array with the x-kubernetes-list-type `map` by specifying the keys used as the index of the map.
    ///
    /// This tag MUST only be used on lists that have the "x-kubernetes-list-type" extension set to "map". Also, the values specified for this attribute must be a scalar typed field of the child structure (no nesting is supported).
    ///
    /// The properties specified must either be required or have a default value, to ensure those properties are present for all list items.
    pub x_kubernetes_list_map_keys: Option<Vec<String>>,

    /// x-kubernetes-list-type annotates an array to further describe its topology. This extension must only be used on lists and may have 3 possible values:
    ///
    /// 1) `atomic`: the list is treated as a single entity, like a scalar.
    ///      Atomic lists will be entirely replaced when updated. This extension
    ///      may be used on any type of list (struct, scalar, ...).
    /// 2) `set`:
    ///      Sets are lists that must not have multiple items with the same value. Each
    ///      value must be a scalar, an object with x-kubernetes-map-type `atomic` or an
    ///      array with x-kubernetes-list-type `atomic`.
    /// 3) `map`:
    ///      These lists are like maps in that their elements have a non-index key
    ///      used to identify them. Order is preserved upon merge. The map tag
    ///      must only be used on a list with elements of type object.
    /// Defaults to atomic for arrays.
    pub x_kubernetes_list_type: Option<String>,

    /// x-kubernetes-map-type annotates an object to further describe its topology. This extension must only be used when type is object and may have 2 possible values:
    ///
    /// 1) `granular`:
    ///      These maps are actual maps (key-value pairs) and each fields are independent
    ///      from each other (they can each be manipulated by separate actors). This is
    ///      the default behaviour for all maps.
    /// 2) `atomic`: the list is treated as a single entity, like a scalar.
    ///      Atomic maps will be entirely replaced when updated.
    pub x_kubernetes_map_type: Option<String>,

    /// x-kubernetes-preserve-unknown-fields stops the API server decoding step from pruning fields which are not specified in the validation schema. This affects fields recursively, but switches back to normal pruning behaviour if nested properties or additionalProperties are specified in the schema. This can either be true or undefined. False is forbidden.
    pub x_kubernetes_preserve_unknown_fields: Option<bool>,

}

#[cfg(feature = "dsl")]
impl JSONSchemaProps  {
    /// Set [`Self::ref_path`]
    pub  fn ref_path_set(&mut self, ref_path: impl Into<Option<String>>) -> &mut Self {
        self.ref_path = ref_path.into(); self
    }

    pub  fn ref_path(&mut self) -> &mut String {
        if self.ref_path.is_none() { self.ref_path = Some(Default::default()) }
        self.ref_path.as_mut().unwrap()
    }

    /// Modify [`Self::ref_path`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ref_path_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.ref_path.is_none() { self.ref_path = Some(Default::default()) };
        func(self.ref_path.as_mut().unwrap()); self
    }


    /// Set [`Self::schema`]
    pub  fn schema_set(&mut self, schema: impl Into<Option<String>>) -> &mut Self {
        self.schema = schema.into(); self
    }

    pub  fn schema(&mut self) -> &mut String {
        if self.schema.is_none() { self.schema = Some(Default::default()) }
        self.schema.as_mut().unwrap()
    }

    /// Modify [`Self::schema`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn schema_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.schema.is_none() { self.schema = Some(Default::default()) };
        func(self.schema.as_mut().unwrap()); self
    }


    /// Set [`Self::additional_items`]
    pub  fn additional_items_set(&mut self, additional_items: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool>>) -> &mut Self {
        self.additional_items = additional_items.into(); self
    }


    /// Set [`Self::additional_properties`]
    pub  fn additional_properties_set(&mut self, additional_properties: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool>>) -> &mut Self {
        self.additional_properties = additional_properties.into(); self
    }


    /// Set [`Self::all_of`]
    pub  fn all_of_set(&mut self, all_of: impl Into<Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>>) -> &mut Self {
        self.all_of = all_of.into(); self
    }

    pub  fn all_of(&mut self) -> &mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps> {
        if self.all_of.is_none() { self.all_of = Some(Default::default()) }
        self.all_of.as_mut().unwrap()
    }

    /// Modify [`Self::all_of`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn all_of_with(&mut self, func: impl FnOnce(&mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>)) -> &mut Self {
        if self.all_of.is_none() { self.all_of = Some(Default::default()) };
        func(self.all_of.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::all_of`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn all_of_push_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps)) -> &mut Self {
        if self.all_of.is_none() {
            self.all_of = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.all_of.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::all_of`]
    pub  fn all_of_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps]>) -> &mut Self {
         if self.all_of.is_none() { self.all_of = Some(Vec::new()); }
         let all_of = &mut self.all_of.as_mut().unwrap();
         for item in other.borrow() {
             all_of.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::any_of`]
    pub  fn any_of_set(&mut self, any_of: impl Into<Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>>) -> &mut Self {
        self.any_of = any_of.into(); self
    }

    pub  fn any_of(&mut self) -> &mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps> {
        if self.any_of.is_none() { self.any_of = Some(Default::default()) }
        self.any_of.as_mut().unwrap()
    }

    /// Modify [`Self::any_of`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn any_of_with(&mut self, func: impl FnOnce(&mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>)) -> &mut Self {
        if self.any_of.is_none() { self.any_of = Some(Default::default()) };
        func(self.any_of.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::any_of`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn any_of_push_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps)) -> &mut Self {
        if self.any_of.is_none() {
            self.any_of = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.any_of.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::any_of`]
    pub  fn any_of_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps]>) -> &mut Self {
         if self.any_of.is_none() { self.any_of = Some(Vec::new()); }
         let any_of = &mut self.any_of.as_mut().unwrap();
         for item in other.borrow() {
             any_of.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::default`]
    pub  fn default_set(&mut self, default: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>>) -> &mut Self {
        self.default = default.into(); self
    }

    pub  fn default(&mut self) -> &mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON {
        if self.default.is_none() { self.default = Some(Default::default()) }
        self.default.as_mut().unwrap()
    }

    /// Modify [`Self::default`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn default_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON)) -> &mut Self {
        if self.default.is_none() { self.default = Some(Default::default()) };
        func(self.default.as_mut().unwrap()); self
    }


    /// Set [`Self::definitions`]
    pub  fn definitions_set(&mut self, definitions: impl Into<Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>>) -> &mut Self {
        self.definitions = definitions.into(); self
    }

    pub  fn definitions(&mut self) -> &mut std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps> {
        if self.definitions.is_none() { self.definitions = Some(Default::default()) }
        self.definitions.as_mut().unwrap()
    }

    /// Modify [`Self::definitions`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn definitions_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>)) -> &mut Self {
        if self.definitions.is_none() { self.definitions = Some(Default::default()) };
        func(self.definitions.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::definitions`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn definitions_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps)) -> &mut Self {
        if self.definitions.is_none() {
            self.definitions = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.definitions.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::definitions`]
    pub  fn definitions_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>) -> &mut Self {
         if self.definitions.is_none() { self.definitions = Some(std::collections::BTreeMap::new()); }
         let definitions = &mut self.definitions.as_mut().unwrap();
         for (name, value) in other.borrow() {
             definitions.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::dependencies`]
    pub  fn dependencies_set(&mut self, dependencies: impl Into<Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray>>>) -> &mut Self {
        self.dependencies = dependencies.into(); self
    }

    pub  fn dependencies(&mut self) -> &mut std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray> {
        if self.dependencies.is_none() { self.dependencies = Some(Default::default()) }
        self.dependencies.as_mut().unwrap()
    }

    /// Modify [`Self::dependencies`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn dependencies_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray>)) -> &mut Self {
        if self.dependencies.is_none() { self.dependencies = Some(Default::default()) };
        func(self.dependencies.as_mut().unwrap()); self
    }

    /// Insert all elements from `other` into [`Self::dependencies`]
    pub  fn dependencies_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray>>) -> &mut Self {
         if self.dependencies.is_none() { self.dependencies = Some(std::collections::BTreeMap::new()); }
         let dependencies = &mut self.dependencies.as_mut().unwrap();
         for (name, value) in other.borrow() {
             dependencies.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::description`]
    pub  fn description_set(&mut self, description: impl Into<Option<String>>) -> &mut Self {
        self.description = description.into(); self
    }

    pub  fn description(&mut self) -> &mut String {
        if self.description.is_none() { self.description = Some(Default::default()) }
        self.description.as_mut().unwrap()
    }

    /// Modify [`Self::description`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn description_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.description.is_none() { self.description = Some(Default::default()) };
        func(self.description.as_mut().unwrap()); self
    }


    /// Set [`Self::enum_`]
    pub  fn enum_set(&mut self, enum_: impl Into<Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>>>) -> &mut Self {
        self.enum_ = enum_.into(); self
    }

    pub  fn enum_(&mut self) -> &mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON> {
        if self.enum_.is_none() { self.enum_ = Some(Default::default()) }
        self.enum_.as_mut().unwrap()
    }

    /// Modify [`Self::enum_`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn enum_with(&mut self, func: impl FnOnce(&mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>)) -> &mut Self {
        if self.enum_.is_none() { self.enum_ = Some(Default::default()) };
        func(self.enum_.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::enum_`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn enum_push_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON)) -> &mut Self {
        if self.enum_.is_none() {
            self.enum_ = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.enum_.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::enum_`]
    pub  fn enum_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON]>) -> &mut Self {
         if self.enum_.is_none() { self.enum_ = Some(Vec::new()); }
         let enum_ = &mut self.enum_.as_mut().unwrap();
         for item in other.borrow() {
             enum_.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::example`]
    pub  fn example_set(&mut self, example: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>>) -> &mut Self {
        self.example = example.into(); self
    }

    pub  fn example(&mut self) -> &mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON {
        if self.example.is_none() { self.example = Some(Default::default()) }
        self.example.as_mut().unwrap()
    }

    /// Modify [`Self::example`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn example_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON)) -> &mut Self {
        if self.example.is_none() { self.example = Some(Default::default()) };
        func(self.example.as_mut().unwrap()); self
    }


    /// Set [`Self::exclusive_maximum`]
    pub  fn exclusive_maximum_set(&mut self, exclusive_maximum: impl Into<Option<bool>>) -> &mut Self {
        self.exclusive_maximum = exclusive_maximum.into(); self
    }

    pub  fn exclusive_maximum(&mut self) -> &mut bool {
        if self.exclusive_maximum.is_none() { self.exclusive_maximum = Some(Default::default()) }
        self.exclusive_maximum.as_mut().unwrap()
    }

    /// Modify [`Self::exclusive_maximum`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn exclusive_maximum_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.exclusive_maximum.is_none() { self.exclusive_maximum = Some(Default::default()) };
        func(self.exclusive_maximum.as_mut().unwrap()); self
    }


    /// Set [`Self::exclusive_minimum`]
    pub  fn exclusive_minimum_set(&mut self, exclusive_minimum: impl Into<Option<bool>>) -> &mut Self {
        self.exclusive_minimum = exclusive_minimum.into(); self
    }

    pub  fn exclusive_minimum(&mut self) -> &mut bool {
        if self.exclusive_minimum.is_none() { self.exclusive_minimum = Some(Default::default()) }
        self.exclusive_minimum.as_mut().unwrap()
    }

    /// Modify [`Self::exclusive_minimum`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn exclusive_minimum_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.exclusive_minimum.is_none() { self.exclusive_minimum = Some(Default::default()) };
        func(self.exclusive_minimum.as_mut().unwrap()); self
    }


    /// Set [`Self::external_docs`]
    pub  fn external_docs_set(&mut self, external_docs: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation>>) -> &mut Self {
        self.external_docs = external_docs.into(); self
    }

    pub  fn external_docs(&mut self) -> &mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation {
        if self.external_docs.is_none() { self.external_docs = Some(Default::default()) }
        self.external_docs.as_mut().unwrap()
    }

    /// Modify [`Self::external_docs`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn external_docs_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation)) -> &mut Self {
        if self.external_docs.is_none() { self.external_docs = Some(Default::default()) };
        func(self.external_docs.as_mut().unwrap()); self
    }


    /// Set [`Self::format`]
    pub  fn format_set(&mut self, format: impl Into<Option<String>>) -> &mut Self {
        self.format = format.into(); self
    }

    pub  fn format(&mut self) -> &mut String {
        if self.format.is_none() { self.format = Some(Default::default()) }
        self.format.as_mut().unwrap()
    }

    /// Modify [`Self::format`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn format_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.format.is_none() { self.format = Some(Default::default()) };
        func(self.format.as_mut().unwrap()); self
    }


    /// Set [`Self::id`]
    pub  fn id_set(&mut self, id: impl Into<Option<String>>) -> &mut Self {
        self.id = id.into(); self
    }

    pub  fn id(&mut self) -> &mut String {
        if self.id.is_none() { self.id = Some(Default::default()) }
        self.id.as_mut().unwrap()
    }

    /// Modify [`Self::id`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn id_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.id.is_none() { self.id = Some(Default::default()) };
        func(self.id.as_mut().unwrap()); self
    }


    /// Set [`Self::items`]
    pub  fn items_set(&mut self, items: impl Into<Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray>>) -> &mut Self {
        self.items = items.into(); self
    }


    /// Set [`Self::max_items`]
    pub  fn max_items_set(&mut self, max_items: impl Into<Option<i64>>) -> &mut Self {
        self.max_items = max_items.into(); self
    }

    pub  fn max_items(&mut self) -> &mut i64 {
        if self.max_items.is_none() { self.max_items = Some(Default::default()) }
        self.max_items.as_mut().unwrap()
    }

    /// Modify [`Self::max_items`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn max_items_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.max_items.is_none() { self.max_items = Some(Default::default()) };
        func(self.max_items.as_mut().unwrap()); self
    }


    /// Set [`Self::max_length`]
    pub  fn max_length_set(&mut self, max_length: impl Into<Option<i64>>) -> &mut Self {
        self.max_length = max_length.into(); self
    }

    pub  fn max_length(&mut self) -> &mut i64 {
        if self.max_length.is_none() { self.max_length = Some(Default::default()) }
        self.max_length.as_mut().unwrap()
    }

    /// Modify [`Self::max_length`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn max_length_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.max_length.is_none() { self.max_length = Some(Default::default()) };
        func(self.max_length.as_mut().unwrap()); self
    }


    /// Set [`Self::max_properties`]
    pub  fn max_properties_set(&mut self, max_properties: impl Into<Option<i64>>) -> &mut Self {
        self.max_properties = max_properties.into(); self
    }

    pub  fn max_properties(&mut self) -> &mut i64 {
        if self.max_properties.is_none() { self.max_properties = Some(Default::default()) }
        self.max_properties.as_mut().unwrap()
    }

    /// Modify [`Self::max_properties`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn max_properties_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.max_properties.is_none() { self.max_properties = Some(Default::default()) };
        func(self.max_properties.as_mut().unwrap()); self
    }


    /// Set [`Self::maximum`]
    pub  fn maximum_set(&mut self, maximum: impl Into<Option<f64>>) -> &mut Self {
        self.maximum = maximum.into(); self
    }

    pub  fn maximum(&mut self) -> &mut f64 {
        if self.maximum.is_none() { self.maximum = Some(Default::default()) }
        self.maximum.as_mut().unwrap()
    }

    /// Modify [`Self::maximum`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn maximum_with(&mut self, func: impl FnOnce(&mut f64)) -> &mut Self {
        if self.maximum.is_none() { self.maximum = Some(Default::default()) };
        func(self.maximum.as_mut().unwrap()); self
    }


    /// Set [`Self::min_items`]
    pub  fn min_items_set(&mut self, min_items: impl Into<Option<i64>>) -> &mut Self {
        self.min_items = min_items.into(); self
    }

    pub  fn min_items(&mut self) -> &mut i64 {
        if self.min_items.is_none() { self.min_items = Some(Default::default()) }
        self.min_items.as_mut().unwrap()
    }

    /// Modify [`Self::min_items`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn min_items_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.min_items.is_none() { self.min_items = Some(Default::default()) };
        func(self.min_items.as_mut().unwrap()); self
    }


    /// Set [`Self::min_length`]
    pub  fn min_length_set(&mut self, min_length: impl Into<Option<i64>>) -> &mut Self {
        self.min_length = min_length.into(); self
    }

    pub  fn min_length(&mut self) -> &mut i64 {
        if self.min_length.is_none() { self.min_length = Some(Default::default()) }
        self.min_length.as_mut().unwrap()
    }

    /// Modify [`Self::min_length`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn min_length_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.min_length.is_none() { self.min_length = Some(Default::default()) };
        func(self.min_length.as_mut().unwrap()); self
    }


    /// Set [`Self::min_properties`]
    pub  fn min_properties_set(&mut self, min_properties: impl Into<Option<i64>>) -> &mut Self {
        self.min_properties = min_properties.into(); self
    }

    pub  fn min_properties(&mut self) -> &mut i64 {
        if self.min_properties.is_none() { self.min_properties = Some(Default::default()) }
        self.min_properties.as_mut().unwrap()
    }

    /// Modify [`Self::min_properties`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn min_properties_with(&mut self, func: impl FnOnce(&mut i64)) -> &mut Self {
        if self.min_properties.is_none() { self.min_properties = Some(Default::default()) };
        func(self.min_properties.as_mut().unwrap()); self
    }


    /// Set [`Self::minimum`]
    pub  fn minimum_set(&mut self, minimum: impl Into<Option<f64>>) -> &mut Self {
        self.minimum = minimum.into(); self
    }

    pub  fn minimum(&mut self) -> &mut f64 {
        if self.minimum.is_none() { self.minimum = Some(Default::default()) }
        self.minimum.as_mut().unwrap()
    }

    /// Modify [`Self::minimum`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn minimum_with(&mut self, func: impl FnOnce(&mut f64)) -> &mut Self {
        if self.minimum.is_none() { self.minimum = Some(Default::default()) };
        func(self.minimum.as_mut().unwrap()); self
    }


    /// Set [`Self::multiple_of`]
    pub  fn multiple_of_set(&mut self, multiple_of: impl Into<Option<f64>>) -> &mut Self {
        self.multiple_of = multiple_of.into(); self
    }

    pub  fn multiple_of(&mut self) -> &mut f64 {
        if self.multiple_of.is_none() { self.multiple_of = Some(Default::default()) }
        self.multiple_of.as_mut().unwrap()
    }

    /// Modify [`Self::multiple_of`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn multiple_of_with(&mut self, func: impl FnOnce(&mut f64)) -> &mut Self {
        if self.multiple_of.is_none() { self.multiple_of = Some(Default::default()) };
        func(self.multiple_of.as_mut().unwrap()); self
    }


    /// Set [`Self::not`]
    pub  fn not_set(&mut self, not: impl Into<Option<Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>>) -> &mut Self {
        self.not = not.into(); self
    }

    pub  fn not(&mut self) -> &mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps {
        if self.not.is_none() { self.not = Some(Default::default()) }
        self.not.as_mut().unwrap()
    }

    /// Modify [`Self::not`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn not_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps)) -> &mut Self {
        if self.not.is_none() { self.not = Some(Default::default()) };
        func(self.not.as_mut().unwrap()); self
    }


    /// Set [`Self::nullable`]
    pub  fn nullable_set(&mut self, nullable: impl Into<Option<bool>>) -> &mut Self {
        self.nullable = nullable.into(); self
    }

    pub  fn nullable(&mut self) -> &mut bool {
        if self.nullable.is_none() { self.nullable = Some(Default::default()) }
        self.nullable.as_mut().unwrap()
    }

    /// Modify [`Self::nullable`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn nullable_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.nullable.is_none() { self.nullable = Some(Default::default()) };
        func(self.nullable.as_mut().unwrap()); self
    }


    /// Set [`Self::one_of`]
    pub  fn one_of_set(&mut self, one_of: impl Into<Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>>) -> &mut Self {
        self.one_of = one_of.into(); self
    }

    pub  fn one_of(&mut self) -> &mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps> {
        if self.one_of.is_none() { self.one_of = Some(Default::default()) }
        self.one_of.as_mut().unwrap()
    }

    /// Modify [`Self::one_of`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn one_of_with(&mut self, func: impl FnOnce(&mut Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>)) -> &mut Self {
        if self.one_of.is_none() { self.one_of = Some(Default::default()) };
        func(self.one_of.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::one_of`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn one_of_push_with(&mut self, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps)) -> &mut Self {
        if self.one_of.is_none() {
            self.one_of = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.one_of.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::one_of`]
    pub  fn one_of_append_from(&mut self, other: impl std::borrow::Borrow<[crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps]>) -> &mut Self {
         if self.one_of.is_none() { self.one_of = Some(Vec::new()); }
         let one_of = &mut self.one_of.as_mut().unwrap();
         for item in other.borrow() {
             one_of.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::pattern`]
    pub  fn pattern_set(&mut self, pattern: impl Into<Option<String>>) -> &mut Self {
        self.pattern = pattern.into(); self
    }

    pub  fn pattern(&mut self) -> &mut String {
        if self.pattern.is_none() { self.pattern = Some(Default::default()) }
        self.pattern.as_mut().unwrap()
    }

    /// Modify [`Self::pattern`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn pattern_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.pattern.is_none() { self.pattern = Some(Default::default()) };
        func(self.pattern.as_mut().unwrap()); self
    }


    /// Set [`Self::pattern_properties`]
    pub  fn pattern_properties_set(&mut self, pattern_properties: impl Into<Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>>) -> &mut Self {
        self.pattern_properties = pattern_properties.into(); self
    }

    pub  fn pattern_properties(&mut self) -> &mut std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps> {
        if self.pattern_properties.is_none() { self.pattern_properties = Some(Default::default()) }
        self.pattern_properties.as_mut().unwrap()
    }

    /// Modify [`Self::pattern_properties`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn pattern_properties_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>)) -> &mut Self {
        if self.pattern_properties.is_none() { self.pattern_properties = Some(Default::default()) };
        func(self.pattern_properties.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::pattern_properties`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn pattern_properties_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps)) -> &mut Self {
        if self.pattern_properties.is_none() {
            self.pattern_properties = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.pattern_properties.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::pattern_properties`]
    pub  fn pattern_properties_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>) -> &mut Self {
         if self.pattern_properties.is_none() { self.pattern_properties = Some(std::collections::BTreeMap::new()); }
         let pattern_properties = &mut self.pattern_properties.as_mut().unwrap();
         for (name, value) in other.borrow() {
             pattern_properties.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::properties`]
    pub  fn properties_set(&mut self, properties: impl Into<Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>>) -> &mut Self {
        self.properties = properties.into(); self
    }

    pub  fn properties(&mut self) -> &mut std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps> {
        if self.properties.is_none() { self.properties = Some(Default::default()) }
        self.properties.as_mut().unwrap()
    }

    /// Modify [`Self::properties`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn properties_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>)) -> &mut Self {
        if self.properties.is_none() { self.properties = Some(Default::default()) };
        func(self.properties.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::properties`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn properties_insert_with(&mut self, name: &str, func: impl FnOnce(&mut crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps)) -> &mut Self {
        if self.properties.is_none() {
            self.properties = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.properties.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::properties`]
    pub  fn properties_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>>) -> &mut Self {
         if self.properties.is_none() { self.properties = Some(std::collections::BTreeMap::new()); }
         let properties = &mut self.properties.as_mut().unwrap();
         for (name, value) in other.borrow() {
             properties.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::required`]
    pub  fn required_set(&mut self, required: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.required = required.into(); self
    }

    pub  fn required(&mut self) -> &mut Vec<String> {
        if self.required.is_none() { self.required = Some(Default::default()) }
        self.required.as_mut().unwrap()
    }

    /// Modify [`Self::required`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn required_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.required.is_none() { self.required = Some(Default::default()) };
        func(self.required.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::required`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn required_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.required.is_none() {
            self.required = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.required.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::required`]
    pub  fn required_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.required.is_none() { self.required = Some(Vec::new()); }
         let required = &mut self.required.as_mut().unwrap();
         for item in other.borrow() {
             required.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::title`]
    pub  fn title_set(&mut self, title: impl Into<Option<String>>) -> &mut Self {
        self.title = title.into(); self
    }

    pub  fn title(&mut self) -> &mut String {
        if self.title.is_none() { self.title = Some(Default::default()) }
        self.title.as_mut().unwrap()
    }

    /// Modify [`Self::title`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn title_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.title.is_none() { self.title = Some(Default::default()) };
        func(self.title.as_mut().unwrap()); self
    }


    /// Set [`Self::type_`]
    pub  fn type_set(&mut self, type_: impl Into<Option<String>>) -> &mut Self {
        self.type_ = type_.into(); self
    }

    pub  fn type_(&mut self) -> &mut String {
        if self.type_.is_none() { self.type_ = Some(Default::default()) }
        self.type_.as_mut().unwrap()
    }

    /// Modify [`Self::type_`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.type_.is_none() { self.type_ = Some(Default::default()) };
        func(self.type_.as_mut().unwrap()); self
    }


    /// Set [`Self::unique_items`]
    pub  fn unique_items_set(&mut self, unique_items: impl Into<Option<bool>>) -> &mut Self {
        self.unique_items = unique_items.into(); self
    }

    pub  fn unique_items(&mut self) -> &mut bool {
        if self.unique_items.is_none() { self.unique_items = Some(Default::default()) }
        self.unique_items.as_mut().unwrap()
    }

    /// Modify [`Self::unique_items`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn unique_items_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.unique_items.is_none() { self.unique_items = Some(Default::default()) };
        func(self.unique_items.as_mut().unwrap()); self
    }


    /// Set [`Self::x_kubernetes_embedded_resource`]
    pub  fn x_kubernetes_embedded_resource_set(&mut self, x_kubernetes_embedded_resource: impl Into<Option<bool>>) -> &mut Self {
        self.x_kubernetes_embedded_resource = x_kubernetes_embedded_resource.into(); self
    }

    pub  fn x_kubernetes_embedded_resource(&mut self) -> &mut bool {
        if self.x_kubernetes_embedded_resource.is_none() { self.x_kubernetes_embedded_resource = Some(Default::default()) }
        self.x_kubernetes_embedded_resource.as_mut().unwrap()
    }

    /// Modify [`Self::x_kubernetes_embedded_resource`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn x_kubernetes_embedded_resource_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.x_kubernetes_embedded_resource.is_none() { self.x_kubernetes_embedded_resource = Some(Default::default()) };
        func(self.x_kubernetes_embedded_resource.as_mut().unwrap()); self
    }


    /// Set [`Self::x_kubernetes_int_or_string`]
    pub  fn x_kubernetes_int_or_string_set(&mut self, x_kubernetes_int_or_string: impl Into<Option<bool>>) -> &mut Self {
        self.x_kubernetes_int_or_string = x_kubernetes_int_or_string.into(); self
    }

    pub  fn x_kubernetes_int_or_string(&mut self) -> &mut bool {
        if self.x_kubernetes_int_or_string.is_none() { self.x_kubernetes_int_or_string = Some(Default::default()) }
        self.x_kubernetes_int_or_string.as_mut().unwrap()
    }

    /// Modify [`Self::x_kubernetes_int_or_string`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn x_kubernetes_int_or_string_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.x_kubernetes_int_or_string.is_none() { self.x_kubernetes_int_or_string = Some(Default::default()) };
        func(self.x_kubernetes_int_or_string.as_mut().unwrap()); self
    }


    /// Set [`Self::x_kubernetes_list_map_keys`]
    pub  fn x_kubernetes_list_map_keys_set(&mut self, x_kubernetes_list_map_keys: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.x_kubernetes_list_map_keys = x_kubernetes_list_map_keys.into(); self
    }

    pub  fn x_kubernetes_list_map_keys(&mut self) -> &mut Vec<String> {
        if self.x_kubernetes_list_map_keys.is_none() { self.x_kubernetes_list_map_keys = Some(Default::default()) }
        self.x_kubernetes_list_map_keys.as_mut().unwrap()
    }

    /// Modify [`Self::x_kubernetes_list_map_keys`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn x_kubernetes_list_map_keys_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.x_kubernetes_list_map_keys.is_none() { self.x_kubernetes_list_map_keys = Some(Default::default()) };
        func(self.x_kubernetes_list_map_keys.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::x_kubernetes_list_map_keys`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn x_kubernetes_list_map_keys_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.x_kubernetes_list_map_keys.is_none() {
            self.x_kubernetes_list_map_keys = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.x_kubernetes_list_map_keys.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::x_kubernetes_list_map_keys`]
    pub  fn x_kubernetes_list_map_keys_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.x_kubernetes_list_map_keys.is_none() { self.x_kubernetes_list_map_keys = Some(Vec::new()); }
         let x_kubernetes_list_map_keys = &mut self.x_kubernetes_list_map_keys.as_mut().unwrap();
         for item in other.borrow() {
             x_kubernetes_list_map_keys.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::x_kubernetes_list_type`]
    pub  fn x_kubernetes_list_type_set(&mut self, x_kubernetes_list_type: impl Into<Option<String>>) -> &mut Self {
        self.x_kubernetes_list_type = x_kubernetes_list_type.into(); self
    }

    pub  fn x_kubernetes_list_type(&mut self) -> &mut String {
        if self.x_kubernetes_list_type.is_none() { self.x_kubernetes_list_type = Some(Default::default()) }
        self.x_kubernetes_list_type.as_mut().unwrap()
    }

    /// Modify [`Self::x_kubernetes_list_type`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn x_kubernetes_list_type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.x_kubernetes_list_type.is_none() { self.x_kubernetes_list_type = Some(Default::default()) };
        func(self.x_kubernetes_list_type.as_mut().unwrap()); self
    }


    /// Set [`Self::x_kubernetes_map_type`]
    pub  fn x_kubernetes_map_type_set(&mut self, x_kubernetes_map_type: impl Into<Option<String>>) -> &mut Self {
        self.x_kubernetes_map_type = x_kubernetes_map_type.into(); self
    }

    pub  fn x_kubernetes_map_type(&mut self) -> &mut String {
        if self.x_kubernetes_map_type.is_none() { self.x_kubernetes_map_type = Some(Default::default()) }
        self.x_kubernetes_map_type.as_mut().unwrap()
    }

    /// Modify [`Self::x_kubernetes_map_type`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn x_kubernetes_map_type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.x_kubernetes_map_type.is_none() { self.x_kubernetes_map_type = Some(Default::default()) };
        func(self.x_kubernetes_map_type.as_mut().unwrap()); self
    }


    /// Set [`Self::x_kubernetes_preserve_unknown_fields`]
    pub  fn x_kubernetes_preserve_unknown_fields_set(&mut self, x_kubernetes_preserve_unknown_fields: impl Into<Option<bool>>) -> &mut Self {
        self.x_kubernetes_preserve_unknown_fields = x_kubernetes_preserve_unknown_fields.into(); self
    }

    pub  fn x_kubernetes_preserve_unknown_fields(&mut self) -> &mut bool {
        if self.x_kubernetes_preserve_unknown_fields.is_none() { self.x_kubernetes_preserve_unknown_fields = Some(Default::default()) }
        self.x_kubernetes_preserve_unknown_fields.as_mut().unwrap()
    }

    /// Modify [`Self::x_kubernetes_preserve_unknown_fields`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn x_kubernetes_preserve_unknown_fields_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.x_kubernetes_preserve_unknown_fields.is_none() { self.x_kubernetes_preserve_unknown_fields = Some(Default::default()) };
        func(self.x_kubernetes_preserve_unknown_fields.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for JSONSchemaProps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ref_path,
            Key_schema,
            Key_additional_items,
            Key_additional_properties,
            Key_all_of,
            Key_any_of,
            Key_default,
            Key_definitions,
            Key_dependencies,
            Key_description,
            Key_enum_,
            Key_example,
            Key_exclusive_maximum,
            Key_exclusive_minimum,
            Key_external_docs,
            Key_format,
            Key_id,
            Key_items,
            Key_max_items,
            Key_max_length,
            Key_max_properties,
            Key_maximum,
            Key_min_items,
            Key_min_length,
            Key_min_properties,
            Key_minimum,
            Key_multiple_of,
            Key_not,
            Key_nullable,
            Key_one_of,
            Key_pattern,
            Key_pattern_properties,
            Key_properties,
            Key_required,
            Key_title,
            Key_type_,
            Key_unique_items,
            Key_x_kubernetes_embedded_resource,
            Key_x_kubernetes_int_or_string,
            Key_x_kubernetes_list_map_keys,
            Key_x_kubernetes_list_type,
            Key_x_kubernetes_map_type,
            Key_x_kubernetes_preserve_unknown_fields,
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
                            "$ref" => Field::Key_ref_path,
                            "$schema" => Field::Key_schema,
                            "additionalItems" => Field::Key_additional_items,
                            "additionalProperties" => Field::Key_additional_properties,
                            "allOf" => Field::Key_all_of,
                            "anyOf" => Field::Key_any_of,
                            "default" => Field::Key_default,
                            "definitions" => Field::Key_definitions,
                            "dependencies" => Field::Key_dependencies,
                            "description" => Field::Key_description,
                            "enum" => Field::Key_enum_,
                            "example" => Field::Key_example,
                            "exclusiveMaximum" => Field::Key_exclusive_maximum,
                            "exclusiveMinimum" => Field::Key_exclusive_minimum,
                            "externalDocs" => Field::Key_external_docs,
                            "format" => Field::Key_format,
                            "id" => Field::Key_id,
                            "items" => Field::Key_items,
                            "maxItems" => Field::Key_max_items,
                            "maxLength" => Field::Key_max_length,
                            "maxProperties" => Field::Key_max_properties,
                            "maximum" => Field::Key_maximum,
                            "minItems" => Field::Key_min_items,
                            "minLength" => Field::Key_min_length,
                            "minProperties" => Field::Key_min_properties,
                            "minimum" => Field::Key_minimum,
                            "multipleOf" => Field::Key_multiple_of,
                            "not" => Field::Key_not,
                            "nullable" => Field::Key_nullable,
                            "oneOf" => Field::Key_one_of,
                            "pattern" => Field::Key_pattern,
                            "patternProperties" => Field::Key_pattern_properties,
                            "properties" => Field::Key_properties,
                            "required" => Field::Key_required,
                            "title" => Field::Key_title,
                            "type" => Field::Key_type_,
                            "uniqueItems" => Field::Key_unique_items,
                            "x-kubernetes-embedded-resource" => Field::Key_x_kubernetes_embedded_resource,
                            "x-kubernetes-int-or-string" => Field::Key_x_kubernetes_int_or_string,
                            "x-kubernetes-list-map-keys" => Field::Key_x_kubernetes_list_map_keys,
                            "x-kubernetes-list-type" => Field::Key_x_kubernetes_list_type,
                            "x-kubernetes-map-type" => Field::Key_x_kubernetes_map_type,
                            "x-kubernetes-preserve-unknown-fields" => Field::Key_x_kubernetes_preserve_unknown_fields,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = JSONSchemaProps;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("JSONSchemaProps")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ref_path: Option<String> = None;
                let mut value_schema: Option<String> = None;
                let mut value_additional_items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool> = None;
                let mut value_additional_properties: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool> = None;
                let mut value_all_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_any_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_default: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON> = None;
                let mut value_definitions: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_dependencies: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray>> = None;
                let mut value_description: Option<String> = None;
                let mut value_enum_: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>> = None;
                let mut value_example: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON> = None;
                let mut value_exclusive_maximum: Option<bool> = None;
                let mut value_exclusive_minimum: Option<bool> = None;
                let mut value_external_docs: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation> = None;
                let mut value_format: Option<String> = None;
                let mut value_id: Option<String> = None;
                let mut value_items: Option<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray> = None;
                let mut value_max_items: Option<i64> = None;
                let mut value_max_length: Option<i64> = None;
                let mut value_max_properties: Option<i64> = None;
                let mut value_maximum: Option<f64> = None;
                let mut value_min_items: Option<i64> = None;
                let mut value_min_length: Option<i64> = None;
                let mut value_min_properties: Option<i64> = None;
                let mut value_minimum: Option<f64> = None;
                let mut value_multiple_of: Option<f64> = None;
                let mut value_not: Option<Box<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_nullable: Option<bool> = None;
                let mut value_one_of: Option<Vec<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_pattern: Option<String> = None;
                let mut value_pattern_properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_properties: Option<std::collections::BTreeMap<String, crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>> = None;
                let mut value_required: Option<Vec<String>> = None;
                let mut value_title: Option<String> = None;
                let mut value_type_: Option<String> = None;
                let mut value_unique_items: Option<bool> = None;
                let mut value_x_kubernetes_embedded_resource: Option<bool> = None;
                let mut value_x_kubernetes_int_or_string: Option<bool> = None;
                let mut value_x_kubernetes_list_map_keys: Option<Vec<String>> = None;
                let mut value_x_kubernetes_list_type: Option<String> = None;
                let mut value_x_kubernetes_map_type: Option<String> = None;
                let mut value_x_kubernetes_preserve_unknown_fields: Option<bool> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ref_path => value_ref_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_schema => value_schema = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_additional_items => value_additional_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_additional_properties => value_additional_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_all_of => value_all_of = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_any_of => value_any_of = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default => value_default = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_definitions => value_definitions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_dependencies => value_dependencies = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_description => value_description = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_enum_ => value_enum_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_example => value_example = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_exclusive_maximum => value_exclusive_maximum = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_exclusive_minimum => value_exclusive_minimum = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_docs => value_external_docs = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_format => value_format = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_id => value_id = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_items => value_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_items => value_max_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_length => value_max_length = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_max_properties => value_max_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_maximum => value_maximum = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_items => value_min_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_length => value_min_length = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_min_properties => value_min_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_minimum => value_minimum = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_multiple_of => value_multiple_of = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_not => value_not = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_nullable => value_nullable = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_one_of => value_one_of = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pattern => value_pattern = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pattern_properties => value_pattern_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_properties => value_properties = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_required => value_required = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_title => value_title = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unique_items => value_unique_items = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_embedded_resource => value_x_kubernetes_embedded_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_int_or_string => value_x_kubernetes_int_or_string = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_list_map_keys => value_x_kubernetes_list_map_keys = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_list_type => value_x_kubernetes_list_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_map_type => value_x_kubernetes_map_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_x_kubernetes_preserve_unknown_fields => value_x_kubernetes_preserve_unknown_fields = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JSONSchemaProps {
                    ref_path: value_ref_path,
                    schema: value_schema,
                    additional_items: value_additional_items,
                    additional_properties: value_additional_properties,
                    all_of: value_all_of,
                    any_of: value_any_of,
                    default: value_default,
                    definitions: value_definitions,
                    dependencies: value_dependencies,
                    description: value_description,
                    enum_: value_enum_,
                    example: value_example,
                    exclusive_maximum: value_exclusive_maximum,
                    exclusive_minimum: value_exclusive_minimum,
                    external_docs: value_external_docs,
                    format: value_format,
                    id: value_id,
                    items: value_items,
                    max_items: value_max_items,
                    max_length: value_max_length,
                    max_properties: value_max_properties,
                    maximum: value_maximum,
                    min_items: value_min_items,
                    min_length: value_min_length,
                    min_properties: value_min_properties,
                    minimum: value_minimum,
                    multiple_of: value_multiple_of,
                    not: value_not,
                    nullable: value_nullable,
                    one_of: value_one_of,
                    pattern: value_pattern,
                    pattern_properties: value_pattern_properties,
                    properties: value_properties,
                    required: value_required,
                    title: value_title,
                    type_: value_type_,
                    unique_items: value_unique_items,
                    x_kubernetes_embedded_resource: value_x_kubernetes_embedded_resource,
                    x_kubernetes_int_or_string: value_x_kubernetes_int_or_string,
                    x_kubernetes_list_map_keys: value_x_kubernetes_list_map_keys,
                    x_kubernetes_list_type: value_x_kubernetes_list_type,
                    x_kubernetes_map_type: value_x_kubernetes_map_type,
                    x_kubernetes_preserve_unknown_fields: value_x_kubernetes_preserve_unknown_fields,
                })
            }
        }

        deserializer.deserialize_struct(
            "JSONSchemaProps",
            &[
                "$ref",
                "$schema",
                "additionalItems",
                "additionalProperties",
                "allOf",
                "anyOf",
                "default",
                "definitions",
                "dependencies",
                "description",
                "enum",
                "example",
                "exclusiveMaximum",
                "exclusiveMinimum",
                "externalDocs",
                "format",
                "id",
                "items",
                "maxItems",
                "maxLength",
                "maxProperties",
                "maximum",
                "minItems",
                "minLength",
                "minProperties",
                "minimum",
                "multipleOf",
                "not",
                "nullable",
                "oneOf",
                "pattern",
                "patternProperties",
                "properties",
                "required",
                "title",
                "type",
                "uniqueItems",
                "x-kubernetes-embedded-resource",
                "x-kubernetes-int-or-string",
                "x-kubernetes-list-map-keys",
                "x-kubernetes-list-type",
                "x-kubernetes-map-type",
                "x-kubernetes-preserve-unknown-fields",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for JSONSchemaProps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JSONSchemaProps",
            self.ref_path.as_ref().map_or(0, |_| 1) +
            self.schema.as_ref().map_or(0, |_| 1) +
            self.additional_items.as_ref().map_or(0, |_| 1) +
            self.additional_properties.as_ref().map_or(0, |_| 1) +
            self.all_of.as_ref().map_or(0, |_| 1) +
            self.any_of.as_ref().map_or(0, |_| 1) +
            self.default.as_ref().map_or(0, |_| 1) +
            self.definitions.as_ref().map_or(0, |_| 1) +
            self.dependencies.as_ref().map_or(0, |_| 1) +
            self.description.as_ref().map_or(0, |_| 1) +
            self.enum_.as_ref().map_or(0, |_| 1) +
            self.example.as_ref().map_or(0, |_| 1) +
            self.exclusive_maximum.as_ref().map_or(0, |_| 1) +
            self.exclusive_minimum.as_ref().map_or(0, |_| 1) +
            self.external_docs.as_ref().map_or(0, |_| 1) +
            self.format.as_ref().map_or(0, |_| 1) +
            self.id.as_ref().map_or(0, |_| 1) +
            self.items.as_ref().map_or(0, |_| 1) +
            self.max_items.as_ref().map_or(0, |_| 1) +
            self.max_length.as_ref().map_or(0, |_| 1) +
            self.max_properties.as_ref().map_or(0, |_| 1) +
            self.maximum.as_ref().map_or(0, |_| 1) +
            self.min_items.as_ref().map_or(0, |_| 1) +
            self.min_length.as_ref().map_or(0, |_| 1) +
            self.min_properties.as_ref().map_or(0, |_| 1) +
            self.minimum.as_ref().map_or(0, |_| 1) +
            self.multiple_of.as_ref().map_or(0, |_| 1) +
            self.not.as_ref().map_or(0, |_| 1) +
            self.nullable.as_ref().map_or(0, |_| 1) +
            self.one_of.as_ref().map_or(0, |_| 1) +
            self.pattern.as_ref().map_or(0, |_| 1) +
            self.pattern_properties.as_ref().map_or(0, |_| 1) +
            self.properties.as_ref().map_or(0, |_| 1) +
            self.required.as_ref().map_or(0, |_| 1) +
            self.title.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1) +
            self.unique_items.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_embedded_resource.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_int_or_string.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_list_map_keys.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_list_type.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_map_type.as_ref().map_or(0, |_| 1) +
            self.x_kubernetes_preserve_unknown_fields.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ref_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "$ref", value)?;
        }
        if let Some(value) = &self.schema {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "$schema", value)?;
        }
        if let Some(value) = &self.additional_items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "additionalItems", value)?;
        }
        if let Some(value) = &self.additional_properties {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "additionalProperties", value)?;
        }
        if let Some(value) = &self.all_of {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allOf", value)?;
        }
        if let Some(value) = &self.any_of {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "anyOf", value)?;
        }
        if let Some(value) = &self.default {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "default", value)?;
        }
        if let Some(value) = &self.definitions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "definitions", value)?;
        }
        if let Some(value) = &self.dependencies {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "dependencies", value)?;
        }
        if let Some(value) = &self.description {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.enum_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "enum", value)?;
        }
        if let Some(value) = &self.example {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "example", value)?;
        }
        if let Some(value) = &self.exclusive_maximum {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exclusiveMaximum", value)?;
        }
        if let Some(value) = &self.exclusive_minimum {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "exclusiveMinimum", value)?;
        }
        if let Some(value) = &self.external_docs {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalDocs", value)?;
        }
        if let Some(value) = &self.format {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "format", value)?;
        }
        if let Some(value) = &self.id {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "id", value)?;
        }
        if let Some(value) = &self.items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "items", value)?;
        }
        if let Some(value) = &self.max_items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxItems", value)?;
        }
        if let Some(value) = &self.max_length {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxLength", value)?;
        }
        if let Some(value) = &self.max_properties {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maxProperties", value)?;
        }
        if let Some(value) = &self.maximum {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "maximum", value)?;
        }
        if let Some(value) = &self.min_items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minItems", value)?;
        }
        if let Some(value) = &self.min_length {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minLength", value)?;
        }
        if let Some(value) = &self.min_properties {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minProperties", value)?;
        }
        if let Some(value) = &self.minimum {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "minimum", value)?;
        }
        if let Some(value) = &self.multiple_of {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "multipleOf", value)?;
        }
        if let Some(value) = &self.not {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "not", value)?;
        }
        if let Some(value) = &self.nullable {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nullable", value)?;
        }
        if let Some(value) = &self.one_of {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "oneOf", value)?;
        }
        if let Some(value) = &self.pattern {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pattern", value)?;
        }
        if let Some(value) = &self.pattern_properties {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "patternProperties", value)?;
        }
        if let Some(value) = &self.properties {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "properties", value)?;
        }
        if let Some(value) = &self.required {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "required", value)?;
        }
        if let Some(value) = &self.title {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "title", value)?;
        }
        if let Some(value) = &self.type_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        if let Some(value) = &self.unique_items {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uniqueItems", value)?;
        }
        if let Some(value) = &self.x_kubernetes_embedded_resource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-embedded-resource", value)?;
        }
        if let Some(value) = &self.x_kubernetes_int_or_string {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-int-or-string", value)?;
        }
        if let Some(value) = &self.x_kubernetes_list_map_keys {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-list-map-keys", value)?;
        }
        if let Some(value) = &self.x_kubernetes_list_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-list-type", value)?;
        }
        if let Some(value) = &self.x_kubernetes_map_type {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-map-type", value)?;
        }
        if let Some(value) = &self.x_kubernetes_preserve_unknown_fields {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "x-kubernetes-preserve-unknown-fields", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JSONSchemaProps {
    fn schema_name() -> String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaProps".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/).".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "$ref".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "$schema".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "additionalItems".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool>(),
                    ),
                    (
                        "additionalProperties".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool>(),
                    ),
                    (
                        "allOf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "anyOf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "default".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("default is a default value for undefined object fields. Defaulting is a beta feature under the CustomResourceDefaulting feature gate. Defaulting requires spec.preserveUnknownFields to be false.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "definitions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "dependencies".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "description".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "enum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "example".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON>(),
                    ),
                    (
                        "exclusiveMaximum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "exclusiveMinimum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "externalDocs".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation>(),
                    ),
                    (
                        "format".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("format is an OpenAPI v3 format string. Unknown formats are ignored. The following formats are validated:\n\n- bsonobjectid: a bson object ID, i.e. a 24 characters hex string - uri: an URI as parsed by Golang net/url.ParseRequestURI - email: an email address as parsed by Golang net/mail.ParseAddress - hostname: a valid representation for an Internet host name, as defined by RFC 1034, section 3.1 [RFC1034]. - ipv4: an IPv4 IP as parsed by Golang net.ParseIP - ipv6: an IPv6 IP as parsed by Golang net.ParseIP - cidr: a CIDR as parsed by Golang net.ParseCIDR - mac: a MAC address as parsed by Golang net.ParseMAC - uuid: an UUID that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{12}$ - uuid3: an UUID3 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?3[0-9a-f]{3}-?[0-9a-f]{4}-?[0-9a-f]{12}$ - uuid4: an UUID4 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?4[0-9a-f]{3}-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}$ - uuid5: an UUID5 that allows uppercase defined by the regex (?i)^[0-9a-f]{8}-?[0-9a-f]{4}-?5[0-9a-f]{3}-?[89ab][0-9a-f]{3}-?[0-9a-f]{12}$ - isbn: an ISBN10 or ISBN13 number string like \"0321751043\" or \"978-0321751041\" - isbn10: an ISBN10 number string like \"0321751043\" - isbn13: an ISBN13 number string like \"978-0321751041\" - creditcard: a credit card number defined by the regex ^(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9][0-9])[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|(?:2131|1800|35\\d{3})\\d{11})$ with any non digit characters mixed in - ssn: a U.S. social security number following the regex ^\\d{3}[- ]?\\d{2}[- ]?\\d{4}$ - hexcolor: an hexadecimal color code like \"#FFFFFF: following the regex ^#?([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$ - rgbcolor: an RGB color code like rgb like \"rgb(255,255,2559\" - byte: base64 encoded binary data - password: any kind of string - date: a date string like \"2006-01-02\" as defined by full-date in RFC3339 - duration: a duration string like \"22 ns\" as parsed by Golang time.ParseDuration or compatible with Scala duration format - datetime: a date time string like \"2014-12-15T19:30:20.000Z\" as defined by date-time in RFC3339.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "id".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "items".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray>(),
                    ),
                    (
                        "maxItems".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maxLength".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maxProperties".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "maximum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Number))),
                            format: Some("double".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minItems".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minLength".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minProperties".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "minimum".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Number))),
                            format: Some("double".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "multipleOf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Number))),
                            format: Some("double".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "not".to_owned(),
                        __gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>(),
                    ),
                    (
                        "nullable".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "oneOf".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "pattern".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "patternProperties".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "properties".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(__gen.subschema_for::<crate::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps>())),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "required".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "title".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "uniqueItems".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "x-kubernetes-embedded-resource".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-embedded-resource defines that the value is an embedded Kubernetes runtime.Object, with TypeMeta and ObjectMeta. The type must be object. It is allowed to further restrict the embedded object. kind, apiVersion and metadata are validated automatically. x-kubernetes-preserve-unknown-fields is allowed to be true, but does not have to be if the object is fully specified (up to kind, apiVersion, metadata).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "x-kubernetes-int-or-string".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-int-or-string specifies that this value is either an integer or a string. If this is true, an empty type is allowed and type as child of anyOf is permitted if following one of the following patterns:\n\n1) anyOf:\n   - type: integer\n   - type: string\n2) allOf:\n   - anyOf:\n     - type: integer\n     - type: string\n   - ... zero or more".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "x-kubernetes-list-map-keys".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-list-map-keys annotates an array with the x-kubernetes-list-type `map` by specifying the keys used as the index of the map.\n\nThis tag MUST only be used on lists that have the \"x-kubernetes-list-type\" extension set to \"map\". Also, the values specified for this attribute must be a scalar typed field of the child structure (no nesting is supported).\n\nThe properties specified must either be required or have a default value, to ensure those properties are present for all list items.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "x-kubernetes-list-type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-list-type annotates an array to further describe its topology. This extension must only be used on lists and may have 3 possible values:\n\n1) `atomic`: the list is treated as a single entity, like a scalar.\n     Atomic lists will be entirely replaced when updated. This extension\n     may be used on any type of list (struct, scalar, ...).\n2) `set`:\n     Sets are lists that must not have multiple items with the same value. Each\n     value must be a scalar, an object with x-kubernetes-map-type `atomic` or an\n     array with x-kubernetes-list-type `atomic`.\n3) `map`:\n     These lists are like maps in that their elements have a non-index key\n     used to identify them. Order is preserved upon merge. The map tag\n     must only be used on a list with elements of type object.\nDefaults to atomic for arrays.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "x-kubernetes-map-type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-map-type annotates an object to further describe its topology. This extension must only be used when type is object and may have 2 possible values:\n\n1) `granular`:\n     These maps are actual maps (key-value pairs) and each fields are independent\n     from each other (they can each be manipulated by separate actors). This is\n     the default behaviour for all maps.\n2) `atomic`: the list is treated as a single entity, like a scalar.\n     Atomic maps will be entirely replaced when updated.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "x-kubernetes-preserve-unknown-fields".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("x-kubernetes-preserve-unknown-fields stops the API server decoding step from pruning fields which are not specified in the validation schema. This affects fields recursively, but switches back to normal pruning behaviour if nested properties or additionalProperties are specified in the schema. This can either be true or undefined. False is forbidden.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
