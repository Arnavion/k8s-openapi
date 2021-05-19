#![deny(rust_2018_idioms, warnings)]

fn main() {
	// Assert that the DEP_K8S_OPENAPI_*_VERSION is set by the k8s-openapi crate's build script correctly.

	const MIN: usize = 11;
	const MAX: usize = 21;

	let enabled_version = {
		let mut enabled_versions = (MIN..=MAX).filter(|v| std::env::var(format!("CARGO_FEATURE_TEST_V1_{}", v)).is_ok());
		let v1 = enabled_versions.next().expect("None of the test_v1_* features are enabled on the k8s-openapi-tests crate.");
		if let Some(v2) = enabled_versions.next() {
			panic!("Both test_v1_{} and test_v1_{} features are enabled on the k8s-openapi-tests crate.", v1, v2);
		}
		v1
	};
	let expected_k8s_openapi_version = 0x00_01_00_00_u32 | ((enabled_version as u32) << 8);

	let actual_k8s_openapi_version: u32 =
		std::env::vars_os()
		.find_map(|(key, value)| {
			let key = key.into_string().ok()?;
			if key.starts_with("DEP_K8S_OPENAPI_") && key.ends_with("_VERSION") {
				let value = value.into_string().ok()?;
				Some(value)
			}
			else {
				None
			}
		}).expect("DEP_K8S_OPENAPI_*_VERSION must have been set by k8s-openapi")
		.parse().expect("DEP_K8S_OPENAPI_*_VERSION is malformed");

	assert_eq!(actual_k8s_openapi_version, expected_k8s_openapi_version);

	if actual_k8s_openapi_version >= 0x00_01_10_00 {
		println!(r#"cargo:rustc-cfg=k8s_apiextensions="v1""#);
	}
	else {
		println!(r#"cargo:rustc-cfg=k8s_apiextensions="v1beta1""#);
	}
}
