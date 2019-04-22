use k8s_openapi::{http, serde_json};

#[test]
fn create() {
	use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

	#[derive(Debug, Default, serde_derive::Deserialize, serde_derive::Serialize)]
	struct FooBar {
		#[serde(rename = "apiVersion")]
		pub api_version: Option<String>,
		pub kind: Option<String>,
		pub metadata: Option<meta::ObjectMeta>,
		pub spec: Option<FooBarSpec>,
	}

	#[derive(Debug, Default, serde_derive::Deserialize, serde_derive::Serialize)]
	struct FooBarSpec {
		prop1: String,
		prop2: Vec<bool>,
		#[serde(skip_serializing_if = "Option::is_none")]
		prop3: Option<i32>,
	}

	#[derive(Debug)]
	enum CreateFooBarResponse {
		Created(FooBar),
		UnprocessableEntity(meta::Status),
		Other,
	}

	impl k8s_openapi::Response for CreateFooBarResponse {
		fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
			match status_code {
				http::StatusCode::CREATED => {
					let result = match serde_json::from_slice(buf) {
						Ok(value) => value,
						Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
						Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
					};
					Ok((CreateFooBarResponse::Created(result), buf.len()))
				},
				http::StatusCode::UNPROCESSABLE_ENTITY => {
					let result = match serde_json::from_slice(buf) {
						Ok(value) => value,
						Err(ref err) if err.is_eof() => return Err(k8s_openapi::ResponseError::NeedMoreData),
						Err(err) => return Err(k8s_openapi::ResponseError::Json(err)),
					};
					Ok((CreateFooBarResponse::UnprocessableEntity(result), buf.len()))
				},
				_ => Ok((CreateFooBarResponse::Other, 0)),
			}
		}
	}

	#[derive(Debug)]
	enum DeleteFooBarResponse {
		Ok,
		Other,
	}

	impl k8s_openapi::Response for DeleteFooBarResponse {
		fn try_from_parts(status_code: http::StatusCode, _: &[u8]) -> Result<(Self, usize), k8s_openapi::ResponseError> {
			match status_code {
				http::StatusCode::OK => Ok((DeleteFooBarResponse::Ok, 0)),
				_ => Ok((DeleteFooBarResponse::Other, 0)),
			}
		}
	}

	crate::Client::with("custom_resource_definition-create", |client| {
		let custom_resource_definition_spec = apiextensions::CustomResourceDefinitionSpec {
			group: "k8s-openapi-tests-custom-resource-definition.com".to_string(),
			names: apiextensions::CustomResourceDefinitionNames {
				kind: "FooBar".to_string(),
				plural: "foobars".to_string(),
				short_names: Some(vec!["fb".to_string()]),
				singular: Some("foobar".to_string()),
				..Default::default()
			},
			scope: "Namespaced".to_string(),
			version: "v1".to_string().into(),
			..Default::default()
		};

		k8s_if_ge_1_9! {
			// CRD validation entered beta in v1.9
			let custom_resource_definition_spec = apiextensions::CustomResourceDefinitionSpec {
				validation: Some(apiextensions::CustomResourceValidation {
					open_api_v3_schema: Some(apiextensions::JSONSchemaProps {
						properties: Some(vec![
							("spec".to_string(), apiextensions::JSONSchemaProps {
								properties: Some(vec![
									("prop1".to_string(), apiextensions::JSONSchemaProps {
										type_: Some("string".to_string()),
										..Default::default()
									}),
									("prop2".to_string(), apiextensions::JSONSchemaProps {
										type_: Some("array".to_string()),
										items: Some(apiextensions::JSONSchemaPropsOrArray::Schema(Box::new(apiextensions::JSONSchemaProps {
											type_: Some("boolean".to_string()),
											..Default::default()
										}))),
										..Default::default()
									}),
									("prop3".to_string(), apiextensions::JSONSchemaProps {
										format: Some("int32".to_string()),
										type_: Some("integer".to_string()),
										..Default::default()
									}),
								].into_iter().collect()),
								required: Some(vec![
									"prop1".to_string(),
									"prop2".to_string(),
								]),
								..Default::default()
							}),
						].into_iter().collect()),
						..Default::default()
					}),
				}),
				..custom_resource_definition_spec
			};
		}

		let custom_resource_definition = apiextensions::CustomResourceDefinition {
			metadata: Some(meta::ObjectMeta {
				name: Some("foobars.k8s-openapi-tests-custom-resource-definition.com".to_string()),
				..Default::default()
			}),
			spec: custom_resource_definition_spec.into(),
			..Default::default()
		};

		loop {
			let (request, response_body) =
				apiextensions::CustomResourceDefinition::create_custom_resource_definition(&custom_resource_definition, Default::default())
				.expect("couldn't create custom resource definition");
			let response = client.execute(request).expect("couldn't create custom resource definition");

			let success =
				crate::get_single_value(response, response_body, |response, status_code| k8s_match!((response, status_code), {
					k8s_if_1_8!((apiextensions::CreateCustomResourceDefinitionResponse::Other(value), http::StatusCode::CREATED) => {
						let value = match value? {
							Some(value) => value,
							None => return Ok(crate::ValueResult::NeedMoreData),
						};
						let _: apiextensions::CustomResourceDefinition = serde::Deserialize::deserialize(value)?;
						Ok(crate::ValueResult::GotValue(true))
					}),

					k8s_if_ge_1_9!((apiextensions::CreateCustomResourceDefinitionResponse::Created(_), _) =>
						Ok(crate::ValueResult::GotValue(true))),

					(_, http::StatusCode::CONFLICT) =>
						Ok(crate::ValueResult::GotValue(true)),

					(_, http::StatusCode::INTERNAL_SERVER_ERROR) =>
						Ok(crate::ValueResult::GotValue(false)),

					(other, status_code) => Err(format!("{:?} {}", other, status_code).into()),
				})).expect("couldn't create custom resource definition");

			if success {
				break;
			}
		}

		// Wait for CRD to be registered
		let custom_resource_definition = loop {
			let (request, response_body) =
				apiextensions::CustomResourceDefinition::read_custom_resource_definition(
					"foobars.k8s-openapi-tests-custom-resource-definition.com", Default::default())
				.expect("couldn't get custom resource definition");
			let custom_resource_definition = {
				let response = client.execute(request).expect("couldn't get custom resource definition");
				crate::get_single_value(response, response_body, |response, status_code| match response {
					apiextensions::ReadCustomResourceDefinitionResponse::Ok(custom_resource_definition) => Ok(crate::ValueResult::GotValue(custom_resource_definition)),
					other => Err(format!("{:?} {}", other, status_code).into()),
				}).expect("couldn't get custom resource definition")
			};

			if custom_resource_definition.status.as_ref().map_or(false, |status| status.accepted_names.kind == "FooBar") {
				break custom_resource_definition;
			}

			client.sleep(std::time::Duration::from_secs(1));
		};

		let fb1 = FooBar {
			api_version: Some("k8s-openapi-tests-custom-resource-definition.com/v1".to_string()),
			kind: Some("FooBar".to_string()),
			metadata: Some(meta::ObjectMeta {
				name: Some("fb1".to_string()),
				..Default::default()
			}),
			spec: Some(FooBarSpec {
				prop1: "value1".to_string(),
				prop2: vec![true, false, true],
				..Default::default()
			}),
			..Default::default()
		};
		let request =
			http::Request::post("/apis/k8s-openapi-tests-custom-resource-definition.com/v1/namespaces/default/foobars")
			.body(serde_json::to_vec(&fb1).expect("couldn't create custom resource definition"))
			.expect("couldn't create custom resource");
		let fb1 = {
			let response = client.execute(request).expect("couldn't create custom resource");
			crate::get_single_value(response, k8s_openapi::ResponseBody::new, |response, status_code| match response {
				CreateFooBarResponse::Created(fb) => Ok(crate::ValueResult::GotValue(fb)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't create custom resource")
		};

		let fb1_self_link = {
			let metadata = fb1.metadata.expect("couldn't get custom resource metadata");
			metadata.self_link.expect("couldn't get custom resource self link")
		};
		let request = http::Request::delete(fb1_self_link).body(vec![]).expect("couldn't delete custom resource");
		{
			let response = client.execute(request).expect("couldn't delete custom resource");
			crate::get_single_value(response, k8s_openapi::ResponseBody::new, |response, status_code| match response {
				DeleteFooBarResponse::Ok => Ok(crate::ValueResult::GotValue(())),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't delete custom resource");
		}

		k8s_if_ge_1_9! {
			let fb2 = serde_json::Value::Object(vec![
				("apiVersion".to_string(), serde_json::Value::String("k8s-openapi-tests-custom-resource-definition.com/v1".to_string())),
				("kind".to_string(), serde_json::Value::String("FooBar".to_string())),
				("metadata".to_string(), serde_json::Value::Object(vec![
					("name".to_string(), serde_json::Value::String("fb1".to_string())),
				].into_iter().collect())),
				("spec".to_string(), serde_json::Value::Object(vec![
					("prop1".to_string(), serde_json::Value::String("value1".to_string())),
				].into_iter().collect())),
			].into_iter().collect());
			let request =
				http::Request::post("/apis/k8s-openapi-tests-custom-resource-definition.com/v1/namespaces/default/foobars")
				.body(serde_json::to_vec(&fb2).expect("couldn't create custom resource definition"))
				.expect("couldn't create custom resource");
			{
				let response = client.execute(request).expect("couldn't create custom resource");
				crate::get_single_value(response, k8s_openapi::ResponseBody::new, |response, status_code| match response {
					CreateFooBarResponse::UnprocessableEntity(_) => Ok(crate::ValueResult::GotValue(())),
					other => Err(format!("{:?} {}", other, status_code).into()),
				}).expect("expected custom resource creation to fail validation");
			}
		}

		k8s_if_ge_1_9! {
			let fb3 = serde_json::Value::Object(vec![
				("apiVersion".to_string(), serde_json::Value::String("k8s-openapi-tests-custom-resource-definition.com/v1".to_string())),
				("kind".to_string(), serde_json::Value::String("FooBar".to_string())),
				("metadata".to_string(), serde_json::Value::Object(vec![
					("name".to_string(), serde_json::Value::String("fb1".to_string())),
				].into_iter().collect())),
				("spec".to_string(), serde_json::Value::Object(vec![
					("prop1".to_string(), serde_json::Value::String("value1".to_string())),
					("prop2".to_string(), serde_json::Value::Bool(true)),
				].into_iter().collect())),
			].into_iter().collect());
			let request =
				http::Request::post("/apis/k8s-openapi-tests-custom-resource-definition.com/v1/namespaces/default/foobars")
				.body(serde_json::to_vec(&fb3).expect("couldn't create custom resource definition"))
				.expect("couldn't create custom resource");
			{
				let response = client.execute(request).expect("couldn't create custom resource");
				crate::get_single_value(response, k8s_openapi::ResponseBody::new, |response, status_code| match response {
					CreateFooBarResponse::UnprocessableEntity(_) => Ok(crate::ValueResult::GotValue(())),
					other => Err(format!("{:?} {}", other, status_code).into()),
				}).expect("expected custom resource creation to fail validation");
			}
		}

		let custom_resource_definition_self_link = {
			let metadata = custom_resource_definition.metadata.expect("couldn't get custom resource definition metadata");
			metadata.self_link.expect("couldn't get custom resource definition self link")
		};

		let request = http::Request::delete(custom_resource_definition_self_link).body(vec![]).expect("couldn't delete custom resource definition");
		{
			let response = client.execute(request).expect("couldn't delete custom resource definition");
			crate::get_single_value(response, k8s_openapi::ResponseBody::new, |response, status_code| match response {
				apiextensions::DeleteCollectionCustomResourceDefinitionResponse::OkStatus(_) |
				apiextensions::DeleteCollectionCustomResourceDefinitionResponse::OkValue(_) => Ok(crate::ValueResult::GotValue(())),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't delete custom resource definition");
		}
	});
}
