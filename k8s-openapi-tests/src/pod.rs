#[tokio::test]
async fn list() {
    use k8s_openapi::api::core::v1 as api;

    let mut client = crate::Client::new("pod-list");

    let (request, response_body) = crate::clientset::list_namespaced::<api::Pod>("kube-system");
    let pod_list = match client.get_single_value(request, response_body).await {
        (crate::clientset::ListResponse::Ok(pod_list), _) => pod_list,
        (other, status_code) => panic!("{other:?} {status_code}"),
    };

    assert_eq!(k8s_openapi::kind(&pod_list), "PodList");

    let apiserver_pod =
        pod_list
        .items.into_iter()
        .find_map(|pod| {
            let name = pod.metadata.name.as_deref()?;
            if name.starts_with("kube-apiserver-") {
                Some(pod)
            }
            else {
                None
            }
        })
        .expect("couldn't find apiserver pod");

    let apiserver_container_spec =
        apiserver_pod
        .spec.expect("couldn't get apiserver pod spec")
        .containers
        .into_iter()
        .next().expect("couldn't get apiserver container spec");
    assert_eq!(apiserver_container_spec.name, "kube-apiserver");

    let apiserver_pod_status = apiserver_pod.status.expect("couldn't get apiserver pod status");
    assert_eq!(apiserver_pod_status.phase, Some("Running".to_string()));
}
