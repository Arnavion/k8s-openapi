#[test]
fn list() {
	k8s_if_1_7! {
		use ::k8s_openapi::v1_7 as k8s;
	}
	k8s_if_1_8! {
		use ::k8s_openapi::v1_8 as k8s;
	}
	k8s_if_1_9! {
		use ::k8s_openapi::v1_9 as k8s;
	}
	k8s_if_1_10! {
		use ::k8s_openapi::v1_10 as k8s;
	}
	k8s_if_1_11! {
		use ::k8s_openapi::v1_11 as k8s;
	}
	k8s_if_1_12! {
		use ::k8s_openapi::v1_12 as k8s;
	}

	let client = ::Client::new().expect("couldn't create client");

	let request = k8s::get_api_versions().expect("couldn't get API versions");
	let response = client.execute(request).expect("couldn't get API versions");
	let api_versions =
		::get_single_value(response, |response, status_code, _| match response {
			k8s::GetAPIVersionsResponse::Ok(api_versions) => Ok(::ValueResult::GotValue(api_versions)),
			other => Err(format!("{:?} {}", other, status_code).into()),
		}).expect("couldn't get API versions");

	assert_eq!(::k8s_openapi::kind(&api_versions), "APIGroupList");
}
