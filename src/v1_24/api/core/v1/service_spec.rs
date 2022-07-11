// Generated from definition io.k8s.api.core.v1.ServiceSpec

/// ServiceSpec describes the attributes that a user creates on a service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServiceSpec {
    /// allocateLoadBalancerNodePorts defines if NodePorts will be automatically allocated for services with type LoadBalancer.  Default is "true". It may be set to "false" if the cluster load-balancer does not rely on NodePorts.  If the caller requests specific NodePorts (by specifying a value), those requests will be respected, regardless of this field. This field may only be set for services with type LoadBalancer and will be cleared if the type is changed to any other type.
    pub allocate_load_balancer_node_ports: Option<bool>,

    /// clusterIP is the IP address of the service and is usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be blank) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address. Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub cluster_ip: Option<String>,

    /// ClusterIPs is a list of IP addresses assigned to this service, and are usually assigned randomly.  If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be empty) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address.  Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName.  If this field is not specified, it will be initialized from the clusterIP field.  If this field is specified, clients must ensure that clusterIPs\[0\] and clusterIP have the same value.
    ///
    /// This field may hold a maximum of two entries (dual-stack IPs, in either order). These IPs must correspond to the values of the ipFamilies field. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub cluster_ips: Option<Vec<String>>,

    /// externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.
    pub external_ips: Option<Vec<String>>,

    /// externalName is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved.  Must be a lowercase RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires `type` to be "ExternalName".
    pub external_name: Option<String>,

    /// externalTrafficPolicy denotes if this Service desires to route external traffic to node-local or cluster-wide endpoints. "Local" preserves the client source IP and avoids a second hop for LoadBalancer and Nodeport type services, but risks potentially imbalanced traffic spreading. "Cluster" obscures the client source IP and may cause a second hop to another node, but should have good overall load-spreading.
    ///
    pub external_traffic_policy: Option<String>,

    /// healthCheckNodePort specifies the healthcheck nodePort for the service. This only applies when type is set to LoadBalancer and externalTrafficPolicy is set to Local. If a value is specified, is in-range, and is not in use, it will be used.  If not specified, a value will be automatically allocated.  External systems (e.g. load-balancers) can use this port to determine if a given node holds endpoints for this service or not.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type).
    pub health_check_node_port: Option<i32>,

    /// InternalTrafficPolicy specifies if the cluster internal traffic should be routed to all endpoints or node-local endpoints only. "Cluster" routes internal traffic to a Service to all endpoints. "Local" routes traffic to node-local endpoints only, traffic is dropped if no node-local endpoints are ready. The default value is "Cluster".
    pub internal_traffic_policy: Option<String>,

    /// IPFamilies is a list of IP families (e.g. IPv4, IPv6) assigned to this service. This field is usually assigned automatically based on cluster configuration and the ipFamilyPolicy field. If this field is specified manually, the requested family is available in the cluster, and ipFamilyPolicy allows it, it will be used; otherwise creation of the service will fail. This field is conditionally mutable: it allows for adding or removing a secondary IP family, but it does not allow changing the primary IP family of the Service. Valid values are "IPv4" and "IPv6".  This field only applies to Services of types ClusterIP, NodePort, and LoadBalancer, and does apply to "headless" services. This field will be wiped when updating a Service to type ExternalName.
    ///
    /// This field may hold a maximum of two entries (dual-stack families, in either order).  These families must correspond to the values of the clusterIPs field, if specified. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field.
    pub ip_families: Option<Vec<String>>,

    /// IPFamilyPolicy represents the dual-stack-ness requested or required by this Service. If there is no value provided, then this field will be set to SingleStack. Services can be "SingleStack" (a single IP family), "PreferDualStack" (two IP families on dual-stack configured clusters or a single IP family on single-stack clusters), or "RequireDualStack" (two IP families on dual-stack configured clusters, otherwise fail). The ipFamilies and clusterIPs fields depend on the value of this field. This field will be wiped when updating a service to type ExternalName.
    pub ip_family_policy: Option<String>,

    /// loadBalancerClass is the class of the load balancer implementation this Service belongs to. If specified, the value of this field must be a label-style identifier, with an optional prefix, e.g. "internal-vip" or "example.com/internal-vip". Unprefixed names are reserved for end-users. This field can only be set when the Service type is 'LoadBalancer'. If not set, the default load balancer implementation is used, today this is typically done through the cloud provider integration, but should apply for any default implementation. If set, it is assumed that a load balancer implementation is watching for Services with a matching class. Any default load balancer implementation (e.g. cloud providers) should ignore Services that set this field. This field can only be set when creating or updating a Service to type 'LoadBalancer'. Once set, it can not be changed. This field will be wiped when a service is updated to a non 'LoadBalancer' type.
    pub load_balancer_class: Option<String>,

    /// Only applies to Service Type: LoadBalancer. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature. Deprecated: This field was under-specified and its meaning varies across implementations, and it cannot support dual-stack. As of Kubernetes v1.24, users are encouraged to use implementation-specific annotations when available. This field may be removed in a future API version.
    pub load_balancer_ip: Option<String>,

    /// If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature." More info: https://kubernetes.io/docs/tasks/access-application-cluster/create-external-load-balancer/
    pub load_balancer_source_ranges: Option<Vec<String>>,

    /// The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    pub ports: Option<Vec<crate::api::core::v1::ServicePort>>,

    /// publishNotReadyAddresses indicates that any agent which deals with endpoints for this Service should disregard any indications of ready/not-ready. The primary use case for setting this field is for a StatefulSet's Headless Service to propagate SRV DNS records for its Pods for the purpose of peer discovery. The Kubernetes controllers that generate Endpoints and EndpointSlice resources for Services interpret this to mean that all endpoints are considered "ready" even if the Pods themselves are not. Agents which consume only Kubernetes generated endpoints through the Endpoints or EndpointSlice resources can safely assume this behavior.
    pub publish_not_ready_addresses: Option<bool>,

    /// Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/
    pub selector: Option<std::collections::BTreeMap<String, String>>,

    /// Supports "ClientIP" and "None". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    ///
    pub session_affinity: Option<String>,

    /// sessionAffinityConfig contains the configurations of session affinity.
    pub session_affinity_config: Option<crate::api::core::v1::SessionAffinityConfig>,

    /// type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. "ClusterIP" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object or EndpointSlice objects. If clusterIP is "None", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a virtual IP. "NodePort" builds on ClusterIP and allocates a port on every node which routes to the same endpoints as the clusterIP. "LoadBalancer" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the same endpoints as the clusterIP. "ExternalName" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types
    ///
    pub type_: Option<String>,

}

