use std::collections::BTreeMap;

use k8s_openapi::{
    api::{
        apps::v1::StatefulSetSpec,
        core::v1::{PersistentVolumeClaim, PersistentVolumeClaimSpec, VolumeResourceRequirements},
    },
    apimachinery::pkg::{api::resource::Quantity, apis::meta::v1::ObjectMeta},
    DeepMerge,
};

#[test]
fn stateful_set_volume_claim_templates_merge_by_name() {
    let make_pvc = |name: &str, storage_class: Option<&str>, storage: &str| PersistentVolumeClaim {
        metadata: ObjectMeta {
            name: Some(name.to_owned()),
            ..Default::default()
        },
        spec: Some(PersistentVolumeClaimSpec {
            storage_class_name: storage_class.map(ToOwned::to_owned),
            resources: Some(VolumeResourceRequirements {
                requests: Some(BTreeMap::from([(
                    "storage".to_owned(),
                    Quantity(storage.to_owned()),
                )])),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut base = StatefulSetSpec {
        volume_claim_templates: Some(vec![
            make_pvc("data", Some("fast"), "10Gi"),
            make_pvc("logs", Some("slow"), "1Gi"),
        ]),
        ..Default::default()
    };

    // Let's assume the data PVC needs more storage space
    let patch = StatefulSetSpec {
        volume_claim_templates: Some(vec![make_pvc("data", None, "1024Gi")]),
        ..Default::default()
    };

    base.merge_from(patch);

    let pvcs = base
        .volume_claim_templates
        .expect("volume_claim_templates should be present");

    assert_eq!(pvcs.len(), 2, "both PVCs should be present after map merge");

    let data_pvc = pvcs
        .iter()
        .find(|p| p.metadata.name.as_deref() == Some("data"))
        .expect("data PVC should be present");
    assert_eq!(
        data_pvc
            .spec
            .as_ref()
            .and_then(|s| s.storage_class_name.as_deref()),
        Some("fast"),
        "data PVC storage class should be untouched",
    );
    assert_eq!(
        data_pvc
            .spec
            .as_ref()
            .and_then(|s| s.resources.as_ref())
            .and_then(|r| r.requests.as_ref())
            .and_then(|r| r.get("storage"))
            .map(|q| q.0.as_str()),
        Some("1024Gi"),
        "data PVC storage request should be updated",
    );

    let logs_pvc = pvcs
        .iter()
        .find(|p| p.metadata.name.as_deref() == Some("logs"))
        .expect("logs PVC should be retained after map merge");
    assert_eq!(
        logs_pvc,
        &make_pvc("logs", Some("slow"), "1Gi"),
        "logs PVC should be untouched"
    );
}
