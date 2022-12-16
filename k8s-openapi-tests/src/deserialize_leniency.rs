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
