use k8s_openapi::{http, serde_json};

#[test]
fn test() {
	use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1 as apiextensions;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

	#[derive(
		Clone, Debug, PartialEq,
		k8s_openapi_derive::CustomResourceDefinition,
		serde_derive::Deserialize, serde_derive::Serialize,
	)]
	#[custom_resource_definition(
		group = "k8s-openapi-tests-custom-resource-definition.com",
		version = "v1",
		plural = "foobars",
		namespaced,
	)]
	struct FooBarSpec {
		prop1: String,
		prop2: Vec<bool>,
		#[serde(skip_serializing_if = "Option::is_none")]
		prop3: Option<i32>,
	}

	crate::Client::with("custom_resource_definition-test", |client| {
		let plural = "foobars";

		let custom_resource_definition_spec = apiextensions::CustomResourceDefinitionSpec {
			group: <FooBar as k8s_openapi::Resource>::group().to_owned(),
			names: apiextensions::CustomResourceDefinitionNames {
				kind: <FooBar as k8s_openapi::Resource>::kind().to_owned(),
				plural: plural.to_owned(),
				short_names: Some(vec!["fb".to_owned()]),
				singular: Some("foobar".to_owned()),
				..Default::default()
			},
			scope: "Namespaced".to_owned(),
			version: <FooBar as k8s_openapi::Resource>::version().to_owned().into(),
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
				name: Some(format!("{}.{}", plural, <FooBar as k8s_openapi::Resource>::group())),
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
					&format!("{}.{}", plural, <FooBar as k8s_openapi::Resource>::group().to_owned()), Default::default())
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


		// Create CR
		let fb1 = FooBar {
			metadata: Some(meta::ObjectMeta {
				name: Some("fb1".to_string()),
				..Default::default()
			}),
			spec: Some(FooBarSpec {
				prop1: "value1".to_string(),
				prop2: vec![true, false, true],
				prop3: None,
			}),
		};
		let (request, response_body) =
			FooBar::create_namespaced_foo_bar("default", &fb1)
			.expect("couldn't create FooBar");
		let fb1 = {
			let response = client.execute(request).expect("couldn't create FooBar");
			crate::get_single_value(response, response_body, |response, status_code| match response {
				CreateNamespacedFooBarResponse::Ok(fb) |
				CreateNamespacedFooBarResponse::Created(fb) =>
					Ok(crate::ValueResult::GotValue(fb)),

				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't create FooBar")
		};


		// List CR
		let (request, response_body) = FooBar::list_namespaced_foo_bar("default", Default::default()).expect("couldn't list FooBars");
		let response = client.execute(request).expect("couldn't list FooBars");
		let foo_bar_list =
			crate::get_single_value(response, response_body, |response, status_code| match response {
				ListNamespacedFooBarResponse::Ok(foo_bar_list) => Ok(crate::ValueResult::GotValue(foo_bar_list)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't list FooBars");
		assert_eq!(k8s_openapi::kind(&foo_bar_list), "FooBarList");
		let _ =
			foo_bar_list
			.items.into_iter()
			.find(|fb| fb.metadata.as_ref().and_then(|metadata| metadata.name.as_ref()).map_or(false, |name| name == "fb1"))
			.expect("couldn't find FooBar");


		// Read CR
		let (request, response_body) = FooBar::read_namespaced_foo_bar("fb1", "default").expect("couldn't read FooBar");
		let response = client.execute(request).expect("couldn't read FooBar");
		let fb1_2 =
			crate::get_single_value(response, response_body, |response, status_code| match response {
				ReadNamespacedFooBarResponse::Ok(fb) => Ok(crate::ValueResult::GotValue(fb)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't read FooBar");
		assert_eq!(fb1_2.metadata.as_ref().unwrap().name.as_ref().unwrap(), "fb1");


		// Watch CR
		let (request, response_body) = FooBar::watch_namespaced_foo_bar("default", Default::default()).expect("couldn't watch FooBars");
		let response = client.execute(request).expect("couldn't watch FooBars");
		let foo_bar_watch_events =
			crate::get_multiple_values(response, response_body, |response, status_code| match response {
				WatchNamespacedFooBarResponse::Ok(foo_bar_watch_event) =>
					Ok(crate::ValueResult::GotValue(foo_bar_watch_event)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't watch FooBars");
		let _ =
			foo_bar_watch_events
			.map(|foo_bar_watch_event| foo_bar_watch_event.expect("couldn't get FooBar watch event"))
			.find_map(|foo_bar_watch_event| {
				println!("{:?}", foo_bar_watch_event);

				let fb = match foo_bar_watch_event {
					meta::WatchEvent::Added(fb) => fb,
					_ => return None,
				};

				if fb.metadata.as_ref().and_then(|metadata| metadata.name.as_ref()).map_or(false, |name| name == "fb1") {
					Some(fb)
				}
				else {
					None
				}
			}).expect("couldn't find FooBar");


		// Delete CR
		let (request, response_body) = {
			let metadata = fb1.metadata.as_ref().expect("create FooBar response did not set metadata");
			let name = metadata.name.as_ref().expect("create FooBar response did not set metadata.name");
			let namespace = metadata.namespace.as_ref().expect("create FooBar response did not set metadata.namespace");
			FooBar::delete_namespaced_foo_bar(name, namespace, Default::default()).expect("couldn't delete FooBar")
		};
		let () = {
			let response = client.execute(request).expect("couldn't delete FooBar");
			crate::get_single_value(response, response_body, |response, status_code| match response {
				DeleteNamespacedFooBarResponse::OkStatus(_) |
				DeleteNamespacedFooBarResponse::OkValue(_) =>
					Ok(crate::ValueResult::GotValue(())),

				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't delete FooBar")
		};


		// Create invalid CR
		k8s_if_ge_1_9! {
			let fb2 = serde_json::Value::Object(vec![
				("apiVersion".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::api_version().to_owned())),
				("kind".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::kind().to_owned())),
				("metadata".to_string(), serde_json::Value::Object(vec![
					("name".to_string(), serde_json::Value::String("fb2".to_string())),
				].into_iter().collect())),
				("spec".to_string(), serde_json::Value::Object(vec![
					("prop1".to_string(), serde_json::Value::String("value1".to_string())),
				].into_iter().collect())),
			].into_iter().collect());
			let request =
				http::Request::post(format!("/apis/{}/{}/namespaces/default/{}",
					<FooBar as k8s_openapi::Resource>::group(), <FooBar as k8s_openapi::Resource>::version(), plural))
				.body(serde_json::to_vec(&fb2).expect("couldn't create custom resource definition"))
				.expect("couldn't create custom resource");
			{
				let response = client.execute(request).expect("couldn't create custom resource");
				crate::get_single_value(response, k8s_openapi::ResponseBody::new, |response, status_code| match response {
					CreateNamespacedFooBarResponse::UnprocessableEntity(_) => Ok(crate::ValueResult::GotValue(())),
					other => Err(format!("{:?} {}", other, status_code).into()),
				}).expect("expected custom resource creation to fail validation");
			}

			let fb3 = serde_json::Value::Object(vec![
				("apiVersion".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::api_version().to_owned())),
				("kind".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::kind().to_owned())),
				("metadata".to_string(), serde_json::Value::Object(vec![
					("name".to_string(), serde_json::Value::String("fb3".to_string())),
				].into_iter().collect())),
				("spec".to_string(), serde_json::Value::Object(vec![
					("prop1".to_string(), serde_json::Value::String("value1".to_string())),
					("prop2".to_string(), serde_json::Value::Bool(true)),
				].into_iter().collect())),
			].into_iter().collect());
			let request =
				http::Request::post(format!("/apis/{}/{}/namespaces/default/{}",
					<FooBar as k8s_openapi::Resource>::group(), <FooBar as k8s_openapi::Resource>::version(), plural))
				.body(serde_json::to_vec(&fb3).expect("couldn't create custom resource definition"))
				.expect("couldn't create custom resource");
			{
				let response = client.execute(request).expect("couldn't create custom resource");
				crate::get_single_value(response, k8s_openapi::ResponseBody::new, |response, status_code| match response {
					CreateNamespacedFooBarResponse::UnprocessableEntity(_) => Ok(crate::ValueResult::GotValue(())),
					other => Err(format!("{:?} {}", other, status_code).into()),
				}).expect("expected custom resource creation to fail validation");
			}
		}


		// Delete CRD
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
