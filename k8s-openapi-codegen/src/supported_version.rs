pub(crate) const ALL: &[SupportedVersion] = &[
    SupportedVersion::V1_30,
    SupportedVersion::V1_31,
    SupportedVersion::V1_32,
    SupportedVersion::V1_33,
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum SupportedVersion {
    V1_30,
    V1_31,
    V1_32,
    V1_33,
}

impl SupportedVersion {
    pub(crate) fn name(self) -> &'static str {
        match self {
            SupportedVersion::V1_30 => "1.30",
            SupportedVersion::V1_31 => "1.31",
            SupportedVersion::V1_32 => "1.32",
            SupportedVersion::V1_33 => "1.33",
        }
    }

    pub(crate) fn mod_root(self) -> &'static str {
        match self {
            SupportedVersion::V1_30 => "v1_30",
            SupportedVersion::V1_31 => "v1_31",
            SupportedVersion::V1_32 => "v1_32",
            SupportedVersion::V1_33 => "v1_33",
        }
    }

    pub(crate) fn spec_url(self) -> &'static str {
        match self {
            SupportedVersion::V1_30 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.30.14/api/openapi-spec/swagger.json",
            SupportedVersion::V1_31 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.31.10/api/openapi-spec/swagger.json",
            SupportedVersion::V1_32 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.32.6/api/openapi-spec/swagger.json",
            SupportedVersion::V1_33 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.33.2/api/openapi-spec/swagger.json",
        }
    }

    pub(crate) fn fixup(self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
        #[allow(clippy::match_same_arms, clippy::type_complexity)]
        let upstream_bugs_fixups: &[fn(&mut crate::swagger20::Spec) -> Result<(), crate::Error>] = match self {
            SupportedVersion::V1_30 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::appsv1_statefulsetspec,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::config_map_env_source,
                crate::fixups::upstream_bugs::required_properties::config_map_key_selector,
                crate::fixups::upstream_bugs::required_properties::config_map_projection,
                crate::fixups::upstream_bugs::required_properties::config_map_volume_source,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
                crate::fixups::upstream_bugs::required_properties::secret_env_source,
                crate::fixups::upstream_bugs::required_properties::secret_key_selector,
                crate::fixups::upstream_bugs::required_properties::secret_projection,
                crate::fixups::upstream_bugs::status_extra_gvk,
            ],

            SupportedVersion::V1_31 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::appsv1_statefulsetspec,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::config_map_env_source,
                crate::fixups::upstream_bugs::required_properties::config_map_key_selector,
                crate::fixups::upstream_bugs::required_properties::config_map_projection,
                crate::fixups::upstream_bugs::required_properties::config_map_volume_source,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
                crate::fixups::upstream_bugs::required_properties::secret_env_source,
                crate::fixups::upstream_bugs::required_properties::secret_key_selector,
                crate::fixups::upstream_bugs::required_properties::secret_projection,
            ],

            SupportedVersion::V1_32 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::appsv1_statefulsetspec,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::config_map_env_source,
                crate::fixups::upstream_bugs::required_properties::config_map_key_selector,
                crate::fixups::upstream_bugs::required_properties::config_map_projection,
                crate::fixups::upstream_bugs::required_properties::config_map_volume_source,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
                crate::fixups::upstream_bugs::required_properties::secret_env_source,
                crate::fixups::upstream_bugs::required_properties::secret_key_selector,
                crate::fixups::upstream_bugs::required_properties::secret_projection,
            ],

            SupportedVersion::V1_33 => &[
                crate::fixups::upstream_bugs::connect_options_gvk,
                crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
                crate::fixups::upstream_bugs::required_properties::config_map_env_source,
                crate::fixups::upstream_bugs::required_properties::config_map_key_selector,
                crate::fixups::upstream_bugs::required_properties::config_map_projection,
                crate::fixups::upstream_bugs::required_properties::config_map_volume_source,
                crate::fixups::upstream_bugs::required_properties::local_object_reference,
                crate::fixups::upstream_bugs::required_properties::secret_env_source,
                crate::fixups::upstream_bugs::required_properties::secret_key_selector,
                crate::fixups::upstream_bugs::required_properties::secret_projection,
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
