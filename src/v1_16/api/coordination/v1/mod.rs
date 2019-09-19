
mod lease;
pub use self::lease::Lease;
#[cfg(feature = "api")] pub use self::lease::{CreateNamespacedLeaseOptional, CreateNamespacedLeaseResponse};
#[cfg(feature = "api")] pub use self::lease::DeleteCollectionNamespacedLeaseResponse;
#[cfg(feature = "api")] pub use self::lease::DeleteNamespacedLeaseResponse;
#[cfg(feature = "api")] pub use self::lease::ListLeaseForAllNamespacesResponse;
#[cfg(feature = "api")] pub use self::lease::ListNamespacedLeaseResponse;
#[cfg(feature = "api")] pub use self::lease::PatchNamespacedLeaseResponse;
#[cfg(feature = "api")] pub use self::lease::{ReadNamespacedLeaseOptional, ReadNamespacedLeaseResponse};
#[cfg(feature = "api")] pub use self::lease::{ReplaceNamespacedLeaseOptional, ReplaceNamespacedLeaseResponse};
#[cfg(feature = "api")] pub use self::lease::WatchLeaseForAllNamespacesResponse;
#[cfg(feature = "api")] pub use self::lease::WatchNamespacedLeaseResponse;

mod lease_list;
pub use self::lease_list::LeaseList;

mod lease_spec;
pub use self::lease_spec::LeaseSpec;
