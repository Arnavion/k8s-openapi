use k8s_openapi::serde_json;

pub(crate) fn watch_namespaced<T>(
    namespace: &str,
    optional: WatchOptional<'_>,
) -> (http::Request<Vec<u8>>, fn(reqwest::StatusCode) -> super::ResponseBody<WatchResponse<T>>)
where
    T: serde::de::DeserializeOwned + k8s_openapi::Resource<Scope = k8s_openapi::NamespaceResourceScope>,
{
    let first_segment = if T::GROUP.is_empty() { "api" } else { "apis" };
    let url = format!("/{first_segment}/{api_version}/namespaces/{namespace}/{url_path_segment}?",
        api_version = T::API_VERSION,
        namespace = percent_encoding::percent_encode(namespace.as_bytes(), super::PATH_SEGMENT_ENCODE_SET),
        url_path_segment = T::URL_PATH_SEGMENT,
    );
    let mut query_pairs = url::form_urlencoded::Serializer::new(url);
    optional.serialize(&mut query_pairs);
    let url = query_pairs.finish();

    let request = http::Request::get(url);
    let request = request.body(vec![]).unwrap();
    (request, super::ResponseBody::new)
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct WatchOptional<'a> {
    pub(crate) allow_watch_bookmarks: Option<bool>,
    pub(crate) resource_version: Option<&'a str>,
    pub(crate) resource_version_match: Option<&'a str>,
    pub(crate) send_initial_events: Option<bool>,
}

impl<'a> WatchOptional<'a> {
    fn serialize<T>(
        self,
        query_pairs: &mut url::form_urlencoded::Serializer<'_, T>,
    ) where T: url::form_urlencoded::Target {
        if let Some(value) = self.allow_watch_bookmarks {
            query_pairs.append_pair("allowWatchBookmarks", if value { "true" } else { "false" });
        }
        if let Some(value) = self.resource_version {
            query_pairs.append_pair("resourceVersion", value);
        }
        if let Some(value) = self.resource_version_match {
            query_pairs.append_pair("resourceVersionMatch", value);
        }
        if let Some(value) = self.send_initial_events {
            query_pairs.append_pair("sendInitialEvents", if value { "true" } else { "false" });
        }
        query_pairs.append_pair("watch", "true");
    }
}

#[derive(Debug)]
pub(crate) enum WatchResponse<T> where T: serde::de::DeserializeOwned {
    Ok(k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent<T>),
    Other(#[allow(dead_code)] Result<Option<serde_json::Value>, serde_json::Error>),
}

impl<T> super::Response for WatchResponse<T> where T: serde::de::DeserializeOwned {
    fn try_from_parts(status_code: reqwest::StatusCode, buf: &[u8]) -> Result<(Self, usize), super::ResponseError> {
        #[allow(clippy::single_match_else)]
        match status_code {
            reqwest::StatusCode::OK => {
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(err)) if err.is_eof() => return Err(super::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(super::ResponseError::Json(err)),
                    None => return Err(super::ResponseError::NeedMoreData),
                };
                Ok((Self::Ok(result), byte_offset))
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
