/// The value of an `"x-kubernetes-action"` annotation on an operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KubernetesAction {
    Connect,
    Delete,
    DeleteCollection,
    Get,
    List,
    Patch,
    Post,
    Proxy,
    Put,
    Watch,
    WatchList,
}

#[cfg(feature = "serde")]
#[allow(clippy::use_self)]
impl<'de> serde::Deserialize<'de> for KubernetesAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = KubernetesAction;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("x-kubernetes-action")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                Ok(match v {
                    "connect" => KubernetesAction::Connect,
                    "delete" => KubernetesAction::Delete,
                    "deletecollection" => KubernetesAction::DeleteCollection,
                    "get" => KubernetesAction::Get,
                    "list" => KubernetesAction::List,
                    "patch" => KubernetesAction::Patch,
                    "post" => KubernetesAction::Post,
                    "proxy" => KubernetesAction::Proxy,
                    "put" => KubernetesAction::Put,
                    "watch" => KubernetesAction::Watch,
                    "watchlist" => KubernetesAction::WatchList,
                    v => return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self)),
                })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/// An API operation.
#[derive(Clone, Debug)]
pub struct Operation {
    pub id: String,
    pub kubernetes_action: Option<KubernetesAction>,
    pub kubernetes_group_kind_version: Option<super::KubernetesGroupKindVersion>,
    pub path: Path,
}

/// The path of an API operation.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Path(pub String);

impl std::ops::Deref for Path {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
