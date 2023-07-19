pub(crate) const ALL: &[SupportedVersion] = &[
	SupportedVersion::V1_20,
	SupportedVersion::V1_21,
	SupportedVersion::V1_22,
	SupportedVersion::V1_23,
	SupportedVersion::V1_24,
	SupportedVersion::V1_25,
	SupportedVersion::V1_26,
	SupportedVersion::V1_27,
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum SupportedVersion {
	V1_20,
	V1_21,
	V1_22,
	V1_23,
	V1_24,
	V1_25,
	V1_26,
	V1_27,
}

impl SupportedVersion {
	pub(crate) fn name(self) -> &'static str {
		match self {
			SupportedVersion::V1_20 => "1.20",
			SupportedVersion::V1_21 => "1.21",
			SupportedVersion::V1_22 => "1.22",
			SupportedVersion::V1_23 => "1.23",
			SupportedVersion::V1_24 => "1.24",
			SupportedVersion::V1_25 => "1.25",
			SupportedVersion::V1_26 => "1.26",
			SupportedVersion::V1_27 => "1.27",
		}
	}

	pub(crate) fn mod_root(self) -> &'static str {
		match self {
			SupportedVersion::V1_20 => "v1_20",
			SupportedVersion::V1_21 => "v1_21",
			SupportedVersion::V1_22 => "v1_22",
			SupportedVersion::V1_23 => "v1_23",
			SupportedVersion::V1_24 => "v1_24",
			SupportedVersion::V1_25 => "v1_25",
			SupportedVersion::V1_26 => "v1_26",
			SupportedVersion::V1_27 => "v1_27",
		}
	}

	pub(crate) fn spec_url(self) -> &'static str {
		match self {
			SupportedVersion::V1_20 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.20.15/api/openapi-spec/swagger.json",
			SupportedVersion::V1_21 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.21.14/api/openapi-spec/swagger.json",
			SupportedVersion::V1_22 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.22.17/api/openapi-spec/swagger.json",
			SupportedVersion::V1_23 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.23.17/api/openapi-spec/swagger.json",
			SupportedVersion::V1_24 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.24.15/api/openapi-spec/swagger.json",
			SupportedVersion::V1_25 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.25.12/api/openapi-spec/swagger.json",
			SupportedVersion::V1_26 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.26.7/api/openapi-spec/swagger.json",
			SupportedVersion::V1_27 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.27.4/api/openapi-spec/swagger.json",
		}
	}

	pub(crate) fn fixup(self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		#[allow(clippy::match_same_arms, clippy::type_complexity)]
		let upstream_bugs_fixups: &[fn(&mut crate::swagger20::Spec) -> Result<(), crate::Error>] = match self {
			SupportedVersion::V1_20 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::containerimage,
				crate::fixups::upstream_bugs::optional_properties::eventsv1beta1_event,
				crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_21 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::containerimage,
				crate::fixups::upstream_bugs::optional_properties::eventsv1beta1_event,
				crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_22 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::eventsv1beta1_event,
				crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_23 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::eventsv1beta1_event,
				crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_24 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::eventsv1beta1_event,
				crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_25 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
			],

			SupportedVersion::V1_26 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
				crate::fixups::upstream_bugs::required_properties::validating_admission_policy_binding_list,
				crate::fixups::upstream_bugs::required_properties::validating_admission_policy_list,
				crate::fixups::upstream_bugs::status_extra_gvk,
			],

			SupportedVersion::V1_27 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::eventsv1_event,
				crate::fixups::upstream_bugs::pod_exec_command_parameter_type,
				crate::fixups::upstream_bugs::required_properties::validating_admission_policy_binding_list,
				crate::fixups::upstream_bugs::required_properties::validating_admission_policy_list,
				crate::fixups::upstream_bugs::status_extra_gvk,
			],
		};

		let special_fixups = &[
			crate::fixups::special::json_ty::json_schema_props_or_array,
			crate::fixups::special::json_ty::json_schema_props_or_bool,
			crate::fixups::special::json_ty::json_schema_props_or_string_array,
			crate::fixups::special::create_delete_optional,
			crate::fixups::special::create_optionals,
			crate::fixups::special::patch,
			crate::fixups::special::remove_delete_collection_operations_query_parameters,
			crate::fixups::special::remove_delete_operations_query_parameters,
			crate::fixups::special::remove_read_operations_query_parameters,
			crate::fixups::special::separate_watch_from_list_operations,
			crate::fixups::special::watch_event,
			crate::fixups::special::list, // Must run after separate_watch_from_list_operations
			crate::fixups::special::response_types,
			crate::fixups::special::resource_metadata_not_optional,
		];

		for fixup in upstream_bugs_fixups.iter().chain(special_fixups) {
			fixup(spec)?;
		}

		Ok(())
	}
}
