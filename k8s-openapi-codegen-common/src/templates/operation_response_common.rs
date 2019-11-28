pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	crate_root: &str,
	operation_action: OperationAction,
) -> Result<(), crate::Error> {
	let type_generics_impl = "<T>";
	let type_generics_type = "<T>";
	let type_generics_where: std::borrow::Cow<'_, str> = match operation_action {
		OperationAction::List => format!(" where T: serde::de::DeserializeOwned + {}::ListableResource", crate_root).into(),

		OperationAction::Create |
		OperationAction::Delete |
		OperationAction::Replace |
		OperationAction::Patch |
		OperationAction::Watch => " where T: serde::de::DeserializeOwned".into(),
	};

	let mut variants = String::new();
	let mut variant_match_arms = String::new();
	match operation_action {
		OperationAction::Create => {
			use std::fmt::Write;

			writeln!(variants, "    Ok(T),")?;
			writeln!(variants, "    Created(T),")?;
			writeln!(variants, "    Accepted(T),")?;

			variant_match_arms.push_str(&deserialize_single(crate_root, "OK", type_name, "Ok")?);
			variant_match_arms.push_str(&deserialize_single(crate_root, "CREATED", type_name, "Created")?);
			variant_match_arms.push_str(&deserialize_single(crate_root, "ACCEPTED", type_name, "Accepted")?);
		},

		OperationAction::Delete => {
			use std::fmt::Write;

			// Delete and delete-collection operations that return metav1.Status for HTTP 200 can also return the object itself instead.
			//
			// In case of delete-collection operations, this is the list object corresponding to the associated type.
			//
			// Ref https://github.com/kubernetes/kubernetes/issues/59501

			writeln!(variants, "    OkStatus({}::apimachinery::pkg::apis::meta::v1::Status),", crate_root)?;
			writeln!(variants, "    OkValue(T),")?;
			writeln!(variants, "    Accepted(T),")?;

			writeln!(variant_match_arms, "            http::StatusCode::OK => {{")?;
			writeln!(variant_match_arms, "                let result: serde_json::Map<String, serde_json::Value> = match serde_json::from_slice(buf) {{")?;
			writeln!(variant_match_arms, "                    Ok(value) => value,")?;
			writeln!(variant_match_arms, "                    Err(ref err) if err.is_eof() => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
			writeln!(variant_match_arms, "                    Err(err) => return Err({}::ResponseError::Json(err)),", crate_root)?;
			writeln!(variant_match_arms, "                }};")?;
			writeln!(variant_match_arms, r#"                let is_status = match result.get("kind") {{"#)?;
			writeln!(variant_match_arms, r#"                    Some(serde_json::Value::String(s)) if s == "Status" => true,"#)?;
			writeln!(variant_match_arms, "                    _ => false,")?;
			writeln!(variant_match_arms, "                }};")?;
			writeln!(variant_match_arms, "                if is_status {{")?;
			writeln!(variant_match_arms, "                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));")?;
			writeln!(variant_match_arms, "                    let result = result.map_err({}::ResponseError::Json)?;", crate_root)?;
			writeln!(variant_match_arms, "                    Ok(({}::OkStatus(result), buf.len()))", type_name)?;
			writeln!(variant_match_arms, "                }}")?;
			writeln!(variant_match_arms, "                else {{")?;
			writeln!(variant_match_arms, "                    let result = serde::Deserialize::deserialize(serde_json::Value::Object(result));")?;
			writeln!(variant_match_arms, "                    let result = result.map_err({}::ResponseError::Json)?;", crate_root)?;
			writeln!(variant_match_arms, "                    Ok(({}::OkValue(result), buf.len()))", type_name)?;
			writeln!(variant_match_arms, "                }}")?;
			writeln!(variant_match_arms, "            }},")?;
			variant_match_arms.push_str(&deserialize_single(crate_root, "ACCEPTED", type_name, "Accepted")?);
		},

		OperationAction::List => {
			use std::fmt::Write;

			writeln!(variants, "    Ok({}::List<T>),", crate_root)?;

			variant_match_arms.push_str(&deserialize_single(crate_root, "OK", type_name, "Ok")?);
		},

		OperationAction::Patch => {
			use std::fmt::Write;

			writeln!(variants, "    Ok(T),")?;

			variant_match_arms.push_str(&deserialize_single(crate_root, "OK", type_name, "Ok")?);
		},

		OperationAction::Replace => {
			use std::fmt::Write;

			writeln!(variants, "    Ok(T),")?;
			writeln!(variants, "    Created(T),")?;

			variant_match_arms.push_str(&deserialize_single(crate_root, "OK", type_name, "Ok")?);
			variant_match_arms.push_str(&deserialize_single(crate_root, "CREATED", type_name, "Created")?);
		},

		OperationAction::Watch => {
			use std::fmt::Write;

			writeln!(variants, "    Ok({}::apimachinery::pkg::apis::meta::v1::WatchEvent<T>),", crate_root)?;

			writeln!(variant_match_arms, "            http::StatusCode::OK => {{")?;
			writeln!(variant_match_arms, "                let mut deserializer = serde_json::Deserializer::from_slice(buf).into_iter();")?;
			writeln!(variant_match_arms, "                let (result, byte_offset) = match deserializer.next() {{")?;
			writeln!(variant_match_arms, "                    Some(Ok(value)) => (value, deserializer.byte_offset()),")?;
			writeln!(variant_match_arms, "                    Some(Err(ref err)) if err.is_eof() => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
			writeln!(variant_match_arms, "                    Some(Err(err)) => return Err({}::ResponseError::Json(err)),", crate_root)?;
			writeln!(variant_match_arms, "                    None => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
			writeln!(variant_match_arms, "                }};")?;
			writeln!(variant_match_arms, "                Ok(({}::Ok(result), byte_offset))", type_name)?;
			writeln!(variant_match_arms, "            }},")?;
		},
	};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/operation_response_common.rs")),
		type_name = type_name,
		type_generics_impl = type_generics_impl,
		type_generics_type = type_generics_type,
		type_generics_where = type_generics_where,
		crate_root = crate_root,
		variants = variants,
		variant_match_arms = variant_match_arms,
	)?;

	Ok(())
}

fn deserialize_single(crate_root: &str, status_code: &str, type_name: &str, variant_name: &str) -> Result<String, std::fmt::Error> {
	use std::fmt::Write;

	let mut result = String::new();

	writeln!(result, "            http::StatusCode::{} => {{", status_code)?;
	writeln!(result, "                let result = match serde_json::from_slice(buf) {{")?;
	writeln!(result, "                    Ok(value) => value,")?;
	writeln!(result, "                    Err(ref err) if err.is_eof() => return Err({}::ResponseError::NeedMoreData),", crate_root)?;
	writeln!(result, "                    Err(err) => return Err({}::ResponseError::Json(err)),", crate_root)?;
	writeln!(result, "                }};")?;
	writeln!(result, "                Ok(({}::{}(result), buf.len()))", type_name, variant_name)?;
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