#[cfg(feature = "dsl")]
impl ServiceSpec  {
    /// Set [`Self::allocate_load_balancer_node_ports`]
    pub  fn allocate_load_balancer_node_ports_set(&mut self, allocate_load_balancer_node_ports: impl Into<Option<bool>>) -> &mut Self {
        self.allocate_load_balancer_node_ports = allocate_load_balancer_node_ports.into(); self
    }

    pub  fn allocate_load_balancer_node_ports(&mut self) -> &mut bool {
        if self.allocate_load_balancer_node_ports.is_none() { self.allocate_load_balancer_node_ports = Some(Default::default()) }
        self.allocate_load_balancer_node_ports.as_mut().unwrap()
    }

    /// Modify [`Self::allocate_load_balancer_node_ports`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn allocate_load_balancer_node_ports_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.allocate_load_balancer_node_ports.is_none() { self.allocate_load_balancer_node_ports = Some(Default::default()) };
        func(self.allocate_load_balancer_node_ports.as_mut().unwrap()); self
    }


    /// Set [`Self::cluster_ip`]
    pub  fn cluster_ip_set(&mut self, cluster_ip: impl Into<Option<String>>) -> &mut Self {
        self.cluster_ip = cluster_ip.into(); self
    }

    pub  fn cluster_ip(&mut self) -> &mut String {
        if self.cluster_ip.is_none() { self.cluster_ip = Some(Default::default()) }
        self.cluster_ip.as_mut().unwrap()
    }

    /// Modify [`Self::cluster_ip`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn cluster_ip_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.cluster_ip.is_none() { self.cluster_ip = Some(Default::default()) };
        func(self.cluster_ip.as_mut().unwrap()); self
    }


    /// Set [`Self::cluster_ips`]
    pub  fn cluster_ips_set(&mut self, cluster_ips: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.cluster_ips = cluster_ips.into(); self
    }

    pub  fn cluster_ips(&mut self) -> &mut Vec<String> {
        if self.cluster_ips.is_none() { self.cluster_ips = Some(Default::default()) }
        self.cluster_ips.as_mut().unwrap()
    }

    /// Modify [`Self::cluster_ips`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn cluster_ips_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.cluster_ips.is_none() { self.cluster_ips = Some(Default::default()) };
        func(self.cluster_ips.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::cluster_ips`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn cluster_ips_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.cluster_ips.is_none() {
            self.cluster_ips = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.cluster_ips.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::cluster_ips`]
    pub  fn cluster_ips_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.cluster_ips.is_none() { self.cluster_ips = Some(Vec::new()); }
         let cluster_ips = &mut self.cluster_ips.as_mut().unwrap();
         for item in other.borrow() {
             cluster_ips.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::external_ips`]
    pub  fn external_ips_set(&mut self, external_ips: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.external_ips = external_ips.into(); self
    }

    pub  fn external_ips(&mut self) -> &mut Vec<String> {
        if self.external_ips.is_none() { self.external_ips = Some(Default::default()) }
        self.external_ips.as_mut().unwrap()
    }

    /// Modify [`Self::external_ips`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn external_ips_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.external_ips.is_none() { self.external_ips = Some(Default::default()) };
        func(self.external_ips.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::external_ips`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn external_ips_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.external_ips.is_none() {
            self.external_ips = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.external_ips.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::external_ips`]
    pub  fn external_ips_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.external_ips.is_none() { self.external_ips = Some(Vec::new()); }
         let external_ips = &mut self.external_ips.as_mut().unwrap();
         for item in other.borrow() {
             external_ips.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::external_name`]
    pub  fn external_name_set(&mut self, external_name: impl Into<Option<String>>) -> &mut Self {
        self.external_name = external_name.into(); self
    }

    pub  fn external_name(&mut self) -> &mut String {
        if self.external_name.is_none() { self.external_name = Some(Default::default()) }
        self.external_name.as_mut().unwrap()
    }

    /// Modify [`Self::external_name`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn external_name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.external_name.is_none() { self.external_name = Some(Default::default()) };
        func(self.external_name.as_mut().unwrap()); self
    }


    /// Set [`Self::external_traffic_policy`]
    pub  fn external_traffic_policy_set(&mut self, external_traffic_policy: impl Into<Option<String>>) -> &mut Self {
        self.external_traffic_policy = external_traffic_policy.into(); self
    }

    pub  fn external_traffic_policy(&mut self) -> &mut String {
        if self.external_traffic_policy.is_none() { self.external_traffic_policy = Some(Default::default()) }
        self.external_traffic_policy.as_mut().unwrap()
    }

    /// Modify [`Self::external_traffic_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn external_traffic_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.external_traffic_policy.is_none() { self.external_traffic_policy = Some(Default::default()) };
        func(self.external_traffic_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::health_check_node_port`]
    pub  fn health_check_node_port_set(&mut self, health_check_node_port: impl Into<Option<i32>>) -> &mut Self {
        self.health_check_node_port = health_check_node_port.into(); self
    }

    pub  fn health_check_node_port(&mut self) -> &mut i32 {
        if self.health_check_node_port.is_none() { self.health_check_node_port = Some(Default::default()) }
        self.health_check_node_port.as_mut().unwrap()
    }

    /// Modify [`Self::health_check_node_port`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn health_check_node_port_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.health_check_node_port.is_none() { self.health_check_node_port = Some(Default::default()) };
        func(self.health_check_node_port.as_mut().unwrap()); self
    }


    /// Set [`Self::internal_traffic_policy`]
    pub  fn internal_traffic_policy_set(&mut self, internal_traffic_policy: impl Into<Option<String>>) -> &mut Self {
        self.internal_traffic_policy = internal_traffic_policy.into(); self
    }

    pub  fn internal_traffic_policy(&mut self) -> &mut String {
        if self.internal_traffic_policy.is_none() { self.internal_traffic_policy = Some(Default::default()) }
        self.internal_traffic_policy.as_mut().unwrap()
    }

    /// Modify [`Self::internal_traffic_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn internal_traffic_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.internal_traffic_policy.is_none() { self.internal_traffic_policy = Some(Default::default()) };
        func(self.internal_traffic_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::ip_families`]
    pub  fn ip_families_set(&mut self, ip_families: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.ip_families = ip_families.into(); self
    }

    pub  fn ip_families(&mut self) -> &mut Vec<String> {
        if self.ip_families.is_none() { self.ip_families = Some(Default::default()) }
        self.ip_families.as_mut().unwrap()
    }

    /// Modify [`Self::ip_families`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ip_families_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.ip_families.is_none() { self.ip_families = Some(Default::default()) };
        func(self.ip_families.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::ip_families`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn ip_families_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.ip_families.is_none() {
            self.ip_families = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.ip_families.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::ip_families`]
    pub  fn ip_families_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.ip_families.is_none() { self.ip_families = Some(Vec::new()); }
         let ip_families = &mut self.ip_families.as_mut().unwrap();
         for item in other.borrow() {
             ip_families.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::ip_family_policy`]
    pub  fn ip_family_policy_set(&mut self, ip_family_policy: impl Into<Option<String>>) -> &mut Self {
        self.ip_family_policy = ip_family_policy.into(); self
    }

    pub  fn ip_family_policy(&mut self) -> &mut String {
        if self.ip_family_policy.is_none() { self.ip_family_policy = Some(Default::default()) }
        self.ip_family_policy.as_mut().unwrap()
    }

    /// Modify [`Self::ip_family_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ip_family_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.ip_family_policy.is_none() { self.ip_family_policy = Some(Default::default()) };
        func(self.ip_family_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::load_balancer_class`]
    pub  fn load_balancer_class_set(&mut self, load_balancer_class: impl Into<Option<String>>) -> &mut Self {
        self.load_balancer_class = load_balancer_class.into(); self
    }

    pub  fn load_balancer_class(&mut self) -> &mut String {
        if self.load_balancer_class.is_none() { self.load_balancer_class = Some(Default::default()) }
        self.load_balancer_class.as_mut().unwrap()
    }

    /// Modify [`Self::load_balancer_class`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn load_balancer_class_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.load_balancer_class.is_none() { self.load_balancer_class = Some(Default::default()) };
        func(self.load_balancer_class.as_mut().unwrap()); self
    }


    /// Set [`Self::load_balancer_ip`]
    pub  fn load_balancer_ip_set(&mut self, load_balancer_ip: impl Into<Option<String>>) -> &mut Self {
        self.load_balancer_ip = load_balancer_ip.into(); self
    }

    pub  fn load_balancer_ip(&mut self) -> &mut String {
        if self.load_balancer_ip.is_none() { self.load_balancer_ip = Some(Default::default()) }
        self.load_balancer_ip.as_mut().unwrap()
    }

    /// Modify [`Self::load_balancer_ip`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn load_balancer_ip_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.load_balancer_ip.is_none() { self.load_balancer_ip = Some(Default::default()) };
        func(self.load_balancer_ip.as_mut().unwrap()); self
    }


    /// Set [`Self::load_balancer_source_ranges`]
    pub  fn load_balancer_source_ranges_set(&mut self, load_balancer_source_ranges: impl Into<Option<Vec<String>>>) -> &mut Self {
        self.load_balancer_source_ranges = load_balancer_source_ranges.into(); self
    }

    pub  fn load_balancer_source_ranges(&mut self) -> &mut Vec<String> {
        if self.load_balancer_source_ranges.is_none() { self.load_balancer_source_ranges = Some(Default::default()) }
        self.load_balancer_source_ranges.as_mut().unwrap()
    }

    /// Modify [`Self::load_balancer_source_ranges`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn load_balancer_source_ranges_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        if self.load_balancer_source_ranges.is_none() { self.load_balancer_source_ranges = Some(Default::default()) };
        func(self.load_balancer_source_ranges.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::load_balancer_source_ranges`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn load_balancer_source_ranges_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.load_balancer_source_ranges.is_none() {
            self.load_balancer_source_ranges = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.load_balancer_source_ranges.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::load_balancer_source_ranges`]
    pub  fn load_balancer_source_ranges_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         if self.load_balancer_source_ranges.is_none() { self.load_balancer_source_ranges = Some(Vec::new()); }
         let load_balancer_source_ranges = &mut self.load_balancer_source_ranges.as_mut().unwrap();
         for item in other.borrow() {
             load_balancer_source_ranges.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::ports`]
    pub  fn ports_set(&mut self, ports: impl Into<Option<Vec<crate::api::core::v1::ServicePort>>>) -> &mut Self {
        self.ports = ports.into(); self
    }

    pub  fn ports(&mut self) -> &mut Vec<crate::api::core::v1::ServicePort> {
        if self.ports.is_none() { self.ports = Some(Default::default()) }
        self.ports.as_mut().unwrap()
    }

    /// Modify [`Self::ports`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn ports_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::core::v1::ServicePort>)) -> &mut Self {
        if self.ports.is_none() { self.ports = Some(Default::default()) };
        func(self.ports.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::ports`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn ports_push_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::ServicePort)) -> &mut Self {
        if self.ports.is_none() {
            self.ports = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.ports.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::ports`]
    pub  fn ports_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::core::v1::ServicePort]>) -> &mut Self {
         if self.ports.is_none() { self.ports = Some(Vec::new()); }
         let ports = &mut self.ports.as_mut().unwrap();
         for item in other.borrow() {
             ports.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::publish_not_ready_addresses`]
    pub  fn publish_not_ready_addresses_set(&mut self, publish_not_ready_addresses: impl Into<Option<bool>>) -> &mut Self {
        self.publish_not_ready_addresses = publish_not_ready_addresses.into(); self
    }

    pub  fn publish_not_ready_addresses(&mut self) -> &mut bool {
        if self.publish_not_ready_addresses.is_none() { self.publish_not_ready_addresses = Some(Default::default()) }
        self.publish_not_ready_addresses.as_mut().unwrap()
    }

    /// Modify [`Self::publish_not_ready_addresses`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn publish_not_ready_addresses_with(&mut self, func: impl FnOnce(&mut bool)) -> &mut Self {
        if self.publish_not_ready_addresses.is_none() { self.publish_not_ready_addresses = Some(Default::default()) };
        func(self.publish_not_ready_addresses.as_mut().unwrap()); self
    }


    /// Set [`Self::selector`]
    pub  fn selector_set(&mut self, selector: impl Into<Option<std::collections::BTreeMap<String, String>>>) -> &mut Self {
        self.selector = selector.into(); self
    }

    pub  fn selector(&mut self) -> &mut std::collections::BTreeMap<String, String> {
        if self.selector.is_none() { self.selector = Some(Default::default()) }
        self.selector.as_mut().unwrap()
    }

    /// Modify [`Self::selector`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn selector_with(&mut self, func: impl FnOnce(&mut std::collections::BTreeMap<String, String>)) -> &mut Self {
        if self.selector.is_none() { self.selector = Some(Default::default()) };
        func(self.selector.as_mut().unwrap()); self
    }

    /// Insert a new element to [`Self::selector`] and modify with a `func`
    ///
    /// The field will be overwritten or set to `Default::default()` if not set before 
    pub  fn selector_insert_with(&mut self, name: &str, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.selector.is_none() {
            self.selector = Some(std::collections::BTreeMap::new());
        }
        let mut new = Default::default();
        func(&mut new);
        self.selector.as_mut().unwrap().insert(name.to_owned(), new);
        self
    }

    /// Insert all elements from `other` into [`Self::selector`]
    pub  fn selector_insert_from(&mut self, other: impl std::borrow::Borrow<std::collections::BTreeMap<String, String>>) -> &mut Self {
         if self.selector.is_none() { self.selector = Some(std::collections::BTreeMap::new()); }
         let selector = &mut self.selector.as_mut().unwrap();
         for (name, value) in other.borrow() {
             selector.insert(name.to_owned(), value.to_owned());
         }
         self
    }


    /// Set [`Self::session_affinity`]
    pub  fn session_affinity_set(&mut self, session_affinity: impl Into<Option<String>>) -> &mut Self {
        self.session_affinity = session_affinity.into(); self
    }

    pub  fn session_affinity(&mut self) -> &mut String {
        if self.session_affinity.is_none() { self.session_affinity = Some(Default::default()) }
        self.session_affinity.as_mut().unwrap()
    }

    /// Modify [`Self::session_affinity`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn session_affinity_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.session_affinity.is_none() { self.session_affinity = Some(Default::default()) };
        func(self.session_affinity.as_mut().unwrap()); self
    }


    /// Set [`Self::session_affinity_config`]
    pub  fn session_affinity_config_set(&mut self, session_affinity_config: impl Into<Option<crate::api::core::v1::SessionAffinityConfig>>) -> &mut Self {
        self.session_affinity_config = session_affinity_config.into(); self
    }

    pub  fn session_affinity_config(&mut self) -> &mut crate::api::core::v1::SessionAffinityConfig {
        if self.session_affinity_config.is_none() { self.session_affinity_config = Some(Default::default()) }
        self.session_affinity_config.as_mut().unwrap()
    }

    /// Modify [`Self::session_affinity_config`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn session_affinity_config_with(&mut self, func: impl FnOnce(&mut crate::api::core::v1::SessionAffinityConfig)) -> &mut Self {
        if self.session_affinity_config.is_none() { self.session_affinity_config = Some(Default::default()) };
        func(self.session_affinity_config.as_mut().unwrap()); self
    }


    /// Set [`Self::type_`]
    pub  fn type_set(&mut self, type_: impl Into<Option<String>>) -> &mut Self {
        self.type_ = type_.into(); self
    }

    pub  fn type_(&mut self) -> &mut String {
        if self.type_.is_none() { self.type_ = Some(Default::default()) }
        self.type_.as_mut().unwrap()
    }

    /// Modify [`Self::type_`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn type_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.type_.is_none() { self.type_ = Some(Default::default()) };
        func(self.type_.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for ServiceSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allocate_load_balancer_node_ports,
            Key_cluster_ip,
            Key_cluster_ips,
            Key_external_ips,
            Key_external_name,
            Key_external_traffic_policy,
            Key_health_check_node_port,
            Key_internal_traffic_policy,
            Key_ip_families,
            Key_ip_family_policy,
            Key_load_balancer_class,
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
                            "clusterIPs" => Field::Key_cluster_ips,
                            "externalIPs" => Field::Key_external_ips,
                            "externalName" => Field::Key_external_name,
                            "externalTrafficPolicy" => Field::Key_external_traffic_policy,
                            "healthCheckNodePort" => Field::Key_health_check_node_port,
                            "internalTrafficPolicy" => Field::Key_internal_traffic_policy,
                            "ipFamilies" => Field::Key_ip_families,
                            "ipFamilyPolicy" => Field::Key_ip_family_policy,
                            "loadBalancerClass" => Field::Key_load_balancer_class,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ServiceSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allocate_load_balancer_node_ports: Option<bool> = None;
                let mut value_cluster_ip: Option<String> = None;
                let mut value_cluster_ips: Option<Vec<String>> = None;
                let mut value_external_ips: Option<Vec<String>> = None;
                let mut value_external_name: Option<String> = None;
                let mut value_external_traffic_policy: Option<String> = None;
                let mut value_health_check_node_port: Option<i32> = None;
                let mut value_internal_traffic_policy: Option<String> = None;
                let mut value_ip_families: Option<Vec<String>> = None;
                let mut value_ip_family_policy: Option<String> = None;
                let mut value_load_balancer_class: Option<String> = None;
                let mut value_load_balancer_ip: Option<String> = None;
                let mut value_load_balancer_source_ranges: Option<Vec<String>> = None;
                let mut value_ports: Option<Vec<crate::api::core::v1::ServicePort>> = None;
                let mut value_publish_not_ready_addresses: Option<bool> = None;
                let mut value_selector: Option<std::collections::BTreeMap<String, String>> = None;
                let mut value_session_affinity: Option<String> = None;
                let mut value_session_affinity_config: Option<crate::api::core::v1::SessionAffinityConfig> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allocate_load_balancer_node_ports => value_allocate_load_balancer_node_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cluster_ip => value_cluster_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_cluster_ips => value_cluster_ips = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_ips => value_external_ips = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_name => value_external_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_traffic_policy => value_external_traffic_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_health_check_node_port => value_health_check_node_port = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_internal_traffic_policy => value_internal_traffic_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_families => value_ip_families = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_family_policy => value_ip_family_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer_class => value_load_balancer_class = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer_ip => value_load_balancer_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_load_balancer_source_ranges => value_load_balancer_source_ranges = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ports => value_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_publish_not_ready_addresses => value_publish_not_ready_addresses = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_session_affinity => value_session_affinity = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_session_affinity_config => value_session_affinity_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ServiceSpec {
                    allocate_load_balancer_node_ports: value_allocate_load_balancer_node_ports,
                    cluster_ip: value_cluster_ip,
                    cluster_ips: value_cluster_ips,
                    external_ips: value_external_ips,
                    external_name: value_external_name,
                    external_traffic_policy: value_external_traffic_policy,
                    health_check_node_port: value_health_check_node_port,
                    internal_traffic_policy: value_internal_traffic_policy,
                    ip_families: value_ip_families,
                    ip_family_policy: value_ip_family_policy,
                    load_balancer_class: value_load_balancer_class,
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
                "allocateLoadBalancerNodePorts",
                "clusterIP",
                "clusterIPs",
                "externalIPs",
                "externalName",
                "externalTrafficPolicy",
                "healthCheckNodePort",
                "internalTrafficPolicy",
                "ipFamilies",
                "ipFamilyPolicy",
                "loadBalancerClass",
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

impl crate::serde::Serialize for ServiceSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ServiceSpec",
            self.allocate_load_balancer_node_ports.as_ref().map_or(0, |_| 1) +
            self.cluster_ip.as_ref().map_or(0, |_| 1) +
            self.cluster_ips.as_ref().map_or(0, |_| 1) +
            self.external_ips.as_ref().map_or(0, |_| 1) +
            self.external_name.as_ref().map_or(0, |_| 1) +
            self.external_traffic_policy.as_ref().map_or(0, |_| 1) +
            self.health_check_node_port.as_ref().map_or(0, |_| 1) +
            self.internal_traffic_policy.as_ref().map_or(0, |_| 1) +
            self.ip_families.as_ref().map_or(0, |_| 1) +
            self.ip_family_policy.as_ref().map_or(0, |_| 1) +
            self.load_balancer_class.as_ref().map_or(0, |_| 1) +
            self.load_balancer_ip.as_ref().map_or(0, |_| 1) +
            self.load_balancer_source_ranges.as_ref().map_or(0, |_| 1) +
            self.ports.as_ref().map_or(0, |_| 1) +
            self.publish_not_ready_addresses.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1) +
            self.session_affinity.as_ref().map_or(0, |_| 1) +
            self.session_affinity_config.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allocate_load_balancer_node_ports {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allocateLoadBalancerNodePorts", value)?;
        }
        if let Some(value) = &self.cluster_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterIP", value)?;
        }
        if let Some(value) = &self.cluster_ips {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clusterIPs", value)?;
        }
        if let Some(value) = &self.external_ips {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "externalIPs", value)?;
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
        if let Some(value) = &self.internal_traffic_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "internalTrafficPolicy", value)?;
        }
        if let Some(value) = &self.ip_families {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipFamilies", value)?;
        }
        if let Some(value) = &self.ip_family_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipFamilyPolicy", value)?;
        }
        if let Some(value) = &self.load_balancer_class {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancerClass", value)?;
        }
        if let Some(value) = &self.load_balancer_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancerIP", value)?;
        }
        if let Some(value) = &self.load_balancer_source_ranges {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "loadBalancerSourceRanges", value)?;
        }
        if let Some(value) = &self.ports {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", value)?;
        }
        if let Some(value) = &self.publish_not_ready_addresses {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "publishNotReadyAddresses", value)?;
        }
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        if let Some(value) = &self.session_affinity {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sessionAffinity", value)?;
        }
        if let Some(value) = &self.session_affinity_config {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sessionAffinityConfig", value)?;
        }
        if let Some(value) = &self.type_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ServiceSpec {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.ServiceSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ServiceSpec describes the attributes that a user creates on a service.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allocateLoadBalancerNodePorts".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("allocateLoadBalancerNodePorts defines if NodePorts will be automatically allocated for services with type LoadBalancer.  Default is \"true\". It may be set to \"false\" if the cluster load-balancer does not rely on NodePorts.  If the caller requests specific NodePorts (by specifying a value), those requests will be respected, regardless of this field. This field may only be set for services with type LoadBalancer and will be cleared if the type is changed to any other type.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "clusterIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("clusterIP is the IP address of the service and is usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be blank) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are \"None\", empty string (\"\"), or a valid IP address. Setting this to \"None\" makes a \"headless service\" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "clusterIPs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ClusterIPs is a list of IP addresses assigned to this service, and are usually assigned randomly.  If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be empty) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are \"None\", empty string (\"\"), or a valid IP address.  Setting this to \"None\" makes a \"headless service\" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName.  If this field is not specified, it will be initialized from the clusterIP field.  If this field is specified, clients must ensure that clusterIPs[0] and clusterIP have the same value.\n\nThis field may hold a maximum of two entries (dual-stack IPs, in either order). These IPs must correspond to the values of the ipFamilies field. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "externalIPs".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "externalName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("externalName is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved.  Must be a lowercase RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires `type` to be \"ExternalName\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "externalTrafficPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("externalTrafficPolicy denotes if this Service desires to route external traffic to node-local or cluster-wide endpoints. \"Local\" preserves the client source IP and avoids a second hop for LoadBalancer and Nodeport type services, but risks potentially imbalanced traffic spreading. \"Cluster\" obscures the client source IP and may cause a second hop to another node, but should have good overall load-spreading.\n\n".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "healthCheckNodePort".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("healthCheckNodePort specifies the healthcheck nodePort for the service. This only applies when type is set to LoadBalancer and externalTrafficPolicy is set to Local. If a value is specified, is in-range, and is not in use, it will be used.  If not specified, a value will be automatically allocated.  External systems (e.g. load-balancers) can use this port to determine if a given node holds endpoints for this service or not.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type).".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "internalTrafficPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("InternalTrafficPolicy specifies if the cluster internal traffic should be routed to all endpoints or node-local endpoints only. \"Cluster\" routes internal traffic to a Service to all endpoints. \"Local\" routes traffic to node-local endpoints only, traffic is dropped if no node-local endpoints are ready. The default value is \"Cluster\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ipFamilies".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPFamilies is a list of IP families (e.g. IPv4, IPv6) assigned to this service. This field is usually assigned automatically based on cluster configuration and the ipFamilyPolicy field. If this field is specified manually, the requested family is available in the cluster, and ipFamilyPolicy allows it, it will be used; otherwise creation of the service will fail. This field is conditionally mutable: it allows for adding or removing a secondary IP family, but it does not allow changing the primary IP family of the Service. Valid values are \"IPv4\" and \"IPv6\".  This field only applies to Services of types ClusterIP, NodePort, and LoadBalancer, and does apply to \"headless\" services. This field will be wiped when updating a Service to type ExternalName.\n\nThis field may hold a maximum of two entries (dual-stack families, in either order).  These families must correspond to the values of the clusterIPs field, if specified. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ipFamilyPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPFamilyPolicy represents the dual-stack-ness requested or required by this Service. If there is no value provided, then this field will be set to SingleStack. Services can be \"SingleStack\" (a single IP family), \"PreferDualStack\" (two IP families on dual-stack configured clusters or a single IP family on single-stack clusters), or \"RequireDualStack\" (two IP families on dual-stack configured clusters, otherwise fail). The ipFamilies and clusterIPs fields depend on the value of this field. This field will be wiped when updating a service to type ExternalName.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "loadBalancerClass".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("loadBalancerClass is the class of the load balancer implementation this Service belongs to. If specified, the value of this field must be a label-style identifier, with an optional prefix, e.g. \"internal-vip\" or \"example.com/internal-vip\". Unprefixed names are reserved for end-users. This field can only be set when the Service type is 'LoadBalancer'. If not set, the default load balancer implementation is used, today this is typically done through the cloud provider integration, but should apply for any default implementation. If set, it is assumed that a load balancer implementation is watching for Services with a matching class. Any default load balancer implementation (e.g. cloud providers) should ignore Services that set this field. This field can only be set when creating or updating a Service to type 'LoadBalancer'. Once set, it can not be changed. This field will be wiped when a service is updated to a non 'LoadBalancer' type.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "loadBalancerIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Only applies to Service Type: LoadBalancer. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature. Deprecated: This field was under-specified and its meaning varies across implementations, and it cannot support dual-stack. As of Kubernetes v1.24, users are encouraged to use implementation-specific annotations when available. This field may be removed in a future API version.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "loadBalancerSourceRanges".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature.\" More info: https://kubernetes.io/docs/tasks/access-application-cluster/create-external-load-balancer/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ports".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::core::v1::ServicePort>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "publishNotReadyAddresses".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("publishNotReadyAddresses indicates that any agent which deals with endpoints for this Service should disregard any indications of ready/not-ready. The primary use case for setting this field is for a StatefulSet's Headless Service to propagate SRV DNS records for its Pods for the purpose of peer discovery. The Kubernetes controllers that generate Endpoints and EndpointSlice resources for Services interpret this to mean that all endpoints are considered \"ready\" even if the Pods themselves are not. Agents which consume only Kubernetes generated endpoints through the Endpoints or EndpointSlice resources can safely assume this behavior.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selector".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
                            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                                additional_properties: Some(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                )),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "sessionAffinity".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Supports \"ClientIP\" and \"None\". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies\n\n".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "sessionAffinityConfig".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::SessionAffinityConfig>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("sessionAffinityConfig contains the configurations of session affinity.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. \"ClusterIP\" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object or EndpointSlice objects. If clusterIP is \"None\", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a virtual IP. \"NodePort\" builds on ClusterIP and allocates a port on every node which routes to the same endpoints as the clusterIP. \"LoadBalancer\" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the same endpoints as the clusterIP. \"ExternalName\" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types\n\n".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
