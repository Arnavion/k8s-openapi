// Generated from definition io.k8s.DeleteResponse

/// The common response type for all delete API operations and delete-collection API operations.
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum DeleteResponse<T> where T: crate::serde::de::DeserializeOwned {
    OkStatus(crate::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(T),
    Accepted(T),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl<T> crate::Response for DeleteResponse<T> where T: crate::serde::de::DeserializeOwned {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result: crate::serde_json::Map<String, crate::serde_json::Value> = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                let is_status = matches!(result.get("kind"), Some(crate::serde_json::Value::String(s)) if s == "Status");
                if is_status {
                    let result = crate::serde::Deserialize::deserialize(crate::serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = crate::serde::Deserialize::deserialize(crate::serde_json::Value::Object(result));
                    let result = result.map_err(crate::ResponseError::Json)?;
                    Ok((DeleteResponse::OkValue(result), buf.len()))
                }
            },
            http::StatusCode::ACCEPTED => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((DeleteResponse::Accepted(result), buf.len()))
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
                Ok((DeleteResponse::Other(result), read))
            },
        }
    }
}
