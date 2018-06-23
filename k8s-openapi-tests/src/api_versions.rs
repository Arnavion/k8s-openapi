#[test]
fn list() {
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7 as k8s;

	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8 as k8s;

	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9 as k8s;

	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10 as k8s;

	let client = ::Client::new().expect("couldn't create client");

	let api_versions =
		k8s::get_api_versions(&client)
		.expect("couldn't get API versions");
	let api_versions = match api_versions {
		k8s::GetAPIVersionsResponse::Ok(api_versions) => api_versions,
		other => panic!("couldn't get API versions: {:?}", other),
	};
	assert_eq!(api_versions.kind, Some("APIGroupList".to_string()));
}
