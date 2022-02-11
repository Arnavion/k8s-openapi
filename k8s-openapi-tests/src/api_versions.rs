#[test]
fn list() {
	let mut client = crate::Client::new("api_versions-list");

	let (request, response_body) = k8s_openapi::get_api_versions().expect("couldn't get API versions");
	let response = client.execute(request);
	let api_versions =
		crate::get_single_value(response, response_body, |response, status_code| match response {
			k8s_openapi::GetAPIVersionsResponse::Ok(api_versions) => crate::ValueResult::GotValue(api_versions),
			other => panic!("{:?} {}", other, status_code),
		});

	assert_eq!(k8s_openapi::kind(&api_versions), "APIGroupList");
}
