
mod storage_class;
pub use self::storage_class::StorageClass;
#[cfg(feature = "api")] pub use self::storage_class::{ReadStorageClassOptional, ReadStorageClassResponse};
