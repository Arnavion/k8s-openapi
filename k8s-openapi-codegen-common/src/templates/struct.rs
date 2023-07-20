pub(crate) fn generate(
    mut writer: impl std::io::Write,
    vis: &str,
    type_name: &str,
    generics: super::Generics<'_>,
    fields: &[super::Property<'_>],
) -> Result<(), crate::Error> {
    use std::fmt::Write;

    let type_generics_type = generics.type_part.map(|part| format!("<{part}>")).unwrap_or_default();
    let type_generics_where = generics.where_part.map(|part| format!(" where {part}")).unwrap_or_default();

    let mut fields_string = String::new();
    for super::Property { comment, field_name, field_type_name, .. } in fields {
        if !fields_string.is_empty() {
            writeln!(fields_string)?;
        }

        if let Some(comment) = comment {
            for line in crate::get_comment_text(comment, "") {
                writeln!(fields_string, "    ///{line}")?;
            }
        }

        writeln!(
            fields_string,
            "    {vis}{field_name}: {field_type_name},",
        )?;
    }

    writeln!(
        writer,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/struct.rs")),
        type_name = type_name,
        type_generics_type = type_generics_type,
        type_generics_where = type_generics_where,
        fields = fields_string,
    )?;

    Ok(())
}
