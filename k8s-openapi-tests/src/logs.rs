#[test]
fn get() {
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::kubernetes::pkg::api::v1 as api;

	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::api::core::v1 as api;

	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::api::core::v1 as api;

	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::api::core::v1 as api;

	let client = ::Client::new().expect("couldn't create client");

	#[cfg(feature = "v1_7")] let pod_list =
		api::Pod::list_core_v1_namespaced_pod(
			&client, "kube-system",
			None, None, None, None, None, None, None)
		.expect("couldn't list pods");
	#[cfg(not(feature = "v1_7"))] let pod_list =
		api::Pod::list_core_v1_namespaced_pod(
			&client, "kube-system",
			None, None, None, None, None, None, None, None, None)
		.expect("couldn't list pods");
	let pod_list = match pod_list {
		#[cfg(feature = "v1_7")] api::ListCoreV1NamespacedPodResponse::Ok(pod_list) => pod_list,
		#[cfg(not(feature = "v1_7"))] api::ListCoreV1NamespacedPodResponse::Ok(pod_list) => pod_list,
		other => panic!("couldn't list pods: {:?}", other),
	};

	let addon_manager_pod =
		pod_list
		.items.into_iter()
		.find(|pod| pod.metadata.as_ref().and_then(|metadata| metadata.name.as_ref()).map_or(false, |name| name.starts_with("kube-addon-manager-")))
		.expect("couldn't find addon-manager pod");

	let addon_manager_pod_name =
		addon_manager_pod
		.metadata.as_ref().expect("couldn't get addon-manager pod metadata")
		.name.as_ref().expect("couldn't get addon-manager pod name");

	let addon_manager_logs =
		api::Pod::read_core_v1_namespaced_pod_log(
			&client,
			addon_manager_pod_name, "kube-system", Some("kube-addon-manager"), None, Some(4096), None, None, None, None, None)
		.expect("couldn't get addon-manager pod logs");
	let addon_manager_logs = match addon_manager_logs {
		api::ReadCoreV1NamespacedPodLogResponse::Ok(logs) => logs,
		other => panic!("couldn't get addon-manager pod logs: {:?}", other),
	};
	assert!(addon_manager_logs.contains("INFO: == Kubernetes addon manager started at"));
}
