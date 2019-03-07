
mod cross_version_object_reference;
pub use self::cross_version_object_reference::{
    CrossVersionObjectReference,
};

mod horizontal_pod_autoscaler;
pub use self::horizontal_pod_autoscaler::{
    HorizontalPodAutoscaler,
    CreateNamespacedHorizontalPodAutoscalerOptional, CreateNamespacedHorizontalPodAutoscalerResponse,
    DeleteCollectionNamespacedHorizontalPodAutoscalerOptional, DeleteCollectionNamespacedHorizontalPodAutoscalerResponse,
    DeleteNamespacedHorizontalPodAutoscalerOptional, DeleteNamespacedHorizontalPodAutoscalerResponse,
    ListHorizontalPodAutoscalerForAllNamespacesOptional, ListHorizontalPodAutoscalerForAllNamespacesResponse,
    ListNamespacedHorizontalPodAutoscalerOptional, ListNamespacedHorizontalPodAutoscalerResponse,
    PatchNamespacedHorizontalPodAutoscalerOptional, PatchNamespacedHorizontalPodAutoscalerResponse,
    PatchNamespacedHorizontalPodAutoscalerStatusOptional, PatchNamespacedHorizontalPodAutoscalerStatusResponse,
    ReadNamespacedHorizontalPodAutoscalerOptional, ReadNamespacedHorizontalPodAutoscalerResponse,
    ReadNamespacedHorizontalPodAutoscalerStatusOptional, ReadNamespacedHorizontalPodAutoscalerStatusResponse,
    ReplaceNamespacedHorizontalPodAutoscalerOptional, ReplaceNamespacedHorizontalPodAutoscalerResponse,
    ReplaceNamespacedHorizontalPodAutoscalerStatusOptional, ReplaceNamespacedHorizontalPodAutoscalerStatusResponse,
    WatchHorizontalPodAutoscalerForAllNamespacesOptional, WatchHorizontalPodAutoscalerForAllNamespacesResponse,
    WatchNamespacedHorizontalPodAutoscalerOptional, WatchNamespacedHorizontalPodAutoscalerResponse,
};

mod horizontal_pod_autoscaler_list;
pub use self::horizontal_pod_autoscaler_list::{
    HorizontalPodAutoscalerList,
};

mod horizontal_pod_autoscaler_spec;
pub use self::horizontal_pod_autoscaler_spec::{
    HorizontalPodAutoscalerSpec,
};

mod horizontal_pod_autoscaler_status;
pub use self::horizontal_pod_autoscaler_status::{
    HorizontalPodAutoscalerStatus,
};

mod scale;
pub use self::scale::{
    Scale,
    PatchNamespacedReplicationControllerScaleOptional, PatchNamespacedReplicationControllerScaleResponse,
    ReadNamespacedReplicationControllerScaleOptional, ReadNamespacedReplicationControllerScaleResponse,
    ReplaceNamespacedReplicationControllerScaleOptional, ReplaceNamespacedReplicationControllerScaleResponse,
};

mod scale_spec;
pub use self::scale_spec::{
    ScaleSpec,
};

mod scale_status;
pub use self::scale_status::{
    ScaleStatus,
};
