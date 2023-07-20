use k8s_openapi::serde_json;

use k8s_openapi::api::core::v1 as api;
use k8s_openapi::api::apps::v1 as apps;
use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

#[tokio::test]
async fn deployment() {
    let mut client = crate::Client::new("patch-deployment");

    // Create deployment with container that uses alpine:3.6
    let deployment_spec = apps::DeploymentSpec {
        replicas: Some(1),
        selector: meta::LabelSelector {
            match_labels: Some([
                ("k8s-openapi-tests-patch-deployment-key".to_owned(), "k8s-openapi-tests-patch-deployment-value".to_owned()),
            ].into()),
            ..Default::default()
        },
        template: api::PodTemplateSpec {
            metadata: Some(meta::ObjectMeta {
                labels: Some([
                    ("k8s-openapi-tests-patch-deployment-key".to_owned(), "k8s-openapi-tests-patch-deployment-value".to_owned()),
                ].into()),
                ..Default::default()
            }),
            spec: Some(api::PodSpec {
                containers: vec![
                    api::Container {
                        name: "k8s-openapi-tests-patch-deployment".to_owned(),
                        image: "alpine:3.6".to_owned().into(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
        },
        ..Default::default()
    };
    let deployment = apps::Deployment {
        metadata: meta::ObjectMeta {
            name: Some("k8s-openapi-tests-patch-deployment".to_owned()),
            ..Default::default()
        },
        spec: Some(deployment_spec),
        ..Default::default()
    };
    let (request, response_body) =
        apps::Deployment::create("default", &deployment, Default::default())
        .expect("couldn't create deployment");
    match client.get_single_value(request, response_body).await {
        (k8s_openapi::CreateResponse::Created(_), _) => (),
        (other, status_code) => panic!("{other:?} {status_code}"),
    }


    // Use JSON patch to patch deployment with alpine:3.7 container
    let patch = meta::Patch::Json(vec![
        serde_json::Value::Object([
            ("op".to_owned(), serde_json::Value::String("test".to_owned())),
            ("path".to_owned(), serde_json::Value::String("/spec/template/spec/containers/0/image".to_owned())),
            ("value".to_owned(), serde_json::Value::String("alpine:3.6".to_owned())),
        ].into_iter().collect()),
        serde_json::Value::Object([
            ("op".to_owned(), serde_json::Value::String("replace".to_owned())),
            ("path".to_owned(), serde_json::Value::String("/spec/template/spec/containers/0/image".to_owned())),
            ("value".to_owned(), serde_json::Value::String("alpine:3.7".to_owned())),
        ].into_iter().collect()),
    ]);
    patch_and_assert_container_has_image(&mut client, &patch, "alpine:3.7").await;


    // Use merge patch to patch deployment with alpine:3.8 container
    let patch = apps::Deployment {
        spec: Some(apps::DeploymentSpec {
            template: api::PodTemplateSpec {
                spec: Some(api::PodSpec {
                    containers: vec![
                        api::Container {
                            name: "k8s-openapi-tests-patch-deployment".to_owned(),
                            image: "alpine:3.8".to_owned().into(),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    };
    let patch = meta::Patch::Merge(serde_json::to_value(&patch).expect("couldn't create patch"));
    patch_and_assert_container_has_image(&mut client, &patch, "alpine:3.8").await;


    // Use strategic merge patch to patch deployment with alpine:3.9 container
    let patch = apps::Deployment {
        spec: Some(apps::DeploymentSpec {
            template: api::PodTemplateSpec {
                spec: Some(api::PodSpec {
                    containers: vec![
                        api::Container {
                            name: "k8s-openapi-tests-patch-deployment".to_owned(),
                            image: "alpine:3.9".to_owned().into(),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    };
    let patch = meta::Patch::StrategicMerge(serde_json::to_value(&patch).expect("couldn't create patch"));
    patch_and_assert_container_has_image(&mut client, &patch, "alpine:3.9").await;


    // Delete deployment
    let (request, response_body) =
        apps::Deployment::delete("k8s-openapi-tests-patch-deployment", "default", Default::default())
        .expect("couldn't delete deployment");
    match client.get_single_value(request, response_body).await {
        (k8s_openapi::DeleteResponse::OkStatus(_) | k8s_openapi::DeleteResponse::OkValue(_), _) => (),
        (other, status_code) => panic!("{other:?} {status_code}"),
    }

    // Delete all pods of the deployment using label selector
    let (request, response_body) =
        api::Pod::delete_collection(
            "default",
            Default::default(),
            k8s_openapi::ListOptional {
                label_selector: Some("k8s-openapi-tests-patch-deployment-key=k8s-openapi-tests-patch-deployment-value"),
                ..Default::default()
            },
        )
        .expect("couldn't delete pods collection");
    match client.get_single_value(request, response_body).await {
        (k8s_openapi::DeleteResponse::OkStatus(_) | k8s_openapi::DeleteResponse::OkValue(_), _) => (),
        (other, status_code) => panic!("{other:?} {status_code}"),
    }
}

/// Patch the deployment with the given path, and assert that the patched deployment has a container with the expected image
async fn patch_and_assert_container_has_image(client: &mut crate::Client, patch: &meta::Patch, expected_image: &str) {
    let (request, response_body) =
        apps::Deployment::patch("k8s-openapi-tests-patch-deployment", "default", patch, Default::default())
        .expect("couldn't create patch");

    let deployment = match client.get_single_value(request, response_body).await {
        (k8s_openapi::PatchResponse::Ok(deployment), _) => deployment,
        (other, status_code) => panic!("{other:?} {status_code}"),
    };

    let image =
        deployment
        .spec.expect("couldn't get deployment spec")
        .template
        .spec.expect("couldn't get pod spec")
        .containers
        .into_iter()
        .next().expect("couldn't get container spec")
        .image.expect("couldn't get image from container spec");

    assert_eq!(image, expected_image);
}
