#[test]
fn list() {
	crate::Client::with("api_versions-list", |client| {
		let request = k8s_openapi::get_api_versions().expect("couldn't get API versions");
		let response = client.execute(request).expect("couldn't get API versions");
		let api_versions =
			crate::get_single_value(response, |response, status_code, _| match response {
				k8s_openapi::GetAPIVersionsResponse::Ok(api_versions) => Ok(crate::ValueResult::GotValue(api_versions)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't get API versions");

		assert_eq!(k8s_openapi::kind(&api_versions), "APIGroupList");
	});
}
