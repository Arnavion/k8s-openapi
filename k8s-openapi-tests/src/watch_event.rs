use futures_util::StreamExt;

#[tokio::test]
async fn watch_pods() {
	use k8s_openapi::api::core::v1 as api;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

	let mut client = crate::Client::new("watch_event-watch_pods");

	let (request, response_body) =
		api::Pod::watch_namespaced_pod("kube-system", Default::default()).expect("couldn't watch pods");
	let pod_watch_events = client.get_multiple_values(request, response_body);
	futures_util::pin_mut!(pod_watch_events);

	let apiserver_pod =
		pod_watch_events
		.filter_map(|pod_watch_event| {
			let pod = match pod_watch_event {
				(k8s_openapi::WatchResponse::Ok(meta::WatchEvent::Added(pod)), _) => pod,
				(k8s_openapi::WatchResponse::Ok(_), _) => return std::future::ready(None),
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

#[test]
fn bookmark_events() {
	use k8s_openapi::api::core::v1 as api;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

	const SUCCESS_TEST_CASES: &[&[u8]] = &[
		// Minimal number of required fields
		br#"{
			"type": "BOOKMARK",
			"object": {
				"metadata": {
					"resourceVersion": "123"
				}
			}
		}"#,

		// Extra fields that should be ignored
		br#"{
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
		}"#,
	];

	const FAILURE_TEST_CASES: &[&[u8]] = &[
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

	for test_case in SUCCESS_TEST_CASES {
		let watch_response =
			k8s_openapi::Response::try_from_parts(
				k8s_openapi::http::StatusCode::OK,
				test_case,
			)
			.expect("expected hard-coded test case to be deserialized successfully but it failed to deserialize");
		let watch_event = match watch_response {
			(k8s_openapi::WatchResponse::<api::Pod>::Ok(watch_event), read) if read == test_case.len() => watch_event,
			watch_response => panic!("hard-coded test case did not deserialize as expected: {watch_response:?}"),
		};
		assert_eq!(watch_event, meta::WatchEvent::Bookmark {
			resource_version: "123".to_owned(),
		});
	}

	for test_case in FAILURE_TEST_CASES {
		let err =
			<k8s_openapi::WatchResponse::<api::Pod> as k8s_openapi::Response>::try_from_parts(
				k8s_openapi::http::StatusCode::OK,
				test_case,
			)
			.expect_err("expected hard-coded failure test case to fail to deserialize but it deserialized successfully");
		match err {
			k8s_openapi::ResponseError::Json(_) => (),
			err => panic!("hard-coded test case did not fail to deserialize as expected: {err:?}"),
		}
	}
}
