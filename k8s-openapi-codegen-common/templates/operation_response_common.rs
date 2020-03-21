enum {type_name}{type_generics_type}{type_generics_where} {{
{variants}    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}}

#[cfg(feature = "api")]
impl{type_generics_impl} {crate_root}::Response for {type_name}{type_generics_type}{type_generics_where} {{
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), {crate_root}::ResponseError> {{
        match status_code {{
{variant_match_arms}            _ => {{
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