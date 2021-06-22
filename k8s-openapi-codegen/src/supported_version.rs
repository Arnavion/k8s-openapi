use crate::fixups::Fixup;

pub(crate) const ALL: &[SupportedVersion] = &[
	SupportedVersion::V1_11,
	SupportedVersion::V1_12,
	SupportedVersion::V1_13,
	SupportedVersion::V1_14,
	SupportedVersion::V1_15,
	SupportedVersion::V1_16,
	SupportedVersion::V1_17,
	SupportedVersion::V1_18,
	SupportedVersion::V1_19,
	SupportedVersion::V1_20,
	SupportedVersion::V1_21,
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum SupportedVersion {
	V1_11,
	V1_12,
	V1_13,
	V1_14,
	V1_15,
	V1_16,
	V1_17,
	V1_18,
	V1_19,
	V1_20,
	V1_21,
}

impl SupportedVersion {
	pub(crate) fn name(self) -> &'static str {
		match self {
			SupportedVersion::V1_11 => "1.11",
			SupportedVersion::V1_12 => "1.12",
			SupportedVersion::V1_13 => "1.13",
			SupportedVersion::V1_14 => "1.14",
			SupportedVersion::V1_15 => "1.15",
			SupportedVersion::V1_16 => "1.16",
			SupportedVersion::V1_17 => "1.17",
			SupportedVersion::V1_18 => "1.18",
			SupportedVersion::V1_19 => "1.19",
			SupportedVersion::V1_20 => "1.20",
			SupportedVersion::V1_21 => "1.21",
		}
	}

	pub(crate) fn mod_root(self) -> &'static str {
		match self {
			SupportedVersion::V1_11 => "v1_11",
			SupportedVersion::V1_12 => "v1_12",
			SupportedVersion::V1_13 => "v1_13",
			SupportedVersion::V1_14 => "v1_14",
			SupportedVersion::V1_15 => "v1_15",
			SupportedVersion::V1_16 => "v1_16",
			SupportedVersion::V1_17 => "v1_17",
			SupportedVersion::V1_18 => "v1_18",
			SupportedVersion::V1_19 => "v1_19",
			SupportedVersion::V1_20 => "v1_20",
			SupportedVersion::V1_21 => "v1_21",
		}
	}

	pub(crate) fn spec_url(self) -> &'static str {
		match self {
			SupportedVersion::V1_11 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.11.10/api/openapi-spec/swagger.json",
			SupportedVersion::V1_12 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.12.10/api/openapi-spec/swagger.json",
			SupportedVersion::V1_13 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.13.12/api/openapi-spec/swagger.json",
			SupportedVersion::V1_14 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.14.10/api/openapi-spec/swagger.json",
			SupportedVersion::V1_15 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.15.12/api/openapi-spec/swagger.json",
			SupportedVersion::V1_16 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.16.15/api/openapi-spec/swagger.json",
			SupportedVersion::V1_17 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.17.17/api/openapi-spec/swagger.json",
			SupportedVersion::V1_18 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.18.20/api/openapi-spec/swagger.json",
			SupportedVersion::V1_19 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.19.12/api/openapi-spec/swagger.json",
			SupportedVersion::V1_20 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.20.8/api/openapi-spec/swagger.json",
			SupportedVersion::V1_21 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.21.2/api/openapi-spec/swagger.json",
		}
	}

	pub(crate) fn fixup(self, spec: &mut crate::swagger20::Spec, skip_client_generation: bool) -> Result<(), crate::Error> {
		#[allow(clippy::match_same_arms)]
		let upstream_bugs_fixups: &[&'static dyn Fixup] = match self {
			SupportedVersion::V1_11 => &[
				&crate::fixups::upstream_bugs::DeploymentRollbackCreateResponseType,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
				&crate::fixups::upstream_bugs::optional_properties::CrdStatus,
				&crate::fixups::upstream_bugs::optional_properties::PdbStatus,
				&crate::fixups::upstream_bugs::RawExtensionTy,
				&crate::fixups::upstream_bugs::RemoveCompatRefs,
			],

			SupportedVersion::V1_12 => &[
				&crate::fixups::upstream_bugs::ConnectOptionsGvk,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
				&crate::fixups::upstream_bugs::optional_properties::CrdStatus,
				&crate::fixups::upstream_bugs::RawExtensionTy,
				&crate::fixups::upstream_bugs::RemoveCompatRefs,
			],

			SupportedVersion::V1_13 => &[
				&crate::fixups::upstream_bugs::ConnectOptionsGvk,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
				&crate::fixups::upstream_bugs::optional_properties::CrdStatus,
				&crate::fixups::upstream_bugs::RawExtensionTy,
				&crate::fixups::upstream_bugs::RemoveCompatRefs,
			],

			SupportedVersion::V1_14 => &[
				&crate::fixups::upstream_bugs::ConnectOptionsGvk,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
				&crate::fixups::upstream_bugs::optional_properties::CrdStatus,
				&crate::fixups::upstream_bugs::RawExtensionTy,
			],

			SupportedVersion::V1_15 => &[
				&crate::fixups::upstream_bugs::ConnectOptionsGvk,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
				&crate::fixups::upstream_bugs::optional_properties::CrdStatus,
				&crate::fixups::upstream_bugs::RawExtensionTy,
			],

			SupportedVersion::V1_16 => &[
				&crate::fixups::upstream_bugs::ConnectOptionsGvk,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
			],

			SupportedVersion::V1_17 => &[
				&crate::fixups::upstream_bugs::ConnectOptionsGvk,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
			],

			SupportedVersion::V1_18 => &[
				&crate::fixups::upstream_bugs::ConnectOptionsGvk,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
			],

			SupportedVersion::V1_19 => &[
				&crate::fixups::upstream_bugs::ConnectOptionsGvk,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
			],

			SupportedVersion::V1_20 => &[
				&crate::fixups::upstream_bugs::ConnectOptionsGvk,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
			],

			SupportedVersion::V1_21 => &[
				&crate::fixups::upstream_bugs::ConnectOptionsGvk,
				&crate::fixups::upstream_bugs::optional_properties::ContainerImage,
			],
		};

		let special_fixups: &[&'static dyn Fixup] = &[
			&crate::fixups::special::json_ty::JsonSchemaPropsOrArray,
			&crate::fixups::special::json_ty::JsonSchemaPropsOrBool,
			&crate::fixups::special::json_ty::JsonSchemaPropsOrStringArray,
			&crate::fixups::special::CreateDeleteOptional,
			&crate::fixups::special::CreateOptionals,
			&crate::fixups::special::Patch,
			&crate::fixups::special::RemoveDeleteCollectionOperationsQueryParameters,
			&crate::fixups::special::RemoveDeleteOperationsQueryParameters,
			&crate::fixups::special::SeparateWatchFromListOperations,
			&crate::fixups::special::WatchEvent,
			&crate::fixups::special::List, // Must run after separate_watch_from_list_operations
			&crate::fixups::special::ResponseTypes,
			&crate::fixups::special::ResourceMetadataNotOptional,
		];

		let _ = skip_client_generation;

		for fixup in upstream_bugs_fixups.iter().chain(special_fixups).copied() {
			if skip_client_generation && fixup.requires_client_generation() {
				continue;
			}
			fixup.fixup(spec)?;
		}

		Ok(())
	}
}