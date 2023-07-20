pub(crate) fn generate(
    mut writer: impl std::io::Write,
    vis: &str,
    type_name: &str,
    inner_type_name: &str,
    datetime_serialization_format: super::DateTimeSerializationFormat,
    map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
    let local = crate::map_namespace_local_to_string(map_namespace)?;

    let inner_value = match datetime_serialization_format {
        super::DateTimeSerializationFormat::Default => "&self.0",
        super::DateTimeSerializationFormat::SixDecimalDigits => "&self.0.to_rfc3339_opts(chrono::SecondsFormat::Micros, true)",
        super::DateTimeSerializationFormat::ZeroDecimalDigits => "&self.0.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)",
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
