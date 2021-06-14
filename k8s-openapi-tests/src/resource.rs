#[test]
fn impl_resource() {
	k8s_openapi::k8s_if_ge_1_11! {
		k8s_openapi::k8s_if_le_1_13! {
			assert_is_cluster_scoped::<k8s_openapi::api::admissionregistration::v1alpha1::InitializerConfiguration>("initializerconfigurations");
		}
	}

	k8s_openapi::k8s_if_ge_1_11! {
		k8s_openapi::k8s_if_le_1_17! {
			assert_is_subresource::<k8s_openapi::api::apps::v1beta1::DeploymentRollback>("rollback");
		}
	}

	k8s_openapi::k8s_if_ge_1_11! {
		k8s_openapi::k8s_if_le_1_17! {
			assert_is_subresource::<k8s_openapi::api::apps::v1beta1::Scale>("scale");
		}
	}

	k8s_openapi::k8s_if_ge_1_11! {
		k8s_openapi::k8s_if_le_1_17! {
			assert_is_subresource::<k8s_openapi::api::apps::v1beta2::Scale>("scale");
		}
	}

	assert_is_cluster_scoped::<k8s_openapi::api::authentication::v1beta1::TokenReview>("tokenreviews");

	k8s_openapi::k8s_if_ge_1_16! {
		assert_is_subresource::<k8s_openapi::api::authentication::v1::TokenRequest>("token");
	}

	assert_is_cluster_scoped::<k8s_openapi::api::authentication::v1::TokenReview>("tokenreviews");

	assert_is_namespace_scoped::<k8s_openapi::api::authorization::v1beta1::LocalSubjectAccessReview>("localsubjectaccessreviews");

	assert_is_cluster_scoped::<k8s_openapi::api::authorization::v1beta1::SelfSubjectAccessReview>("selfsubjectaccessreviews");

	assert_is_cluster_scoped::<k8s_openapi::api::authorization::v1beta1::SelfSubjectRulesReview>("selfsubjectrulesreviews");

	assert_is_cluster_scoped::<k8s_openapi::api::authorization::v1beta1::SubjectAccessReview>("subjectaccessreviews");

	assert_is_namespace_scoped::<k8s_openapi::api::authorization::v1::LocalSubjectAccessReview>("localsubjectaccessreviews");

	assert_is_cluster_scoped::<k8s_openapi::api::authorization::v1::SelfSubjectAccessReview>("selfsubjectaccessreviews");

	assert_is_cluster_scoped::<k8s_openapi::api::authorization::v1::SelfSubjectRulesReview>("selfsubjectrulesreviews");

	assert_is_cluster_scoped::<k8s_openapi::api::authorization::v1::SubjectAccessReview>("subjectaccessreviews");

	assert_is_subresource::<k8s_openapi::api::autoscaling::v1::Scale>("scale");

	assert_is_namespace_scoped::<k8s_openapi::api::core::v1::Binding>("bindings");

	assert_is_namespace_scoped::<k8s_openapi::api::apps::v1::DaemonSet>("daemonsets");

	k8s_openapi::k8s_if_ge_1_21! {
		assert_is_subresource::<k8s_openapi::api::core::v1::EphemeralContainers>("ephemeralcontainers");
	}

	assert_is_cluster_scoped::<k8s_openapi::api::core::v1::Namespace>("namespaces");

	assert_is_namespace_scoped::<k8s_openapi::api::core::v1::Pod>("pods");

	k8s_openapi::k8s_if_ge_1_11! {
		k8s_openapi::k8s_if_le_1_17! {
			assert_is_subresource::<k8s_openapi::api::extensions::v1beta1::DeploymentRollback>("rollback");
		}
	}

	k8s_openapi::k8s_if_ge_1_11! {
		k8s_openapi::k8s_if_le_1_17! {
			assert_is_subresource::<k8s_openapi::api::extensions::v1beta1::Scale>("scale");
		}
	}

	assert_is_cluster_scoped::<k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList>("");

	assert_is_cluster_scoped::<k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup>("");

	assert_is_cluster_scoped::<k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList>("");

	assert_is_cluster_scoped::<k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions>("");

	assert_is_subresource::<k8s_openapi::apimachinery::pkg::apis::meta::v1::Status>("status");

	assert_is_subresource::<k8s_openapi::api::policy::v1beta1::Eviction>("eviction");

	assert_is_cluster_scoped::<k8s_openapi::List<k8s_openapi::api::core::v1::Namespace>>("");
	assert_is_namespace_scoped::<k8s_openapi::List<k8s_openapi::api::core::v1::Pod>>("");
}

fn assert_is_cluster_scoped<T: k8s_openapi::Resource<Scope = k8s_openapi::ClusterResourceScope>>(expected_url_path_segment: &str) {
	assert_inner::<T>(expected_url_path_segment);
}

fn assert_is_namespace_scoped<T: k8s_openapi::Resource<Scope = k8s_openapi::NamespaceResourceScope>>(expected_url_path_segment: &str) {
	assert_inner::<T>(expected_url_path_segment);
}

fn assert_is_subresource<T: k8s_openapi::Resource<Scope = k8s_openapi::SubResourceScope>>(expected_url_path_segment: &str) {
	assert_inner::<T>(expected_url_path_segment);
}

fn assert_inner<T: k8s_openapi::Resource>(expected_url_path_segment: &str) {
	assert_eq!(<T as k8s_openapi::Resource>::URL_PATH_SEGMENT, expected_url_path_segment);
}
