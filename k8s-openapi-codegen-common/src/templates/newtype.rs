pub(crate) fn generate(
    mut writer: impl std::io::Write,
    vis: &str,
    type_name: &str,
    inner_type_name: &str,
    datetime_serialization_format: super::DateTimeSerializationFormat,
    map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
    let local = crate::map_namespace_local_to_string(map_namespace)?;

    let jiff_fmt = |fmt: &str| {
        format!(
            r#"&{local}jiff::fmt::strtime::format("{fmt}", self.0).map_err({local}serde::ser::Error::custom)?"#
        )
    };
    let inner_value = match datetime_serialization_format {
        super::DateTimeSerializationFormat::Default => "&self.0".to_string(),
        super::DateTimeSerializationFormat::SixDecimalDigits => jiff_fmt("%Y-%m-%dT%H:%M:%S%.6fZ"),
        super::DateTimeSerializationFormat::ZeroDecimalDigits => jiff_fmt("%Y-%m-%dT%H:%M:%SZ"),
    };

    writeln!(
        writer,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/newtype.rs")),
        local = local,
        type_name = type_name,
        vis = vis,
        inner_type_name = inner_type_name,
        inner_value = inner_value,
    )?;

    Ok(())
}
