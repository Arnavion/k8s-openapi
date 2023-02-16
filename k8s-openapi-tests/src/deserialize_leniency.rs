use k8s_openapi::serde_json;

#[test]
fn bytestring_null() {
	for (input, expected) in [
		(r#""azhzLW9wZW5hcGk=""#, &b"k8s-openapi"[..]),
		("null", &b""[..]),
	] {
		let actual: k8s_openapi::ByteString = serde_json::from_str(input).expect("couldn't deserialize ByteString");
		assert_eq!(actual.0, expected);
	}
}

#[test]
fn daemon_set() {
	for input in [
		r#"{"apiVersion":"apps/v1","kind":"DaemonSet","metadata":{},"spec":{"selector":{},"template":{"spec":{"containers":[]}}}}"#,
		r#"{"apiVersion":"apps/v1","kind":"DaemonSet","metadata":{},"spec":{"selector":{},"template":{"spec":{"containers":null}}}}"#,
		r#"{"apiVersion":"apps/v1","kind":"DaemonSet","metadata":{},"spec":{"selector":{},"template":{"spec":{}}}}"#,
	] {
		let daemon_set: k8s_openapi::api::apps::v1::DaemonSet = serde_json::from_str(input).expect("couldn't deserialize DaemonSet");
		let containers =
			daemon_set
			.spec.expect("couldn't get DaemonSetSpec")
			.template
			.spec.expect("couldn't get PodTemplateSpec")
			.containers;
		assert!(containers.is_empty());
	}
}

#[test]
fn event() {
	let input = r#"{
		"apiVersion": "events.k8s.io/v1",
		"deprecatedCount": 1,
		"deprecatedFirstTimestamp": "2023-02-16T18:25:07Z",
		"deprecatedLastTimestamp": "2023-02-16T18:25:07Z",
		"deprecatedSource": {
			"component": "default-scheduler"
		},
		"eventTime": null,
		"kind": "Event",
		"metadata": {
			"creationTimestamp": "2023-02-16T18:25:07Z",
			"name": "some-pod.174461445d6d3cff",
			"namespace": "test-530513",
			"resourceVersion": "320932",
			"uid": "f3ac47c9-cc86-4193-a2ed-2e4428b7ad77"
		},
		"note": "Successfully assigned test-530513/some-pod to test-pgd-control-plane",
		"reason": "Scheduled",
		"regarding": {
			"apiVersion": "v1",
			"kind": "Pod",
			"name": "some-pod",
			"namespace": "test-530513",
			"resourceVersion": "320930",
			"uid": "ace87ee8-15fb-46ae-9dae-c77422c2010f"
		},
		"type": "Normal"
	}"#;
	let actual: k8s_openapi::api::events::v1::Event = serde_json::from_str(input).expect("couldn't deserialize MicroTime");
	assert!(actual.event_time.is_none());
}
