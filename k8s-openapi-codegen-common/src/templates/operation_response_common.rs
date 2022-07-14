pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	map_namespace: &impl crate::MapNamespace,
	operation_action: OperationAction,
	operation_feature: Option<&str>,
) -> Result<(), crate::Error> {
	let local = crate::map_namespace_local_to_string(map_namespace)?;
	let metav1 = {
		let namespace_parts =
			map_namespace.map_namespace(&["io", "k8s", "apimachinery", "pkg", "apis", "meta", "v1"])
			.ok_or(r#"unexpected path "io.k8s.apimachinery.pkg.apis.meta.v1""#)?;
		let mut result = String::new();
		for namespace_part in namespace_parts {
			result.push_str(&crate::get_rust_ident(namespace_part));
			result.push_str("::");
		}
		result
	};

	let type_generics_impl = "<T>";
	let type_generics_type = "<T>";
	let type_generics_where = match operation_action {
		OperationAction::Create |
		OperationAction::Delete |
		OperationAction::Replace |
		OperationAction::Patch |
		OperationAction::Watch => format!(" where T: {local}serde::de::DeserializeOwned"),
		OperationAction::List => format!(" where T: {local}serde::de::DeserializeOwned + {local}ListableResource"),
	};

	let operation_feature_attribute: std::borrow::Cow<'static, str> =
		operation_feature.map_or("".into(), |operation_feature| format!("#[cfg(feature = {operation_feature:?})]\n").into());

	let mut variants = String::new();
	let mut variant_match_arms = String::new();
	match operation_action {
		OperationAction::Create => {
			use std::fmt::Write;

			writeln!(variants, "    Ok(T),")?;
			writeln!(variants, "    Created(T),")?;
			writeln!(variants, "    Accepted(T),")?;

			variant_match_arms.push_str(&deserialize_single(&local, "OK", type_name, "Ok")?);
			variant_match_arms.push_str(&deserialize_single(&local, "CREATED", type_name, "Created")?);
			variant_match_arms.push_str(&deserialize_single(&local, "ACCEPTED", type_name, "Accepted")?);
		},

		OperationAction::Delete => {
			use std::fmt::Write;

			// Delete and delete-collection operations that return metav1.Status for HTTP 200 can also return the object itself instead.
			//
			// In case of delete-collection operations, this is the list object corresponding to the associated type.
			//
			// Ref https://github.com/kubernetes/kubernetes/issues/59501

			writeln!(variants, "    OkStatus({metav1}Status),")?;
			writeln!(variants, "    OkValue(T),")?;
			writeln!(variants, "    Accepted(T),")?;

			writeln!(variant_match_arms, "            http::StatusCode::OK => {{")?;
			writeln!(variant_match_arms, "                let result: {local}serde_json::Map<String, {local}serde_json::Value> = match {local}serde_json::from_slice(buf) {{")?;
			writeln!(variant_match_arms, "                    Ok(value) => value,")?;
			writeln!(variant_match_arms, "                    Err(err) if err.is_eof() => return Err({local}ResponseError::NeedMoreData),")?;
			writeln!(variant_match_arms, "                    Err(err) => return Err({local}ResponseError::Json(err)),")?;
			writeln!(variant_match_arms, "                }};")?;
			writeln!(variant_match_arms, r#"                let is_status = matches!(result.get("kind"), Some({local}serde_json::Value::String(s)) if s == "Status");"#)?;
			writeln!(variant_match_arms, "                if is_status {{")?;
			writeln!(variant_match_arms, "                    let result = {local}serde::Deserialize::deserialize({local}serde_json::Value::Object(result));")?;
			writeln!(variant_match_arms, "                    let result = result.map_err({local}ResponseError::Json)?;")?;
			writeln!(variant_match_arms, "                    Ok(({type_name}::OkStatus(result), buf.len()))")?;
			writeln!(variant_match_arms, "                }}")?;
			writeln!(variant_match_arms, "                else {{")?;
			writeln!(variant_match_arms, "                    let result = {local}serde::Deserialize::deserialize({local}serde_json::Value::Object(result));")?;
			writeln!(variant_match_arms, "                    let result = result.map_err({local}ResponseError::Json)?;")?;
			writeln!(variant_match_arms, "                    Ok(({type_name}::OkValue(result), buf.len()))")?;
			writeln!(variant_match_arms, "                }}")?;
			writeln!(variant_match_arms, "            }},")?;
			variant_match_arms.push_str(&deserialize_single(&local, "ACCEPTED", type_name, "Accepted")?);
		},

		OperationAction::List => {
			use std::fmt::Write;

			writeln!(variants, "    Ok({local}List<T>),")?;

			variant_match_arms.push_str(&deserialize_single(&local, "OK", type_name, "Ok")?);
		},

		OperationAction::Patch => {
			use std::fmt::Write;

			writeln!(variants, "    Ok(T),")?;
			writeln!(variants, "    Created(T),")?;

			variant_match_arms.push_str(&deserialize_single(&local, "OK", type_name, "Ok")?);
			variant_match_arms.push_str(&deserialize_single(&local, "CREATED", type_name, "Created")?);
		},

		OperationAction::Replace => {
			use std::fmt::Write;

			writeln!(variants, "    Ok(T),")?;
			writeln!(variants, "    Created(T),")?;

			variant_match_arms.push_str(&deserialize_single(&local, "OK", type_name, "Ok")?);
			variant_match_arms.push_str(&deserialize_single(&local, "CREATED", type_name, "Created")?);
		},

		OperationAction::Watch => {
			use std::fmt::Write;

			writeln!(variants, "    Ok({metav1}WatchEvent<T>),")?;

			writeln!(variant_match_arms, "            http::StatusCode::OK => {{")?;
			writeln!(variant_match_arms, "                let mut deserializer = {local}serde_json::Deserializer::from_slice(buf).into_iter();")?;
			writeln!(variant_match_arms, "                let (result, byte_offset) = match deserializer.next() {{")?;
			writeln!(variant_match_arms, "                    Some(Ok(value)) => (value, deserializer.byte_offset()),")?;
			writeln!(variant_match_arms, "                    Some(Err(err)) if err.is_eof() => return Err({local}ResponseError::NeedMoreData),")?;
			writeln!(variant_match_arms, "                    Some(Err(err)) => return Err({local}ResponseError::Json(err)),")?;
			writeln!(variant_match_arms, "                    None => return Err({local}ResponseError::NeedMoreData),")?;
			writeln!(variant_match_arms, "                }};")?;
			writeln!(variant_match_arms, "                Ok(({type_name}::Ok(result), byte_offset))")?;
			writeln!(variant_match_arms, "            }},")?;
		},
	};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/operation_response_common.rs")),
		local = local,
		type_name = type_name,
		type_generics_impl = type_generics_impl,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		variants = variants,
		operation_feature_attribute = operation_feature_attribute,
		variant_match_arms = variant_match_arms,
	)?;

	Ok(())
}

fn deserialize_single(local: &str, status_code: &str, type_name: &str, variant_name: &str) -> Result<String, std::fmt::Error> {
	use std::fmt::Write;

	let mut result = String::new();

	writeln!(result, "            http::StatusCode::{status_code} => {{")?;
	writeln!(result, "                let result = match {local}serde_json::from_slice(buf) {{")?;
	writeln!(result, "                    Ok(value) => value,")?;
	writeln!(result, "                    Err(err) if err.is_eof() => return Err({local}ResponseError::NeedMoreData),")?;
	writeln!(result, "                    Err(err) => return Err({local}ResponseError::Json(err)),")?;
	writeln!(result, "                }};")?;
	writeln!(result, "                Ok(({type_name}::{variant_name}(result), buf.len()))")?;
	writeln!(result, "            }},")?;

	Ok(result)
}

#[derive(Clone, Copy)]
pub(crate) enum OperationAction {
	Create,
	Delete,
	List,
	Patch,
	Replace,
	Watch,
}
