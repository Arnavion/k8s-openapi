extern crate openssl;

pub(crate) fn pkcs12(public_key: &::std::path::Path, private_key: &::std::path::Path) -> Result<Vec<u8>, ::Error> {
	let public_key = ::std::fs::read(public_key)?;
	let public_key = openssl::x509::X509::from_pem(&public_key)?;

	let private_key = ::std::fs::read(private_key)?;
	let private_key = openssl::pkey::PKey::private_key_from_pem(&private_key)?;

	Ok(openssl::pkcs12::Pkcs12::builder().build("", "admin", &private_key, &public_key)?.to_der()?)
}

pub(crate) fn x509_from_pem(path: &::std::path::Path) -> Result<Vec<u8>, ::Error> {
	let buf = ::std::fs::read(path)?;
	Ok(openssl::x509::X509::from_pem(&buf)?.to_der()?)
}
