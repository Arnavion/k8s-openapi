use k8s_openapi::serde_json;

#[test]
fn time() {
	use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

	for (input, expected_time, expected_micro_time) in [
		(
			r#""2020-03-05T12:34:56.789012345Z""#,
			r#""2020-03-05T12:34:56Z""#,
			r#""2020-03-05T12:34:56.789012Z""#,
		),
		(
			r#""2020-03-05T12:34:56.789012Z""#,
			r#""2020-03-05T12:34:56Z""#,
			r#""2020-03-05T12:34:56.789012Z""#,
		),
		(
			r#""2020-03-05T12:34:56.789000Z""#,
			r#""2020-03-05T12:34:56Z""#,
			r#""2020-03-05T12:34:56.789000Z""#,
		),
		(
			r#""2020-03-05T12:34:56.789Z""#,
			r#""2020-03-05T12:34:56Z""#,
			r#""2020-03-05T12:34:56.789000Z""#,
		),
	] {
		let time: meta::Time = serde_json::from_str(input).unwrap();
		let time_serialized = serde_json::to_string(&time).unwrap();
		assert_eq!(time_serialized, expected_time);

		let micro_time: meta::MicroTime = serde_json::from_str(input).unwrap();
		let micro_time_serialized = serde_json::to_string(&micro_time).unwrap();
		assert_eq!(micro_time_serialized, expected_micro_time);
	}
}
