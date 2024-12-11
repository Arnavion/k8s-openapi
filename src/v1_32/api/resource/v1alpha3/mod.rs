
mod allocated_device_status;
pub use self::allocated_device_status::AllocatedDeviceStatus;

mod allocation_result;
pub use self::allocation_result::AllocationResult;

mod basic_device;
pub use self::basic_device::BasicDevice;

mod cel_device_selector;
pub use self::cel_device_selector::CELDeviceSelector;

mod device;
pub use self::device::Device;

mod device_allocation_configuration;
pub use self::device_allocation_configuration::DeviceAllocationConfiguration;

mod device_allocation_result;
pub use self::device_allocation_result::DeviceAllocationResult;

mod device_attribute;
pub use self::device_attribute::DeviceAttribute;

mod device_claim;
pub use self::device_claim::DeviceClaim;

mod device_claim_configuration;
pub use self::device_claim_configuration::DeviceClaimConfiguration;

mod device_class;
pub use self::device_class::DeviceClass;

mod device_class_configuration;
pub use self::device_class_configuration::DeviceClassConfiguration;

mod device_class_spec;
pub use self::device_class_spec::DeviceClassSpec;

mod device_constraint;
pub use self::device_constraint::DeviceConstraint;

mod device_request;
pub use self::device_request::DeviceRequest;

mod device_request_allocation_result;
pub use self::device_request_allocation_result::DeviceRequestAllocationResult;

mod device_selector;
pub use self::device_selector::DeviceSelector;

mod network_device_data;
pub use self::network_device_data::NetworkDeviceData;

mod opaque_device_configuration;
pub use self::opaque_device_configuration::OpaqueDeviceConfiguration;

mod resource_claim;
pub use self::resource_claim::ResourceClaim;

mod resource_claim_consumer_reference;
pub use self::resource_claim_consumer_reference::ResourceClaimConsumerReference;

mod resource_claim_spec;
pub use self::resource_claim_spec::ResourceClaimSpec;

mod resource_claim_status;
pub use self::resource_claim_status::ResourceClaimStatus;

mod resource_claim_template;
pub use self::resource_claim_template::ResourceClaimTemplate;

mod resource_claim_template_spec;
pub use self::resource_claim_template_spec::ResourceClaimTemplateSpec;

mod resource_pool;
pub use self::resource_pool::ResourcePool;

mod resource_slice;
pub use self::resource_slice::ResourceSlice;

mod resource_slice_spec;
pub use self::resource_slice_spec::ResourceSliceSpec;
