enum {type_name}{type_generics_type}{type_generics_where} {{
{variants}    Other(Result<Option<{local}serde_json::Value>, {local}serde_json::Error>),
}}

{operation_feature_attribute}impl{type_generics_impl} {local}Response for {type_name}{type_generics_type}{type_generics_where} {{
    fn try_from_parts(status_code: {local}http::StatusCode, buf: &[u8]) -> Result<(Self, usize), {local}ResponseError> {{
        match status_code {{
{variant_match_arms}            _ => {{
                let (result, read) =
                    if buf.is_empty() {{
                        (Ok(None), 0)
                    }}
                    else {{
                        match {local}serde_json::from_slice(buf) {{
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err({local}ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }}
                    }};
                Ok(({type_name}::Other(result), read))
            }},
        }}
    }}
}}