#[test]
fn watch_pods() {
	use k8s_openapi::api::core::v1 as api;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

	crate::Client::with("watch_event-watch_pods", |client| {
		let (request, response_body) =
			api::Pod::watch_namespaced_pod("kube-system", Default::default()).expect("couldn't watch pods");
		let response = client.execute(request).expect("couldn't watch pods");
		let pod_watch_events =
			crate::get_multiple_values(response, response_body, |response, status_code| match response {
				k8s_openapi::WatchResponse::Ok(pod_watch_event) =>
					Ok(crate::ValueResult::GotValue(pod_watch_event)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't watch pods");

		let addon_manager_pod =
			pod_watch_events
			.map(|pod_watch_event| pod_watch_event.expect("couldn't get pod watch event"))
			.filter_map(|pod_watch_event| {
				println!("{:?}", pod_watch_event);

				let pod = match pod_watch_event {
					meta::WatchEvent::Added(pod) => pod,
					_ => return None,
				};

				if pod.metadata.as_ref().and_then(|metadata| metadata.name.as_ref()).map_or(false, |name| name.starts_with("kube-addon-manager-")) {
					Some(pod)
				}
				else {
					None
				}
			})
			.next().expect("couldn't find addon-manager pod");

		let addon_manager_container_spec =
			addon_manager_pod
			.spec.expect("couldn't get addon-manager pod spec")
			.containers
			.into_iter().next().expect("couldn't get addon-manager container spec");
		assert_eq!(addon_manager_container_spec.name, "kube-addon-manager");

		let addon_manager_pod_status = addon_manager_pod.status.expect("couldn't get addon-manager pod status");
		assert_eq!(addon_manager_pod_status.phase, Some("Running".to_string()));
	});
}
