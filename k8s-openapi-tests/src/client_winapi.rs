#![cfg_attr(feature = "cargo-clippy", allow(
	cast_possible_truncation,
	cast_ptr_alignment,
	unreadable_literal,
))]

extern crate winapi;

pub(crate) fn pkcs12(public_key: &::std::path::Path, private_key: &::std::path::Path) -> Result<Vec<u8>, ::Error> {
	unsafe {
		let public_key = parse_pem(public_key)?;

		let private_key = parse_pem(private_key)?;

		let cert_store = {
			let cert_store = winapi::um::wincrypt::CertOpenStore(winapi::um::wincrypt::CERT_STORE_PROV_MEMORY, 0, 0, 0, ::std::ptr::null());
			if cert_store.is_null() {
				Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
			}

			CertStore(cert_store)
		};

		let public_key_context = {
			let mut public_key_context = ::std::ptr::null();
			if winapi::um::wincrypt::CertAddEncodedCertificateToStore(
				cert_store.0,
				winapi::um::wincrypt::X509_ASN_ENCODING,
				public_key.as_ptr(),
				public_key.len() as _,
				winapi::um::wincrypt::CERT_STORE_ADD_NEW,
				&mut public_key_context,
			) != winapi::shared::minwindef::TRUE {
				Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
			}

			CertContext(public_key_context)
		};

		let private_key_decoded_buf = {
			let mut private_key_decoded_buf_len = 0;
			if winapi::um::wincrypt::CryptDecodeObjectEx(
				winapi::um::wincrypt::X509_ASN_ENCODING,
				winapi::um::wincrypt::PKCS_RSA_PRIVATE_KEY,
				private_key.as_ptr(),
				private_key.len() as _,
				0,
				::std::ptr::null_mut(),
				::std::ptr::null_mut(),
				&mut private_key_decoded_buf_len,
			) != winapi::shared::minwindef::TRUE {
				Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
			}

			let mut private_key_decoded_buf = vec![0u8; private_key_decoded_buf_len as _];
			if winapi::um::wincrypt::CryptDecodeObjectEx(
				winapi::um::wincrypt::X509_ASN_ENCODING,
				winapi::um::wincrypt::PKCS_RSA_PRIVATE_KEY,
				private_key.as_ptr(),
				private_key.len() as _,
				0,
				::std::ptr::null_mut(),
				private_key_decoded_buf.as_mut_ptr() as _,
				&mut private_key_decoded_buf_len,
			) != winapi::shared::minwindef::TRUE {
				Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
			}
			private_key_decoded_buf.resize(private_key_decoded_buf_len as _, 0);

			private_key_decoded_buf
		};

		let crypto_provider = {
			let mut crypto_provider = 0;
			let err = winapi2::um::ncrypt::NCryptOpenStorageProvider(
				&mut crypto_provider,
				winapi2::um::ncrypt::MS_KEY_STORAGE_PROVIDER,
				0,
			);
			if !winapi::shared::bcrypt::BCRYPT_SUCCESS(err) {
				Err(format!("0x{:08X}", err))?;
			}

			NCryptObject(crypto_provider)
		};

		let private_key = {
			let mut private_key = 0;
			let err = winapi2::um::ncrypt::NCryptImportKey(
				crypto_provider.0,
				0,
				winapi2::shared::bcrypt::LEGACY_RSAPRIVATE_BLOB,
				::std::ptr::null(),
				&mut private_key,
				private_key_decoded_buf.as_ptr() as _,
				private_key_decoded_buf.len() as _,
				winapi2::um::ncrypt::NCRYPT_SILENT_FLAG,
			);
			if !winapi::shared::bcrypt::BCRYPT_SUCCESS(err) {
				Err(format!("0x{:08X}", err))?;
			}

			NCryptObject(private_key)
		};

		{
			let export_policy_property_value = winapi2::um::ncrypt::NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG;
			let err = winapi2::um::ncrypt::NCryptSetProperty(
				private_key.0,
				winapi2::um::ncrypt::NCRYPT_EXPORT_POLICY_PROPERTY,
				&export_policy_property_value as *const _ as _,
				::std::mem::size_of_val(&export_policy_property_value) as _,
				0,
			);
			if !winapi::shared::bcrypt::BCRYPT_SUCCESS(err) {
				Err(format!("0x{:08X}", err))?;
			}
		}

		let mut private_key_context = winapi::um::wincrypt::CERT_KEY_CONTEXT {
			cbSize: 0,
			u: ::std::mem::zeroed(),
			dwKeySpec: winapi::um::wincrypt::CERT_NCRYPT_KEY_SPEC,
		};
		private_key_context.cbSize = ::std::mem::size_of_val(&private_key_context) as _;
		*private_key_context.u.hNCryptKey_mut() = private_key.0;

		if winapi::um::wincrypt::CertSetCertificateContextProperty(
			public_key_context.0,
			winapi::um::wincrypt::CERT_KEY_CONTEXT_PROP_ID,
			0,
			&private_key_context as *const _ as _,
		) != winapi::shared::minwindef::TRUE {
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
		) != winapi::shared::minwindef::TRUE {
			Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
		}

		let mut result = vec![0u8; private_key_data.cbData as _];

		private_key_data.pbData = result.as_mut_ptr();

		if winapi::um::wincrypt::PFXExportCertStoreEx(
			cert_store.0,
			&mut private_key_data,
			b"\0\0".as_ptr() as _,
			::std::ptr::null_mut(),
			winapi::um::wincrypt::EXPORT_PRIVATE_KEYS,
		) != winapi::shared::minwindef::TRUE {
			Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
		}

		result.resize(private_key_data.cbData as _, 0);

		Ok(result)
	}
}

