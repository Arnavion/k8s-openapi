
mod pod_preset;
pub use self::pod_preset::PodPreset;
#[cfg(feature = "api")] pub use self::pod_preset::{ReadNamespacedPodPresetOptional, ReadNamespacedPodPresetResponse};

mod pod_preset_spec;
pub use self::pod_preset_spec::PodPresetSpec;
