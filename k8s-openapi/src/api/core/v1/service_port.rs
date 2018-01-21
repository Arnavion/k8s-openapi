// Generated from definition io.k8s.api.core.v1.ServicePort

/// ServicePort contains information on service's port.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ServicePort {
    /// The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. This maps to the 'Name' field in EndpointPort objects. Optional if only one ServicePort is defined on this service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The port on each node on which this service is exposed when type=NodePort or LoadBalancer. Usually assigned by the system. If specified, it will be allocated to the service if unused or else creation of the service will fail. Default is to auto-allocate a port if the ServiceType of this Service requires one. More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport
    #[serde(rename = "nodePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i32>,

    /// The port that will be exposed by this service.
    pub port: i32,

    /// The IP protocol for this port. Supports "TCP" and "UDP". Default is TCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service
    #[serde(rename = "targetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_port: Option<::apimachinery::pkg::util::intstr::IntOrString>,
}
