#[test]
fn list() {
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::kubernetes::pkg::apis::apps::v1beta1 as apps;

	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::api::apps::v1beta2 as apps;

	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::api::apps::v1 as apps;

	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::api::apps::v1 as apps;

	let client = ::Client::new().expect("couldn't create client");

	#[cfg(feature = "v1_7")] let deployment_list =
		apps::Deployment::list_apps_v1beta1_namespaced_deployment(
			&client, "kube-system",
			None, None, None, None, None, None, None).expect("couldn't list deployments");
	#[cfg(feature = "v1_8")] let deployment_list =
		apps::Deployment::list_apps_v1beta2_namespaced_deployment(
			&client, "kube-system",
			None, None, None, None, None, None, None, None, None).expect("couldn't list deployments");
	#[cfg(any(feature = "v1_9", feature = "v1_10"))] let deployment_list =
		apps::Deployment::list_apps_v1_namespaced_deployment(
			&client, "kube-system",
			None, None, None, None, None, None, None, None, None).expect("couldn't list deployments");
	let deployment_list = match deployment_list {
		#[cfg(feature = "v1_7")] apps::ListAppsV1beta1NamespacedDeploymentResponse::Ok(deployment_list) => deployment_list,
		#[cfg(feature = "v1_8")] apps::ListAppsV1beta2NamespacedDeploymentResponse::Ok(deployment_list) => deployment_list,
		#[cfg(any(feature = "v1_9", feature = "v1_10"))] apps::ListAppsV1NamespacedDeploymentResponse::Ok(deployment_list) => deployment_list,
		other => panic!("couldn't list deployments: {:?}", other),
	};
	assert_eq!(deployment_list.kind, Some("DeploymentList".to_string()));

	let kube_dns_deployment =
		deployment_list
		.items.into_iter()
		.find(|deployment| deployment.metadata.as_ref().and_then(|metadata| metadata.name.as_ref()).map(AsRef::as_ref) == Some("kube-dns"))
		.expect("couldn't find kube-dns deployment");

	let kube_dns_deployment_spec =
		kube_dns_deployment
		.spec.expect("couldn't get kube-dns deployment spec");

	let kube_dns_deployment_spec_selector =
		kube_dns_deployment_spec.selector;

	#[cfg(feature = "v1_7")]
	let kube_dns_deployment_spec_selector =
		kube_dns_deployment_spec_selector
		.expect("couldn't get kube-dns deployment spec selector");

	let mut kube_dns_deployment_spec_selector_match_labels =
		kube_dns_deployment_spec_selector
		.match_labels.expect("couldn't get kube-dns deployment spec selector match labels");
	assert_eq!(kube_dns_deployment_spec_selector_match_labels.remove("k8s-app"), Some("kube-dns".to_string()));

	let kube_dns_deployment_spec_template = kube_dns_deployment_spec.template;

	let mut kube_dns_deployment_spec_template_metadata_labels =
		kube_dns_deployment_spec_template
		.metadata.expect("couldn't get kube-dns deployment spec template metadata")
		.labels.expect("couldn't get kube-dns deployment spec template metadata labels");
	assert_eq!(kube_dns_deployment_spec_template_metadata_labels.remove("k8s-app"), Some("kube-dns".to_string()));

	let kube_dns_container_liveness_probe_http_get_action =
		kube_dns_deployment_spec_template
		.spec.expect("couldn't get kube-dns deployment spec template spec")
		.containers
		.into_iter().find(|container| container.name == "kubedns").expect("couldn't get kube-dns container spec")
		.liveness_probe.expect("couldn't get kube-dns container spec liveness probe")
		.http_get.expect("couldn't get kube-dns container spec liveness probe HTTP get action");
	assert_eq!(kube_dns_container_liveness_probe_http_get_action.path, Some("/healthcheck/kubedns".to_string()));
	assert_eq!(kube_dns_container_liveness_probe_http_get_action.port, ::k8s_openapi::IntOrString::Int(10054));

	let kube_dns_deployment_status = kube_dns_deployment.status.expect("couldn't get kube-dns deployment status");
	assert_eq!(kube_dns_deployment_status.replicas, Some(1));
}
