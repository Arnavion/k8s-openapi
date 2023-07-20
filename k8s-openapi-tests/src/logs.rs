use futures_util::StreamExt;

#[tokio::test]
async fn get() {
    use k8s_openapi::api::core::v1 as api;

    let mut client = crate::Client::new("logs-get");

    let (request, response_body) = api::Pod::list("kube-system", Default::default()).expect("couldn't list pods");
    let pod_list = match client.get_single_value(request, response_body).await {
        (k8s_openapi::ListResponse::Ok(pod_list), _) => pod_list,
        (other, status_code) => panic!("{other:?} {status_code}"),
    };

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

    let apiserver_pod_name =
        apiserver_pod
        .metadata
        .name.as_ref().expect("couldn't get apiserver pod name");

    let (request, response_body) =
        api::Pod::read_log(apiserver_pod_name, "kube-system", api::ReadPodLogOptional {
            container: Some("kube-apiserver"),
            ..Default::default()
        })
        .expect("couldn't get apiserver pod logs");
    let mut apiserver_logs = String::new();
    let chunks = client.get_multiple_values(request, response_body);
    futures_util::pin_mut!(chunks);
    let mut found_line = false;
    while let Some(chunk) = chunks.next().await {
        let s = match chunk {
            (api::ReadPodLogResponse::Ok(s), _) => s,
            (other, status_code) => panic!("{other:?} {status_code}"),
        };
        apiserver_logs.push_str(&s);

        if apiserver_logs.contains("Serving securely on [::]:6443") {
            found_line = true;
            break;
        }

        if apiserver_logs.len() > 65536 {
            break;
        }
    }
    assert!(found_line, "did not find expected text in apiserver pod logs: {apiserver_logs}");
}

#[test]
fn partial_and_invalid_utf8_sequences() {
    use k8s_openapi::api::core::v1 as api;

    let mut response_body: k8s_openapi::ResponseBody<api::ReadPodLogResponse> =
        k8s_openapi::ResponseBody::new(reqwest::StatusCode::OK);

    // Empty buffer
    match response_body.parse() {
        Err(k8s_openapi::ResponseError::NeedMoreData) => (),
        result => panic!("expected empty buffer to return Err(NeedMoreData), but it returned {result:?}"),
    }

    response_body.append_slice(b"a");

    // Entire buffer is valid
    match response_body.parse() {
        Ok(api::ReadPodLogResponse::Ok(s)) if s == "a" => (),
        result => panic!(r#"expected empty buffer to return Ok("a"), but it returned {result:?}"#),
    }

    // Entire buffer must have been consumed, and it should now be empty
    assert_eq!(&*response_body, b"");

    // First byte of buffer is invalid
    response_body.append_slice(b"\xff");

    match response_body.parse() {
        Err(k8s_openapi::ResponseError::Utf8(err)) if err.valid_up_to() == 0 && err.error_len() == Some(1) => (),
        result => panic!("expected empty buffer to return Err(NeedMoreData), but it returned {result:?}"),
    }

    // First byte of buffer must not have been consumed, so it's still invalid
    match response_body.parse() {
        Err(k8s_openapi::ResponseError::Utf8(err)) if err.valid_up_to() == 0 && err.error_len() == Some(1) => (),
        result => panic!("expected empty buffer to return Err(Utf8(0, Some(1))), but it returned {result:?}"),
    }

    let mut response_body: k8s_openapi::ResponseBody<api::ReadPodLogResponse> =
        k8s_openapi::ResponseBody::new(reqwest::StatusCode::OK);

    response_body.append_slice(b"\xe4");

    // First byte of buffer is partial
    match response_body.parse() {
        Err(k8s_openapi::ResponseError::NeedMoreData) => (),
        result => panic!("expected empty buffer to return Err(NeedMoreData), but it returned {result:?}"),
    }

    response_body.append_slice(b"\xb8");

    // First two bytes of buffer are partial
    match response_body.parse() {
        Err(k8s_openapi::ResponseError::NeedMoreData) => (),
        result => panic!("expected empty buffer to return Err(NeedMoreData), but it returned {result:?}"),
    }

    // Entire buffer is valid
    response_body.append_slice(b"\x96");

    match response_body.parse() {
        Ok(api::ReadPodLogResponse::Ok(s)) if s == "\u{4e16}" => (),
        result => panic!(r#"expected empty buffer to return Ok("\u{{4e16}}"), but it returned {result:?}"#),
    }

    let mut response_body: k8s_openapi::ResponseBody<api::ReadPodLogResponse> =
        k8s_openapi::ResponseBody::new(reqwest::StatusCode::OK);

    response_body.append_slice(b"\xe4\xb8\x96\xe7");

    // First three bytes are valid. Fourth byte is partial.
    match response_body.parse() {
        Ok(api::ReadPodLogResponse::Ok(s)) if s == "\u{4e16}" => (),
        result => panic!(r#"expected empty buffer to return Ok("\u{{4e16}}"), but it returned {result:?}"#),
    }

    // First three bytes must have been consumed. Remaining byte is partial.
    assert_eq!(&*response_body, b"\xe7");
    match response_body.parse() {
        Err(k8s_openapi::ResponseError::NeedMoreData) => (),
        result => panic!("expected empty buffer to return Err(NeedMoreData), but it returned {result:?}"),
    }

    response_body.append_slice(b"\x95\x8c");

    // Entire buffer is valid
    match response_body.parse() {
        Ok(api::ReadPodLogResponse::Ok(s)) if s == "\u{754c}" => (),
        result => panic!(r#"expected empty buffer to return Ok("\u{{754c}}"), but it returned {result:?}"#),
    }

    let mut response_body: k8s_openapi::ResponseBody<api::ReadPodLogResponse> =
        k8s_openapi::ResponseBody::new(reqwest::StatusCode::OK);

    response_body.append_slice(b"\xe4\xb8\x96\xff");

    // First three bytes are valid. Fourth byte is invalid.
    match response_body.parse() {
        Ok(api::ReadPodLogResponse::Ok(s)) if s == "\u{4e16}" => (),
        result => panic!(r#"expected empty buffer to return Ok("\u{{4e16}}"), but it returned {result:?}"#),
    }

    // First three bytes must have been consumed. Remaining byte is invalid.
    assert_eq!(&*response_body, b"\xff");
    match response_body.parse() {
        Err(k8s_openapi::ResponseError::Utf8(err)) if err.valid_up_to() == 0 && err.error_len() == Some(1) => (),
        result => panic!("expected empty buffer to return Err(Utf8(0, Some(1))), but it returned {result:?}"),
    }
}
