// Generated from definition io.k8s.ReplaceResponse

/// The common response type for all replace API operations.
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReplaceResponse<T> where T: crate::serde::de::DeserializeOwned {
    Ok(T),
    Created(T),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl<T> crate::Response for ReplaceResponse<T> where T: crate::serde::de::DeserializeOwned {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReplaceResponse::Created(result), buf.len()))
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
                Ok((ReplaceResponse::Other(result), read))
            },
        }
    }
}
