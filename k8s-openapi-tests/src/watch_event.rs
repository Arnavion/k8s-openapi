use k8s_openapi::serde_json;

#[test]
fn watch_pods() {
	use k8s_openapi::api::core::v1 as api;

	crate::Client::with("watch_event-watch_pods", |client| {
		let request = api::Pod::list_namespaced_pod("kube-system", api::ListNamespacedPodOptional {
			watch: Some(true),
			..Default::default()
		}).expect("couldn't watch pods");
		let response = client.execute(request).expect("couldn't watch pods");
		let pod_watch_events =
			crate::get_multiple_values(response, |response, status_code, _| match response {
				// The response is a WatchNamespacedPodListResponse, not a ListNamespacedPodResponse,
				// because of the watch=true parameter
				api::WatchNamespacedPodListResponse::Ok(pod_watch_event) =>
					Ok(crate::ValueResult::GotValue(pod_watch_event)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't watch pods");

		let addon_manager_pod =
			pod_watch_events
			.map(|pod_watch_event| pod_watch_event.expect("couldn't get pod watch event"))
			.filter_map(|pod_watch_event| {
				println!("{:?}", pod_watch_event);

				if pod_watch_event.type_ != "ADDED" {
					return None;
				}

				let pod: api::Pod = serde_json::from_value(pod_watch_event.object.0).expect("couldn't re-deserialize pod watch event object");
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
