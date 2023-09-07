initSidebarItems({"enum":[["Patch","Patch is provided to give a concrete name and type to the Kubernetes PATCH request body."],["WatchEvent","Event represents a single event to a watched resource."]],"struct":[["APIGroup","APIGroup contains the name, the supported versions, and the preferred version of a group."],["APIGroupList","APIGroupList is a list of APIGroup, to allow clients to discover the API at /apis."],["APIResource","APIResource specifies the name of a resource and whether it is namespaced."],["APIResourceList","APIResourceList is a list of APIResource, it is used to expose the name of the resources supported in a specific group and version, and if the resource is namespaced."],["APIVersions","APIVersions lists the versions that are available, to allow clients to discover the API at /api, which is the root path of the legacy v1 API."],["Condition","Condition contains details for one aspect of the current state of this API Resource."],["DeleteOptions","DeleteOptions may be provided when deleting an API object."],["FieldsV1","FieldsV1 stores a set of fields in a data structure like a Trie, in JSON format."],["GroupVersionForDiscovery","GroupVersion contains the \"group/version\" and \"version\" string of a version. It is made a struct to keep extensibility."],["LabelSelector","A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects."],["LabelSelectorRequirement","A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values."],["ListMeta","ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}."],["ManagedFieldsEntry","ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to."],["MicroTime","MicroTime is version of Time with microsecond level precision."],["ObjectMeta","ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create."],["OwnerReference","OwnerReference contains enough information to let you identify an owning object. An owning object must be in the same namespace as the dependent, or be cluster-scoped, so there is no namespace field."],["Preconditions","Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out."],["ServerAddressByClientCIDR","ServerAddressByClientCIDR helps the client to determine the server address that they should use, depending on the clientCIDR that they match."],["Status","Status is a return value for calls that don't return other objects."],["StatusCause","StatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered."],["StatusDetails","StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined."],["Time","Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers."]]});