use futures_util::StreamExt;

// rustc wants us to remove this redundant import because we already have it as a dep.
// But it's a good test that `schemars` is actually re-exported from `k8s-openapi`.
#[allow(unused_imports)]
use k8s_openapi::schemars;

use k8s_openapi::serde_json;

#[tokio::test]
async fn test() {
    use k8s_openapi::{
        apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiextensions,
        apimachinery::pkg::{
            api::resource::Quantity,
            apis::meta::v1 as meta,
            util::intstr::IntOrString,
        },
    };

    #[derive(
        Clone, Debug, PartialEq,
        k8s_openapi_derive::CustomResourceDefinition,
        schemars::JsonSchema,
        serde::Deserialize, serde::Serialize,
    )]
    #[custom_resource_definition(
        group = "k8s-openapi-tests-custom-resource-definition.com",
        version = "v1",
        plural = "foobars",
        generate_schema,
        namespaced,
        has_subresources = "v1",
        impl_deep_merge,
    )]
    struct FooBarSpec {
        prop1: String,
        prop2: Vec<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        prop3: Option<i32>,
        prop4: IntOrString,
        prop5: Quantity,
    }

    impl k8s_openapi::DeepMerge for FooBarSpec {
        fn merge_from(&mut self, other: Self) where Self: Sized {
            self.prop1.merge_from(other.prop1);
            k8s_openapi::merge_strategies::list::atomic(&mut self.prop2, other.prop2);
            self.prop3.merge_from(other.prop3);
        }
    }

    assert_eq!(<FooBar as k8s_openapi::Resource>::API_VERSION, "k8s-openapi-tests-custom-resource-definition.com/v1");
    assert_eq!(<FooBar as k8s_openapi::Resource>::GROUP, "k8s-openapi-tests-custom-resource-definition.com");
    assert_eq!(<FooBar as k8s_openapi::Resource>::KIND, "FooBar");
    assert_eq!(<FooBar as k8s_openapi::Resource>::VERSION, "v1");

    assert_eq!(<FooBar as k8s_openapi::ListableResource>::LIST_KIND, "FooBarList");

    {
        let fb: FooBar =
            serde_json::from_str(r#"{ "metadata": {}, "spec": { "prop1": "foo", "prop2": [true, false], "prop3": 5, "prop4": 6, "prop5": 7 } }"#).unwrap();
        assert_eq!(fb, FooBar {
            spec: Some(FooBarSpec { prop1: "foo".to_owned(), prop2: vec![true, false], prop3: Some(5), prop4: IntOrString::Int(6), prop5: Quantity("7".to_owned()) }),
            metadata: Default::default(),
            subresources: Default::default(),
        });
        let fb = serde_json::to_string(&fb).unwrap();
        assert_eq!(fb, "\
            {\
                \"apiVersion\":\"k8s-openapi-tests-custom-resource-definition.com/v1\",\
                \"kind\":\"FooBar\",\
                \"metadata\":{},\
                \"spec\":{\"prop1\":\"foo\",\"prop2\":[true,false],\"prop3\":5,\"prop4\":6,\"prop5\":\"7\"}\
            }\
        ");

        let fb: FooBar =
            serde_json::from_str(r#"{ "metadata": {}, "spec": { "prop1": "foo", "prop2": [true, false], "prop3": 5, "prop4": "bar", "prop5": "7G" } }"#).unwrap();
        assert_eq!(fb, FooBar {
            spec: Some(FooBarSpec { prop1: "foo".to_owned(), prop2: vec![true, false], prop3: Some(5), prop4: IntOrString::String("bar".to_owned()), prop5: Quantity("7G".to_owned()) }),
            metadata: Default::default(),
            subresources: Default::default(),
        });
        let fb = serde_json::to_string(&fb).unwrap();
        assert_eq!(fb, "\
            {\
                \"apiVersion\":\"k8s-openapi-tests-custom-resource-definition.com/v1\",\
                \"kind\":\"FooBar\",\
                \"metadata\":{},\
                \"spec\":{\"prop1\":\"foo\",\"prop2\":[true,false],\"prop3\":5,\"prop4\":\"bar\",\"prop5\":\"7G\"}\
            }\
        ");

        let fb: FooBar =
            serde_json::from_str(r#"{ "metadata": {}, "spec": { "prop1": "foo", "prop2": [true, false], "prop3": 5, "prop4": 6, "prop5": 7 }, "status": { "bar": "baz" } }"#).unwrap();
        assert_eq!(fb, FooBar {
            spec: Some(FooBarSpec { prop1: "foo".to_owned(), prop2: vec![true, false], prop3: Some(5), prop4: IntOrString::Int(6), prop5: Quantity("7".to_owned()) }),
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
                \"spec\":{\"prop1\":\"foo\",\"prop2\":[true,false],\"prop3\":5,\"prop4\":6,\"prop5\":\"7\"},\
                \"status\":{\"bar\":\"baz\"}\
            }\
        ");

        let fb: FooBar =
            serde_json::from_str(r#"{ "metadata": {}, "spec": { "prop1": "foo", "prop2": [true, false], "prop3": 5, "prop4": "bar", "prop5": "7G" }, "status": { "bar": "baz" } }"#).unwrap();
        assert_eq!(fb, FooBar {
            spec: Some(FooBarSpec { prop1: "foo".to_owned(), prop2: vec![true, false], prop3: Some(5), prop4: IntOrString::String("bar".to_owned()), prop5: Quantity("7G".to_owned()) }),
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
                \"spec\":{\"prop1\":\"foo\",\"prop2\":[true,false],\"prop3\":5,\"prop4\":\"bar\",\"prop5\":\"7G\"},\
                \"status\":{\"bar\":\"baz\"}\
            }\
        ");
    }

    let mut client = crate::Client::new("custom_resource_definition-test");
    let plural = "foobars";


    let custom_resource_validation = apiextensions::CustomResourceValidation {
        open_api_v3_schema: Some(apiextensions::JSONSchemaProps {
            properties: Some([
                ("spec".to_string(), apiextensions::JSONSchemaProps {
                    type_: Some("object".to_owned()),
                    properties: Some([
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
                        ("prop4".to_string(), apiextensions::JSONSchemaProps {
                            x_kubernetes_int_or_string: Some(true),
                            ..Default::default()
                        }),
                        ("prop5".to_string(), apiextensions::JSONSchemaProps {
                            x_kubernetes_int_or_string: Some(true),
                            ..Default::default()
                        }),
                    ].into()),
                    required: Some(vec![
                        "prop1".to_string(),
                        "prop2".to_string(),
                        "prop4".to_string(),
                        "prop5".to_string(),
                    ]),
                    ..Default::default()
                }),
            ].into()),
            type_: Some("object".to_owned()),
            ..Default::default()
        }),
    };

    let custom_resource_definition = apiextensions::CustomResourceDefinition {
        metadata: meta::ObjectMeta {
            name: Some(format!("{plural}.{}", <FooBar as k8s_openapi::Resource>::GROUP)),
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
        let (request, response_body) = crate::clientset::create_cluster::<apiextensions::CustomResourceDefinition>(&custom_resource_definition);
        match client.get_single_value(request, response_body).await {
            (crate::clientset::CreateResponse::Created(_), _) |
            (_, reqwest::StatusCode::CONFLICT) => break,
            (_, reqwest::StatusCode::INTERNAL_SERVER_ERROR) => (),
            (other, status_code) => panic!("{other:?} {status_code}"),
        }
    }

    // Wait for CRD to be registered
    loop {
        let (request, response_body) =
            crate::clientset::read_cluster::<apiextensions::CustomResourceDefinition>(&format!("{plural}.{}", <FooBar as k8s_openapi::Resource>::GROUP.to_owned()));
        let custom_resource_definition = match client.get_single_value(request, response_body).await {
            (crate::clientset::ReadResponse::Ok(custom_resource_definition), _) => custom_resource_definition,
            (other, status_code) => panic!("{other:?} {status_code}"),
        };

        let accepted_names_kind = {
            let status = custom_resource_definition.status.as_ref();
            let accepted_names = status.and_then(|status| status.accepted_names.as_ref());

            accepted_names.map(|accepted_names| &accepted_names.kind)
        };
        if accepted_names_kind.is_some_and(|accepted_names_kind| accepted_names_kind == "FooBar") {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
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
            prop4: IntOrString::Int(6),
            prop5: Quantity("7".to_owned()),
        }),
        subresources: Default::default(),
    };
    let (request, response_body) = crate::clientset::create_namespaced::<FooBar>("default", &fb1);
    let fb1 = match client.get_single_value(request, response_body).await {
        (crate::clientset::CreateResponse::Ok(fb) | crate::clientset::CreateResponse::Created(fb), _) => fb,
        (other, status_code) => panic!("{other:?} {status_code}"),
    };

    let fb2 = FooBar {
        metadata: meta::ObjectMeta {
            name: Some("fb2".to_string()),
            ..Default::default()
        },
        spec: Some(FooBarSpec {
            prop1: "value1".to_string(),
            prop2: vec![true, false, true],
            prop3: Some(5),
            prop4: IntOrString::String("value4".to_owned()),
            prop5: Quantity("7G".to_owned()),
        }),
        subresources: Default::default(),
    };
    let (request, response_body) = crate::clientset::create_namespaced::<FooBar>("default", &fb2);
    let fb2 = match client.get_single_value(request, response_body).await {
        (crate::clientset::CreateResponse::Ok(fb) | crate::clientset::CreateResponse::Created(fb), _) => fb,
        (other, status_code) => panic!("{other:?} {status_code}"),
    };


    // List CR
    {
        let (request, response_body) = crate::clientset::list_namespaced::<FooBar>("default");
        let mut foo_bar_list = match client.get_single_value(request, response_body).await {
            (crate::clientset::ListResponse::Ok(foo_bar_list), _) => foo_bar_list,
            (other, status_code) => panic!("{other:?} {status_code}"),
        };
        assert_eq!(k8s_openapi::kind(&foo_bar_list), "FooBarList");

        foo_bar_list.items.sort_by(|item1, item2| item1.metadata.name.as_deref().cmp(&item2.metadata.name.as_deref()));
        let [fb1_2, fb2_2] = &foo_bar_list.items[..] else {
            panic!("didn't find expected FooBars in {foo_bar_list:?}");
        };
        assert_eq!(*fb1_2, fb1);
        assert_eq!(*fb2_2, fb2);
    }


    // Read CR
    {
        let (request, response_body) = crate::clientset::read_namespaced::<FooBar>("default", "fb1");
        let fb1_2 = match client.get_single_value(request, response_body).await {
            (crate::clientset::ReadResponse::Ok(fb), _) => fb,
            (other, status_code) => panic!("{other:?} {status_code}"),
        };
        assert_eq!(fb1_2, fb1);

        let (request, response_body) = crate::clientset::read_namespaced::<FooBar>("default", "fb2");
        let fb2_2 = match client.get_single_value(request, response_body).await {
            (crate::clientset::ReadResponse::Ok(fb), _) => fb,
            (other, status_code) => panic!("{other:?} {status_code}"),
        };
        assert_eq!(fb2_2, fb2);
    }


    // Watch CR
    {
        let (request, response_body) = crate::clientset::watch_namespaced::<FooBar>("default", Default::default());
        let foo_bar_watch_events = std::pin::pin!(client.get_multiple_values(request, response_body));
        let mut items: Vec<_> =
            foo_bar_watch_events
            .filter_map(|foo_bar_watch_event| match foo_bar_watch_event {
                (crate::clientset::WatchResponse::Ok(meta::WatchEvent::Added(fb)), _) => std::future::ready(Some(fb)),
                (crate::clientset::WatchResponse::Ok(_), _) => std::future::ready(None),
                (other, status_code) => panic!("{other:?} {status_code}"),
            })
            .take(2)
            .collect().await;

        items.sort_by(|item1, item2| item1.metadata.name.as_deref().cmp(&item2.metadata.name.as_deref()));
        let [fb1_2, fb2_2] = &items[..] else {
            panic!("didn't find expected FooBars in {items:?}");
        };
        assert_eq!(*fb1_2, fb1);
        assert_eq!(*fb2_2, fb2);
    }


    // Delete CR
    let (request, response_body) = {
        let metadata = &fb1.metadata;
        let name = metadata.name.as_deref().expect("create FooBar response did not set metadata.name");
        let namespace = metadata.namespace.as_deref().expect("create FooBar response did not set metadata.namespace");
        crate::clientset::delete_namespaced::<FooBar>(namespace, name)
    };
    let () = match client.get_single_value(request, response_body).await {
        (crate::clientset::DeleteResponse::OkStatus(_) | crate::clientset::DeleteResponse::OkValue(_), _) => (),
        (other, status_code) => panic!("{other:?} {status_code}"),
    };

    let (request, response_body) = {
        let metadata = &fb2.metadata;
        let name = metadata.name.as_deref().expect("create FooBar response did not set metadata.name");
        let namespace = metadata.namespace.as_deref().expect("create FooBar response did not set metadata.namespace");
        crate::clientset::delete_namespaced::<FooBar>(namespace, name)
    };
    let () = match client.get_single_value(request, response_body).await {
        (crate::clientset::DeleteResponse::OkStatus(_) | crate::clientset::DeleteResponse::OkValue(_), _) => (),
        (other, status_code) => panic!("{other:?} {status_code}"),
    };


    // Create invalid CR.
    let fb3 = serde_json::Value::Object([
        ("apiVersion".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::API_VERSION.to_owned())),
        ("kind".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::KIND.to_owned())),
        ("metadata".to_string(), serde_json::Value::Object([
            ("name".to_string(), serde_json::Value::String("fb3".to_string())),
        ].into_iter().collect())),
        ("spec".to_string(), serde_json::Value::Object([
            ("prop1".to_string(), serde_json::Value::String("value1".to_string())),
            ("prop4".to_string(), serde_json::Value::String("value4".to_string())),
            ("prop5".to_string(), serde_json::Value::String("7G".to_string())),
        ].into_iter().collect())),
    ].into_iter().collect());
    let request =
        http::Request::post(format!("/apis/{}/{}/namespaces/default/{plural}",
            <FooBar as k8s_openapi::Resource>::GROUP, <FooBar as k8s_openapi::Resource>::VERSION))
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_vec(&fb3).expect("couldn't create custom resource definition"))
        .expect("couldn't create custom resource");
    match client.get_single_value(request, crate::clientset::ResponseBody::<crate::clientset::CreateResponse<FooBar>>::new).await {
        (crate::clientset::CreateResponse::Other(Ok(Some(value))), reqwest::StatusCode::UNPROCESSABLE_ENTITY) => {
            let status = <meta::Status as serde::Deserialize>::deserialize(value).unwrap();
            let causes = status.details.as_ref().unwrap().causes.as_deref().unwrap();
            match causes {
                [meta::StatusCause { reason: Some(reason), field: Some(field), .. }] if reason == "FieldValueRequired" && field == "spec.prop2" => (),
                _ => panic!("CR is invalid but for unexpected reason: {status:?}"),
            }
        },
        (other, status_code) => panic!("{other:?} {status_code}"),
    }

    let fb3 = serde_json::Value::Object([
        ("apiVersion".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::API_VERSION.to_owned())),
        ("kind".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::KIND.to_owned())),
        ("metadata".to_string(), serde_json::Value::Object([
            ("name".to_string(), serde_json::Value::String("fb3".to_string())),
        ].into_iter().collect())),
        ("spec".to_string(), serde_json::Value::Object([
            ("prop1".to_string(), serde_json::Value::String("value1".to_string())),
            ("prop2".to_string(), serde_json::Value::Bool(true)),
            ("prop4".to_string(), serde_json::Value::String("value4".to_string())),
            ("prop5".to_string(), serde_json::Value::String("7G".to_string())),
        ].into_iter().collect())),
    ].into_iter().collect());
    let request =
        http::Request::post(format!("/apis/{}/{}/namespaces/default/{plural}",
            <FooBar as k8s_openapi::Resource>::GROUP, <FooBar as k8s_openapi::Resource>::VERSION))
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_vec(&fb3).expect("couldn't create custom resource definition"))
        .expect("couldn't create custom resource");
    match client.get_single_value(request, crate::clientset::ResponseBody::<crate::clientset::CreateResponse<FooBar>>::new).await {
        (crate::clientset::CreateResponse::Other(Ok(Some(value))), reqwest::StatusCode::UNPROCESSABLE_ENTITY) => {
            let status = <meta::Status as serde::Deserialize>::deserialize(value).unwrap();
            let causes = status.details.as_ref().unwrap().causes.as_deref().unwrap();
            match causes {
                [meta::StatusCause { reason: Some(reason), field: Some(field), .. }] if reason == "FieldValueTypeInvalid" && field == "spec.prop2" => (),
                _ => panic!("CR is invalid but for unexpected reason: {status:?}"),
            }
        },
        (other, status_code) => panic!("{other:?} {status_code}"),
    }

    let fb3 = serde_json::Value::Object([
        ("apiVersion".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::API_VERSION.to_owned())),
        ("kind".to_string(), serde_json::Value::String(<FooBar as k8s_openapi::Resource>::KIND.to_owned())),
        ("metadata".to_string(), serde_json::Value::Object([
            ("name".to_string(), serde_json::Value::String("fb3".to_string())),
        ].into_iter().collect())),
        ("spec".to_string(), serde_json::Value::Object([
            ("prop1".to_string(), serde_json::Value::String("value1".to_string())),
            ("prop2".to_string(), serde_json::Value::Array(vec![serde_json::Value::Bool(true)])),
            ("prop4".to_string(), serde_json::Value::Bool(true)),
            ("prop5".to_string(), serde_json::Value::String("7G".to_string())),
        ].into_iter().collect())),
    ].into_iter().collect());
    let request =
        http::Request::post(format!("/apis/{}/{}/namespaces/default/{plural}",
            <FooBar as k8s_openapi::Resource>::GROUP, <FooBar as k8s_openapi::Resource>::VERSION))
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_vec(&fb3).expect("couldn't create custom resource definition"))
        .expect("couldn't create custom resource");
    match client.get_single_value(request, crate::clientset::ResponseBody::<crate::clientset::CreateResponse<FooBar>>::new).await {
        (crate::clientset::CreateResponse::Other(Ok(Some(value))), reqwest::StatusCode::UNPROCESSABLE_ENTITY) => {
            let status = <meta::Status as serde::Deserialize>::deserialize(value).unwrap();
            let causes = status.details.as_ref().unwrap().causes.as_deref().unwrap();
            match causes {
                [meta::StatusCause { reason: Some(reason), field: Some(field), .. }] if reason == "FieldValueTypeInvalid" && field == "spec.prop4" => (),
                _ => panic!("CR is invalid but for unexpected reason: {status:?}"),
            }
        },
        (other, status_code) => panic!("{other:?} {status_code}"),
    }


    // Delete CRD
    let (request, response_body) =
        crate::clientset::delete_cluster::<apiextensions::CustomResourceDefinition>(&format!("{plural}.{}", <FooBar as k8s_openapi::Resource>::GROUP));
    match client.get_single_value(request, response_body).await {
        (crate::clientset::DeleteResponse::OkStatus(_) | crate::clientset::DeleteResponse::OkValue(_), _) => (),
        (other, status_code) => panic!("{other:?} {status_code}"),
    }
}

#[test]
fn dont_require_deep_merge_when_not_requested() {
    #[allow(unused)]
    #[derive(
        Clone, Debug, PartialEq,
        k8s_openapi_derive::CustomResourceDefinition,
        schemars::JsonSchema,
        serde::Deserialize, serde::Serialize,
    )]
    #[custom_resource_definition(
        group = "k8s-openapi-tests-custom-resource-definition.com",
        version = "v1",
        plural = "foobars",
        generate_schema,
        namespaced,
        has_subresources = "v1",
    )]
    struct FooBarSpec {
        prop1: String,
        prop2: Vec<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        prop3: Option<i32>,
    }
}
