enum {type_name}<T> {{
    OkStatus({crate_root}::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(T),
    Accepted({crate_root}::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}}

impl<'de, T> {crate_root}::Response for {type_name}<T> where T: serde::Deserialize<'de> {{
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), {crate_root}::ResponseError> {{
        match status_code {{
            http::StatusCode::OK => {{
                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {{
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err({crate_root}::ResponseError::NeedMoreData),
                    Err(err) => return Err({crate_root}::ResponseError::Json(err)),
                }};
                let is_status = match result.get("kind") {{
                    Some(serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                }};
                if is_status {{
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err({crate_root}::ResponseError::Json)?;
                    Ok(({type_name}::OkStatus(result), buf.len()))
                }}
                else {{
                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));
                    let result = result.map_err({crate_root}::ResponseError::Json)?;
                    Ok(({type_name}::OkValue(result), buf.len()))
                }}
            }},
            http::StatusCode::ACCEPTED => {{
                let result = match serde_json::from_slice(buf) {{
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err({crate_root}::ResponseError::NeedMoreData),
                    Err(err) => return Err({crate_root}::ResponseError::Json(err)),
                }};
                Ok(({type_name}::Accepted(result), buf.len()))
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