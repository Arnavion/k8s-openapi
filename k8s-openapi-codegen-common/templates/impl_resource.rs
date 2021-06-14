
impl{type_generics_impl} {local}Resource for {type_name}{type_generics_type}{type_generics_where} {{
    const API_VERSION: &'static str = {api_version};
    const GROUP: &'static str = {group};
    const KIND: &'static str = {kind};
    const VERSION: &'static str = {version};
    const URL_PATH_SEGMENT: &'static str = {url_path_segment};
    type Scope = {scope};
}}