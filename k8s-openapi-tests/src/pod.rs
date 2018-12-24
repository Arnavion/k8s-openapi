#[test]
fn list() {
	use k8s_openapi::api::core::v1 as api;

	crate::Client::with("pod-list", |client| {
		let request = api::Pod::list_core_v1_namespaced_pod("kube-system", Default::default()).expect("couldn't list pods");
		let response = client.execute(request).expect("couldn't list pods");;
		let pod_list =
			crate::get_single_value(response, |response, status_code, _| match response {
				api::ListCoreV1NamespacedPodResponse::Ok(pod_list) => Ok(crate::ValueResult::GotValue(pod_list)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't list pods");

		assert_eq!(k8s_openapi::kind(&pod_list), "PodList");

		let addon_manager_pod =
			pod_list
			.items.into_iter()
			.find(|pod| pod.metadata.as_ref().and_then(|metadata| metadata.name.as_ref()).map_or(false, |name| name.starts_with("kube-addon-manager-")))
			.expect("couldn't find addon-manager pod");

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
