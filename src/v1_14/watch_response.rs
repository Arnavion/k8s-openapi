// Generated from definition io.k8s.WatchResponse

/// The common response type for all watch API operations.
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum WatchResponse<T> where T: crate::serde::de::DeserializeOwned {
    Ok(crate::apimachinery::pkg::apis::meta::v1::WatchEvent<T>),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl<T> crate::Response for WatchResponse<T> where T: crate::serde::de::DeserializeOwned {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let mut deserializer = crate::serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(err)) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(crate::ResponseError::Json(err)),
                    None => return Err(crate::ResponseError::NeedMoreData),
                };
                Ok((WatchResponse::Ok(result), byte_offset))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((WatchResponse::Other(result), read))
            },
        }
    }
}
