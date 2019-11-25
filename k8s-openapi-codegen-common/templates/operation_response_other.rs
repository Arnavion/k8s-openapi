enum {type_name}<T>{type_generics_where} {{
    Ok({ok_type_name}),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}}

impl<T> {crate_root}::Response for {type_name}<T>{response_type_generics_where} {{
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), {crate_root}::ResponseError> {{
        match status_code {{
            http::StatusCode::OK => {{
                let result = match serde_json::from_slice(buf) {{
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err({crate_root}::ResponseError::NeedMoreData),
                    Err(err) => return Err({crate_root}::ResponseError::Json(err)),
                }};
                Ok(({type_name}::Ok(result), buf.len()))
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