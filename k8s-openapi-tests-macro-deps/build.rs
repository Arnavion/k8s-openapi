#![deny(rust_2018_idioms, warnings)]

fn main() {
	let k8s_openapi_version: u32 =
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

	if k8s_openapi_version >= 0x00_01_10_00 {
		println!(r#"cargo:rustc-cfg=k8s_apiextensions="v1""#);
	}
	else {
		println!(r#"cargo:rustc-cfg=k8s_apiextensions="v1beta1""#);
	}
}
