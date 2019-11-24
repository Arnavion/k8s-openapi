
mod storage_class;
pub use self::storage_class::StorageClass;
#[cfg(feature = "api")] pub use self::storage_class::{CreateStorageClassOptional, CreateStorageClassResponse};
#[cfg(feature = "api")] pub use self::storage_class::DeleteCollectionStorageClassResponse;
#[cfg(feature = "api")] pub use self::storage_class::DeleteStorageClassResponse;
#[cfg(feature = "api")] pub use self::storage_class::ListStorageClassResponse;
#[cfg(feature = "api")] pub use self::storage_class::PatchStorageClassResponse;
#[cfg(feature = "api")] pub use self::storage_class::{ReadStorageClassOptional, ReadStorageClassResponse};
#[cfg(feature = "api")] pub use self::storage_class::{ReplaceStorageClassOptional, ReplaceStorageClassResponse};
#[cfg(feature = "api")] pub use self::storage_class::WatchStorageClassResponse;
