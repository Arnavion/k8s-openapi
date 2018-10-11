pub(crate) const ALL: &[SupportedVersion] = &[
	SupportedVersion::V1_7,
	SupportedVersion::V1_8,
	SupportedVersion::V1_9,
	SupportedVersion::V1_10,
	SupportedVersion::V1_11,
	SupportedVersion::V1_12,
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum SupportedVersion {
	V1_7,
	V1_8,
	V1_9,
	V1_10,
	V1_11,
	V1_12,
}

impl SupportedVersion {
	pub(crate) fn mod_root(self) -> &'static str {
		match self {
			SupportedVersion::V1_7 => "v1_7",
			SupportedVersion::V1_8 => "v1_8",
			SupportedVersion::V1_9 => "v1_9",
			SupportedVersion::V1_10 => "v1_10",
			SupportedVersion::V1_11 => "v1_11",
			SupportedVersion::V1_12 => "v1_12",
		}
	}

	pub(crate) fn spec_url(self) -> &'static str {
		match self {
			SupportedVersion::V1_7 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.7.16/api/openapi-spec/swagger.json",
			SupportedVersion::V1_8 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.8.15/api/openapi-spec/swagger.json",
			SupportedVersion::V1_9 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.9.11/api/openapi-spec/swagger.json",
			SupportedVersion::V1_10 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.10.8/api/openapi-spec/swagger.json",
			SupportedVersion::V1_11 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.11.3/api/openapi-spec/swagger.json",
			SupportedVersion::V1_12 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.12.1/api/openapi-spec/swagger.json",
		}
	}

	pub(crate) fn fixup(self, spec: &mut ::swagger20::Spec) -> Result<(), ::Error> {
		#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
		let fixups: &[fn(&mut ::swagger20::Spec) -> Result<(), ::Error>] = match self {
			SupportedVersion::V1_7 => &[
				::fixups::apigroup_optional_properties,
				::fixups::apiservicev1beta1_gkv,
				::fixups::raw_extension_ty,
			],

			SupportedVersion::V1_8 => &[
				::fixups::apigroup_optional_properties,
				::fixups::apiservicev1beta1_gkv,
				::fixups::crd_gkv,
				::fixups::crdstatus_optional_properties,
				::fixups::deployment_rollback_create_response_type,
				::fixups::json_ty,
				::fixups::json_schema_props_or_array_ty,
				::fixups::json_schema_props_or_bool_ty,
				::fixups::json_schema_props_or_string_array_ty,
				::fixups::poddisruptionbudgetstatus_optional_properties,
				::fixups::raw_extension_ty,
				::fixups::remove_compat_refs,
			],

			SupportedVersion::V1_9 => &[
				::fixups::apigroup_optional_properties,
				::fixups::apiservicev1beta1_gkv,
				::fixups::crd_gkv,
				::fixups::crdstatus_optional_properties,
				::fixups::deployment_rollback_create_response_type,
				::fixups::json_ty,
				::fixups::json_schema_props_or_array_ty,
				::fixups::json_schema_props_or_bool_ty,
				::fixups::json_schema_props_or_string_array_ty,
				::fixups::poddisruptionbudgetstatus_optional_properties,
				::fixups::raw_extension_ty,
				::fixups::remove_compat_refs,
			],

			SupportedVersion::V1_10 => &[
				::fixups::apigroup_optional_properties,
				::fixups::apiservicev1beta1_gkv,
				::fixups::apiservicev1_gkv,
				::fixups::crd_gkv,
				::fixups::crdstatus_optional_properties,
				::fixups::deployment_rollback_create_response_type,
				::fixups::json_ty,
				::fixups::json_schema_props_or_array_ty,
				::fixups::json_schema_props_or_bool_ty,
				::fixups::json_schema_props_or_string_array_ty,
				::fixups::poddisruptionbudgetstatus_optional_properties,
				::fixups::raw_extension_ty,
				::fixups::remove_compat_refs,
			],

			SupportedVersion::V1_11 => &[
				::fixups::crdstatus_optional_properties,
				::fixups::deployment_rollback_create_response_type,
				::fixups::json_ty,
				::fixups::json_schema_props_or_array_ty,
				::fixups::json_schema_props_or_bool_ty,
				::fixups::json_schema_props_or_string_array_ty,
				::fixups::poddisruptionbudgetstatus_optional_properties,
				::fixups::raw_extension_ty,
				::fixups::remove_compat_refs,
			],

			SupportedVersion::V1_12 => &[
				::fixups::connect_options_gvk,
				::fixups::crdstatus_optional_properties,
				::fixups::json_ty,
				::fixups::json_schema_props_or_array_ty,
				::fixups::json_schema_props_or_bool_ty,
				::fixups::json_schema_props_or_string_array_ty,
				::fixups::raw_extension_ty,
				::fixups::remove_compat_refs,
			],
		};

		for fixup in fixups {
			fixup(spec)?;
		}

		Ok(())
	}
}
