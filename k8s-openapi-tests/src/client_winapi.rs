extern crate winapi;

pub(crate) fn pkcs12(public_key: &::std::path::Path, private_key: &::std::path::Path) -> Result<Vec<u8>, ::Error> {
	unsafe {
		let public_key = {
			let public_key = ::std::fs::read(public_key)?;
			::pem::parse(public_key)?.contents
		};

		let private_key = {
			let private_key = ::std::fs::read(private_key)?;
			::pem::parse(private_key)?.contents
		};

		let cert_store = {
			let cert_store = winapi::um::wincrypt::CertOpenStore(winapi::um::wincrypt::CERT_STORE_PROV_MEMORY, 0, 0, 0, ::std::ptr::null());
			if cert_store.is_null() {
				Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
			}

			CertStore(cert_store)
		};

		let public_key_context = {
			let mut public_key_context: winapi::um::wincrypt::PCCERT_CONTEXT = ::std::ptr::null_mut();
			if winapi::um::wincrypt::CertAddEncodedCertificateToStore(
				cert_store.0,
				winapi::um::wincrypt::X509_ASN_ENCODING,
				public_key.as_ptr(),
				public_key.len() as winapi::shared::minwindef::DWORD,
				winapi::um::wincrypt::CERT_STORE_ADD_NEW,
				&mut public_key_context,
			) != winapi::shared::minwindef::TRUE {
				Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
			}

			CertContext(public_key_context)
		};

		let private_key_decoded_buf = {
			let mut private_key_decoded_buf_len: winapi::shared::minwindef::DWORD = 0;
			if winapi::um::wincrypt::CryptDecodeObjectEx(
				winapi::um::wincrypt::X509_ASN_ENCODING,
				winapi::um::wincrypt::PKCS_RSA_PRIVATE_KEY,
				private_key.as_ptr(),
				private_key.len() as winapi::shared::minwindef::DWORD,
				0,
				::std::ptr::null_mut(),
				::std::ptr::null_mut(),
				&mut private_key_decoded_buf_len,
			) != winapi::shared::minwindef::TRUE
			{
				Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
			}

			let mut private_key_decoded_buf = vec![0u8; private_key_decoded_buf_len as usize];
			if winapi::um::wincrypt::CryptDecodeObjectEx(
				winapi::um::wincrypt::X509_ASN_ENCODING,
				winapi::um::wincrypt::PKCS_RSA_PRIVATE_KEY,
				private_key.as_ptr(),
				private_key.len() as winapi::shared::minwindef::DWORD,
				0,
				::std::ptr::null_mut(),
				private_key_decoded_buf.as_mut_ptr() as _,
				&mut private_key_decoded_buf_len,
			) != winapi::shared::minwindef::TRUE
			{
				Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
			}

			private_key_decoded_buf
		};

		let crypto_provider = {
			let mut crypto_provider: winapi::um::wincrypt::HCRYPTPROV = 0;
			if winapi::um::wincrypt::CryptAcquireContextA(
				&mut crypto_provider,
				::std::ptr::null_mut(),
				b"Microsoft Enhanced Cryptographic Provider v1.0\0".as_ptr() as _,
				winapi::um::wincrypt::PROV_RSA_FULL,
				winapi::um::wincrypt::CRYPT_VERIFYCONTEXT | winapi::um::wincrypt::CRYPT_SILENT,
			) != winapi::shared::minwindef::TRUE
			{
				Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
			}

			CryptoProvider(crypto_provider)
		};

		{
			let mut private_key: winapi::um::wincrypt::HCRYPTKEY = 0;
			if winapi::um::wincrypt::CryptImportKey(
				crypto_provider.0,
				private_key_decoded_buf.as_ptr(),
				private_key_decoded_buf.len() as winapi::shared::minwindef::DWORD,
				0,
				winapi::um::wincrypt::CRYPT_EXPORTABLE,
				&mut private_key,
			) != winapi::shared::minwindef::TRUE
			{
				Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
			}

			winapi::um::wincrypt::CryptDestroyKey(private_key);
		}

		let mut private_key_context = winapi::um::wincrypt::CERT_KEY_CONTEXT {
			cbSize: ::std::mem::size_of::<winapi::um::wincrypt::CERT_KEY_CONTEXT>() as winapi::shared::minwindef::DWORD,
			u: ::std::mem::zeroed(),
			dwKeySpec: winapi::um::wincrypt::AT_KEYEXCHANGE,
		};
		*private_key_context.u.hCryptProv_mut() = crypto_provider.0;

		if winapi::um::wincrypt::CertSetCertificateContextProperty(
			public_key_context.0,
			winapi::um::wincrypt::CERT_KEY_CONTEXT_PROP_ID,
			0,
			&private_key_context as *const _ as _,
		) != winapi::shared::minwindef::TRUE
		{
			Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
		}

		let mut private_key_data = winapi::um::wincrypt::CRYPT_DATA_BLOB {
			cbData: 0,
			pbData: ::std::ptr::null_mut(),
		};

		if winapi::um::wincrypt::PFXExportCertStoreEx(
			cert_store.0,
			&mut private_key_data,
			b"\0\0".as_ptr() as _,
			::std::ptr::null_mut(),
			winapi::um::wincrypt::EXPORT_PRIVATE_KEYS,
		) != winapi::shared::minwindef::TRUE
		{
			Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
		}

		let mut result = vec![0u8; private_key_data.cbData as usize];

		private_key_data.pbData = result.as_mut_ptr();

		if winapi::um::wincrypt::PFXExportCertStoreEx(
			cert_store.0,
			&mut private_key_data,
			b"\0\0".as_ptr() as _,
			::std::ptr::null_mut(),
			winapi::um::wincrypt::EXPORT_PRIVATE_KEYS,
		) != winapi::shared::minwindef::TRUE
		{
			Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
		}

		Ok(result)
	}
}

struct CertStore(winapi::um::wincrypt::HCERTSTORE);

impl Drop for CertStore {
	fn drop(&mut self) {
		unsafe {
			winapi::um::wincrypt::CertCloseStore(self.0, 0);
		}
	}
}

struct CertContext(winapi::um::wincrypt::PCCERT_CONTEXT);

impl Drop for CertContext {
	fn drop(&mut self) {
		unsafe {
			winapi::um::wincrypt::CertFreeCertificateContext(self.0);
		}
	}
}

struct CryptoProvider(winapi::um::wincrypt::HCRYPTPROV);

impl Drop for CryptoProvider {
	fn drop(&mut self) {
		unsafe {
			winapi::um::wincrypt::CryptReleaseContext(self.0, 0);
		}
	}
}
