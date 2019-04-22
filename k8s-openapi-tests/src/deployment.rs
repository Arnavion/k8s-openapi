#[test]
fn list() {
	k8s_if_1_8! {
		use k8s_openapi::api::apps::v1beta2 as apps;
	}
	k8s_if_ge_1_9! {
		use k8s_openapi::api::apps::v1 as apps;
	}
	use k8s_openapi::apimachinery::pkg::util as util;

	crate::Client::with("deployment-list", |client| {
		let (request, response_body) =
			apps::Deployment::list_namespaced_deployment("kube-system", Default::default())
			.expect("couldn't list deployments");
		let response = client.execute(request).expect("couldn't list deployments");;
		let deployment_list =
			crate::get_single_value(response, response_body, |response, status_code| match response {
				apps::ListNamespacedDeploymentResponse::Ok(deployment_list) =>
					Ok(crate::ValueResult::GotValue(deployment_list)),
				other => Err(format!("{:?} {}", other, status_code).into()),
			}).expect("couldn't list deployments");

		assert_eq!(k8s_openapi::kind(&deployment_list), "DeploymentList");

		k8s_if_le_1_10! {
			let dns_deployment_name = "kube-dns";
		}
		k8s_if_ge_1_11! {
			let dns_deployment_name = "coredns";
		}

		let dns_deployment =
			deployment_list
			.items.into_iter()
			.find(|deployment|
				deployment.metadata.as_ref().and_then(|metadata|
					metadata.name.as_ref()).map(AsRef::as_ref) == Some(dns_deployment_name))
			.expect("couldn't find dns deployment");

		let dns_deployment_spec =
			dns_deployment
			.spec.expect("couldn't get dns deployment spec");

		let mut dns_deployment_spec_selector_match_labels =
			dns_deployment_spec
			.selector
			.match_labels.expect("couldn't get dns deployment spec selector match labels");
		assert_eq!(dns_deployment_spec_selector_match_labels.remove("k8s-app"), Some("kube-dns".to_string()));

		let dns_deployment_spec_template = dns_deployment_spec.template;

		let mut dns_deployment_spec_template_metadata_labels =
			dns_deployment_spec_template
			.metadata.expect("couldn't get dns deployment spec template metadata")
			.labels.expect("couldn't get dns deployment spec template metadata labels");
		assert_eq!(dns_deployment_spec_template_metadata_labels.remove("k8s-app"), Some("kube-dns".to_string()));

		k8s_if_le_1_10! {
			let dns_container_name = "kubedns";
		}
		k8s_if_ge_1_11! {
			let dns_container_name = "coredns";
		}

		let dns_container_liveness_probe_http_get_action =
			dns_deployment_spec_template
			.spec.expect("couldn't get dns deployment spec template spec")
			.containers
			.into_iter().find(|container| container.name == dns_container_name).expect("couldn't get dns container spec")
			.liveness_probe.expect("couldn't get dns container spec liveness probe")
			.http_get.expect("couldn't get dns container spec liveness probe HTTP get action");

		k8s_if_le_1_10! {
			assert_eq!(dns_container_liveness_probe_http_get_action.path, Some("/healthcheck/kubedns".to_string()));
			assert_eq!(dns_container_liveness_probe_http_get_action.port, util::intstr::IntOrString::Int(10054));
		}
		k8s_if_ge_1_11! {
			assert_eq!(dns_container_liveness_probe_http_get_action.path, Some("/health".to_string()));
			assert_eq!(dns_container_liveness_probe_http_get_action.port, util::intstr::IntOrString::Int(8080));
		}

		let dns_deployment_status = dns_deployment.status.expect("couldn't get dns deployment status");
		assert!(dns_deployment_status.replicas > Some(0));
	});
}
