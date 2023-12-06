use futures_util::StreamExt;

#[tokio::test]
async fn watch_pods() {
    use k8s_openapi::api::core::v1 as api;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

    let mut client = crate::Client::new("watch_event-watch_pods");

    let (request, response_body) = crate::clientset::watch_namespaced::<api::Pod>("kube-system", Default::default());
    let pod_watch_events = std::pin::pin!(client.get_multiple_values(request, response_body));

    let apiserver_pod =
        pod_watch_events
        .filter_map(|pod_watch_event| {
            let pod = match pod_watch_event {
                (crate::clientset::WatchResponse::Ok(meta::WatchEvent::Added(pod)), _) => pod,
                (crate::clientset::WatchResponse::Ok(_), _) => return std::future::ready(None),
                (other, status_code) => panic!("{other:?} {status_code}"),
            };

            let name = pod.metadata.name.as_deref();
            if name.map(|name| name.starts_with("kube-apiserver-")).unwrap_or_default() {
                std::future::ready(Some(pod))
            }
            else {
                std::future::ready(None)
            }
        })
        .next().await
        .expect("couldn't find apiserver pod");

    let apiserver_container_spec =
        apiserver_pod
        .spec.expect("couldn't get apiserver pod spec")
        .containers
        .into_iter()
        .next().expect("couldn't get apiserver container spec");
    assert_eq!(apiserver_container_spec.name, "kube-apiserver");

    let apiserver_pod_status = apiserver_pod.status.expect("couldn't get apiserver pod status");
    assert_eq!(apiserver_pod_status.phase, Some("Running".to_string()));
}

#[cfg(k8s_watch_send_initial_events)]
#[tokio::test]
async fn watch_pods_without_initial_events() {
    use k8s_openapi::api::core::v1 as api;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

    let mut client = crate::Client::new("watch_event-watch_pods_without_initial_events");

    let (request, response_body) = crate::clientset::watch_namespaced::<api::Pod>("kube-system", crate::clientset::WatchOptional {
        allow_watch_bookmarks: Some(true),
        resource_version: Some("0"),
        resource_version_match: Some("NotOlderThan"),
        send_initial_events: Some(true),
    });
    let pod_watch_events = std::pin::pin!(client.get_multiple_values(request, response_body));

    let initial_events_end_annotation =
        pod_watch_events
        .filter_map(|pod_watch_event| {
            let initial_events_end_annotation = match pod_watch_event {
                (crate::clientset::WatchResponse::Ok(meta::WatchEvent::Bookmark { mut annotations, resource_version: _ }), _) => annotations.remove("k8s.io/initial-events-end"),
                (crate::clientset::WatchResponse::Ok(_), _) => return std::future::ready(None),
                (other, status_code) => panic!("{other:?} {status_code}"),
            };

            std::future::ready(initial_events_end_annotation)
        })
        .next().await
        .expect("couldn't find initial events end annotation");

    assert_eq!(initial_events_end_annotation, "true");
}

#[test]
fn bookmark_events() {
    use k8s_openapi::api::core::v1 as api;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

    let success_test_cases: &[(&[u8], _)] = &[
        // Minimal number of required fields
        (br#"{
            "type": "BOOKMARK",
            "object": {
                "metadata": {
                    "resourceVersion": "123"
                }
            }
        }"#, meta::WatchEvent::Bookmark {
            annotations: Default::default(),
            resource_version: "123".to_owned(),
        }),

        // Optionally contains annotations
        (br#"{
            "type": "BOOKMARK",
            "object": {
                "metadata": {
                    "annotations": {
                        "foo": "bar"
                    },
                    "resourceVersion": "123"
                }
            }
        }"#, meta::WatchEvent::Bookmark {
            annotations: [("foo".to_owned(), "bar".to_owned())].into_iter().collect(),
            resource_version: "123".to_owned(),
        }),

        // Extra fields that should be ignored
        (br#"{
            "type": "BOOKMARK",
            "object": {
                "apiVersion": "v1",
                "kind": "Pod",
                "metadata": {
                    "resourceVersion": "123",
                    "creationTimestamp": null
                },
                "spec": {
                    "containers": null
                },
                "status": {}
            }
        }"#, meta::WatchEvent::Bookmark {
            annotations: Default::default(),
            resource_version: "123".to_owned(),
        }),
    ];

    let failure_test_cases: &[&[u8]] = &[
        // Missing metadata
        br#"{
            "type": "BOOKMARK",
            "object": {
            }
        }"#,

        // Missing metadata.resourceVersion
        br#"{
            "type": "BOOKMARK",
            "object": {
                "metadata": {}
            }
        }"#,
    ];

    for (input, expected) in success_test_cases {
        let watch_response =
            crate::clientset::Response::try_from_parts(reqwest::StatusCode::OK, input)
            .expect("expected hard-coded test case to be deserialized successfully but it failed to deserialize");
        let watch_event = match watch_response {
            (crate::clientset::WatchResponse::<api::Pod>::Ok(watch_event), read) if read == input.len() => watch_event,
            watch_response => panic!("hard-coded test case did not deserialize as expected: {watch_response:?}"),
        };
        assert_eq!(watch_event, *expected);
    }

    for input in failure_test_cases {
        let err =
            <crate::clientset::WatchResponse::<api::Pod> as crate::clientset::Response>::try_from_parts(reqwest::StatusCode::OK, input)
            .expect_err("expected hard-coded failure test case to fail to deserialize but it deserialized successfully");
        match err {
            crate::clientset::ResponseError::Json(_) => (),
            crate::clientset::ResponseError::NeedMoreData => panic!("hard-coded test case did not fail to deserialize as expected: {err:?}"),
        }
    }
}
