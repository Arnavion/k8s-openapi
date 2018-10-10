#[test]
fn get() {
	k8s_if_1_7! {
		use ::k8s_openapi::v1_7::kubernetes::pkg::api::v1 as api;
	}
	k8s_if_1_8! {
		use ::k8s_openapi::v1_8::api::core::v1 as api;
	}
	k8s_if_1_9! {
		use ::k8s_openapi::v1_9::api::core::v1 as api;
	}
	k8s_if_1_10! {
		use ::k8s_openapi::v1_10::api::core::v1 as api;
	}
	k8s_if_1_11! {
		use ::k8s_openapi::v1_11::api::core::v1 as api;
	}
	k8s_if_1_12! {
		use ::k8s_openapi::v1_12::api::core::v1 as api;
	}

	let client = ::Client::new().expect("couldn't create client");

	k8s_if_le_1_7! {
		let request =
			api::Pod::list_core_v1_namespaced_pod("kube-system", None, None, None, None, None, None, None);
	}
	k8s_if_ge_1_8! {
		let request =
			api::Pod::list_core_v1_namespaced_pod("kube-system", None, None, None, None, None, None, None, None, None);
	}
	let request = request.expect("couldn't list pods");
	let response = client.execute(request).expect("couldn't list pods");;
	let pod_list =
		::get_single_value(response, |response, status_code, _| match response {
			api::ListCoreV1NamespacedPodResponse::Ok(pod_list) => Ok(::ValueResult::GotValue(pod_list)),
			other => Err(format!("{:?} {}", other, status_code).into()),
		}).expect("couldn't list pods");

	let addon_manager_pod =
		pod_list
		.items.into_iter()
		.find(|pod| pod.metadata.as_ref().and_then(|metadata| metadata.name.as_ref()).map_or(false, |name| name.starts_with("kube-addon-manager-")))
		.expect("couldn't find addon-manager pod");

	let addon_manager_pod_name =
		addon_manager_pod
		.metadata.as_ref().expect("couldn't get addon-manager pod metadata")
		.name.as_ref().expect("couldn't get addon-manager pod name");

	let request =
		api::Pod::read_core_v1_namespaced_pod_log(
			addon_manager_pod_name, "kube-system", Some("kube-addon-manager"), None, None, None, None, None, None, None)
		.expect("couldn't get addon-manager pod logs");
	let response = client.execute(request).expect("couldn't get addon-manager pod logs");
	let mut addon_manager_logs = String::new();
	let strings = ::get_multiple_values(response, |response, status_code, _| match response {
		api::ReadCoreV1NamespacedPodLogResponse::Ok(s) => Ok(::ValueResult::GotValue(s)),
		other => Err(format!("{:?} {}", other, status_code).into()),
	}).expect("couldn't get addon-manager pod logs");
	for s in strings {
		let s = s.expect("couldn't get addon-manager pod logs");

		addon_manager_logs.push_str(&s);

		if addon_manager_logs.contains("INFO: == Kubernetes addon manager started at") {
			break;
		}

		if addon_manager_logs.len() > 4096 {
			panic!("did not find expected text in addon-manager pod logs: {}", addon_manager_logs);
		}
	}
}
