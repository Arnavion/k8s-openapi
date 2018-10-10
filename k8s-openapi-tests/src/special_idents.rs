#[test]
fn special_idents() {
	k8s_if_1_7! {
		use ::k8s_openapi::v1_7::kubernetes::pkg::api::v1 as api;
		use ::k8s_openapi::v1_7::apimachinery::pkg::apis::meta::v1 as meta;
		use ::k8s_openapi::v1_7::kubernetes::pkg::apis::rbac::v1beta1 as rbac;
	}
	k8s_if_1_8! {
		use ::k8s_openapi::v1_8::api::core::v1 as api;
		use ::k8s_openapi::v1_8::api::authorization::v1 as authorization;
		use ::k8s_openapi::v1_8::apimachinery::pkg::apis::meta::v1 as meta;
		use ::k8s_openapi::v1_8::api::rbac::v1beta1 as rbac;
	}
	k8s_if_1_9! {
		use ::k8s_openapi::v1_9::api::core::v1 as api;
		use ::k8s_openapi::v1_9::api::authorization::v1 as authorization;
		use ::k8s_openapi::v1_9::apimachinery::pkg::apis::meta::v1 as meta;
		use ::k8s_openapi::v1_9::api::rbac::v1beta1 as rbac;
	}
	k8s_if_1_10! {
		use ::k8s_openapi::v1_10::api::core::v1 as api;
		use ::k8s_openapi::v1_10::api::authorization::v1 as authorization;
		use ::k8s_openapi::v1_10::apimachinery::pkg::apis::meta::v1 as meta;
		use ::k8s_openapi::v1_10::api::rbac::v1beta1 as rbac;
	}
	k8s_if_1_11! {
		use ::k8s_openapi::v1_11::api::core::v1 as api;
		use ::k8s_openapi::v1_11::api::authorization::v1 as authorization;
		use ::k8s_openapi::v1_11::apimachinery::pkg::apis::meta::v1 as meta;
		use ::k8s_openapi::v1_11::api::rbac::v1beta1 as rbac;
	}
	k8s_if_1_12! {
		use ::k8s_openapi::v1_12::api::core::v1 as api;
		use ::k8s_openapi::v1_12::api::authorization::v1 as authorization;
		use ::k8s_openapi::v1_12::apimachinery::pkg::apis::meta::v1 as meta;
		use ::k8s_openapi::v1_12::api::rbac::v1beta1 as rbac;
	}

	let _ = api::FCVolumeSource { target_wwns: Default::default(), ..Default::default() };

	let _ = api::ServiceSpec { external_ips: Default::default(), ..Default::default() };

	k8s_if_ge_1_8! {
		let _ = authorization::NonResourceRule { non_resource_urls: Default::default(), ..Default::default() };
	}

	let _ = meta::APIGroup { server_address_by_client_cidrs: Default::default(), ..Default::default() };

	let _ = meta::APIVersions { server_address_by_client_cidrs: Default::default(), ..Default::default() };

	let _ = rbac::PolicyRule { non_resource_urls: Default::default(), ..Default::default() };
}
