#[test]
fn list() {
	use k8s_openapi::api::apps::v1 as apps;
	use k8s_openapi::apimachinery::pkg::util as util;

	let mut client = crate::Client::new("deployment-list");

	let (request, response_body) =
		apps::Deployment::list_namespaced_deployment("kube-system", Default::default())
		.expect("couldn't list deployments");
	let response = client.execute(request);
	let deployment_list =
		crate::get_single_value(response, response_body, |response, status_code| match response {
			k8s_openapi::ListResponse::Ok(deployment_list) =>
				crate::ValueResult::GotValue(deployment_list),
			other => panic!("{:?} {}", other, status_code),
		});

	assert_eq!(k8s_openapi::kind(&deployment_list), "DeploymentList");

	let dns_deployment =
		deployment_list
		.items.into_iter()
		.find(|deployment| deployment.metadata.name.as_deref() == Some("coredns"))
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

	let dns_container_liveness_probe_http_get_action =
		dns_deployment_spec_template
		.spec.expect("couldn't get dns deployment spec template spec")
		.containers
		.into_iter().find(|container| container.name == "coredns").expect("couldn't get dns container spec")
		.liveness_probe.expect("couldn't get dns container spec liveness probe")
		.http_get.expect("couldn't get dns container spec liveness probe HTTP get action");

	assert_eq!(dns_container_liveness_probe_http_get_action.path, Some("/health".to_string()));
	assert_eq!(dns_container_liveness_probe_http_get_action.port, util::intstr::IntOrString::Int(8080));

	let dns_deployment_status = dns_deployment.status.expect("couldn't get dns deployment status");
	assert!(dns_deployment_status.replicas > Some(0));
}
