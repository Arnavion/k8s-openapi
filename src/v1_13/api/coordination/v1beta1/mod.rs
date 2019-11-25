
mod lease;
pub use self::lease::Lease;
#[cfg(feature = "api")] pub use self::lease::{CreateNamespacedLeaseOptional, CreateNamespacedLeaseResponse};
#[cfg(feature = "api")] pub use self::lease::{ReadNamespacedLeaseOptional, ReadNamespacedLeaseResponse};
#[cfg(feature = "api")] pub use self::lease::{ReplaceNamespacedLeaseOptional, ReplaceNamespacedLeaseResponse};

mod lease_spec;
pub use self::lease_spec::LeaseSpec;
