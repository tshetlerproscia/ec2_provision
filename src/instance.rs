use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reservation {
    pub Groups: Vec<Group>,
    pub Instances: Vec<Instance>,
    pub OwnerId: String,
    pub ReservationId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EC2Output {
    pub Reservations: Vec<Reservation>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub GroupName: String,
    pub GroupId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instance {
    pub AmiLaunchIndex: u32,
    pub ImageId: String,
    pub InstanceId: String,
    pub InstanceType: String,
    pub KeyName: Option<String>,
    pub LaunchTime: String,
    pub Monitoring: Monitoring,
    pub Placement: Placement,
    pub PrivateDnsName: String,
    pub PrivateIpAddress: Option<String>,
    pub ProductCodes: Vec<String>,
    pub PublicDnsName: String,
    pub PublicIpAddress: Option<String>,
    pub State: InstanceState,
    pub StateTransitionReason: String,
    pub SubnetId: Option<String>,
    pub VpcId: Option<String>,
    pub Architecture: String,
    pub BlockDeviceMappings: Vec<BlockDeviceMapping>,
    pub ClientToken: String,
    pub EbsOptimized: bool,
    pub EnaSupport: bool,
    pub Hypervisor: String,
    pub NetworkInterfaces: Vec<NetworkInterface>,
    pub RootDeviceName: String,
    pub RootDeviceType: String,
    pub SecurityGroups: Vec<Group>,
    pub SourceDestCheck: Option<bool>,
    pub Tags: Option<Vec<Tag>>,
    pub VirtualizationType: String,
    pub CpuOptions: CpuOptions,
    pub CapacityReservationSpecification: CapacityReservationSpecification,
    pub HibernationOptions: HibernationOptions,
    pub MetadataOptions: MetadataOptions,
    pub EnclaveOptions: EnclaveOptions,
    pub BootMode: Option<String>,
    pub PlatformDetails: String,
    pub UsageOperation: String,
    pub UsageOperationUpdateTime: String,
    pub PrivateDnsNameOptions: Option<PrivateDnsNameOptions>,
    pub MaintenanceOptions: MaintenanceOptions,
    pub CurrentInstanceBootMode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monitoring {
    pub State: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Placement {
    pub AvailabilityZone: String,
    pub GroupName: String,
    pub Tenancy: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceState {
    pub Code: u32,
    pub Name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDeviceMapping {
    pub DeviceName: String,
    pub Ebs: Ebs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ebs {
    pub AttachTime: String,
    pub DeleteOnTermination: bool,
    pub Status: String,
    pub VolumeId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub Association: Option<Association>,
    pub Attachment: Option<Attachment>,
    pub Description: String,
    pub Groups: Vec<Group>,
    pub Ipv6Addresses: Vec<String>,
    pub MacAddress: String,
    pub NetworkInterfaceId: String,
    pub OwnerId: String,
    pub PrivateDnsName: String,
    pub PrivateIpAddress: String,
    pub PrivateIpAddresses: Vec<PrivateIpAddress>,
    pub SourceDestCheck: bool,
    pub Status: String,
    pub SubnetId: String,
    pub VpcId: Option<String>,
    pub InterfaceType: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Association {
    pub IpOwnerId: String,
    pub PublicDnsName: String,
    pub PublicIp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub AttachTime: String,
    pub AttachmentId: String,
    pub DeleteOnTermination: bool,
    pub DeviceIndex: u32,
    pub Status: String,
    pub NetworkCardIndex: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateIpAddress {
    pub Association: Option<Association>,
    pub Primary: bool,
    pub PrivateDnsName: String,
    pub PrivateIpAddress: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub Key: String,
    pub Value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuOptions {
    pub CoreCount: u32,
    pub ThreadsPerCore: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapacityReservationSpecification {
    pub CapacityReservationPreference: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HibernationOptions {
    pub Configured: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataOptions {
    pub State: String,
    pub HttpTokens: String,
    pub HttpPutResponseHopLimit: u32,
    pub HttpEndpoint: String,
    pub HttpProtocolIpv6: String,
    pub InstanceMetadataTags: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnclaveOptions {
    pub Enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateDnsNameOptions {
    pub HostnameType: String,
    pub EnableResourceNameDnsARecord: bool,
    pub EnableResourceNameDnsAAAARecord: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenanceOptions {
    pub AutoRecovery: String,
}

