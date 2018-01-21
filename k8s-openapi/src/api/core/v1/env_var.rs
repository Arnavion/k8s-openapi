// Generated from definition io.k8s.api.core.v1.EnvVar

/// EnvVar represents an environment variable present in a Container.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EnvVar {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,

    /// Variable references $(VAR_NAME) are expanded using the previous defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(rename = "valueFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: Option<::api::core::v1::EnvVarSource>,
}
