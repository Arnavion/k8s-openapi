pub(crate) const ALL: &[SupportedVersion] = &[
    SupportedVersion::V1_24,
    SupportedVersion::V1_25,
    SupportedVersion::V1_26,
    SupportedVersion::V1_27,
    SupportedVersion::V1_28,
    SupportedVersion::V1_29,
    SupportedVersion::V1_30,
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum SupportedVersion {
    V1_24,
    V1_25,
    V1_26,
    V1_27,
    V1_28,
    V1_29,
    V1_30,
}

impl SupportedVersion {
    pub(crate) fn name(self) -> &'static str {
        match self {
            SupportedVersion::V1_24 => "1.24",
            SupportedVersion::V1_25 => "1.25",
            SupportedVersion::V1_26 => "1.26",
            SupportedVersion::V1_27 => "1.27",
            SupportedVersion::V1_28 => "1.28",
            SupportedVersion::V1_29 => "1.29",
            SupportedVersion::V1_30 => "1.30",
        }
    }

    pub(crate) fn mod_root(self) -> &'static str {
        match self {
            SupportedVersion::V1_24 => "v1_24",
            SupportedVersion::V1_25 => "v1_25",
            SupportedVersion::V1_26 => "v1_26",
            SupportedVersion::V1_27 => "v1_27",
            SupportedVersion::V1_28 => "v1_28",
            SupportedVersion::V1_29 => "v1_29",
            SupportedVersion::V1_30 => "v1_30",
        }
    }

    pub(crate) fn spec_url(self) -> &'static str {
        match self {
            SupportedVersion::V1_24 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.24.17/api/openapi-spec/swagger.json",
            SupportedVersion::V1_25 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.25.16/api/openapi-spec/swagger.json",
            SupportedVersion::V1_26 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.26.15/api/openapi-spec/swagger.json",
            SupportedVersion::V1_27 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.27.16/api/openapi-spec/swagger.json",
            SupportedVersion::V1_28 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.28.12/api/openapi-spec/swagger.json",
            SupportedVersion::V1_29 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.29.7/api/openapi-spec/swagger.json",
            SupportedVersion::V1_30 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.30.3/api/openapi-spec/swagger.json",
        }
    }

    pub(crate) fn fixup(self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        #[allow(clippy::match_same_arms, clippy::type_complexity)]
        let upstream_bugs_fixups: &[fn(&mut crate::swagger20::Spec) -> Result<(), crate::Error>] = match self {
            SupportedVersion::V1_24 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::eventsv1beta1_event,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::config_map_env_source,
                crate::fixups::upstream_bugs::required_properties::config_map_key_selector,
                crate::fixups::upstream_bugs::required_properties::config_map_projection,
                crate::fixups::upstream_bugs::required_properties::config_map_volume_source,
                crate::fixups::upstream_bugs::required_properties::host_alias,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
                crate::fixups::upstream_bugs::required_properties::secret_env_source,
                crate::fixups::upstream_bugs::required_properties::secret_key_selector,
                crate::fixups::upstream_bugs::required_properties::secret_projection,
            ],

            SupportedVersion::V1_25 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
            ],

            SupportedVersion::V1_26 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::alpha1_validating_admission_policy_binding_list,
                crate::fixups::upstream_bugs::required_properties::alpha1_validating_admission_policy_list,
                crate::fixups::upstream_bugs::required_properties::config_map_env_source,
                crate::fixups::upstream_bugs::required_properties::config_map_key_selector,
                crate::fixups::upstream_bugs::required_properties::config_map_projection,
                crate::fixups::upstream_bugs::required_properties::config_map_volume_source,
                crate::fixups::upstream_bugs::required_properties::host_alias,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
                crate::fixups::upstream_bugs::required_properties::secret_env_source,
                crate::fixups::upstream_bugs::required_properties::secret_key_selector,
                crate::fixups::upstream_bugs::required_properties::secret_projection,
                crate::fixups::upstream_bugs::status_extra_gvk,
                crate::fixups::upstream_bugs::v1alpha1_resource_claim_scheduling_status_unsuitable_nodes_merge_strategy,
            ],

            SupportedVersion::V1_27 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::alpha1_validating_admission_policy_binding_list,
                crate::fixups::upstream_bugs::required_properties::alpha1_validating_admission_policy_list,
                crate::fixups::upstream_bugs::required_properties::config_map_env_source,
                crate::fixups::upstream_bugs::required_properties::config_map_key_selector,
                crate::fixups::upstream_bugs::required_properties::config_map_projection,
                crate::fixups::upstream_bugs::required_properties::config_map_volume_source,
                crate::fixups::upstream_bugs::required_properties::host_alias,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
                crate::fixups::upstream_bugs::required_properties::secret_env_source,
                crate::fixups::upstream_bugs::required_properties::secret_key_selector,
                crate::fixups::upstream_bugs::required_properties::secret_projection,
                crate::fixups::upstream_bugs::pod_scheduling_context_spec_potential_nodes_merge_strategy,
                crate::fixups::upstream_bugs::status_extra_gvk,
                crate::fixups::upstream_bugs::v1alpha2_resource_claim_scheduling_status_unsuitable_nodes_merge_strategy,
            ],

            SupportedVersion::V1_28 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::alpha1_validating_admission_policy_binding_list,
                crate::fixups::upstream_bugs::required_properties::alpha1_validating_admission_policy_list,
                crate::fixups::upstream_bugs::required_properties::beta1_validating_admission_policy_binding_list,
                crate::fixups::upstream_bugs::required_properties::beta1_validating_admission_policy_list,
                crate::fixups::upstream_bugs::required_properties::config_map_env_source,
                crate::fixups::upstream_bugs::required_properties::config_map_key_selector,
                crate::fixups::upstream_bugs::required_properties::config_map_projection,
                crate::fixups::upstream_bugs::required_properties::config_map_volume_source,
                crate::fixups::upstream_bugs::required_properties::host_alias,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
                crate::fixups::upstream_bugs::required_properties::secret_env_source,
                crate::fixups::upstream_bugs::required_properties::secret_key_selector,
                crate::fixups::upstream_bugs::required_properties::secret_projection,
                crate::fixups::upstream_bugs::pod_scheduling_context_spec_potential_nodes_merge_strategy,
                crate::fixups::upstream_bugs::status_extra_gvk,
                crate::fixups::upstream_bugs::v1alpha2_resource_claim_scheduling_status_unsuitable_nodes_merge_strategy,
            ],

            SupportedVersion::V1_29 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::alpha1_validating_admission_policy_binding_list,
                crate::fixups::upstream_bugs::required_properties::alpha1_validating_admission_policy_list,
                crate::fixups::upstream_bugs::required_properties::beta1_validating_admission_policy_binding_list,
                crate::fixups::upstream_bugs::required_properties::beta1_validating_admission_policy_list,
                crate::fixups::upstream_bugs::required_properties::config_map_env_source,
                crate::fixups::upstream_bugs::required_properties::config_map_key_selector,
                crate::fixups::upstream_bugs::required_properties::config_map_projection,
                crate::fixups::upstream_bugs::required_properties::config_map_volume_source,
                crate::fixups::upstream_bugs::required_properties::host_alias,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
                crate::fixups::upstream_bugs::required_properties::secret_env_source,
                crate::fixups::upstream_bugs::required_properties::secret_key_selector,
                crate::fixups::upstream_bugs::required_properties::secret_projection,
                crate::fixups::upstream_bugs::status_extra_gvk,
            ],

            SupportedVersion::V1_30 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::alpha1_validating_admission_policy_binding_list,
                crate::fixups::upstream_bugs::required_properties::alpha1_validating_admission_policy_list,
                crate::fixups::upstream_bugs::required_properties::beta1_validating_admission_policy_binding_list,
                crate::fixups::upstream_bugs::required_properties::beta1_validating_admission_policy_list,
                crate::fixups::upstream_bugs::required_properties::config_map_env_source,
                crate::fixups::upstream_bugs::required_properties::config_map_key_selector,
                crate::fixups::upstream_bugs::required_properties::config_map_projection,
                crate::fixups::upstream_bugs::required_properties::config_map_volume_source,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
                crate::fixups::upstream_bugs::required_properties::secret_env_source,
                crate::fixups::upstream_bugs::required_properties::secret_key_selector,
                crate::fixups::upstream_bugs::required_properties::secret_projection,
                crate::fixups::upstream_bugs::required_properties::validating_admission_policy_binding_list,
                crate::fixups::upstream_bugs::required_properties::validating_admission_policy_list,
                crate::fixups::upstream_bugs::status_extra_gvk,
            ],
        };

        let special_fixups = &[
            crate::fixups::special::json_ty::json_schema_props_or_array,
            crate::fixups::special::json_ty::json_schema_props_or_bool,
            crate::fixups::special::json_ty::json_schema_props_or_string_array,
            crate::fixups::special::patch,
            crate::fixups::special::watch_event,
            crate::fixups::special::list,
            crate::fixups::special::resource_metadata_not_optional,
        ];

        for fixup in upstream_bugs_fixups.iter().chain(special_fixups) {
            fixup(spec)?;
        }

        Ok(())
    }
}
