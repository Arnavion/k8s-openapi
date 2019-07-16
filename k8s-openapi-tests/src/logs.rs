#[test]
fn get() {
	use k8s_openapi::api::core::v1 as api;

	crate::Client::with("logs-get", |client| {
		let (request, response_body) = api::Pod::list_namespaced_pod("kube-system", Default::default()).expect("couldn't list pods");
		let pod_list = {
			let response = client.execute(request).expect("couldn't list pods");
			crate::get_single_value(response, response_body, |response, status_code| match response {
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

		let (request, response_body) =
			api::Pod::read_namespaced_pod_log(addon_manager_pod_name, "kube-system", api::ReadNamespacedPodLogOptional {
				container: Some("kube-addon-manager"),
				..Default::default()
			})
			.expect("couldn't get addon-manager pod logs");
		let mut addon_manager_logs = String::new();
		let strings = {
			let response = client.execute(request).expect("couldn't get addon-manager pod logs");
			crate::get_multiple_values(response, response_body, |response, status_code| match response {
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

#[test]
fn partial_and_invalid_utf8_sequences() {
	use k8s_openapi::api::core::v1 as api;

	let mut response_body: k8s_openapi::ResponseBody<api::ReadNamespacedPodLogResponse> =
		k8s_openapi::ResponseBody::new(reqwest::StatusCode::OK);

	// Empty buffer
	match response_body.parse() {
		Err(k8s_openapi::ResponseError::NeedMoreData) => (),
		result => panic!("expected empty buffer to return Err(NeedMoreData), but it returned {:?}", result),
	}

	response_body.append_slice(b"a");

	// Entire buffer is valid
	match response_body.parse() {
		Ok(api::ReadNamespacedPodLogResponse::Ok(ref s)) if s == "a" => (),
		result => panic!(r#"expected empty buffer to return Ok("a"), but it returned {:?}"#, result),
	}

	// Entire buffer must have been consumed, and it should now be empty
	assert_eq!(&*response_body, b"");

	// First byte of buffer is invalid
	response_body.append_slice(b"\xff");

	match response_body.parse() {
		Err(k8s_openapi::ResponseError::Utf8(ref err)) if err.valid_up_to() == 0 && err.error_len() == Some(1) => (),
		result => panic!("expected empty buffer to return Err(NeedMoreData), but it returned {:?}", result),
	}

	// First byte of buffer must not have been consumed, so it's still invalid
	match response_body.parse() {
		Err(k8s_openapi::ResponseError::Utf8(ref err)) if err.valid_up_to() == 0 && err.error_len() == Some(1) => (),
		result => panic!("expected empty buffer to return Err(Utf8(0, Some(1))), but it returned {:?}", result),
	}

	let mut response_body: k8s_openapi::ResponseBody<api::ReadNamespacedPodLogResponse> =
		k8s_openapi::ResponseBody::new(reqwest::StatusCode::OK);

	response_body.append_slice(b"\xe4");

	// First byte of buffer is partial
	match response_body.parse() {
		Err(k8s_openapi::ResponseError::NeedMoreData) => (),
		result => panic!("expected empty buffer to return Err(NeedMoreData), but it returned {:?}", result),
	}

	response_body.append_slice(b"\xb8");

	// First two bytes of buffer are partial
	match response_body.parse() {
		Err(k8s_openapi::ResponseError::NeedMoreData) => (),
		result => panic!("expected empty buffer to return Err(NeedMoreData), but it returned {:?}", result),
	}

	// Entire buffer is valid
	response_body.append_slice(b"\x96");

	match response_body.parse() {
		Ok(api::ReadNamespacedPodLogResponse::Ok(ref s)) if s == "\u{4e16}" => (),
		result => panic!(r#"expected empty buffer to return Ok("\u{{4e16}}"), but it returned {:?}"#, result),
	}

	let mut response_body: k8s_openapi::ResponseBody<api::ReadNamespacedPodLogResponse> =
		k8s_openapi::ResponseBody::new(reqwest::StatusCode::OK);

	response_body.append_slice(b"\xe4\xb8\x96\xe7");

	// First three bytes are valid. Fourth byte is partial.
	match response_body.parse() {
		Ok(api::ReadNamespacedPodLogResponse::Ok(ref s)) if s == "\u{4e16}" => (),
		result => panic!(r#"expected empty buffer to return Ok("\u{{4e16}}"), but it returned {:?}"#, result),
	}

	// First three bytes must have been consumed. Remaining byte is partial.
	assert_eq!(&*response_body, b"\xe7");
	match response_body.parse() {
		Err(k8s_openapi::ResponseError::NeedMoreData) => (),
		result => panic!("expected empty buffer to return Err(NeedMoreData), but it returned {:?}", result),
	}

	response_body.append_slice(b"\x95\x8c");

	// Entire buffer is valid
	match response_body.parse() {
		Ok(api::ReadNamespacedPodLogResponse::Ok(ref s)) if s == "\u{754c}" => (),
		result => panic!(r#"expected empty buffer to return Ok("\u{{754c}}"), but it returned {:?}"#, result),
	}

	let mut response_body: k8s_openapi::ResponseBody<api::ReadNamespacedPodLogResponse> =
		k8s_openapi::ResponseBody::new(reqwest::StatusCode::OK);

	response_body.append_slice(b"\xe4\xb8\x96\xff");

	// First three bytes are valid. Fourth byte is invalid.
	match response_body.parse() {
		Ok(api::ReadNamespacedPodLogResponse::Ok(ref s)) if s == "\u{4e16}" => (),
		result => panic!(r#"expected empty buffer to return Ok("\u{{4e16}}"), but it returned {:?}"#, result),
	}

	// First three bytes must have been consumed. Remaining byte is invalid.
	assert_eq!(&*response_body, b"\xff");
	match response_body.parse() {
		Err(k8s_openapi::ResponseError::Utf8(ref err)) if err.valid_up_to() == 0 && err.error_len() == Some(1) => (),
		result => panic!("expected empty buffer to return Err(Utf8(0, Some(1))), but it returned {:?}", result),
	}
}
