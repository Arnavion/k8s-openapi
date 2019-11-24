
mod pod_preset;
pub use self::pod_preset::PodPreset;
#[cfg(feature = "api")] pub use self::pod_preset::{CreateNamespacedPodPresetOptional, CreateNamespacedPodPresetResponse};
#[cfg(feature = "api")] pub use self::pod_preset::DeleteCollectionNamespacedPodPresetResponse;
#[cfg(feature = "api")] pub use self::pod_preset::DeleteNamespacedPodPresetResponse;
#[cfg(feature = "api")] pub use self::pod_preset::ListNamespacedPodPresetResponse;
#[cfg(feature = "api")] pub use self::pod_preset::ListPodPresetForAllNamespacesResponse;
#[cfg(feature = "api")] pub use self::pod_preset::PatchNamespacedPodPresetResponse;
#[cfg(feature = "api")] pub use self::pod_preset::{ReadNamespacedPodPresetOptional, ReadNamespacedPodPresetResponse};
#[cfg(feature = "api")] pub use self::pod_preset::{ReplaceNamespacedPodPresetOptional, ReplaceNamespacedPodPresetResponse};
#[cfg(feature = "api")] pub use self::pod_preset::WatchNamespacedPodPresetResponse;
#[cfg(feature = "api")] pub use self::pod_preset::WatchPodPresetForAllNamespacesResponse;

mod pod_preset_spec;
pub use self::pod_preset_spec::PodPresetSpec;
