
mod cross_version_object_reference;
pub use self::cross_version_object_reference::CrossVersionObjectReference;

mod horizontal_pod_autoscaler;
pub use self::horizontal_pod_autoscaler::HorizontalPodAutoscaler;
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::{CreateNamespacedHorizontalPodAutoscalerOptional, CreateNamespacedHorizontalPodAutoscalerResponse};
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::DeleteCollectionNamespacedHorizontalPodAutoscalerResponse;
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::DeleteNamespacedHorizontalPodAutoscalerResponse;
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::ListHorizontalPodAutoscalerForAllNamespacesResponse;
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::ListNamespacedHorizontalPodAutoscalerResponse;
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::PatchNamespacedHorizontalPodAutoscalerResponse;
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::PatchNamespacedHorizontalPodAutoscalerStatusResponse;
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::{ReadNamespacedHorizontalPodAutoscalerOptional, ReadNamespacedHorizontalPodAutoscalerResponse};
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::{ReadNamespacedHorizontalPodAutoscalerStatusOptional, ReadNamespacedHorizontalPodAutoscalerStatusResponse};
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::{ReplaceNamespacedHorizontalPodAutoscalerOptional, ReplaceNamespacedHorizontalPodAutoscalerResponse};
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::{ReplaceNamespacedHorizontalPodAutoscalerStatusOptional, ReplaceNamespacedHorizontalPodAutoscalerStatusResponse};
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::WatchHorizontalPodAutoscalerForAllNamespacesResponse;
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::WatchNamespacedHorizontalPodAutoscalerResponse;

mod horizontal_pod_autoscaler_list;
pub use self::horizontal_pod_autoscaler_list::HorizontalPodAutoscalerList;

mod horizontal_pod_autoscaler_spec;
pub use self::horizontal_pod_autoscaler_spec::HorizontalPodAutoscalerSpec;

mod horizontal_pod_autoscaler_status;
pub use self::horizontal_pod_autoscaler_status::HorizontalPodAutoscalerStatus;

mod scale;
pub use self::scale::Scale;
#[cfg(feature = "api")] pub use self::scale::PatchNamespacedDeploymentScaleResponse;
#[cfg(feature = "api")] pub use self::scale::PatchNamespacedReplicaSetScaleResponse;
#[cfg(feature = "api")] pub use self::scale::PatchNamespacedStatefulSetScaleResponse;
#[cfg(feature = "api")] pub use self::scale::PatchNamespacedReplicationControllerScaleResponse;
#[cfg(feature = "api")] pub use self::scale::{ReadNamespacedDeploymentScaleOptional, ReadNamespacedDeploymentScaleResponse};
#[cfg(feature = "api")] pub use self::scale::{ReadNamespacedReplicaSetScaleOptional, ReadNamespacedReplicaSetScaleResponse};
#[cfg(feature = "api")] pub use self::scale::{ReadNamespacedStatefulSetScaleOptional, ReadNamespacedStatefulSetScaleResponse};
#[cfg(feature = "api")] pub use self::scale::{ReadNamespacedReplicationControllerScaleOptional, ReadNamespacedReplicationControllerScaleResponse};
#[cfg(feature = "api")] pub use self::scale::{ReplaceNamespacedDeploymentScaleOptional, ReplaceNamespacedDeploymentScaleResponse};
#[cfg(feature = "api")] pub use self::scale::{ReplaceNamespacedReplicaSetScaleOptional, ReplaceNamespacedReplicaSetScaleResponse};
#[cfg(feature = "api")] pub use self::scale::{ReplaceNamespacedStatefulSetScaleOptional, ReplaceNamespacedStatefulSetScaleResponse};
#[cfg(feature = "api")] pub use self::scale::{ReplaceNamespacedReplicationControllerScaleOptional, ReplaceNamespacedReplicationControllerScaleResponse};

mod scale_spec;
pub use self::scale_spec::ScaleSpec;

mod scale_status;
pub use self::scale_status::ScaleStatus;
