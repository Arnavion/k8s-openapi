enum {type_name}<T> {{
    Ok({crate_root}::apimachinery::pkg::apis::meta::v1::WatchEvent<T>),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}}

impl<T> {crate_root}::Response for {type_name}<T> where T: serde::de::DeserializeOwned {{
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), {crate_root}::ResponseError> {{
        match status_code {{
            http::StatusCode::OK => {{
                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {{
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err({crate_root}::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err({crate_root}::ResponseError::Json(err)),
                    None => return Err({crate_root}::ResponseError::NeedMoreData),
                }};
                Ok(({type_name}::Ok(result), byte_offset))
            }},
            _ => {{
                let (result, read) =
                    if buf.is_empty() {{
                        (Ok(None), 0)
                    }}
                    else {{
                        match serde_json::from_slice(buf) {{
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err({crate_root}::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }}
                    }};
                Ok(({type_name}::Other(result), read))
            }},
        }}
    }}
}}