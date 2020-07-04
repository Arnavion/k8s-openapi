#[test]
fn watch_pods() {
	use k8s_openapi::api::core::v1 as api;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

	crate::Client::with("watch_event-watch_pods", |client| {
		let (request, response_body) =
			api::Pod::watch_namespaced_pod("kube-system", Default::default()).expect("couldn't watch pods");
		let response = client.execute(request);
		let pod_watch_events =
			crate::get_multiple_values(response, response_body, |response, status_code| match response {
				k8s_openapi::WatchResponse::Ok(pod_watch_event) => crate::ValueResult::GotValue(pod_watch_event),
				other => panic!("{:?} {}", other, status_code),
			});

		let apiserver_pod =
			pod_watch_events
			.filter_map(|pod_watch_event| {
				let pod = match pod_watch_event {
					meta::WatchEvent::Added(pod) => pod,
					_ => return None,
				};

				let name = pod.metadata.name.as_ref()?;
				if name.starts_with("kube-apiserver-") {
					Some(pod)
				}
				else {
					None
				}
			})
			.next().expect("couldn't find apiserver pod");

		let apiserver_container_spec =
			apiserver_pod
			.spec.expect("couldn't get apiserver pod spec")
			.containers
			.into_iter()
			.next().expect("couldn't get apiserver container spec");
		assert_eq!(apiserver_container_spec.name, "kube-apiserver");

		let apiserver_pod_status = apiserver_pod.status.expect("couldn't get apiserver pod status");
		assert_eq!(apiserver_pod_status.phase, Some("Running".to_string()));
	});
}

k8s_if_ge_1_15! {
	#[test]
	fn bookmark_events() {
		use k8s_openapi::api::core::v1 as api;
		use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

		const SUCCESS_TEST_CASES: &[&[u8]] = &[
			// Minimal number of required fields
			br#"{
				"type": "BOOKMARK",
				"object": {
					"apiVersion": "v1",
					"kind": "Pod",
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
			// Mismatched API version
			br#"{
				"type": "BOOKMARK",
				"object": {
					"apiVersion": "v2",
					"kind": "Pod",
					"metadata": {
						"resourceVersion": "123"
					}
				}
			}"#,

			// Mismatched kind
			br#"{
				"type": "BOOKMARK",
				"object": {
					"apiVersion": "v1",
					"kind": "Node",
					"metadata": {
						"resourceVersion": "123"
					}
				}
			}"#,

			// Missing metadata
			br#"{
				"type": "BOOKMARK",
				"object": {
					"apiVersion": "v1",
					"kind": "Pod"
				}
			}"#,

			// Missing metadata.resourceVersion
			br#"{
				"type": "BOOKMARK",
				"object": {
					"apiVersion": "v1",
					"kind": "Pod",
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
				watch_response => panic!("hard-coded test case did not deserialize as expected: {:?}", watch_response),
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
				err => panic!("hard-coded test case did not fail to deserialize as expected: {:?}", err),
			}
		}
	}
}
