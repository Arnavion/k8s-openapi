#[test]
fn list() {
	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;
	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::apimachinery::pkg::apis::meta::v1 as meta;

	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;
	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::apimachinery::pkg::apis::meta::v1 as meta;

	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;
	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::apimachinery::pkg::apis::meta::v1 as meta;

	let client = ::Client::new().expect("couldn't create client");

	let custom_resource_definition = apiextensions::CustomResourceDefinition {
		metadata: Some(meta::ObjectMeta {
			name: Some("foobars.k8s-openapi-tests-custom-resource-definition.com".to_string()),
			..Default::default()
		}),
		spec: Some(apiextensions::CustomResourceDefinitionSpec {
			group: "k8s-openapi-tests-custom-resource-definition.com".to_string(),
			names: apiextensions::CustomResourceDefinitionNames {
				kind: "FooBar".to_string(),
				plural: "foobars".to_string(),
				short_names: Some(vec!["fb".to_string()]),
				singular: Some("foobar".to_string()),
				..Default::default()
			},
			scope: "Namespaced".to_string(),
			version: "v1".to_string(),
			..Default::default()
		}),
		..Default::default()
	};

	let request =
		apiextensions::CustomResourceDefinition::create_apiextensions_v1beta1_custom_resource_definition(&custom_resource_definition, None)
		.expect("couldn't create custom resource definition");
	let response = client.execute(request).expect("couldn't create custom resource definition");
	let custom_resource_definition: apiextensions::CustomResourceDefinition =
		::get_single_value(response, |response, status_code, _response_body| match response {
			#[cfg(feature = "v1_8")] apiextensions::CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Other if status_code == ::http::StatusCode::CREATED =>
				match ::serde_json::from_slice(_response_body) {
					Ok(custom_resource_definition) => Ok(::ValueResult::GotValue(custom_resource_definition)),
					Err(ref err) if err.is_eof() => Ok(::ValueResult::NeedMoreData),
					Err(err) => Err(err.into()),
				},
			#[cfg(not(feature = "v1_8"))] apiextensions::CreateApiextensionsV1beta1CustomResourceDefinitionResponse::Created(custom_resource_definition) =>
				Ok(::ValueResult::GotValue(custom_resource_definition)),
			other => Err(format!("{:?} {}", other, status_code).into()),
		}).expect("couldn't create custom resource definition");

	let custom_resource_definition_self_link = {
		let metadata = custom_resource_definition.metadata.expect("couldn't get custom resource definition metadata");
		metadata.self_link.expect("couldn't get custom resource definition self link")
	};

	client.delete(&custom_resource_definition_self_link).expect("couldn't delete custom resource definition");
}
