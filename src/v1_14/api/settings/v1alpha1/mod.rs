
mod pod_preset;
pub use self::pod_preset::{
    PodPreset,
    CreateNamespacedPodPresetOptional, CreateNamespacedPodPresetResponse,
    DeleteCollectionNamespacedPodPresetOptional, DeleteCollectionNamespacedPodPresetResponse,
    DeleteNamespacedPodPresetOptional, DeleteNamespacedPodPresetResponse,
    ListNamespacedPodPresetResponse,
    ListPodPresetForAllNamespacesResponse,
    PatchNamespacedPodPresetOptional, PatchNamespacedPodPresetResponse,
    ReadNamespacedPodPresetOptional, ReadNamespacedPodPresetResponse,
    ReplaceNamespacedPodPresetOptional, ReplaceNamespacedPodPresetResponse,
    WatchNamespacedPodPresetResponse,
    WatchPodPresetForAllNamespacesResponse,
};

mod pod_preset_list;
pub use self::pod_preset_list::{
    PodPresetList,
};

mod pod_preset_spec;
pub use self::pod_preset_spec::{
    PodPresetSpec,
};
