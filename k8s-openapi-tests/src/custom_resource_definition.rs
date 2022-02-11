use k8s_openapi::{http, schemars, serde_json};

#[test]
fn test() {
	use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiextensions;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

	#[derive(
		Clone, Debug, PartialEq,
		k8s_openapi_derive::CustomResourceDefinition,
		schemars::JsonSchema,
		serde_derive::Deserialize, serde_derive::Serialize,
	)]
	#[custom_resource_definition(
		group = "k8s-openapi-tests-custom-resource-definition.com",
		version = "v1",
		plural = "foobars",
		generate_schema,
		namespaced,
	)]
	#[custom_resource_definition(has_subresources = "v1")]
	struct FooBarSpec {
		prop1: String,
		prop2: Vec<bool>,
		#[serde(skip_serializing_if = "Option::is_none")]
		prop3: Option<i32>,
	}

	assert_eq!(<FooBar as k8s_openapi::Resource>::API_VERSION, "k8s-openapi-tests-custom-resource-definition.com/v1");
	assert_eq!(<FooBar as k8s_openapi::Resource>::GROUP, "k8s-openapi-tests-custom-resource-definition.com");
	assert_eq!(<FooBar as k8s_openapi::Resource>::KIND, "FooBar");
	assert_eq!(<FooBar as k8s_openapi::Resource>::VERSION, "v1");

	assert_eq!(<FooBar as k8s_openapi::ListableResource>::LIST_KIND, "FooBarList");

	assert_eq!(<FooBarList as k8s_openapi::Resource>::API_VERSION, "k8s-openapi-tests-custom-resource-definition.com/v1");
	assert_eq!(<FooBarList as k8s_openapi::Resource>::GROUP, "k8s-openapi-tests-custom-resource-definition.com");
	assert_eq!(<FooBarList as k8s_openapi::Resource>::KIND, "FooBarList");
	assert_eq!(<FooBarList as k8s_openapi::Resource>::VERSION, "v1");

	{
		let fb: FooBar =
			serde_json::from_str(r#"{ "metadata": {}, "spec": { "prop1": "foo", "prop2": [true, false], "prop3": 5 } }"#).unwrap();
		assert_eq!(fb, FooBar {
			spec: Some(FooBarSpec { prop1: "foo".to_owned(), prop2: vec![true, false], prop3: Some(5) }),
			metadata: Default::default(),
			subresources: Default::default(),
		});
		let fb = serde_json::to_string(&fb).unwrap();
		assert_eq!(fb, "\
			{\
				\"apiVersion\":\"k8s-openapi-tests-custom-resource-definition.com/v1\",\
				\"kind\":\"FooBar\",\
				\"metadata\":{},\
				\"spec\":{\"prop1\":\"foo\",\"prop2\":[true,false],\"prop3\":5}\
			}\
		");

		let fb: FooBar =
			serde_json::from_str(r#"{ "metadata": {}, "spec": { "prop1": "foo", "prop2": [true, false], "prop3": 5 }, "status": { "bar": "baz" } }"#).unwrap();
		assert_eq!(fb, FooBar {
			spec: Some(FooBarSpec { prop1: "foo".to_owned(), prop2: vec![true, false], prop3: Some(5) }),
			metadata: Default::default(),
			subresources: apiextensions::CustomResourceSubresources {
				status: Some(apiextensions::CustomResourceSubresourceStatus(serde_json::json!({ "bar": "baz" }))),
				..Default::default()
			},
		});
		let fb = serde_json::to_string(&fb).unwrap();
		assert_eq!(fb, "\
			{\
				\"apiVersion\":\"k8s-openapi-tests-custom-resource-definition.com/v1\",\
				\"kind\":\"FooBar\",\
				\"metadata\":{},\
				\"spec\":{\"prop1\":\"foo\",\"prop2\":[true,false],\"prop3\":5},\
				\"status\":{\"bar\":\"baz\"}\
			}\
		");
	}

	let mut client = crate::Client::new("custom_resource_definition-test");
	let plural = "foobars";


	let custom_resource_validation = apiextensions::CustomResourceValidation {
		open_api_v3_schema: Some(apiextensions::JSONSchemaProps {
			properties: Some(vec![
				("spec".to_string(), apiextensions::JSONSchemaProps {
					type_: Some("object".to_owned()),
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
			type_: Some("object".to_owned()),
			..Default::default()
		}),
	};

	let custom_resource_definition = apiextensions::CustomResourceDefinition {
		metadata: meta::ObjectMeta {
			name: Some(format!("{}.{}", plural, <FooBar as k8s_openapi::Resource>::GROUP)),
			..Default::default()
		},
		spec: apiextensions::CustomResourceDefinitionSpec {
			group: <FooBar as k8s_openapi::Resource>::GROUP.to_owned(),
			names: apiextensions::CustomResourceDefinitionNames {
				kind: <FooBar as k8s_openapi::Resource>::KIND.to_owned(),
				plural: plural.to_owned(),
				short_names: Some(vec!["fb".to_owned()]),
				singular: Some("foobar".to_owned()),
				..Default::default()
			},
			scope: "Namespaced".to_owned(),
			versions: vec![
				apiextensions::CustomResourceDefinitionVersion {
					name: <FooBar as k8s_openapi::Resource>::VERSION.to_owned(),
					schema: Some(custom_resource_validation),
					served: true,
					storage: true,
					subresources: Some(apiextensions::CustomResourceSubresources {
						status: Some(apiextensions::CustomResourceSubresourceStatus(serde_json::Value::Object(Default::default()))),
						..Default::default()
					}),
					..Default::default()
				},
			],
			..Default::default()
		},
		..Default::default()
	};

	loop {
		let (request, response_body) =
			apiextensions::CustomResourceDefinition::create_custom_resource_definition(&custom_resource_definition, Default::default())
			.expect("couldn't create custom resource definition");
		match client.get_single_value(request, response_body) {
			(k8s_openapi::CreateResponse::Created(_), _) |
			(_, http::StatusCode::CONFLICT) => break,
			(_, http::StatusCode::INTERNAL_SERVER_ERROR) => (),
			(other, status_code) => panic!("{:?} {}", other, status_code),
		}
	}

	// Wait for CRD to be registered
	loop {
		let (request, response_body) =
			apiextensions::CustomResourceDefinition::read_custom_resource_definition(
				&format!("{}.{}", plural, <FooBar as k8s_openapi::Resource>::GROUP.to_owned()), Default::default())
			.expect("couldn't get custom resource definition");
		let custom_resource_definition = match client.get_single_value(request, response_body) {
			(apiextensions::ReadCustomResourceDefinitionResponse::Ok(custom_resource_definition), _) => custom_resource_definition,
			(other, status_code) => panic!("{:?} {}", other, status_code),
		};

		let accepted_names_kind = {
			let status = custom_resource_definition.status.as_ref();
			k8s_openapi::k8s_if_le_1_17! {
				let accepted_names = status.map(|status| &status.accepted_names);
			}
			k8s_openapi::k8s_if_ge_1_18! {
				let accepted_names = status.and_then(|status| status.accepted_names.as_ref());
			}
			accepted_names.map(|accepted_names| &accepted_names.kind)
		};
		if accepted_names_kind.map_or(false, |accepted_names_kind| accepted_names_kind == "FooBar") {
			break;
		}

		client.sleep(std::time::Duration::from_secs(1));
	}


	// Create CR
	let fb1 = FooBar {
		metadata: meta::ObjectMeta {
			name: Some("fb1".to_string()),
			..Default::default()
		},
		spec: Some(FooBarSpec {
			prop1: "value1".to_string(),
			prop2: vec![true, false, true],
			prop3: None,
		}),
		subresources: Default::default(),
	};
	let (request, response_body) =
		FooBar::create_namespaced_foo_bar("default", &fb1)
		.expect("couldn't create FooBar");
	let fb1 = match client.get_single_value(request, response_body) {
		(k8s_openapi::CreateResponse::Ok(fb) | k8s_openapi::CreateResponse::Created(fb), _) => fb,
		(other, status_code) => panic!("{:?} {}", other, status_code),
	};


	// List CR
	let (request, response_body) = FooBar::list_namespaced_foo_bar("default", Default::default()).expect("couldn't list FooBars");
	let foo_bar_list = match client.get_single_value(request, response_body) {
		(k8s_openapi::ListResponse::Ok(foo_bar_list), _) => foo_bar_list,
		(other, status_code) => panic!("{:?} {}", other, status_code),
	};
	assert_eq!(k8s_openapi::kind(&foo_bar_list), "FooBarList");
	let _ =
		foo_bar_list
		.items.into_iter()
		.find(|fb| fb.metadata.name.as_ref().map_or(false, |name| name == "fb1"))
		.expect("couldn't find FooBar");


	// Read CR
	let (request, response_body) = FooBar::read_namespaced_foo_bar("fb1", "default").expect("couldn't read FooBar");
	let fb1_2 = match client.get_single_value(request, response_body) {
		(ReadNamespacedFooBarResponse::Ok(fb), _) => fb,
		(other, status_code) => panic!("{:?} {}", other, status_code),
	};
	assert_eq!(fb1_2.metadata.name.as_ref().unwrap(), "fb1");


	// Watch CR
	{
		let (request, response_body) = FooBar::watch_namespaced_foo_bar("default", Default::default()).expect("couldn't watch FooBars");
		let mut foo_bar_watch_events = client.get_multiple_values(request, response_body);
		let _ =
			foo_bar_watch_events
			.find_map(|foo_bar_watch_event| {
				let fb = match foo_bar_watch_event {
					(k8s_openapi::WatchResponse::Ok(meta::WatchEvent::Added(fb)), _) => fb,
					(k8s_openapi::WatchResponse::Ok(_), _) => return None,
					(other, status_code) => panic!("{:?} {}", other, status_code),
				};

				let name = fb.metadata.name.as_deref()?;
				if name == "fb1" {
					Some(fb)
				}
				else {
					None
				}
			}).expect("couldn't find FooBar");
	}


	// Delete CR
	let (request, response_body) = {
		let metadata = &fb1.metadata;
		let name = metadata.name.as_ref().expect("create FooBar response did not set metadata.name");
		let namespace = metadata.namespace.as_ref().expect("create FooBar response did not set metadata.namespace");
		FooBar::delete_namespaced_foo_bar(name, namespace, Default::default()).expect("couldn't delete FooBar")
	};
	let () = match client.get_single_value(request, response_body) {
		(k8s_openapi::DeleteResponse::OkStatus(_) | k8s_openapi::DeleteResponse::OkValue(_), _) => (),
		(other, status_code) => panic!("{:?} {}", other, status_code),
	};


	// Create invalid CR.
	let fb2 = serde_json::Value::Object(vec![
		("apiVersion".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::API_VERSION.to_owned())),
		("kind".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::KIND.to_owned())),
		("metadata".to_string(), serde_json::Value::Object(vec![
			("name".to_string(), serde_json::Value::String("fb2".to_string())),
		].into_iter().collect())),
		("spec".to_string(), serde_json::Value::Object(vec![
			("prop1".to_string(), serde_json::Value::String("value1".to_string())),
		].into_iter().collect())),
	].into_iter().collect());
	let request =
		http::Request::post(format!("/apis/{}/{}/namespaces/default/{}",
			<FooBar as k8s_openapi::Resource>::GROUP, <FooBar as k8s_openapi::Resource>::VERSION, plural))
		.header(http::header::CONTENT_TYPE, "application/json")
		.body(serde_json::to_vec(&fb2).expect("couldn't create custom resource definition"))
		.expect("couldn't create custom resource");
	match client.get_single_value(request, k8s_openapi::ResponseBody::<k8s_openapi::CreateResponse<FooBar>>::new) {
		(_, http::StatusCode::UNPROCESSABLE_ENTITY) => (),
		(other, status_code) => panic!("{:?} {}", other, status_code),
	}

	let fb3 = serde_json::Value::Object(vec![
		("apiVersion".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::API_VERSION.to_owned())),
		("kind".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::KIND.to_owned())),
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
			<FooBar as k8s_openapi::Resource>::GROUP, <FooBar as k8s_openapi::Resource>::VERSION, plural))
		.header(http::header::CONTENT_TYPE, "application/json")
		.body(serde_json::to_vec(&fb3).expect("couldn't create custom resource definition"))
		.expect("couldn't create custom resource");
	match client.get_single_value(request, k8s_openapi::ResponseBody::<k8s_openapi::CreateResponse<FooBar>>::new) {
		(_, http::StatusCode::UNPROCESSABLE_ENTITY) => (),
		(other, status_code) => panic!("{:?} {}", other, status_code),
	}


	// Delete CRD
	let (request, response_body) =
		apiextensions::CustomResourceDefinition::delete_custom_resource_definition(
			&format!("{}.{}", plural, <FooBar as k8s_openapi::Resource>::GROUP),
			Default::default())
		.expect("couldn't delete custom resource definition");
	match client.get_single_value(request, response_body) {
		(k8s_openapi::DeleteResponse::OkStatus(_) | k8s_openapi::DeleteResponse::OkValue(_), _) => (),
		(other, status_code) => panic!("{:?} {}", other, status_code),
	}
}
