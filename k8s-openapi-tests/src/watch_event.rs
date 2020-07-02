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
