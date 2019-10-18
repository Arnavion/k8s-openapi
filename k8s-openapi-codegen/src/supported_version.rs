pub(crate) const ALL: &[SupportedVersion] = &[
	SupportedVersion::V1_8,
	SupportedVersion::V1_9,
	SupportedVersion::V1_10,
	SupportedVersion::V1_11,
	SupportedVersion::V1_12,
	SupportedVersion::V1_13,
	SupportedVersion::V1_14,
	SupportedVersion::V1_15,
	SupportedVersion::V1_16,
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum SupportedVersion {
	V1_8,
	V1_9,
	V1_10,
	V1_11,
	V1_12,
	V1_13,
	V1_14,
	V1_15,
	V1_16,
}

impl SupportedVersion {
	pub(crate) fn mod_root(self) -> &'static str {
		match self {
			SupportedVersion::V1_8 => "v1_8",
			SupportedVersion::V1_9 => "v1_9",
			SupportedVersion::V1_10 => "v1_10",
			SupportedVersion::V1_11 => "v1_11",
			SupportedVersion::V1_12 => "v1_12",
			SupportedVersion::V1_13 => "v1_13",
			SupportedVersion::V1_14 => "v1_14",
			SupportedVersion::V1_15 => "v1_15",
			SupportedVersion::V1_16 => "v1_16",
		}
	}

	pub(crate) fn spec_url(self) -> &'static str {
		match self {
			SupportedVersion::V1_8 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.8.15/api/openapi-spec/swagger.json",
			SupportedVersion::V1_9 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.9.11/api/openapi-spec/swagger.json",
			SupportedVersion::V1_10 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.10.13/api/openapi-spec/swagger.json",
			SupportedVersion::V1_11 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.11.10/api/openapi-spec/swagger.json",
			SupportedVersion::V1_12 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.12.10/api/openapi-spec/swagger.json",
			SupportedVersion::V1_13 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.13.12/api/openapi-spec/swagger.json",
			SupportedVersion::V1_14 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.14.8/api/openapi-spec/swagger.json",
			SupportedVersion::V1_15 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.15.5/api/openapi-spec/swagger.json",
			SupportedVersion::V1_16 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.16.1/api/openapi-spec/swagger.json",
		}
	}

	pub(crate) fn fixup(self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		#[allow(clippy::match_same_arms)]
		let fixups: &[fn(&mut crate::swagger20::Spec) -> Result<(), crate::Error>] = match self {
			SupportedVersion::V1_8 => &[
				crate::fixups::create_delete_optional,
				crate::fixups::create_patch_optional,
				crate::fixups::deployment_rollback_create_response_type,
				crate::fixups::gvk::api_service_list_v1beta1,
				crate::fixups::gvk::api_service_v1beta1,
				crate::fixups::gvk::crd_v1beta1,
				crate::fixups::gvk::crd_list_v1beta1,
				crate::fixups::json_ty::json,
				crate::fixups::json_ty::json_schema_props_or_array,
				crate::fixups::json_ty::json_schema_props_or_bool,
				crate::fixups::json_ty::json_schema_props_or_string_array,
				crate::fixups::optional_properties::apigroup,
				crate::fixups::optional_properties::crdstatus,
				crate::fixups::optional_properties::poddisruptionbudgetstatus,
				crate::fixups::patch,
				crate::fixups::raw_extension_ty,
				crate::fixups::remove_compat_refs,
				crate::fixups::remove_delete_collection_operations_query_parameters,
				crate::fixups::remove_delete_operations_query_parameters,
				crate::fixups::separate_watch_from_list_operations,
				crate::fixups::watch_event,
			],

			SupportedVersion::V1_9 => &[
				crate::fixups::create_delete_optional,
				crate::fixups::create_patch_optional,
				crate::fixups::deployment_rollback_create_response_type,
				crate::fixups::gvk::api_service_list_v1beta1,
				crate::fixups::gvk::api_service_v1beta1,
				crate::fixups::gvk::crd_v1beta1,
				crate::fixups::gvk::crd_list_v1beta1,
				crate::fixups::json_ty::json,
				crate::fixups::json_ty::json_schema_props_or_array,
				crate::fixups::json_ty::json_schema_props_or_bool,
				crate::fixups::json_ty::json_schema_props_or_string_array,
				crate::fixups::optional_properties::apigroup,
				crate::fixups::optional_properties::crdstatus,
				crate::fixups::optional_properties::poddisruptionbudgetstatus,
				crate::fixups::patch,
				crate::fixups::raw_extension_ty,
				crate::fixups::remove_compat_refs,
				crate::fixups::remove_delete_collection_operations_query_parameters,
				crate::fixups::remove_delete_operations_query_parameters,
				crate::fixups::separate_watch_from_list_operations,
				crate::fixups::watch_event,
			],

			SupportedVersion::V1_10 => &[
				crate::fixups::create_delete_optional,
				crate::fixups::create_patch_optional,
				crate::fixups::deployment_rollback_create_response_type,
				crate::fixups::gvk::api_service_list_v1,
				crate::fixups::gvk::api_service_list_v1beta1,
				crate::fixups::gvk::api_service_v1beta1,
				crate::fixups::gvk::api_service_v1,
				crate::fixups::gvk::crd_v1beta1,
				crate::fixups::gvk::crd_list_v1beta1,
				crate::fixups::json_ty::json,
				crate::fixups::json_ty::json_schema_props_or_array,
				crate::fixups::json_ty::json_schema_props_or_bool,
				crate::fixups::json_ty::json_schema_props_or_string_array,
				crate::fixups::optional_properties::apigroup,
				crate::fixups::optional_properties::crdstatus,
				crate::fixups::optional_properties::poddisruptionbudgetstatus,
				crate::fixups::patch,
				crate::fixups::raw_extension_ty,
				crate::fixups::remove_compat_refs,
				crate::fixups::remove_delete_collection_operations_query_parameters,
				crate::fixups::remove_delete_operations_query_parameters,
				crate::fixups::separate_watch_from_list_operations,
				crate::fixups::watch_event,
			],

			SupportedVersion::V1_11 => &[
				crate::fixups::create_delete_optional,
				crate::fixups::create_patch_optional,
				crate::fixups::deployment_rollback_create_response_type,
				crate::fixups::json_ty::json_schema_props_or_array,
				crate::fixups::json_ty::json_schema_props_or_bool,
				crate::fixups::json_ty::json_schema_props_or_string_array,
				crate::fixups::optional_properties::crdstatus,
				crate::fixups::optional_properties::poddisruptionbudgetstatus,
				crate::fixups::patch,
				crate::fixups::raw_extension_ty,
				crate::fixups::remove_compat_refs,
				crate::fixups::remove_delete_collection_operations_query_parameters,
				crate::fixups::remove_delete_operations_query_parameters,
				crate::fixups::separate_watch_from_list_operations,
				crate::fixups::watch_event,
			],

			SupportedVersion::V1_12 => &[
				crate::fixups::connect_options_gvk,
				crate::fixups::create_delete_optional,
				crate::fixups::create_patch_optional,
				crate::fixups::json_ty::json_schema_props_or_array,
				crate::fixups::json_ty::json_schema_props_or_bool,
				crate::fixups::json_ty::json_schema_props_or_string_array,
				crate::fixups::optional_properties::crdstatus,
				crate::fixups::patch,
				crate::fixups::raw_extension_ty,
				crate::fixups::remove_compat_refs,
				crate::fixups::remove_delete_collection_operations_query_parameters,
				crate::fixups::remove_delete_operations_query_parameters,
				crate::fixups::separate_watch_from_list_operations,
				crate::fixups::watch_event,
			],

			SupportedVersion::V1_13 => &[
				crate::fixups::connect_options_gvk,
				crate::fixups::create_delete_optional,
				crate::fixups::create_patch_optional,
				crate::fixups::json_ty::json_schema_props_or_array,
				crate::fixups::json_ty::json_schema_props_or_bool,
				crate::fixups::json_ty::json_schema_props_or_string_array,
				crate::fixups::optional_properties::crdstatus,
				crate::fixups::patch,
				crate::fixups::raw_extension_ty,
				crate::fixups::remove_compat_refs,
				crate::fixups::remove_delete_collection_operations_query_parameters,
				crate::fixups::remove_delete_operations_query_parameters,
				crate::fixups::separate_watch_from_list_operations,
				crate::fixups::watch_event,
			],

			SupportedVersion::V1_14 => &[
				crate::fixups::connect_options_gvk,
				crate::fixups::create_delete_optional,
				crate::fixups::create_patch_optional,
				crate::fixups::json_ty::json_schema_props_or_array,
				crate::fixups::json_ty::json_schema_props_or_bool,
				crate::fixups::json_ty::json_schema_props_or_string_array,
				crate::fixups::optional_properties::crdstatus,
				crate::fixups::patch,
				crate::fixups::raw_extension_ty,
				crate::fixups::remove_delete_collection_operations_query_parameters,
				crate::fixups::remove_delete_operations_query_parameters,
				crate::fixups::separate_watch_from_list_operations,
				crate::fixups::watch_event,
			],

			SupportedVersion::V1_15 => &[
				crate::fixups::connect_options_gvk,
				crate::fixups::create_delete_optional,
				crate::fixups::create_patch_optional,
				crate::fixups::json_ty::json_schema_props_or_array,
				crate::fixups::json_ty::json_schema_props_or_bool,
				crate::fixups::json_ty::json_schema_props_or_string_array,
				crate::fixups::optional_properties::crdstatus,
				crate::fixups::patch,
				crate::fixups::raw_extension_ty,
				crate::fixups::remove_delete_collection_operations_query_parameters,
				crate::fixups::remove_delete_operations_query_parameters,
				crate::fixups::separate_watch_from_list_operations,
				crate::fixups::watch_event,
			],

			SupportedVersion::V1_16 => &[
				crate::fixups::connect_options_gvk,
				crate::fixups::create_delete_optional,
				crate::fixups::create_patch_optional,
				crate::fixups::json_ty::json_schema_props_or_array,
				crate::fixups::json_ty::json_schema_props_or_bool,
				crate::fixups::json_ty::json_schema_props_or_string_array,
				crate::fixups::patch,
				crate::fixups::remove_delete_collection_operations_query_parameters,
				crate::fixups::remove_delete_operations_query_parameters,
				crate::fixups::separate_watch_from_list_operations,
				crate::fixups::watch_event,
			],
		};

		for fixup in fixups {
			fixup(spec)?;
		}

		Ok(())
	}
}
