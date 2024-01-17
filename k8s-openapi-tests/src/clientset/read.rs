use k8s_openapi::serde_json;

pub(crate) fn read_cluster<T>(
    name: &str,
) -> (http::Request<Vec<u8>>, fn(reqwest::StatusCode) -> super::ResponseBody<ReadResponse<T>>)
where
    T: serde::de::DeserializeOwned + k8s_openapi::Resource<Scope = k8s_openapi::ClusterResourceScope>,
{
    let first_segment = if T::GROUP.is_empty() { "api" } else { "apis" };
    let url = format!("/{first_segment}/{api_version}/{url_path_segment}/{name}",
        api_version = T::API_VERSION,
        url_path_segment = T::URL_PATH_SEGMENT,
        name = percent_encoding::percent_encode(name.as_bytes(), super::PATH_SEGMENT_ENCODE_SET),
    );

    let request = http::Request::get(url);
    let request = request.body(vec![]).unwrap();
    (request, super::ResponseBody::new)
}

pub(crate) fn read_namespaced<T>(
    namespace: &str,
    name: &str,
) -> (http::Request<Vec<u8>>, fn(reqwest::StatusCode) -> super::ResponseBody<ReadResponse<T>>)
where
    T: serde::de::DeserializeOwned + k8s_openapi::Resource<Scope = k8s_openapi::NamespaceResourceScope>,
{
    let first_segment = if T::GROUP.is_empty() { "api" } else { "apis" };
    let url = format!("/{first_segment}/{api_version}/namespaces/{namespace}/{url_path_segment}/{name}",
        api_version = T::API_VERSION,
        namespace = percent_encoding::percent_encode(namespace.as_bytes(), super::PATH_SEGMENT_ENCODE_SET),
        url_path_segment = T::URL_PATH_SEGMENT,
        name = percent_encoding::percent_encode(name.as_bytes(), super::PATH_SEGMENT_ENCODE_SET),
    );

    let request = http::Request::get(url);
    let request = request.body(vec![]).unwrap();
    (request, super::ResponseBody::new)
}

#[derive(Debug)]
pub(crate) enum ReadResponse<T> where T: serde::de::DeserializeOwned {
    Ok(T),
    Other(#[allow(dead_code)] Result<Option<serde_json::Value>, serde_json::Error>),
}

impl<T> super::Response for ReadResponse<T> where T: serde::de::DeserializeOwned {
    fn try_from_parts(status_code: reqwest::StatusCode, buf: &[u8]) -> Result<(Self, usize), super::ResponseError> {
        #[allow(clippy::single_match_else)]
        match status_code {
            reqwest::StatusCode::OK => {
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