pub(crate) fn x509_from_pem(path: &::std::path::Path) -> Result<Vec<u8>, ::Error> {
	parse_pem(path)
}

fn parse_pem(path: &::std::path::Path) -> Result<Vec<u8>, ::Error> {
	unsafe {
		let buf = ::std::fs::read(path)?;

		let mut result_len = 0;
		if winapi::um::wincrypt::CryptStringToBinaryA(
			buf.as_ptr() as _,
			buf.len() as _,
			winapi::um::wincrypt::CRYPT_STRING_BASE64HEADER,
			::std::ptr::null_mut(),
			&mut result_len,
			::std::ptr::null_mut(),
			::std::ptr::null_mut(),
		) != winapi::shared::minwindef::TRUE {
			Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
		}

		let mut result = vec![0u8; result_len as _];
		if winapi::um::wincrypt::CryptStringToBinaryA(
			buf.as_ptr() as _,
			buf.len() as _,
			winapi::um::wincrypt::CRYPT_STRING_BASE64HEADER,
			result.as_mut_ptr(),
			&mut result_len,
			::std::ptr::null_mut(),
			::std::ptr::null_mut(),
		) != winapi::shared::minwindef::TRUE {
			Err(format!("0x{:08X}", winapi::um::errhandlingapi::GetLastError()))?;
		}
		result.resize(result_len as _, 0);

		Ok(result)
	}
}

mod winapi2 {
	pub mod shared {
		pub mod bcrypt {
			use ::client::winapi::um::winnt::{ LPCWSTR };

			pub const LEGACY_RSAPRIVATE_BLOB: LPCWSTR = b"C\0A\0P\0I\0P\0R\0I\0V\0A\0T\0E\0B\0L\0O\0B\0\0\0" as *const _ as _;
		}
	}

	pub mod um {
		// TODO: https://github.com/retep998/winapi-rs/pull/630
		pub mod ncrypt {
			#![allow(non_camel_case_types)]

			use ::client::winapi::shared::bcrypt::{ BCryptBufferDesc };
			use ::client::winapi::shared::minwindef::{ DWORD, PBYTE };
			use ::client::winapi::um::winnt::{ LONG, LPCWSTR };
			use ::client::winapi::um::ncrypt::{ NCRYPT_HANDLE, NCRYPT_KEY_HANDLE, NCRYPT_PROV_HANDLE };

			pub type SECURITY_STATUS = LONG;

			pub const MS_KEY_STORAGE_PROVIDER: LPCWSTR = b"M\0i\0c\0r\0o\0s\0o\0f\0t\0 \0S\0o\0f\0t\0w\0a\0r\0e\0 \0K\0e\0y\0 \0S\0t\0o\0r\0a\0g\0e\0 \0P\0r\0o\0v\0i\0d\0e\0r\0\0\0" as *const _ as _;

			pub type NCryptBufferDesc = BCryptBufferDesc;

			pub const NCRYPT_SILENT_FLAG: DWORD = 0x00000040;

			extern "system" {
				pub fn NCryptOpenStorageProvider(
					phProvider: *mut NCRYPT_PROV_HANDLE,
					pszProviderName: LPCWSTR,
					dwFlags: DWORD,
				) -> SECURITY_STATUS;
			}

			pub const NCRYPT_EXPORT_POLICY_PROPERTY: LPCWSTR = b"E\0x\0p\0o\0r\0t\0 \0P\0o\0l\0i\0c\0y\0\0\0" as *const _ as _;

			pub const NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG: DWORD = 0x00000002;

			extern "system" {
				pub fn NCryptSetProperty(
					hObject: NCRYPT_HANDLE,
					pszProperty: LPCWSTR,
					pbInput: PBYTE,
					cbInput: DWORD,
					dwFlags: DWORD,
				) -> SECURITY_STATUS;

				pub fn NCryptImportKey(
					hProvider: NCRYPT_PROV_HANDLE,
					hImportKey: NCRYPT_KEY_HANDLE,
					pszBlobType: LPCWSTR,
					pParameterList: *const NCryptBufferDesc,
					phKey: *mut NCRYPT_KEY_HANDLE,
					pbData: PBYTE,
					cbData: DWORD,
					dwFlags: DWORD,
				) -> SECURITY_STATUS;
			}
		}
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

struct NCryptObject(winapi::um::ncrypt::NCRYPT_HANDLE);

impl Drop for NCryptObject {
	fn drop(&mut self) {
		unsafe {
			winapi::um::ncrypt::NCryptFreeObject(self.0);
		}
	}
}
