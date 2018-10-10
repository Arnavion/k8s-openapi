// Generated from definition io.k8s.api.core.v1.ServiceSpec

/// ServiceSpec describes the attributes that a user creates on a service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceSpec {
    /// clusterIP is the IP address of the service and is usually assigned randomly by the master. If an address is specified manually and is not in use by others, it will be allocated to the service; otherwise, creation of the service will fail. This field can not be changed through updates. Valid values are "None", empty string (""), or a valid IP address. "None" can be specified for headless services when proxying is not required. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub cluster_ip: Option<String>,

    /// externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.
    pub external_ips: Option<Vec<String>>,

    /// externalName is the external reference that kubedns or equivalent will return as a CNAME record for this service. No proxying will be involved. Must be a valid RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires Type to be ExternalName.
    pub external_name: Option<String>,

    /// externalTrafficPolicy denotes if this Service desires to route external traffic to node-local or cluster-wide endpoints. "Local" preserves the client source IP and avoids a second hop for LoadBalancer and Nodeport type services, but risks potentially imbalanced traffic spreading. "Cluster" obscures the client source IP and may cause a second hop to another node, but should have good overall load-spreading.
    pub external_traffic_policy: Option<String>,

    /// healthCheckNodePort specifies the healthcheck nodePort for the service. If not specified, HealthCheckNodePort is created by the service api backend with the allocated nodePort. Will use user-specified nodePort value if specified by the client. Only effects when Type is set to LoadBalancer and ExternalTrafficPolicy is set to Local.
    pub health_check_node_port: Option<i32>,

    /// Only applies to Service Type: LoadBalancer LoadBalancer will get created with the IP specified in this field. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature.
    pub load_balancer_ip: Option<String>,

    /// If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature." More info: https://kubernetes.io/docs/tasks/access-application-cluster/configure-cloud-provider-firewall/
    pub load_balancer_source_ranges: Option<Vec<String>>,

    /// The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub ports: Option<Vec<::v1_12::api::core::v1::ServicePort>>,

    /// publishNotReadyAddresses, when set to true, indicates that DNS implementations must publish the notReadyAddresses of subsets for the Endpoints associated with the Service. The default value is false. The primary use case for setting this field is to use a StatefulSet's Headless Service to propagate SRV records for its Pods without respect to their readiness for purpose of peer discovery.
    pub publish_not_ready_addresses: Option<bool>,

    /// Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/
    pub selector: Option<::std::collections::BTreeMap<String, String>>,

    /// Supports "ClientIP" and "None". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub session_affinity: Option<String>,

    /// sessionAffinityConfig contains the configurations of session affinity.
    pub session_affinity_config: Option<::v1_12::api::core::v1::SessionAffinityConfig>,

    /// type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. "ExternalName" maps to the specified externalName. "ClusterIP" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object. If clusterIP is "None", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a stable IP. "NodePort" builds on ClusterIP and allocates a port on every node which routes to the clusterIP. "LoadBalancer" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the clusterIP. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services---service-types
    pub type_: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for ServiceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cluster_ip,
            Key_external_ips,
            Key_external_name,
            Key_external_traffic_policy,
            Key_health_check_node_port,
            Key_load_balancer_ip,
            Key_load_balancer_source_ranges,
            Key_ports,
            Key_publish_not_ready_addresses,
            Key_selector,
            Key_session_affinity,
            Key_session_affinity_config,
            Key_type_,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "clusterIP" => Field::Key_cluster_ip,
                            "externalIPs" => Field::Key_external_ips,
                            "externalName" => Field::Key_external_name,
                            "externalTrafficPolicy" => Field::Key_external_traffic_policy,
                            "healthCheckNodePort" => Field::Key_health_check_node_port,
                            "loadBalancerIP" => Field::Key_load_balancer_ip,
                            "loadBalancerSourceRanges" => Field::Key_load_balancer_source_ranges,
                            "ports" => Field::Key_ports,
                            "publishNotReadyAddresses" => Field::Key_publish_not_ready_addresses,
                            "selector" => Field::Key_selector,
                            "sessionAffinity" => Field::Key_session_affinity,
                            "sessionAffinityConfig" => Field::Key_session_affinity_config,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceSpec;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ServiceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_cluster_ip: Option<String> = None;
                let mut value_external_ips: Option<Vec<String>> = None;
                let mut value_external_name: Option<String> = None;
                let mut value_external_traffic_policy: Option<String> = None;
                let mut value_health_check_node_port: Option<i32> = None;
                let mut value_load_balancer_ip: Option<String> = None;
                let mut value_load_balancer_source_ranges: Option<Vec<String>> = None;
                let mut value_ports: Option<Vec<::v1_12::api::core::v1::ServicePort>> = None;
                let mut value_publish_not_ready_addresses: Option<bool> = None;
                let mut value_selector: Option<::std::collections::BTreeMap<String, String>> = None;
                let mut value_session_affinity: Option<String> = None;
                let mut value_session_affinity_config: Option<::v1_12::api::core::v1::SessionAffinityConfig> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cluster_ip => value_cluster_ip = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_ips => value_external_ips = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_name => value_external_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_traffic_policy => value_external_traffic_policy = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_health_check_node_port => value_health_check_node_port = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer_ip => value_load_balancer_ip = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer_source_ranges => value_load_balancer_source_ranges = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_publish_not_ready_addresses => value_publish_not_ready_addresses = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_session_affinity => value_session_affinity = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_session_affinity_config => value_session_affinity_config = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceSpec {
                    cluster_ip: value_cluster_ip,
                    external_ips: value_external_ips,
                    external_name: value_external_name,
                    external_traffic_policy: value_external_traffic_policy,
                    health_check_node_port: value_health_check_node_port,
                    load_balancer_ip: value_load_balancer_ip,
                    load_balancer_source_ranges: value_load_balancer_source_ranges,
                    ports: value_ports,
                    publish_not_ready_addresses: value_publish_not_ready_addresses,
                    selector: value_selector,
                    session_affinity: value_session_affinity,
                    session_affinity_config: value_session_affinity_config,
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceSpec",
            &[
                "clusterIP",
                "externalIPs",
                "externalName",
                "externalTrafficPolicy",
                "healthCheckNodePort",
                "loadBalancerIP",
                "loadBalancerSourceRanges",
                "ports",
                "publishNotReadyAddresses",
                "selector",
                "sessionAffinity",
                "sessionAffinityConfig",
                "type",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ServiceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceSpec",
            0 +
            self.cluster_ip.as_ref().map_or(0, |_| 1) +
            self.external_ips.as_ref().map_or(0, |_| 1) +
            self.external_name.as_ref().map_or(0, |_| 1) +
            self.external_traffic_policy.as_ref().map_or(0, |_| 1) +
            self.health_check_node_port.as_ref().map_or(0, |_| 1) +
            self.load_balancer_ip.as_ref().map_or(0, |_| 1) +
            self.load_balancer_source_ranges.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1) +
            self.publish_not_ready_addresses.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.session_affinity.as_ref().map_or(0, |_| 1) +
            self.session_affinity_config.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.cluster_ip {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterIP", value)?;
        }
        if let Some(value) = &self.external_ips {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "externalIPs", value)?;
        }
        if let Some(value) = &self.external_name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "externalName", value)?;
        }
        if let Some(value) = &self.external_traffic_policy {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "externalTrafficPolicy", value)?;
        }
        if let Some(value) = &self.health_check_node_port {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "healthCheckNodePort", value)?;
        }
        if let Some(value) = &self.load_balancer_ip {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancerIP", value)?;
        }
        if let Some(value) = &self.load_balancer_source_ranges {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancerSourceRanges", value)?;
        }
        if let Some(value) = &self.ports {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        if let Some(value) = &self.publish_not_ready_addresses {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "publishNotReadyAddresses", value)?;
        }
        if let Some(value) = &self.selector {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.session_affinity {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "sessionAffinity", value)?;
        }
        if let Some(value) = &self.session_affinity_config {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "sessionAffinityConfig", value)?;
        }
        if let Some(value) = &self.type_ {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
