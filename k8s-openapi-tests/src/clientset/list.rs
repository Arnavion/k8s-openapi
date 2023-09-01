use k8s_openapi::serde_json;

pub(crate) fn list_namespaced<T>(
    namespace: &str,
) -> (http::Request<Vec<u8>>, fn(http::StatusCode) -> super::ResponseBody<ListResponse<T>>)
where
    T: serde::de::DeserializeOwned + k8s_openapi::Resource<Scope = k8s_openapi::NamespaceResourceScope> + k8s_openapi::ListableResource,
{
    let first_segment = if T::GROUP.is_empty() { "api" } else { "apis" };
    let url = format!("/{first_segment}/{api_version}/namespaces/{namespace}/{url_path_segment}",
        api_version = T::API_VERSION,
        namespace = percent_encoding::percent_encode(namespace.as_bytes(), super::PATH_SEGMENT_ENCODE_SET),
        url_path_segment = T::URL_PATH_SEGMENT,
    );

    let request = http::Request::get(url);
    let request = request.body(vec![]).unwrap();
    (request, super::ResponseBody::new)
}

#[derive(Debug)]
pub(crate) enum ListResponse<T> where T: serde::de::DeserializeOwned + k8s_openapi::ListableResource {
    Ok(k8s_openapi::List<T>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

impl<T> super::Response for ListResponse<T> where T: serde::de::DeserializeOwned + k8s_openapi::ListableResource {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), super::ResponseError> {
        #[allow(clippy::single_match_else)]
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(super::ResponseError::NeedMoreData),
                    Err(err) => return Err(super::ResponseError::Json(err)),
                };
                Ok((Self::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(super::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((Self::Other(result), read))
            },
        }
    }
}
