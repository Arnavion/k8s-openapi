pub(crate) fn delete_collection_namespaced<T>(
    namespace: &str,
    list_optional: ListOptional<'_>,
) -> (http::Request<Vec<u8>>, fn(http::StatusCode) -> super::ResponseBody<super::DeleteResponse<k8s_openapi::List<T>>>)
where
    T: serde::de::DeserializeOwned + k8s_openapi::Resource<Scope = k8s_openapi::NamespaceResourceScope> + k8s_openapi::ListableResource,
{
    let first_segment = if T::GROUP.is_empty() { "api" } else { "apis" };
    let url = format!("/{first_segment}/{api_version}/namespaces/{namespace}/{url_path_segment}?",
        api_version = T::API_VERSION,
        namespace = percent_encoding::percent_encode(namespace.as_bytes(), super::PATH_SEGMENT_ENCODE_SET),
        url_path_segment = T::URL_PATH_SEGMENT,
    );
    let mut query_pairs = url::form_urlencoded::Serializer::new(url);
    list_optional.serialize(&mut query_pairs);
    let url = query_pairs.finish();

    let request = http::Request::delete(url);
    let request = request.body(vec![]).unwrap();
    (request, super::ResponseBody::new)
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ListOptional<'a> {
    pub(crate) label_selector: Option<&'a str>,
}

impl<'a> ListOptional<'a> {
    fn serialize<T>(
        self,
        query_pairs: &mut url::form_urlencoded::Serializer<'_, T>,
    ) where T: url::form_urlencoded::Target {
        if let Some(value) = self.label_selector {
            query_pairs.append_pair("labelSelector", value);
        }
    }
}
