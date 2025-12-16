pub(crate) fn generate(
    mut writer: impl std::io::Write,
    vis: &str,
    type_name: &str,
    inner_type_name: &str,
    datetime_serialization_format: super::DateTimeSerializationFormat,
    map_namespace: &impl crate::MapNamespace,
) -> Result<(), crate::Error> {
    let local = crate::map_namespace_local_to_string(map_namespace)?;

    // TODO: This way of serializing `Time` and `MicroTime` ends up creating a `String` and then serializing that. We could do better with `serializer.collect_str(&self.0.strftime("..."))`,
    // but that requires either:
    //
    // - Giving up on the `{deserialize,serialize}_newtype_struct` wrappers. This isn't too big a deal since we expect to be used with only the JSON and YAML serde impls,
    //   for which `*_newtype_struct` just forward to the inner type. It would be nice to not rely on that behavior of specific serde impls though.
    //
    // - Making a wrapper `struct SerializeWithCollectStr<D>; impl<D: Display> Serialize for SerializeWithCollectStr { fn serialize() { serializer.collect_str(&self.0); } }`
    //   and then using it as `serializer.serialize_newtype_struct("Time", &SerializeWithCollectStr(self.0.strftime("...")))`.
    //   This will either require generating that private type within each newtype's `Serialize` impl, or making it a k8s-openapi-level type and referencing that.
    let jiff_fmt = |fmt: &str| format!(r#"&{local}jiff::fmt::strtime::format({fmt:?}, self.0).map_err({local}serde::ser::Error::custom)?"#);
    let inner_value: std::borrow::Cow<'static, str> = match datetime_serialization_format {
        super::DateTimeSerializationFormat::Default => "&self.0".into(),
        super::DateTimeSerializationFormat::SixDecimalDigits => jiff_fmt("%Y-%m-%dT%H:%M:%S%.6fZ").into(),
        super::DateTimeSerializationFormat::ZeroDecimalDigits => jiff_fmt("%Y-%m-%dT%H:%M:%SZ").into(),
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
