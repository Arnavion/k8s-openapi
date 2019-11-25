
mod cross_version_object_reference;
pub use self::cross_version_object_reference::CrossVersionObjectReference;

mod horizontal_pod_autoscaler;
pub use self::horizontal_pod_autoscaler::HorizontalPodAutoscaler;
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::{CreateNamespacedHorizontalPodAutoscalerOptional, CreateNamespacedHorizontalPodAutoscalerResponse};
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::{ReadNamespacedHorizontalPodAutoscalerOptional, ReadNamespacedHorizontalPodAutoscalerResponse};
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::{ReadNamespacedHorizontalPodAutoscalerStatusOptional, ReadNamespacedHorizontalPodAutoscalerStatusResponse};
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::{ReplaceNamespacedHorizontalPodAutoscalerOptional, ReplaceNamespacedHorizontalPodAutoscalerResponse};
#[cfg(feature = "api")] pub use self::horizontal_pod_autoscaler::{ReplaceNamespacedHorizontalPodAutoscalerStatusOptional, ReplaceNamespacedHorizontalPodAutoscalerStatusResponse};

mod horizontal_pod_autoscaler_spec;
pub use self::horizontal_pod_autoscaler_spec::HorizontalPodAutoscalerSpec;

mod horizontal_pod_autoscaler_status;
pub use self::horizontal_pod_autoscaler_status::HorizontalPodAutoscalerStatus;

mod scale;
pub use self::scale::Scale;
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
