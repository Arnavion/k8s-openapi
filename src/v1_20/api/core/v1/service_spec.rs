// Generated from definition io.k8s.api.core.v1.ServiceSpec

/// ServiceSpec describes the attributes that a user creates on a service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceSpec {
    /// allocateLoadBalancerNodePorts defines if NodePorts will be automatically allocated for services with type LoadBalancer.  Default is "true". It may be set to "false" if the cluster load-balancer does not rely on NodePorts. allocateLoadBalancerNodePorts may only be set for services with type LoadBalancer and will be cleared if the type is changed to any other type. This field is alpha-level and is only honored by servers that enable the ServiceLBNodePortControl feature.
    pub allocate_load_balancer_node_ports: Option<bool>,

    /// clusterIP is the IP address of the service and is usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be blank) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address. Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub cluster_ip: Option<String>,

    /// ClusterIPs is a list of IP addresses assigned to this service, and are usually assigned randomly.  If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be empty) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address.  Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName.  If this field is not specified, it will be initialized from the clusterIP field.  If this field is specified, clients must ensure that clusterIPs\[0\] and clusterIP have the same value.
    ///
    /// Unless the "IPv6DualStack" feature gate is enabled, this field is limited to one value, which must be the same as the clusterIP field.  If the feature gate is enabled, this field may hold a maximum of two entries (dual-stack IPs, in either order).  These IPs must correspond to the values of the ipFamilies field. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub cluster_i_ps: Vec<String>,

    /// externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.
    pub external_ips: Vec<String>,

    /// externalName is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved.  Must be a lowercase RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires Type to be
    pub external_name: Option<String>,

    /// externalTrafficPolicy denotes if this Service desires to route external traffic to node-local or cluster-wide endpoints. "Local" preserves the client source IP and avoids a second hop for LoadBalancer and Nodeport type services, but risks potentially imbalanced traffic spreading. "Cluster" obscures the client source IP and may cause a second hop to another node, but should have good overall load-spreading.
    pub external_traffic_policy: Option<String>,

    /// healthCheckNodePort specifies the healthcheck nodePort for the service. This only applies when type is set to LoadBalancer and externalTrafficPolicy is set to Local. If a value is specified, is in-range, and is not in use, it will be used.  If not specified, a value will be automatically allocated.  External systems (e.g. load-balancers) can use this port to determine if a given node holds endpoints for this service or not.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type).
    pub health_check_node_port: Option<i32>,

    /// IPFamilies is a list of IP families (e.g. IPv4, IPv6) assigned to this service, and is gated by the "IPv6DualStack" feature gate.  This field is usually assigned automatically based on cluster configuration and the ipFamilyPolicy field. If this field is specified manually, the requested family is available in the cluster, and ipFamilyPolicy allows it, it will be used; otherwise creation of the service will fail.  This field is conditionally mutable: it allows for adding or removing a secondary IP family, but it does not allow changing the primary IP family of the Service.  Valid values are "IPv4" and "IPv6".  This field only applies to Services of types ClusterIP, NodePort, and LoadBalancer, and does apply to "headless" services.  This field will be wiped when updating a Service to type ExternalName.
    ///
    /// This field may hold a maximum of two entries (dual-stack families, in either order).  These families must correspond to the values of the clusterIPs field, if specified. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field.
    pub ip_families: Vec<String>,

    /// IPFamilyPolicy represents the dual-stack-ness requested or required by this Service, and is gated by the "IPv6DualStack" feature gate.  If there is no value provided, then this field will be set to SingleStack. Services can be "SingleStack" (a single IP family), "PreferDualStack" (two IP families on dual-stack configured clusters or a single IP family on single-stack clusters), or "RequireDualStack" (two IP families on dual-stack configured clusters, otherwise fail). The ipFamilies and clusterIPs fields depend on the value of this field.  This field will be wiped when updating a service to type ExternalName.
    pub ip_family_policy: Option<String>,

    /// Only applies to Service Type: LoadBalancer LoadBalancer will get created with the IP specified in this field. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature.
    pub load_balancer_ip: Option<String>,

    /// If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature." More info: https://kubernetes.io/docs/tasks/access-application-cluster/configure-cloud-provider-firewall/
    pub load_balancer_source_ranges: Vec<String>,

    /// The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub ports: Vec<crate::api::core::v1::ServicePort>,

    /// publishNotReadyAddresses indicates that any agent which deals with endpoints for this Service should disregard any indications of ready/not-ready. The primary use case for setting this field is for a StatefulSet's Headless Service to propagate SRV DNS records for its Pods for the purpose of peer discovery. The Kubernetes controllers that generate Endpoints and EndpointSlice resources for Services interpret this to mean that all endpoints are considered "ready" even if the Pods themselves are not. Agents which consume only Kubernetes generated endpoints through the Endpoints or EndpointSlice resources can safely assume this behavior.
    pub publish_not_ready_addresses: Option<bool>,

    /// Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/
    pub selector: std::collections::BTreeMap<String, String>,

    /// Supports "ClientIP" and "None". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub session_affinity: Option<String>,

    /// sessionAffinityConfig contains the configurations of session affinity.
    pub session_affinity_config: Option<crate::api::core::v1::SessionAffinityConfig>,

    /// topologyKeys is a preference-order list of topology keys which implementations of services should use to preferentially sort endpoints when accessing this Service, it can not be used at the same time as externalTrafficPolicy=Local. Topology keys must be valid label keys and at most 16 keys may be specified. Endpoints are chosen based on the first topology key with available backends. If this field is specified and all entries have no backends that match the topology of the client, the service has no backends for that client and connections should fail. The special value "*" may be used to mean "any topology". This catch-all value, if used, only makes sense as the last value in the list. If this is not specified or empty, no topology constraints will be applied. This field is alpha-level and is only honored by servers that enable the ServiceTopology feature.
    pub topology_keys: Vec<String>,

    /// type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. "ClusterIP" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object or EndpointSlice objects. If clusterIP is "None", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a virtual IP. "NodePort" builds on ClusterIP and allocates a port on every node which routes to the same endpoints as the clusterIP. "LoadBalancer" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the same endpoints as the clusterIP. "ExternalName" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types
    pub type_: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for ServiceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocate_load_balancer_node_ports,
            Key_cluster_ip,
            Key_cluster_i_ps,
            Key_external_ips,
            Key_external_name,
            Key_external_traffic_policy,
            Key_health_check_node_port,
            Key_ip_families,
            Key_ip_family_policy,
            Key_load_balancer_ip,
            Key_load_balancer_source_ranges,
            Key_ports,
            Key_publish_not_ready_addresses,
            Key_selector,
            Key_session_affinity,
            Key_session_affinity_config,
            Key_topology_keys,
            Key_type_,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "allocateLoadBalancerNodePorts" => Field::Key_allocate_load_balancer_node_ports,
                            "clusterIP" => Field::Key_cluster_ip,
                            "clusterIPs" => Field::Key_cluster_i_ps,
                            "externalIPs" => Field::Key_external_ips,
                            "externalName" => Field::Key_external_name,
                            "externalTrafficPolicy" => Field::Key_external_traffic_policy,
                            "healthCheckNodePort" => Field::Key_health_check_node_port,
                            "ipFamilies" => Field::Key_ip_families,
                            "ipFamilyPolicy" => Field::Key_ip_family_policy,
                            "loadBalancerIP" => Field::Key_load_balancer_ip,
                            "loadBalancerSourceRanges" => Field::Key_load_balancer_source_ranges,
                            "ports" => Field::Key_ports,
                            "publishNotReadyAddresses" => Field::Key_publish_not_ready_addresses,
                            "selector" => Field::Key_selector,
                            "sessionAffinity" => Field::Key_session_affinity,
                            "sessionAffinityConfig" => Field::Key_session_affinity_config,
                            "topologyKeys" => Field::Key_topology_keys,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocate_load_balancer_node_ports: Option<bool> = None;
                let mut value_cluster_ip: Option<String> = None;
                let mut value_cluster_i_ps: Option<Vec<String>> = None;
                let mut value_external_ips: Option<Vec<String>> = None;
                let mut value_external_name: Option<String> = None;
                let mut value_external_traffic_policy: Option<String> = None;
                let mut value_health_check_node_port: Option<i32> = None;
                let mut value_ip_families: Option<Vec<String>> = None;
                let mut value_ip_family_policy: Option<String> = None;
                let mut value_load_balancer_ip: Option<String> = None;
                let mut value_load_balancer_source_ranges: Option<Vec<String>> = None;
                let mut value_ports: Option<Vec<crate::api::core::v1::ServicePort>> = None;
                let mut value_publish_not_ready_addresses: Option<bool> = None;
                let mut value_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_session_affinity: Option<String> = None;
                let mut value_session_affinity_config: Option<crate::api::core::v1::SessionAffinityConfig> = None;
                let mut value_topology_keys: Option<Vec<String>> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocate_load_balancer_node_ports => value_allocate_load_balancer_node_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cluster_ip => value_cluster_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cluster_i_ps => value_cluster_i_ps = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_ips => value_external_ips = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_name => value_external_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_traffic_policy => value_external_traffic_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_health_check_node_port => value_health_check_node_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_families => value_ip_families = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_family_policy => value_ip_family_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer_ip => value_load_balancer_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer_source_ranges => value_load_balancer_source_ranges = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_publish_not_ready_addresses => value_publish_not_ready_addresses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_session_affinity => value_session_affinity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_session_affinity_config => value_session_affinity_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_topology_keys => value_topology_keys = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceSpec {
                    allocate_load_balancer_node_ports: value_allocate_load_balancer_node_ports,
                    cluster_ip: value_cluster_ip,
                    cluster_i_ps: value_cluster_i_ps.unwrap_or_default(),
                    external_ips: value_external_ips.unwrap_or_default(),
                    external_name: value_external_name,
                    external_traffic_policy: value_external_traffic_policy,
                    health_check_node_port: value_health_check_node_port,
                    ip_families: value_ip_families.unwrap_or_default(),
                    ip_family_policy: value_ip_family_policy,
                    load_balancer_ip: value_load_balancer_ip,
                    load_balancer_source_ranges: value_load_balancer_source_ranges.unwrap_or_default(),
                    ports: value_ports.unwrap_or_default(),
                    publish_not_ready_addresses: value_publish_not_ready_addresses,
                    selector: value_selector.unwrap_or_default(),
                    session_affinity: value_session_affinity,
                    session_affinity_config: value_session_affinity_config,
                    topology_keys: value_topology_keys.unwrap_or_default(),
                    type_: value_type_,
                })
            }
        }

        deserializer.deserialize_struct(
            "ServiceSpec",
            &[
                "allocateLoadBalancerNodePorts",
                "clusterIP",
                "clusterIPs",
                "externalIPs",
                "externalName",
                "externalTrafficPolicy",
                "healthCheckNodePort",
                "ipFamilies",
                "ipFamilyPolicy",
                "loadBalancerIP",
                "loadBalancerSourceRanges",
                "ports",
                "publishNotReadyAddresses",
                "selector",
                "sessionAffinity",
                "sessionAffinityConfig",
                "topologyKeys",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ServiceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceSpec",
            self.allocate_load_balancer_node_ports.as_ref().map_or(0, |_| 1) +
            self.cluster_ip.as_ref().map_or(0, |_| 1) +
            usize::from(!self.cluster_i_ps.is_empty()) +
            usize::from(!self.external_ips.is_empty()) +
            self.external_name.as_ref().map_or(0, |_| 1) +
            self.external_traffic_policy.as_ref().map_or(0, |_| 1) +
            self.health_check_node_port.as_ref().map_or(0, |_| 1) +
            usize::from(!self.ip_families.is_empty()) +
            self.ip_family_policy.as_ref().map_or(0, |_| 1) +
            self.load_balancer_ip.as_ref().map_or(0, |_| 1) +
            usize::from(!self.load_balancer_source_ranges.is_empty()) +
            usize::from(!self.ports.is_empty()) +
            self.publish_not_ready_addresses.as_ref().map_or(0, |_| 1) +
            usize::from(!self.selector.is_empty()) +
            self.session_affinity.as_ref().map_or(0, |_| 1) +
            self.session_affinity_config.as_ref().map_or(0, |_| 1) +
            usize::from(!self.topology_keys.is_empty()) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocate_load_balancer_node_ports {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocateLoadBalancerNodePorts", value)?;
        }
        if let Some(value) = &self.cluster_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterIP", value)?;
        }
        if !self.cluster_i_ps.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterIPs", &self.cluster_i_ps)?;
        }
        if !self.external_ips.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalIPs", &self.external_ips)?;
        }
        if let Some(value) = &self.external_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalName", value)?;
        }
        if let Some(value) = &self.external_traffic_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalTrafficPolicy", value)?;
        }
        if let Some(value) = &self.health_check_node_port {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "healthCheckNodePort", value)?;
        }
        if !self.ip_families.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipFamilies", &self.ip_families)?;
        }
        if let Some(value) = &self.ip_family_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipFamilyPolicy", value)?;
        }
        if let Some(value) = &self.load_balancer_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancerIP", value)?;
        }
        if !self.load_balancer_source_ranges.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancerSourceRanges", &self.load_balancer_source_ranges)?;
        }
        if !self.ports.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", &self.ports)?;
        }
        if let Some(value) = &self.publish_not_ready_addresses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "publishNotReadyAddresses", value)?;
        }
        if !self.selector.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", &self.selector)?;
        }
        if let Some(value) = &self.session_affinity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sessionAffinity", value)?;
        }
        if let Some(value) = &self.session_affinity_config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sessionAffinityConfig", value)?;
        }
        if !self.topology_keys.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "topologyKeys", &self.topology_keys)?;
        }
        if let Some(value) = &self.type_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ServiceSpec {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ServiceSpec describes the attributes that a user creates on a service.",
          "properties": {
            "allocateLoadBalancerNodePorts": {
              "description": "allocateLoadBalancerNodePorts defines if NodePorts will be automatically allocated for services with type LoadBalancer.  Default is \"true\". It may be set to \"false\" if the cluster load-balancer does not rely on NodePorts. allocateLoadBalancerNodePorts may only be set for services with type LoadBalancer and will be cleared if the type is changed to any other type. This field is alpha-level and is only honored by servers that enable the ServiceLBNodePortControl feature.",
              "type": "boolean"
            },
            "clusterIP": {
              "description": "clusterIP is the IP address of the service and is usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be blank) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are \"None\", empty string (\"\"), or a valid IP address. Setting this to \"None\" makes a \"headless service\" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
              "type": "string"
            },
            "clusterIPs": {
              "description": "ClusterIPs is a list of IP addresses assigned to this service, and are usually assigned randomly.  If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be empty) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are \"None\", empty string (\"\"), or a valid IP address.  Setting this to \"None\" makes a \"headless service\" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName.  If this field is not specified, it will be initialized from the clusterIP field.  If this field is specified, clients must ensure that clusterIPs[0] and clusterIP have the same value.\n\nUnless the \"IPv6DualStack\" feature gate is enabled, this field is limited to one value, which must be the same as the clusterIP field.  If the feature gate is enabled, this field may hold a maximum of two entries (dual-stack IPs, in either order).  These IPs must correspond to the values of the ipFamilies field. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "externalIPs": {
              "description": "externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "externalName": {
              "description": "externalName is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved.  Must be a lowercase RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires Type to be",
              "type": "string"
            },
            "externalTrafficPolicy": {
              "description": "externalTrafficPolicy denotes if this Service desires to route external traffic to node-local or cluster-wide endpoints. \"Local\" preserves the client source IP and avoids a second hop for LoadBalancer and Nodeport type services, but risks potentially imbalanced traffic spreading. \"Cluster\" obscures the client source IP and may cause a second hop to another node, but should have good overall load-spreading.",
              "type": "string"
            },
            "healthCheckNodePort": {
              "description": "healthCheckNodePort specifies the healthcheck nodePort for the service. This only applies when type is set to LoadBalancer and externalTrafficPolicy is set to Local. If a value is specified, is in-range, and is not in use, it will be used.  If not specified, a value will be automatically allocated.  External systems (e.g. load-balancers) can use this port to determine if a given node holds endpoints for this service or not.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type).",
              "format": "int32",
              "type": "integer"
            },
            "ipFamilies": {
              "description": "IPFamilies is a list of IP families (e.g. IPv4, IPv6) assigned to this service, and is gated by the \"IPv6DualStack\" feature gate.  This field is usually assigned automatically based on cluster configuration and the ipFamilyPolicy field. If this field is specified manually, the requested family is available in the cluster, and ipFamilyPolicy allows it, it will be used; otherwise creation of the service will fail.  This field is conditionally mutable: it allows for adding or removing a secondary IP family, but it does not allow changing the primary IP family of the Service.  Valid values are \"IPv4\" and \"IPv6\".  This field only applies to Services of types ClusterIP, NodePort, and LoadBalancer, and does apply to \"headless\" services.  This field will be wiped when updating a Service to type ExternalName.\n\nThis field may hold a maximum of two entries (dual-stack families, in either order).  These families must correspond to the values of the clusterIPs field, if specified. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "ipFamilyPolicy": {
              "description": "IPFamilyPolicy represents the dual-stack-ness requested or required by this Service, and is gated by the \"IPv6DualStack\" feature gate.  If there is no value provided, then this field will be set to SingleStack. Services can be \"SingleStack\" (a single IP family), \"PreferDualStack\" (two IP families on dual-stack configured clusters or a single IP family on single-stack clusters), or \"RequireDualStack\" (two IP families on dual-stack configured clusters, otherwise fail). The ipFamilies and clusterIPs fields depend on the value of this field.  This field will be wiped when updating a service to type ExternalName.",
              "type": "string"
            },
            "loadBalancerIP": {
              "description": "Only applies to Service Type: LoadBalancer LoadBalancer will get created with the IP specified in this field. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature.",
              "type": "string"
            },
            "loadBalancerSourceRanges": {
              "description": "If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature.\" More info: https://kubernetes.io/docs/tasks/access-application-cluster/configure-cloud-provider-firewall/",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "ports": {
              "description": "The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
              "items": crate::api::core::v1::ServicePort::schema(),
              "type": "array"
            },
            "publishNotReadyAddresses": {
              "description": "publishNotReadyAddresses indicates that any agent which deals with endpoints for this Service should disregard any indications of ready/not-ready. The primary use case for setting this field is for a StatefulSet's Headless Service to propagate SRV DNS records for its Pods for the purpose of peer discovery. The Kubernetes controllers that generate Endpoints and EndpointSlice resources for Services interpret this to mean that all endpoints are considered \"ready\" even if the Pods themselves are not. Agents which consume only Kubernetes generated endpoints through the Endpoints or EndpointSlice resources can safely assume this behavior.",
              "type": "boolean"
            },
            "selector": {
              "additionalProperties": {
                "type": "string"
              },
              "description": "Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/",
              "type": "object"
            },
            "sessionAffinity": {
              "description": "Supports \"ClientIP\" and \"None\". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies",
              "type": "string"
            },
            "sessionAffinityConfig": crate::schema_ref_with_description(crate::api::core::v1::SessionAffinityConfig::schema(), "sessionAffinityConfig contains the configurations of session affinity."),
            "topologyKeys": {
              "description": "topologyKeys is a preference-order list of topology keys which implementations of services should use to preferentially sort endpoints when accessing this Service, it can not be used at the same time as externalTrafficPolicy=Local. Topology keys must be valid label keys and at most 16 keys may be specified. Endpoints are chosen based on the first topology key with available backends. If this field is specified and all entries have no backends that match the topology of the client, the service has no backends for that client and connections should fail. The special value \"*\" may be used to mean \"any topology\". This catch-all value, if used, only makes sense as the last value in the list. If this is not specified or empty, no topology constraints will be applied. This field is alpha-level and is only honored by servers that enable the ServiceTopology feature.",
              "items": {
                "type": "string"
              },
              "type": "array"
            },
            "type": {
              "description": "type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. \"ClusterIP\" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object or EndpointSlice objects. If clusterIP is \"None\", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a virtual IP. \"NodePort\" builds on ClusterIP and allocates a port on every node which routes to the same endpoints as the clusterIP. \"LoadBalancer\" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the same endpoints as the clusterIP. \"ExternalName\" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
