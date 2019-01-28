#[test]
fn get() {
	use k8s_openapi::api::core::v1 as api;

	crate::Client::with("logs-get", |client| {
		let request = api::Pod::list_namespaced_pod("kube-system", Default::default()).expect("couldn't list pods");
		let pod_list = {
			let response = client.execute(request).expect("couldn't list pods");;
			crate::get_single_value(response, |response, status_code, _| match response {
				api::ListNamespacedPodResponse::Ok(pod_list) => Ok(crate::ValueResult::GotValue(pod_list)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't list pods")
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

		let request =
			api::Pod::read_namespaced_pod_log(addon_manager_pod_name, "kube-system", api::ReadNamespacedPodLogOptional {
				container: Some("kube-addon-manager"),
				..Default::default()
			})
			.expect("couldn't get addon-manager pod logs");
		let mut addon_manager_logs = String::new();
		let strings = {
			let response = client.execute(request).expect("couldn't get addon-manager pod logs");
			crate::get_multiple_values(response, |response, status_code, _| match response {
				api::ReadNamespacedPodLogResponse::Ok(s) => Ok(crate::ValueResult::GotValue(s)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't get addon-manager pod logs")
		};
		let mut found_line = false;
		for s in strings {
			let s = s.expect("couldn't get addon-manager pod logs");

			addon_manager_logs.push_str(&s);

			if addon_manager_logs.contains("INFO: == Kubernetes addon manager started at") {
				found_line = true;
				break;
			}

			if addon_manager_logs.len() > 4096 {
				panic!("did not find expected text in addon-manager pod logs: {}", addon_manager_logs);
			}
		}
		if !found_line {
			panic!("did not find expected text in addon-manager pod logs: {}", addon_manager_logs);
		}
	});
}
