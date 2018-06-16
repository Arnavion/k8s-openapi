#[test]
fn special_idents() {
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::kubernetes::pkg::api::v1 as api;
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::apimachinery::pkg::apis::meta::v1 as meta;
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::kubernetes::pkg::apis::rbac::v1beta1 as rbac;

	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::api::core::v1 as api;
	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::api::authorization::v1 as authorization;
	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::apimachinery::pkg::apis::meta::v1 as meta;
	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::api::rbac::v1beta1 as rbac;

	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::api::core::v1 as api;
	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::api::authorization::v1 as authorization;
	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::apimachinery::pkg::apis::meta::v1 as meta;
	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::api::rbac::v1beta1 as rbac;

	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::api::core::v1 as api;
	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::api::authorization::v1 as authorization;
	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::apimachinery::pkg::apis::meta::v1 as meta;
	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::api::rbac::v1beta1 as rbac;

	let _ = api::FCVolumeSource { target_wwns: Default::default(), ..Default::default() };

	let _ = api::ServiceSpec { external_ips: Default::default(), ..Default::default() };

	#[cfg(not(feature = "v1_7"))] let _ = authorization::NonResourceRule { non_resource_urls: Default::default(), ..Default::default() };

	let _ = meta::APIGroup { server_address_by_client_cidrs: Default::default(), ..Default::default() };

	let _ = meta::APIVersions { server_address_by_client_cidrs: Default::default(), ..Default::default() };

	let _ = rbac::PolicyRule { non_resource_urls: Default::default(), ..Default::default() };
}
