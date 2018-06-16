#[test]
fn watch_pods() {
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::kubernetes::pkg::api::v1 as api;
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::apimachinery::pkg::apis::meta::v1 as meta;

	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::api::core::v1 as api;
	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::apimachinery::pkg::apis::meta::v1 as meta;

	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::api::core::v1 as api;
	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::apimachinery::pkg::apis::meta::v1 as meta;

	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::api::core::v1 as api;
	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::apimachinery::pkg::apis::meta::v1 as meta;

	let path = "/api/v1/watch/namespaces/kube-system/pods";

	let client = ::Client::new().expect("couldn't create client");

	let pod_watch_events = client.watch::<meta::WatchEvent>(path).expect("couldn't watch pods");

	let addon_manager_pod =
		pod_watch_events
		.map(|pod_watch_event| pod_watch_event.expect("couldn't get pod watch event"))
		.filter_map(|pod_watch_event| {
			if pod_watch_event.type_ != "ADDED" {
				return None;
			}

			let pod: api::Pod = ::serde_json::from_value(pod_watch_event.object).expect("couldn't re-deserialize pod watch event object");
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
}
