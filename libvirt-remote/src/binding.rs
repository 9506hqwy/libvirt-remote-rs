use serde::{Deserialize, Serialize};
pub const REMOTE_STRING_MAX: u32 = 4194304u32;
pub const REMOTE_CONNECT_IDENTITY_PARAMS_MAX: u32 = 20u32;
pub const REMOTE_DOMAIN_LIST_MAX: u32 = 16384u32;
pub const REMOTE_CPUMAP_MAX: u32 = 2048u32;
pub const REMOTE_VCPUINFO_MAX: u32 = 16384u32;
pub const REMOTE_CPUMAPS_MAX: u32 = 8388608u32;
pub const REMOTE_IOTHREAD_INFO_MAX: u32 = 16384u32;
pub const REMOTE_MIGRATE_COOKIE_MAX: u32 = 4194304u32;
pub const REMOTE_NETWORK_LIST_MAX: u32 = 16384u32;
pub const REMOTE_NETWORK_PORT_LIST_MAX: u32 = 16384u32;
pub const REMOTE_INTERFACE_LIST_MAX: u32 = 16384u32;
pub const REMOTE_STORAGE_POOL_LIST_MAX: u32 = 16384u32;
pub const REMOTE_STORAGE_VOL_LIST_MAX: u32 = 16384u32;
pub const REMOTE_NODE_DEVICE_LIST_MAX: u32 = 65536u32;
pub const REMOTE_NODE_DEVICE_CAPS_LIST_MAX: u32 = 65536u32;
pub const REMOTE_NWFILTER_LIST_MAX: u32 = 16384u32;
pub const REMOTE_NWFILTER_BINDING_LIST_MAX: u32 = 16384u32;
pub const REMOTE_DOMAIN_SCHEDULER_PARAMETERS_MAX: u32 = 16u32;
pub const REMOTE_DOMAIN_BLKIO_PARAMETERS_MAX: u32 = 16u32;
pub const REMOTE_DOMAIN_MEMORY_PARAMETERS_MAX: u32 = 16u32;
pub const REMOTE_DOMAIN_BLOCK_IO_TUNE_PARAMETERS_MAX: u32 = 32u32;
pub const REMOTE_DOMAIN_NUMA_PARAMETERS_MAX: u32 = 16u32;
pub const REMOTE_DOMAIN_PERF_EVENTS_MAX: u32 = 64u32;
pub const REMOTE_DOMAIN_BLOCK_COPY_PARAMETERS_MAX: u32 = 16u32;
pub const REMOTE_NODE_CPU_STATS_MAX: u32 = 16u32;
pub const REMOTE_NODE_MEMORY_STATS_MAX: u32 = 16u32;
pub const REMOTE_DOMAIN_BLOCK_STATS_PARAMETERS_MAX: u32 = 16u32;
pub const REMOTE_NODE_MAX_CELLS: u32 = 1024u32;
pub const REMOTE_AUTH_SASL_DATA_MAX: u32 = 65536u32;
pub const REMOTE_AUTH_TYPE_LIST_MAX: u32 = 20u32;
pub const REMOTE_DOMAIN_MEMORY_STATS_MAX: u32 = 1024u32;
pub const REMOTE_DOMAIN_CHECKPOINT_LIST_MAX: u32 = 16384u32;
pub const REMOTE_DOMAIN_SNAPSHOT_LIST_MAX: u32 = 16384u32;
pub const REMOTE_DOMAIN_BLOCK_PEEK_BUFFER_MAX: u32 = 4194304u32;
pub const REMOTE_DOMAIN_MEMORY_PEEK_BUFFER_MAX: u32 = 4194304u32;
pub const REMOTE_SECURITY_LABEL_LIST_MAX: u32 = 64u32;
pub const REMOTE_SECURITY_MODEL_MAX: u32 = VIR_SECURITY_MODEL_BUFLEN;
pub const REMOTE_SECURITY_LABEL_MAX: u32 = VIR_SECURITY_LABEL_BUFLEN;
pub const REMOTE_SECURITY_DOI_MAX: u32 = VIR_SECURITY_DOI_BUFLEN;
pub const REMOTE_SECRET_VALUE_MAX: u32 = 65536u32;
pub const REMOTE_SECRET_LIST_MAX: u32 = 16384u32;
pub const REMOTE_CPU_BASELINE_MAX: u32 = 256u32;
pub const REMOTE_DOMAIN_SEND_KEY_MAX: u32 = 16u32;
pub const REMOTE_DOMAIN_INTERFACE_PARAMETERS_MAX: u32 = 16u32;
pub const REMOTE_DOMAIN_GET_CPU_STATS_NCPUS_MAX: u32 = 128u32;
pub const REMOTE_DOMAIN_GET_CPU_STATS_MAX: u32 = 2048u32;
pub const REMOTE_DOMAIN_DISK_ERRORS_MAX: u32 = 256u32;
pub const REMOTE_NODE_MEMORY_PARAMETERS_MAX: u32 = 64u32;
pub const REMOTE_DOMAIN_MIGRATE_PARAM_LIST_MAX: u32 = 64u32;
pub const REMOTE_DOMAIN_JOB_STATS_MAX: u32 = 64u32;
pub const REMOTE_CONNECT_CPU_MODELS_MAX: u32 = 8192u32;
pub const REMOTE_DOMAIN_FSFREEZE_MOUNTPOINTS_MAX: u32 = 256u32;
pub const REMOTE_NETWORK_DHCP_LEASES_MAX: u32 = 65536u32;
pub const REMOTE_CONNECT_GET_ALL_DOMAIN_STATS_MAX: u32 = 262144u32;
pub const REMOTE_DOMAIN_EVENT_TUNABLE_MAX: u32 = 2048u32;
pub const REMOTE_DOMAIN_FSINFO_MAX: u32 = 256u32;
pub const REMOTE_DOMAIN_FSINFO_DISKS_MAX: u32 = 256u32;
pub const REMOTE_DOMAIN_INTERFACE_MAX: u32 = 2048u32;
pub const REMOTE_DOMAIN_IP_ADDR_MAX: u32 = 2048u32;
pub const REMOTE_DOMAIN_GUEST_VCPU_PARAMS_MAX: u32 = 64u32;
pub const REMOTE_DOMAIN_IOTHREAD_PARAMS_MAX: u32 = 64u32;
pub const REMOTE_NODE_SEV_INFO_MAX: u32 = 64u32;
pub const REMOTE_DOMAIN_LAUNCH_SECURITY_INFO_PARAMS_MAX: u32 = 64u32;
pub const REMOTE_DOMAIN_LAUNCH_SECURITY_STATE_PARAMS_MAX: u32 = 64u32;
pub const REMOTE_DOMAIN_GUEST_INFO_PARAMS_MAX: u32 = 2048u32;
pub const REMOTE_NETWORK_PORT_PARAMETERS_MAX: u32 = 16u32;
pub const REMOTE_DOMAIN_AUTHORIZED_SSH_KEYS_MAX: u32 = 2048u32;
pub const REMOTE_DOMAIN_MESSAGES_MAX: u32 = 2048u32;
pub const REMOTE_DOMAIN_EVENT_GRAPHICS_IDENTITY_MAX: u32 = 20u32;
pub const REMOTE_PROGRAM: u32 = 536903814u32;
pub const REMOTE_PROTOCOL_VERSION: u32 = 1u32;
pub const VIR_SECURITY_MODEL_BUFLEN: u32 = 256u32;
pub const VIR_SECURITY_LABEL_BUFLEN: u32 = 4096u32;
pub const VIR_SECURITY_DOI_BUFLEN: u32 = 256u32;
pub const VIR_UUID_BUFLEN: u32 = 16u32;
pub const VIR_TYPED_PARAM_INT: u32 = 1u32;
pub const VIR_TYPED_PARAM_UINT: u32 = 2u32;
pub const VIR_TYPED_PARAM_LLONG: u32 = 3u32;
pub const VIR_TYPED_PARAM_ULLONG: u32 = 4u32;
pub const VIR_TYPED_PARAM_DOUBLE: u32 = 5u32;
pub const VIR_TYPED_PARAM_BOOLEAN: u32 = 6u32;
pub const VIR_TYPED_PARAM_STRING: u32 = 7u32;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullDomain {
    pub name: String,
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
    pub id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullNetwork {
    pub name: String,
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullNetworkPort {
    pub net: RemoteNonnullNetwork,
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullNwfilter {
    pub name: String,
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullNwfilterBinding {
    pub portdev: String,
    pub filtername: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullInterface {
    pub name: String,
    pub mac: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullStoragePool {
    pub name: String,
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullStorageVol {
    pub pool: String,
    pub name: String,
    pub key: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullNodeDevice {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullSecret {
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
    pub usage_type: i32,
    pub usage_id: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullDomainCheckpoint {
    pub name: String,
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNonnullDomainSnapshot {
    pub name: String,
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteError {
    pub code: i32,
    pub domain: i32,
    pub message: Option<String>,
    pub level: i32,
    pub dom: Option<RemoteNonnullDomain>,
    pub str1: Option<String>,
    pub str2: Option<String>,
    pub str3: Option<String>,
    pub int1: i32,
    pub int2: i32,
    pub net: Option<RemoteNonnullNetwork>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum RemoteAuthType {
    RemoteAuthNone = 0i32,
    RemoteAuthSasl = 1i32,
    RemoteAuthPolkit = 2i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteVcpuInfo {
    pub number: u32,
    pub state: i32,
    pub cpu_time: u64,
    pub cpu: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RemoteTypedParamValue {
    _Reserved0,
    VirTypedParamInt(i32),
    VirTypedParamUint(u32),
    VirTypedParamLlong(i64),
    VirTypedParamUllong(u64),
    VirTypedParamDouble(f64),
    VirTypedParamBoolean(i32),
    VirTypedParamString(String),
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteTypedParam {
    pub field: String,
    pub value: RemoteTypedParamValue,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetCpuStats {
    pub field: String,
    pub value: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetMemoryStats {
    pub field: String,
    pub value: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDiskError {
    pub disk: String,
    pub error: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectOpenArgs {
    pub name: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectSupportsFeatureArgs {
    pub feature: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectSupportsFeatureRet {
    pub supported: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetTypeRet {
    pub r#type: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetVersionRet {
    pub hv_ver: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetLibVersionRet {
    pub lib_ver: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetHostnameRet {
    pub hostname: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetSysinfoArgs {
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetSysinfoRet {
    pub sysinfo: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetUriRet {
    pub uri: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetMaxVcpusArgs {
    pub r#type: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetMaxVcpusRet {
    pub max_vcpus: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetInfoRet {
    pub model: [i8; 32usize],
    pub memory: u64,
    pub cpus: i32,
    pub mhz: i32,
    pub nodes: i32,
    pub sockets: i32,
    pub cores: i32,
    pub threads: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetCapabilitiesRet {
    pub capabilities: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetDomainCapabilitiesArgs {
    pub emulatorbin: Option<String>,
    pub arch: Option<String>,
    pub machine: Option<String>,
    pub virttype: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetDomainCapabilitiesRet {
    pub capabilities: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetCpuStatsArgs {
    pub cpu_num: i32,
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetCpuStatsRet {
    pub params: Vec<RemoteNodeGetCpuStats>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetMemoryStatsArgs {
    pub nparams: i32,
    pub cell_num: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetMemoryStatsRet {
    pub params: Vec<RemoteNodeGetMemoryStats>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetCellsFreeMemoryArgs {
    pub start_cell: i32,
    pub maxcells: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetCellsFreeMemoryRet {
    pub cells: Vec<u64>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetFreeMemoryRet {
    pub free_mem: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetSchedulerTypeArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetSchedulerTypeRet {
    pub r#type: String,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetSchedulerParametersArgs {
    pub dom: RemoteNonnullDomain,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetSchedulerParametersRet {
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetSchedulerParametersFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetSchedulerParametersFlagsRet {
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetSchedulerParametersArgs {
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetSchedulerParametersFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetBlkioParametersArgs {
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetBlkioParametersArgs {
    pub dom: RemoteNonnullDomain,
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetBlkioParametersRet {
    pub params: Vec<RemoteTypedParam>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetMemoryParametersArgs {
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetMemoryParametersArgs {
    pub dom: RemoteNonnullDomain,
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetMemoryParametersRet {
    pub params: Vec<RemoteTypedParam>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockResizeArgs {
    pub dom: RemoteNonnullDomain,
    pub disk: String,
    pub size: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetNumaParametersArgs {
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetNumaParametersArgs {
    pub dom: RemoteNonnullDomain,
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetNumaParametersRet {
    pub params: Vec<RemoteTypedParam>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetPerfEventsArgs {
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetPerfEventsArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetPerfEventsRet {
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockStatsArgs {
    pub dom: RemoteNonnullDomain,
    pub path: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockStatsRet {
    pub rd_req: i64,
    pub rd_bytes: i64,
    pub wr_req: i64,
    pub wr_bytes: i64,
    pub errs: i64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockStatsFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub path: String,
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockStatsFlagsRet {
    pub params: Vec<RemoteTypedParam>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainInterfaceStatsArgs {
    pub dom: RemoteNonnullDomain,
    pub device: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainInterfaceStatsRet {
    pub rx_bytes: i64,
    pub rx_packets: i64,
    pub rx_errs: i64,
    pub rx_drop: i64,
    pub tx_bytes: i64,
    pub tx_packets: i64,
    pub tx_errs: i64,
    pub tx_drop: i64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetInterfaceParametersArgs {
    pub dom: RemoteNonnullDomain,
    pub device: String,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetInterfaceParametersArgs {
    pub dom: RemoteNonnullDomain,
    pub device: String,
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetInterfaceParametersRet {
    pub params: Vec<RemoteTypedParam>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMemoryStatsArgs {
    pub dom: RemoteNonnullDomain,
    pub max_stats: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMemoryStat {
    pub tag: i32,
    pub val: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMemoryStatsRet {
    pub stats: Vec<RemoteDomainMemoryStat>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockPeekArgs {
    pub dom: RemoteNonnullDomain,
    pub path: String,
    pub offset: u64,
    pub size: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockPeekRet {
    pub buffer: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMemoryPeekArgs {
    pub dom: RemoteNonnullDomain,
    pub offset: u64,
    pub size: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMemoryPeekRet {
    pub buffer: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetBlockInfoArgs {
    pub dom: RemoteNonnullDomain,
    pub path: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetBlockInfoRet {
    pub allocation: u64,
    pub capacity: u64,
    pub physical: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListDomainsArgs {
    pub maxids: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListDomainsRet {
    pub ids: Vec<i32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNumOfDomainsRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCreateXmlArgs {
    pub xml_desc: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCreateXmlRet {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCreateXmlWithFilesArgs {
    pub xml_desc: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCreateXmlWithFilesRet {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainLookupByIdArgs {
    pub id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainLookupByIdRet {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainLookupByUuidArgs {
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainLookupByUuidRet {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainLookupByNameArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainLookupByNameRet {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSuspendArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainResumeArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainPmSuspendForDurationArgs {
    pub dom: RemoteNonnullDomain,
    pub target: u32,
    pub duration: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainPmWakeupArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainShutdownArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainRebootArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainResetArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDestroyArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDestroyFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetOsTypeArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetOsTypeRet {
    pub r#type: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetMaxMemoryArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetMaxMemoryRet {
    pub memory: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetMaxMemoryArgs {
    pub dom: RemoteNonnullDomain,
    pub memory: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetMemoryArgs {
    pub dom: RemoteNonnullDomain,
    pub memory: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetMemoryFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub memory: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetMemoryStatsPeriodArgs {
    pub dom: RemoteNonnullDomain,
    pub period: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetInfoArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetInfoRet {
    pub state: u8,
    pub max_mem: u64,
    pub memory: u64,
    pub nr_virt_cpu: u16,
    pub cpu_time: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSaveArgs {
    pub dom: RemoteNonnullDomain,
    pub to: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSaveFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub to: String,
    pub dxml: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainRestoreArgs {
    pub from: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainRestoreFlagsArgs {
    pub from: String,
    pub dxml: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSaveImageGetXmlDescArgs {
    pub file: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSaveImageGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSaveImageDefineXmlArgs {
    pub file: String,
    pub dxml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCoreDumpArgs {
    pub dom: RemoteNonnullDomain,
    pub to: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCoreDumpWithFormatArgs {
    pub dom: RemoteNonnullDomain,
    pub to: String,
    pub dumpformat: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainScreenshotArgs {
    pub dom: RemoteNonnullDomain,
    pub screen: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainScreenshotRet {
    pub mime: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetXmlDescArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepareArgs {
    pub uri_in: Option<String>,
    pub flags: u64,
    pub dname: Option<String>,
    pub resource: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepareRet {
    pub cookie: Vec<u8>,
    pub uri_out: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePerformArgs {
    pub dom: RemoteNonnullDomain,
    pub cookie: Vec<u8>,
    pub uri: String,
    pub flags: u64,
    pub dname: Option<String>,
    pub resource: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateFinishArgs {
    pub dname: String,
    pub cookie: Vec<u8>,
    pub uri: String,
    pub flags: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateFinishRet {
    pub ddom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepare2Args {
    pub uri_in: Option<String>,
    pub flags: u64,
    pub dname: Option<String>,
    pub resource: u64,
    pub dom_xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepare2Ret {
    pub cookie: Vec<u8>,
    pub uri_out: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateFinish2Args {
    pub dname: String,
    pub cookie: Vec<u8>,
    pub uri: String,
    pub flags: u64,
    pub retcode: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateFinish2Ret {
    pub ddom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListDefinedDomainsArgs {
    pub maxnames: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListDefinedDomainsRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNumOfDefinedDomainsRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCreateArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCreateWithFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCreateWithFlagsRet {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCreateWithFilesArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCreateWithFilesRet {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDefineXmlArgs {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDefineXmlRet {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDefineXmlFlagsArgs {
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDefineXmlFlagsRet {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainUndefineArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainUndefineFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainInjectNmiArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSendKeyArgs {
    pub dom: RemoteNonnullDomain,
    pub codeset: u32,
    pub holdtime: u32,
    pub keycodes: Vec<u32>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSendProcessSignalArgs {
    pub dom: RemoteNonnullDomain,
    pub pid_value: i64,
    pub signum: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetVcpusArgs {
    pub dom: RemoteNonnullDomain,
    pub nvcpus: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetVcpusFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub nvcpus: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetVcpusFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetVcpusFlagsRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainPinVcpuArgs {
    pub dom: RemoteNonnullDomain,
    pub vcpu: u32,
    pub cpumap: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainPinVcpuFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub vcpu: u32,
    pub cpumap: Vec<u8>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetVcpuPinInfoArgs {
    pub dom: RemoteNonnullDomain,
    pub ncpumaps: i32,
    pub maplen: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetVcpuPinInfoRet {
    pub cpumaps: Vec<u8>,
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainPinEmulatorArgs {
    pub dom: RemoteNonnullDomain,
    pub cpumap: Vec<u8>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetEmulatorPinInfoArgs {
    pub dom: RemoteNonnullDomain,
    pub maplen: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetEmulatorPinInfoRet {
    pub cpumaps: Vec<u8>,
    pub ret: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetVcpusArgs {
    pub dom: RemoteNonnullDomain,
    pub maxinfo: i32,
    pub maplen: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetVcpusRet {
    pub info: Vec<RemoteVcpuInfo>,
    pub cpumaps: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetMaxVcpusArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetMaxVcpusRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainIothreadInfo {
    pub iothread_id: u32,
    pub cpumap: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetIothreadInfoArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetIothreadInfoRet {
    pub info: Vec<RemoteDomainIothreadInfo>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainPinIothreadArgs {
    pub dom: RemoteNonnullDomain,
    pub iothreads_id: u32,
    pub cpumap: Vec<u8>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainAddIothreadArgs {
    pub dom: RemoteNonnullDomain,
    pub iothread_id: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDelIothreadArgs {
    pub dom: RemoteNonnullDomain,
    pub iothread_id: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetIothreadParamsArgs {
    pub dom: RemoteNonnullDomain,
    pub iothread_id: u32,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetSecurityLabelArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetSecurityLabelRet {
    pub label: Vec<i8>,
    pub enforcing: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetSecurityLabelListArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetSecurityLabelListRet {
    pub labels: Vec<RemoteDomainGetSecurityLabelRet>,
    pub ret: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetSecurityModelRet {
    pub model: Vec<i8>,
    pub doi: Vec<i8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainAttachDeviceArgs {
    pub dom: RemoteNonnullDomain,
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainAttachDeviceFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDetachDeviceArgs {
    pub dom: RemoteNonnullDomain,
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDetachDeviceFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainUpdateDeviceFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainDetachDeviceAliasArgs {
    pub dom: RemoteNonnullDomain,
    pub alias: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetAutostartArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetAutostartRet {
    pub autostart: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetAutostartArgs {
    pub dom: RemoteNonnullDomain,
    pub autostart: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetMetadataArgs {
    pub dom: RemoteNonnullDomain,
    pub r#type: i32,
    pub metadata: Option<String>,
    pub key: Option<String>,
    pub uri: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetMetadataArgs {
    pub dom: RemoteNonnullDomain,
    pub r#type: i32,
    pub uri: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetMetadataRet {
    pub metadata: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockJobAbortArgs {
    pub dom: RemoteNonnullDomain,
    pub path: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetBlockJobInfoArgs {
    pub dom: RemoteNonnullDomain,
    pub path: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetBlockJobInfoRet {
    pub found: i32,
    pub r#type: i32,
    pub bandwidth: u64,
    pub cur: u64,
    pub end: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockJobSetSpeedArgs {
    pub dom: RemoteNonnullDomain,
    pub path: String,
    pub bandwidth: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockPullArgs {
    pub dom: RemoteNonnullDomain,
    pub path: String,
    pub bandwidth: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockRebaseArgs {
    pub dom: RemoteNonnullDomain,
    pub path: String,
    pub base: Option<String>,
    pub bandwidth: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockCopyArgs {
    pub dom: RemoteNonnullDomain,
    pub path: String,
    pub destxml: String,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBlockCommitArgs {
    pub dom: RemoteNonnullDomain,
    pub disk: String,
    pub base: Option<String>,
    pub top: Option<String>,
    pub bandwidth: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetBlockIoTuneArgs {
    pub dom: RemoteNonnullDomain,
    pub disk: String,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetBlockIoTuneArgs {
    pub dom: RemoteNonnullDomain,
    pub disk: Option<String>,
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetBlockIoTuneRet {
    pub params: Vec<RemoteTypedParam>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetCpuStatsArgs {
    pub dom: RemoteNonnullDomain,
    pub nparams: u32,
    pub start_cpu: i32,
    pub ncpus: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetCpuStatsRet {
    pub params: Vec<RemoteTypedParam>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetHostnameArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetHostnameRet {
    pub hostname: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNumOfNetworksRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListNetworksArgs {
    pub maxnames: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListNetworksRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNumOfDefinedNetworksRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListDefinedNetworksArgs {
    pub maxnames: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListDefinedNetworksRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkLookupByUuidArgs {
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkLookupByUuidRet {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkLookupByNameArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkLookupByNameRet {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkCreateXmlArgs {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkCreateXmlRet {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkCreateXmlFlagsArgs {
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkCreateXmlFlagsRet {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkDefineXmlArgs {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkDefineXmlRet {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkDefineXmlFlagsArgs {
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkDefineXmlFlagsRet {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkUndefineArgs {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkUpdateArgs {
    pub net: RemoteNonnullNetwork,
    pub command: u32,
    pub section: u32,
    pub parent_index: i32,
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkCreateArgs {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkDestroyArgs {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkGetXmlDescArgs {
    pub net: RemoteNonnullNetwork,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkGetBridgeNameArgs {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkGetBridgeNameRet {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkGetAutostartArgs {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkGetAutostartRet {
    pub autostart: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkSetAutostartArgs {
    pub net: RemoteNonnullNetwork,
    pub autostart: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNumOfNwfiltersRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListNwfiltersArgs {
    pub maxnames: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListNwfiltersRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterLookupByUuidArgs {
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterLookupByUuidRet {
    pub nwfilter: RemoteNonnullNwfilter,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterLookupByNameArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterLookupByNameRet {
    pub nwfilter: RemoteNonnullNwfilter,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterDefineXmlArgs {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterDefineXmlRet {
    pub nwfilter: RemoteNonnullNwfilter,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterDefineXmlFlagsArgs {
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterDefineXmlFlagsRet {
    pub nwfilter: RemoteNonnullNwfilter,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterUndefineArgs {
    pub nwfilter: RemoteNonnullNwfilter,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterGetXmlDescArgs {
    pub nwfilter: RemoteNonnullNwfilter,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNumOfInterfacesRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListInterfacesArgs {
    pub maxnames: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListInterfacesRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNumOfDefinedInterfacesRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListDefinedInterfacesArgs {
    pub maxnames: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListDefinedInterfacesRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceLookupByNameArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceLookupByNameRet {
    pub iface: RemoteNonnullInterface,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceLookupByMacStringArgs {
    pub mac: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceLookupByMacStringRet {
    pub iface: RemoteNonnullInterface,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceGetXmlDescArgs {
    pub iface: RemoteNonnullInterface,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceDefineXmlArgs {
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceDefineXmlRet {
    pub iface: RemoteNonnullInterface,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceUndefineArgs {
    pub iface: RemoteNonnullInterface,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceCreateArgs {
    pub iface: RemoteNonnullInterface,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceDestroyArgs {
    pub iface: RemoteNonnullInterface,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceChangeBeginArgs {
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceChangeCommitArgs {
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceChangeRollbackArgs {
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteAuthListRet {
    pub types: Vec<RemoteAuthType>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteAuthSaslInitRet {
    pub mechlist: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteAuthSaslStartArgs {
    pub mech: String,
    pub nil: i32,
    pub data: Vec<i8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteAuthSaslStartRet {
    pub complete: i32,
    pub nil: i32,
    pub data: Vec<i8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteAuthSaslStepArgs {
    pub nil: i32,
    pub data: Vec<i8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteAuthSaslStepRet {
    pub complete: i32,
    pub nil: i32,
    pub data: Vec<i8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteAuthPolkitRet {
    pub complete: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNumOfStoragePoolsRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListStoragePoolsArgs {
    pub maxnames: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListStoragePoolsRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNumOfDefinedStoragePoolsRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListDefinedStoragePoolsArgs {
    pub maxnames: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListDefinedStoragePoolsRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectFindStoragePoolSourcesArgs {
    pub r#type: String,
    pub src_spec: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectFindStoragePoolSourcesRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolLookupByUuidArgs {
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolLookupByUuidRet {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolLookupByNameArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolLookupByNameRet {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolLookupByVolumeArgs {
    pub vol: RemoteNonnullStorageVol,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolLookupByVolumeRet {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolLookupByTargetPathArgs {
    pub path: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolLookupByTargetPathRet {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolCreateXmlArgs {
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolCreateXmlRet {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolDefineXmlArgs {
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolDefineXmlRet {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolBuildArgs {
    pub pool: RemoteNonnullStoragePool,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolUndefineArgs {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolCreateArgs {
    pub pool: RemoteNonnullStoragePool,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolDestroyArgs {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolDeleteArgs {
    pub pool: RemoteNonnullStoragePool,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolRefreshArgs {
    pub pool: RemoteNonnullStoragePool,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolGetXmlDescArgs {
    pub pool: RemoteNonnullStoragePool,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolGetInfoArgs {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolGetInfoRet {
    pub state: u8,
    pub capacity: u64,
    pub allocation: u64,
    pub available: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolGetAutostartArgs {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolGetAutostartRet {
    pub autostart: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolSetAutostartArgs {
    pub pool: RemoteNonnullStoragePool,
    pub autostart: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolNumOfVolumesArgs {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolNumOfVolumesRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolListVolumesArgs {
    pub pool: RemoteNonnullStoragePool,
    pub maxnames: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolListVolumesRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolLookupByNameArgs {
    pub pool: RemoteNonnullStoragePool,
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolLookupByNameRet {
    pub vol: RemoteNonnullStorageVol,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolLookupByKeyArgs {
    pub key: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolLookupByKeyRet {
    pub vol: RemoteNonnullStorageVol,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolLookupByPathArgs {
    pub path: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolLookupByPathRet {
    pub vol: RemoteNonnullStorageVol,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolCreateXmlArgs {
    pub pool: RemoteNonnullStoragePool,
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolCreateXmlRet {
    pub vol: RemoteNonnullStorageVol,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolCreateXmlFromArgs {
    pub pool: RemoteNonnullStoragePool,
    pub xml: String,
    pub clonevol: RemoteNonnullStorageVol,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolCreateXmlFromRet {
    pub vol: RemoteNonnullStorageVol,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolDeleteArgs {
    pub vol: RemoteNonnullStorageVol,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolWipeArgs {
    pub vol: RemoteNonnullStorageVol,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolWipePatternArgs {
    pub vol: RemoteNonnullStorageVol,
    pub algorithm: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolGetXmlDescArgs {
    pub vol: RemoteNonnullStorageVol,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolGetInfoArgs {
    pub vol: RemoteNonnullStorageVol,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolGetInfoRet {
    pub r#type: i8,
    pub capacity: u64,
    pub allocation: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolGetInfoFlagsArgs {
    pub vol: RemoteNonnullStorageVol,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolGetInfoFlagsRet {
    pub r#type: i8,
    pub capacity: u64,
    pub allocation: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolGetPathArgs {
    pub vol: RemoteNonnullStorageVol,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolGetPathRet {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolResizeArgs {
    pub vol: RemoteNonnullStorageVol,
    pub capacity: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeNumOfDevicesArgs {
    pub cap: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeNumOfDevicesRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeListDevicesArgs {
    pub cap: Option<String>,
    pub maxnames: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeListDevicesRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceLookupByNameArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceLookupByNameRet {
    pub dev: RemoteNonnullNodeDevice,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceLookupScsiHostByWwnArgs {
    pub wwnn: String,
    pub wwpn: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceLookupScsiHostByWwnRet {
    pub dev: RemoteNonnullNodeDevice,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceGetXmlDescArgs {
    pub name: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceGetParentArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceGetParentRet {
    pub parent_name: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceNumOfCapsArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceNumOfCapsRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceListCapsArgs {
    pub name: String,
    pub maxnames: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceListCapsRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceDettachArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceDetachFlagsArgs {
    pub name: String,
    pub driver_name: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceReAttachArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceResetArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceCreateXmlArgs {
    pub xml_desc: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceCreateXmlRet {
    pub dev: RemoteNonnullNodeDevice,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceDestroyArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceDefineXmlArgs {
    pub xml_desc: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceDefineXmlRet {
    pub dev: RemoteNonnullNodeDevice,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceUndefineArgs {
    pub name: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceCreateArgs {
    pub name: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceGetAutostartArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceGetAutostartRet {
    pub autostart: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceSetAutostartArgs {
    pub name: String,
    pub autostart: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceIsPersistentArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceIsPersistentRet {
    pub persistent: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceIsActiveArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceIsActiveRet {
    pub active: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainEventRegisterRet {
    pub cb_registered: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainEventDeregisterRet {
    pub cb_registered: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventLifecycleMsg {
    pub dom: RemoteNonnullDomain,
    pub event: i32,
    pub detail: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackLifecycleMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventLifecycleMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainXmlFromNativeArgs {
    pub native_format: String,
    pub native_config: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainXmlFromNativeRet {
    pub domain_xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainXmlToNativeArgs {
    pub native_format: String,
    pub domain_xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainXmlToNativeRet {
    pub native_config: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNumOfSecretsRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListSecretsArgs {
    pub maxuuids: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListSecretsRet {
    pub uuids: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretLookupByUuidArgs {
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretLookupByUuidRet {
    pub secret: RemoteNonnullSecret,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretDefineXmlArgs {
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretDefineXmlRet {
    pub secret: RemoteNonnullSecret,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretGetXmlDescArgs {
    pub secret: RemoteNonnullSecret,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretSetValueArgs {
    pub secret: RemoteNonnullSecret,
    pub value: Vec<u8>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretGetValueArgs {
    pub secret: RemoteNonnullSecret,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretGetValueRet {
    pub value: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretUndefineArgs {
    pub secret: RemoteNonnullSecret,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretLookupByUsageArgs {
    pub usage_type: i32,
    pub usage_id: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretLookupByUsageRet {
    pub secret: RemoteNonnullSecret,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepareTunnelArgs {
    pub flags: u64,
    pub dname: Option<String>,
    pub resource: u64,
    pub dom_xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectIsSecureRet {
    pub secure: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainIsActiveArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainIsActiveRet {
    pub active: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainIsPersistentArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainIsPersistentRet {
    pub persistent: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainIsUpdatedArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainIsUpdatedRet {
    pub updated: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkIsActiveArgs {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkIsActiveRet {
    pub active: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkIsPersistentArgs {
    pub net: RemoteNonnullNetwork,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkIsPersistentRet {
    pub persistent: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolIsActiveArgs {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolIsActiveRet {
    pub active: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolIsPersistentArgs {
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolIsPersistentRet {
    pub persistent: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceIsActiveArgs {
    pub iface: RemoteNonnullInterface,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteInterfaceIsActiveRet {
    pub active: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectCompareCpuArgs {
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectCompareCpuRet {
    pub result: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectBaselineCpuArgs {
    pub xml_cpus: Vec<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectBaselineCpuRet {
    pub cpu: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetJobInfoArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetJobInfoRet {
    pub r#type: i32,
    pub time_elapsed: u64,
    pub time_remaining: u64,
    pub data_total: u64,
    pub data_processed: u64,
    pub data_remaining: u64,
    pub mem_total: u64,
    pub mem_processed: u64,
    pub mem_remaining: u64,
    pub file_total: u64,
    pub file_processed: u64,
    pub file_remaining: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetJobStatsArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetJobStatsRet {
    pub r#type: i32,
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainAbortJobArgs {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateGetMaxDowntimeArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateGetMaxDowntimeRet {
    pub downtime: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateSetMaxDowntimeArgs {
    pub dom: RemoteNonnullDomain,
    pub downtime: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateGetCompressionCacheArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateGetCompressionCacheRet {
    pub cache_size: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateSetCompressionCacheArgs {
    pub dom: RemoteNonnullDomain,
    pub cache_size: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateSetMaxSpeedArgs {
    pub dom: RemoteNonnullDomain,
    pub bandwidth: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateGetMaxSpeedArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateGetMaxSpeedRet {
    pub bandwidth: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainEventRegisterAnyArgs {
    pub event_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainEventDeregisterAnyArgs {
    pub event_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainEventCallbackRegisterAnyArgs {
    pub event_id: i32,
    pub dom: Option<RemoteNonnullDomain>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainEventCallbackRegisterAnyRet {
    pub callback_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectDomainEventCallbackDeregisterAnyArgs {
    pub callback_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventRebootMsg {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackRebootMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventRebootMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventRtcChangeMsg {
    pub dom: RemoteNonnullDomain,
    pub offset: i64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackRtcChangeMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventRtcChangeMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventWatchdogMsg {
    pub dom: RemoteNonnullDomain,
    pub action: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackWatchdogMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventWatchdogMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventIoErrorMsg {
    pub dom: RemoteNonnullDomain,
    pub src_path: String,
    pub dev_alias: String,
    pub action: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackIoErrorMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventIoErrorMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventIoErrorReasonMsg {
    pub dom: RemoteNonnullDomain,
    pub src_path: String,
    pub dev_alias: String,
    pub action: i32,
    pub reason: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackIoErrorReasonMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventIoErrorReasonMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventGraphicsAddress {
    pub family: i32,
    pub node: String,
    pub service: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventGraphicsIdentity {
    pub r#type: String,
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventGraphicsMsg {
    pub dom: RemoteNonnullDomain,
    pub phase: i32,
    pub local: RemoteDomainEventGraphicsAddress,
    pub remote: RemoteDomainEventGraphicsAddress,
    pub auth_scheme: String,
    pub subject: Vec<RemoteDomainEventGraphicsIdentity>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackGraphicsMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventGraphicsMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventBlockJobMsg {
    pub dom: RemoteNonnullDomain,
    pub path: String,
    pub r#type: i32,
    pub status: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackBlockJobMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventBlockJobMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventDiskChangeMsg {
    pub dom: RemoteNonnullDomain,
    pub old_src_path: Option<String>,
    pub new_src_path: Option<String>,
    pub dev_alias: String,
    pub reason: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackDiskChangeMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventDiskChangeMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventTrayChangeMsg {
    pub dom: RemoteNonnullDomain,
    pub dev_alias: String,
    pub reason: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackTrayChangeMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventTrayChangeMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventPmwakeupMsg {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackPmwakeupMsg {
    pub callback_id: i32,
    pub reason: i32,
    pub msg: RemoteDomainEventPmwakeupMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventPmsuspendMsg {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackPmsuspendMsg {
    pub callback_id: i32,
    pub reason: i32,
    pub msg: RemoteDomainEventPmsuspendMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventBalloonChangeMsg {
    pub dom: RemoteNonnullDomain,
    pub actual: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackBalloonChangeMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventBalloonChangeMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventPmsuspendDiskMsg {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackPmsuspendDiskMsg {
    pub callback_id: i32,
    pub reason: i32,
    pub msg: RemoteDomainEventPmsuspendDiskMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainManagedSaveArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainHasManagedSaveImageArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainHasManagedSaveImageRet {
    pub result: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainManagedSaveRemoveArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainManagedSaveGetXmlDescArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainManagedSaveGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainManagedSaveDefineXmlArgs {
    pub dom: RemoteNonnullDomain,
    pub dxml: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotCreateXmlArgs {
    pub dom: RemoteNonnullDomain,
    pub xml_desc: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotCreateXmlRet {
    pub snap: RemoteNonnullDomainSnapshot,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotGetXmlDescArgs {
    pub snap: RemoteNonnullDomainSnapshot,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotNumArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotNumRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotListNamesArgs {
    pub dom: RemoteNonnullDomain,
    pub maxnames: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotListNamesRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainListAllSnapshotsArgs {
    pub dom: RemoteNonnullDomain,
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainListAllSnapshotsRet {
    pub snapshots: Vec<RemoteNonnullDomainSnapshot>,
    pub ret: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotNumChildrenArgs {
    pub snap: RemoteNonnullDomainSnapshot,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotNumChildrenRet {
    pub num: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotListChildrenNamesArgs {
    pub snap: RemoteNonnullDomainSnapshot,
    pub maxnames: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotListChildrenNamesRet {
    pub names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotListAllChildrenArgs {
    pub snapshot: RemoteNonnullDomainSnapshot,
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotListAllChildrenRet {
    pub snapshots: Vec<RemoteNonnullDomainSnapshot>,
    pub ret: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotLookupByNameArgs {
    pub dom: RemoteNonnullDomain,
    pub name: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotLookupByNameRet {
    pub snap: RemoteNonnullDomainSnapshot,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainHasCurrentSnapshotArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainHasCurrentSnapshotRet {
    pub result: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotGetParentArgs {
    pub snap: RemoteNonnullDomainSnapshot,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotGetParentRet {
    pub snap: RemoteNonnullDomainSnapshot,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotCurrentArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotCurrentRet {
    pub snap: RemoteNonnullDomainSnapshot,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotIsCurrentArgs {
    pub snap: RemoteNonnullDomainSnapshot,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotIsCurrentRet {
    pub current: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotHasMetadataArgs {
    pub snap: RemoteNonnullDomainSnapshot,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotHasMetadataRet {
    pub metadata: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainRevertToSnapshotArgs {
    pub snap: RemoteNonnullDomainSnapshot,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSnapshotDeleteArgs {
    pub snap: RemoteNonnullDomainSnapshot,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainOpenConsoleArgs {
    pub dom: RemoteNonnullDomain,
    pub dev_name: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainOpenChannelArgs {
    pub dom: RemoteNonnullDomain,
    pub name: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolUploadArgs {
    pub vol: RemoteNonnullStorageVol,
    pub offset: u64,
    pub length: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStorageVolDownloadArgs {
    pub vol: RemoteNonnullStorageVol,
    pub offset: u64,
    pub length: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetStateArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetStateRet {
    pub state: i32,
    pub reason: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateBegin3Args {
    pub dom: RemoteNonnullDomain,
    pub xmlin: Option<String>,
    pub flags: u64,
    pub dname: Option<String>,
    pub resource: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateBegin3Ret {
    pub cookie_out: Vec<u8>,
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepare3Args {
    pub cookie_in: Vec<u8>,
    pub uri_in: Option<String>,
    pub flags: u64,
    pub dname: Option<String>,
    pub resource: u64,
    pub dom_xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepare3Ret {
    pub cookie_out: Vec<u8>,
    pub uri_out: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepareTunnel3Args {
    pub cookie_in: Vec<u8>,
    pub flags: u64,
    pub dname: Option<String>,
    pub resource: u64,
    pub dom_xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepareTunnel3Ret {
    pub cookie_out: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePerform3Args {
    pub dom: RemoteNonnullDomain,
    pub xmlin: Option<String>,
    pub cookie_in: Vec<u8>,
    pub dconnuri: Option<String>,
    pub uri: Option<String>,
    pub flags: u64,
    pub dname: Option<String>,
    pub resource: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePerform3Ret {
    pub cookie_out: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateFinish3Args {
    pub dname: String,
    pub cookie_in: Vec<u8>,
    pub dconnuri: Option<String>,
    pub uri: Option<String>,
    pub flags: u64,
    pub cancelled: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateFinish3Ret {
    pub dom: RemoteNonnullDomain,
    pub cookie_out: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateConfirm3Args {
    pub dom: RemoteNonnullDomain,
    pub cookie_in: Vec<u8>,
    pub flags: u64,
    pub cancelled: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventControlErrorMsg {
    pub dom: RemoteNonnullDomain,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackControlErrorMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventControlErrorMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetControlInfoArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetControlInfoRet {
    pub state: u32,
    pub details: u32,
    pub state_time: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainOpenGraphicsArgs {
    pub dom: RemoteNonnullDomain,
    pub idx: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainOpenGraphicsFdArgs {
    pub dom: RemoteNonnullDomain,
    pub idx: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeSuspendForDurationArgs {
    pub target: u32,
    pub duration: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainShutdownFlagsArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetDiskErrorsArgs {
    pub dom: RemoteNonnullDomain,
    pub maxerrors: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetDiskErrorsRet {
    pub errors: Vec<RemoteDomainDiskError>,
    pub nerrors: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllDomainsArgs {
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllDomainsRet {
    pub domains: Vec<RemoteNonnullDomain>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllStoragePoolsArgs {
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllStoragePoolsRet {
    pub pools: Vec<RemoteNonnullStoragePool>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolListAllVolumesArgs {
    pub pool: RemoteNonnullStoragePool,
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolListAllVolumesRet {
    pub vols: Vec<RemoteNonnullStorageVol>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllNetworksArgs {
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllNetworksRet {
    pub nets: Vec<RemoteNonnullNetwork>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllInterfacesArgs {
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllInterfacesRet {
    pub ifaces: Vec<RemoteNonnullInterface>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllNodeDevicesArgs {
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllNodeDevicesRet {
    pub devices: Vec<RemoteNonnullNodeDevice>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllNwfiltersArgs {
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllNwfiltersRet {
    pub filters: Vec<RemoteNonnullNwfilter>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllSecretsArgs {
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllSecretsRet {
    pub secrets: Vec<RemoteNonnullSecret>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeSetMemoryParametersArgs {
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetMemoryParametersArgs {
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetMemoryParametersRet {
    pub params: Vec<RemoteTypedParam>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetCpuMapArgs {
    pub need_map: i32,
    pub need_online: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetCpuMapRet {
    pub cpumap: Vec<u8>,
    pub online: u32,
    pub ret: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainFstrimArgs {
    pub dom: RemoteNonnullDomain,
    pub mount_point: Option<String>,
    pub minimum: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetTimeArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetTimeRet {
    pub seconds: i64,
    pub nseconds: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetTimeArgs {
    pub dom: RemoteNonnullDomain,
    pub seconds: i64,
    pub nseconds: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateBegin3ParamsArgs {
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateBegin3ParamsRet {
    pub cookie_out: Vec<u8>,
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepare3ParamsArgs {
    pub params: Vec<RemoteTypedParam>,
    pub cookie_in: Vec<u8>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepare3ParamsRet {
    pub cookie_out: Vec<u8>,
    pub uri_out: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepareTunnel3ParamsArgs {
    pub params: Vec<RemoteTypedParam>,
    pub cookie_in: Vec<u8>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePrepareTunnel3ParamsRet {
    pub cookie_out: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePerform3ParamsArgs {
    pub dom: RemoteNonnullDomain,
    pub dconnuri: Option<String>,
    pub params: Vec<RemoteTypedParam>,
    pub cookie_in: Vec<u8>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigratePerform3ParamsRet {
    pub cookie_out: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateFinish3ParamsArgs {
    pub params: Vec<RemoteTypedParam>,
    pub cookie_in: Vec<u8>,
    pub flags: u32,
    pub cancelled: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateFinish3ParamsRet {
    pub dom: RemoteNonnullDomain,
    pub cookie_out: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateConfirm3ParamsArgs {
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
    pub cookie_in: Vec<u8>,
    pub flags: u32,
    pub cancelled: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventDeviceRemovedMsg {
    pub dom: RemoteNonnullDomain,
    pub dev_alias: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackDeviceRemovedMsg {
    pub callback_id: i32,
    pub msg: RemoteDomainEventDeviceRemovedMsg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventBlockJob2Msg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub dst: String,
    pub r#type: i32,
    pub status: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventBlockThresholdMsg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub dev: String,
    pub path: Option<String>,
    pub threshold: u64,
    pub excess: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackTunableMsg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackDeviceAddedMsg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub dev_alias: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectEventConnectionClosedMsg {
    pub reason: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetCpuModelNamesArgs {
    pub arch: String,
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetCpuModelNamesRet {
    pub models: Vec<String>,
    pub ret: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNetworkEventRegisterAnyArgs {
    pub event_id: i32,
    pub net: Option<RemoteNonnullNetwork>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNetworkEventRegisterAnyRet {
    pub callback_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNetworkEventDeregisterAnyArgs {
    pub callback_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkEventLifecycleMsg {
    pub callback_id: i32,
    pub net: RemoteNonnullNetwork,
    pub event: i32,
    pub detail: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectStoragePoolEventRegisterAnyArgs {
    pub event_id: i32,
    pub pool: Option<RemoteNonnullStoragePool>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectStoragePoolEventRegisterAnyRet {
    pub callback_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectStoragePoolEventDeregisterAnyArgs {
    pub callback_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolEventLifecycleMsg {
    pub callback_id: i32,
    pub pool: RemoteNonnullStoragePool,
    pub event: i32,
    pub detail: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteStoragePoolEventRefreshMsg {
    pub callback_id: i32,
    pub pool: RemoteNonnullStoragePool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNodeDeviceEventRegisterAnyArgs {
    pub event_id: i32,
    pub dev: Option<RemoteNonnullNodeDevice>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNodeDeviceEventRegisterAnyRet {
    pub callback_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectNodeDeviceEventDeregisterAnyArgs {
    pub callback_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceEventLifecycleMsg {
    pub callback_id: i32,
    pub dev: RemoteNonnullNodeDevice,
    pub event: i32,
    pub detail: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeDeviceEventUpdateMsg {
    pub callback_id: i32,
    pub dev: RemoteNonnullNodeDevice,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainFsfreezeArgs {
    pub dom: RemoteNonnullDomain,
    pub mountpoints: Vec<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainFsfreezeRet {
    pub filesystems: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainFsthawArgs {
    pub dom: RemoteNonnullDomain,
    pub mountpoints: Vec<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainFsthawRet {
    pub filesystems: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetFreePagesArgs {
    pub pages: Vec<u32>,
    pub start_cell: i32,
    pub cell_count: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetFreePagesRet {
    pub counts: Vec<u64>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeAllocPagesArgs {
    pub page_sizes: Vec<u32>,
    pub page_counts: Vec<u64>,
    pub start_cell: i32,
    pub cell_count: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeAllocPagesRet {
    pub ret: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkDhcpLease {
    pub iface: String,
    pub expirytime: i64,
    pub r#type: i32,
    pub mac: Option<String>,
    pub iaid: Option<String>,
    pub ipaddr: String,
    pub prefix: u32,
    pub hostname: Option<String>,
    pub clientid: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkGetDhcpLeasesArgs {
    pub net: RemoteNonnullNetwork,
    pub mac: Option<String>,
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkGetDhcpLeasesRet {
    pub leases: Vec<RemoteNetworkDhcpLease>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainStatsRecord {
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetAllDomainStatsArgs {
    pub doms: Vec<RemoteNonnullDomain>,
    pub stats: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackAgentLifecycleMsg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub state: i32,
    pub reason: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetAllDomainStatsRet {
    pub ret_stats: Vec<RemoteDomainStatsRecord>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainFsinfo {
    pub mountpoint: String,
    pub name: String,
    pub fstype: String,
    pub dev_aliases: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetFsinfoArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetFsinfoRet {
    pub info: Vec<RemoteDomainFsinfo>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainIpAddr {
    pub r#type: i32,
    pub addr: String,
    pub prefix: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainInterface {
    pub name: String,
    pub hwaddr: Option<String>,
    pub addrs: Vec<RemoteDomainIpAddr>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainInterfaceAddressesArgs {
    pub dom: RemoteNonnullDomain,
    pub source: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainInterfaceAddressesRet {
    pub ifaces: Vec<RemoteDomainInterface>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetUserPasswordArgs {
    pub dom: RemoteNonnullDomain,
    pub user: Option<String>,
    pub password: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainRenameArgs {
    pub dom: RemoteNonnullDomain,
    pub new_name: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainRenameRet {
    pub retcode: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackMigrationIterationMsg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub iteration: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackJobCompletedMsg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainMigrateStartPostCopyArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackDeviceRemovalFailedMsg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub dev_alias: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetGuestVcpusArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetGuestVcpusRet {
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetGuestVcpusArgs {
    pub dom: RemoteNonnullDomain,
    pub cpumap: String,
    pub state: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetVcpuArgs {
    pub dom: RemoteNonnullDomain,
    pub cpumap: String,
    pub state: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventCallbackMetadataChangeMsg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub r#type: i32,
    pub nsuri: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventMemoryFailureMsg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub recipient: i32,
    pub action: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectSecretEventRegisterAnyArgs {
    pub event_id: i32,
    pub secret: Option<RemoteNonnullSecret>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectSecretEventRegisterAnyRet {
    pub callback_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectSecretEventDeregisterAnyArgs {
    pub callback_id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretEventLifecycleMsg {
    pub callback_id: i32,
    pub secret: RemoteNonnullSecret,
    pub event: i32,
    pub detail: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteSecretEventValueChangedMsg {
    pub callback_id: i32,
    pub secret: RemoteNonnullSecret,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetBlockThresholdArgs {
    pub dom: RemoteNonnullDomain,
    pub dev: String,
    pub threshold: u64,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetLifecycleActionArgs {
    pub dom: RemoteNonnullDomain,
    pub r#type: u32,
    pub action: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectCompareHypervisorCpuArgs {
    pub emulator: Option<String>,
    pub arch: Option<String>,
    pub machine: Option<String>,
    pub virttype: Option<String>,
    pub xml_cpu: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectCompareHypervisorCpuRet {
    pub result: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectBaselineHypervisorCpuArgs {
    pub emulator: Option<String>,
    pub arch: Option<String>,
    pub machine: Option<String>,
    pub virttype: Option<String>,
    pub xml_cpus: Vec<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectBaselineHypervisorCpuRet {
    pub cpu: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetSevInfoArgs {
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNodeGetSevInfoRet {
    pub params: Vec<RemoteTypedParam>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetLaunchSecurityInfoArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetLaunchSecurityInfoRet {
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainSetLaunchSecurityStateArgs {
    pub dom: RemoteNonnullDomain,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterBindingLookupByPortDevArgs {
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterBindingLookupByPortDevRet {
    pub nwfilter: RemoteNonnullNwfilterBinding,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterBindingCreateXmlArgs {
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterBindingCreateXmlRet {
    pub nwfilter: RemoteNonnullNwfilterBinding,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterBindingDeleteArgs {
    pub nwfilter: RemoteNonnullNwfilterBinding,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterBindingGetXmlDescArgs {
    pub nwfilter: RemoteNonnullNwfilterBinding,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNwfilterBindingGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllNwfilterBindingsArgs {
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectListAllNwfilterBindingsRet {
    pub bindings: Vec<RemoteNonnullNwfilterBinding>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetStoragePoolCapabilitiesArgs {
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectGetStoragePoolCapabilitiesRet {
    pub capabilities: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkListAllPortsArgs {
    pub network: RemoteNonnullNetwork,
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkListAllPortsRet {
    pub ports: Vec<RemoteNonnullNetworkPort>,
    pub ret: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkPortLookupByUuidArgs {
    pub network: RemoteNonnullNetwork,
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkPortLookupByUuidRet {
    pub port: RemoteNonnullNetworkPort,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkPortCreateXmlArgs {
    pub network: RemoteNonnullNetwork,
    pub xml: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkPortCreateXmlRet {
    pub port: RemoteNonnullNetworkPort,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkPortSetParametersArgs {
    pub port: RemoteNonnullNetworkPort,
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkPortGetParametersArgs {
    pub port: RemoteNonnullNetworkPort,
    pub nparams: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkPortGetParametersRet {
    pub params: Vec<RemoteTypedParam>,
    pub nparams: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkPortGetXmlDescArgs {
    pub port: RemoteNonnullNetworkPort,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkPortGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteNetworkPortDeleteArgs {
    pub port: RemoteNonnullNetworkPort,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointCreateXmlArgs {
    pub dom: RemoteNonnullDomain,
    pub xml_desc: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointCreateXmlRet {
    pub checkpoint: RemoteNonnullDomainCheckpoint,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointGetXmlDescArgs {
    pub checkpoint: RemoteNonnullDomainCheckpoint,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainListAllCheckpointsArgs {
    pub dom: RemoteNonnullDomain,
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainListAllCheckpointsRet {
    pub checkpoints: Vec<RemoteNonnullDomainCheckpoint>,
    pub ret: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointListAllChildrenArgs {
    pub checkpoint: RemoteNonnullDomainCheckpoint,
    pub need_results: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointListAllChildrenRet {
    pub checkpoints: Vec<RemoteNonnullDomainCheckpoint>,
    pub ret: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointLookupByNameArgs {
    pub dom: RemoteNonnullDomain,
    pub name: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointLookupByNameRet {
    pub checkpoint: RemoteNonnullDomainCheckpoint,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointGetParentArgs {
    pub checkpoint: RemoteNonnullDomainCheckpoint,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointGetParentRet {
    pub parent: RemoteNonnullDomainCheckpoint,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainCheckpointDeleteArgs {
    pub checkpoint: RemoteNonnullDomainCheckpoint,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetGuestInfoArgs {
    pub dom: RemoteNonnullDomain,
    pub types: u32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetGuestInfoRet {
    pub params: Vec<RemoteTypedParam>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteConnectSetIdentityArgs {
    pub params: Vec<RemoteTypedParam>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainAgentSetResponseTimeoutArgs {
    pub dom: RemoteNonnullDomain,
    pub timeout: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainAgentSetResponseTimeoutRet {
    pub result: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBackupBeginArgs {
    pub dom: RemoteNonnullDomain,
    pub backup_xml: String,
    pub checkpoint_xml: Option<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBackupGetXmlDescArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainBackupGetXmlDescRet {
    pub xml: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainAuthorizedSshKeysGetArgs {
    pub dom: RemoteNonnullDomain,
    pub user: String,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainAuthorizedSshKeysGetRet {
    pub keys: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainAuthorizedSshKeysSetArgs {
    pub dom: RemoteNonnullDomain,
    pub user: String,
    pub keys: Vec<String>,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetMessagesArgs {
    pub dom: RemoteNonnullDomain,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainGetMessagesRet {
    pub msgs: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainStartDirtyRateCalcArgs {
    pub dom: RemoteNonnullDomain,
    pub seconds: i32,
    pub flags: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteDomainEventMemoryDeviceSizeChangeMsg {
    pub callback_id: i32,
    pub dom: RemoteNonnullDomain,
    pub alias: String,
    pub size: u64,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum RemoteProcedure {
    _Reserved0 = 0i32,
    RemoteProcConnectOpen = 1i32,
    RemoteProcConnectClose = 2i32,
    RemoteProcConnectGetType = 3i32,
    RemoteProcConnectGetVersion = 4i32,
    RemoteProcConnectGetMaxVcpus = 5i32,
    RemoteProcNodeGetInfo = 6i32,
    RemoteProcConnectGetCapabilities = 7i32,
    RemoteProcDomainAttachDevice = 8i32,
    RemoteProcDomainCreate = 9i32,
    RemoteProcDomainCreateXml = 10i32,
    RemoteProcDomainDefineXml = 11i32,
    RemoteProcDomainDestroy = 12i32,
    RemoteProcDomainDetachDevice = 13i32,
    RemoteProcDomainGetXmlDesc = 14i32,
    RemoteProcDomainGetAutostart = 15i32,
    RemoteProcDomainGetInfo = 16i32,
    RemoteProcDomainGetMaxMemory = 17i32,
    RemoteProcDomainGetMaxVcpus = 18i32,
    RemoteProcDomainGetOsType = 19i32,
    RemoteProcDomainGetVcpus = 20i32,
    RemoteProcConnectListDefinedDomains = 21i32,
    RemoteProcDomainLookupById = 22i32,
    RemoteProcDomainLookupByName = 23i32,
    RemoteProcDomainLookupByUuid = 24i32,
    RemoteProcConnectNumOfDefinedDomains = 25i32,
    RemoteProcDomainPinVcpu = 26i32,
    RemoteProcDomainReboot = 27i32,
    RemoteProcDomainResume = 28i32,
    RemoteProcDomainSetAutostart = 29i32,
    RemoteProcDomainSetMaxMemory = 30i32,
    RemoteProcDomainSetMemory = 31i32,
    RemoteProcDomainSetVcpus = 32i32,
    RemoteProcDomainShutdown = 33i32,
    RemoteProcDomainSuspend = 34i32,
    RemoteProcDomainUndefine = 35i32,
    RemoteProcConnectListDefinedNetworks = 36i32,
    RemoteProcConnectListDomains = 37i32,
    RemoteProcConnectListNetworks = 38i32,
    RemoteProcNetworkCreate = 39i32,
    RemoteProcNetworkCreateXml = 40i32,
    RemoteProcNetworkDefineXml = 41i32,
    RemoteProcNetworkDestroy = 42i32,
    RemoteProcNetworkGetXmlDesc = 43i32,
    RemoteProcNetworkGetAutostart = 44i32,
    RemoteProcNetworkGetBridgeName = 45i32,
    RemoteProcNetworkLookupByName = 46i32,
    RemoteProcNetworkLookupByUuid = 47i32,
    RemoteProcNetworkSetAutostart = 48i32,
    RemoteProcNetworkUndefine = 49i32,
    RemoteProcConnectNumOfDefinedNetworks = 50i32,
    RemoteProcConnectNumOfDomains = 51i32,
    RemoteProcConnectNumOfNetworks = 52i32,
    RemoteProcDomainCoreDump = 53i32,
    RemoteProcDomainRestore = 54i32,
    RemoteProcDomainSave = 55i32,
    RemoteProcDomainGetSchedulerType = 56i32,
    RemoteProcDomainGetSchedulerParameters = 57i32,
    RemoteProcDomainSetSchedulerParameters = 58i32,
    RemoteProcConnectGetHostname = 59i32,
    RemoteProcConnectSupportsFeature = 60i32,
    RemoteProcDomainMigratePrepare = 61i32,
    RemoteProcDomainMigratePerform = 62i32,
    RemoteProcDomainMigrateFinish = 63i32,
    RemoteProcDomainBlockStats = 64i32,
    RemoteProcDomainInterfaceStats = 65i32,
    RemoteProcAuthList = 66i32,
    RemoteProcAuthSaslInit = 67i32,
    RemoteProcAuthSaslStart = 68i32,
    RemoteProcAuthSaslStep = 69i32,
    RemoteProcAuthPolkit = 70i32,
    RemoteProcConnectNumOfStoragePools = 71i32,
    RemoteProcConnectListStoragePools = 72i32,
    RemoteProcConnectNumOfDefinedStoragePools = 73i32,
    RemoteProcConnectListDefinedStoragePools = 74i32,
    RemoteProcConnectFindStoragePoolSources = 75i32,
    RemoteProcStoragePoolCreateXml = 76i32,
    RemoteProcStoragePoolDefineXml = 77i32,
    RemoteProcStoragePoolCreate = 78i32,
    RemoteProcStoragePoolBuild = 79i32,
    RemoteProcStoragePoolDestroy = 80i32,
    RemoteProcStoragePoolDelete = 81i32,
    RemoteProcStoragePoolUndefine = 82i32,
    RemoteProcStoragePoolRefresh = 83i32,
    RemoteProcStoragePoolLookupByName = 84i32,
    RemoteProcStoragePoolLookupByUuid = 85i32,
    RemoteProcStoragePoolLookupByVolume = 86i32,
    RemoteProcStoragePoolGetInfo = 87i32,
    RemoteProcStoragePoolGetXmlDesc = 88i32,
    RemoteProcStoragePoolGetAutostart = 89i32,
    RemoteProcStoragePoolSetAutostart = 90i32,
    RemoteProcStoragePoolNumOfVolumes = 91i32,
    RemoteProcStoragePoolListVolumes = 92i32,
    RemoteProcStorageVolCreateXml = 93i32,
    RemoteProcStorageVolDelete = 94i32,
    RemoteProcStorageVolLookupByName = 95i32,
    RemoteProcStorageVolLookupByKey = 96i32,
    RemoteProcStorageVolLookupByPath = 97i32,
    RemoteProcStorageVolGetInfo = 98i32,
    RemoteProcStorageVolGetXmlDesc = 99i32,
    RemoteProcStorageVolGetPath = 100i32,
    RemoteProcNodeGetCellsFreeMemory = 101i32,
    RemoteProcNodeGetFreeMemory = 102i32,
    RemoteProcDomainBlockPeek = 103i32,
    RemoteProcDomainMemoryPeek = 104i32,
    RemoteProcConnectDomainEventRegister = 105i32,
    RemoteProcConnectDomainEventDeregister = 106i32,
    RemoteProcDomainEventLifecycle = 107i32,
    RemoteProcDomainMigratePrepare2 = 108i32,
    RemoteProcDomainMigrateFinish2 = 109i32,
    RemoteProcConnectGetUri = 110i32,
    RemoteProcNodeNumOfDevices = 111i32,
    RemoteProcNodeListDevices = 112i32,
    RemoteProcNodeDeviceLookupByName = 113i32,
    RemoteProcNodeDeviceGetXmlDesc = 114i32,
    RemoteProcNodeDeviceGetParent = 115i32,
    RemoteProcNodeDeviceNumOfCaps = 116i32,
    RemoteProcNodeDeviceListCaps = 117i32,
    RemoteProcNodeDeviceDettach = 118i32,
    RemoteProcNodeDeviceReAttach = 119i32,
    RemoteProcNodeDeviceReset = 120i32,
    RemoteProcDomainGetSecurityLabel = 121i32,
    RemoteProcNodeGetSecurityModel = 122i32,
    RemoteProcNodeDeviceCreateXml = 123i32,
    RemoteProcNodeDeviceDestroy = 124i32,
    RemoteProcStorageVolCreateXmlFrom = 125i32,
    RemoteProcConnectNumOfInterfaces = 126i32,
    RemoteProcConnectListInterfaces = 127i32,
    RemoteProcInterfaceLookupByName = 128i32,
    RemoteProcInterfaceLookupByMacString = 129i32,
    RemoteProcInterfaceGetXmlDesc = 130i32,
    RemoteProcInterfaceDefineXml = 131i32,
    RemoteProcInterfaceUndefine = 132i32,
    RemoteProcInterfaceCreate = 133i32,
    RemoteProcInterfaceDestroy = 134i32,
    RemoteProcConnectDomainXmlFromNative = 135i32,
    RemoteProcConnectDomainXmlToNative = 136i32,
    RemoteProcConnectNumOfDefinedInterfaces = 137i32,
    RemoteProcConnectListDefinedInterfaces = 138i32,
    RemoteProcConnectNumOfSecrets = 139i32,
    RemoteProcConnectListSecrets = 140i32,
    RemoteProcSecretLookupByUuid = 141i32,
    RemoteProcSecretDefineXml = 142i32,
    RemoteProcSecretGetXmlDesc = 143i32,
    RemoteProcSecretSetValue = 144i32,
    RemoteProcSecretGetValue = 145i32,
    RemoteProcSecretUndefine = 146i32,
    RemoteProcSecretLookupByUsage = 147i32,
    RemoteProcDomainMigratePrepareTunnel = 148i32,
    RemoteProcConnectIsSecure = 149i32,
    RemoteProcDomainIsActive = 150i32,
    RemoteProcDomainIsPersistent = 151i32,
    RemoteProcNetworkIsActive = 152i32,
    RemoteProcNetworkIsPersistent = 153i32,
    RemoteProcStoragePoolIsActive = 154i32,
    RemoteProcStoragePoolIsPersistent = 155i32,
    RemoteProcInterfaceIsActive = 156i32,
    RemoteProcConnectGetLibVersion = 157i32,
    RemoteProcConnectCompareCpu = 158i32,
    RemoteProcDomainMemoryStats = 159i32,
    RemoteProcDomainAttachDeviceFlags = 160i32,
    RemoteProcDomainDetachDeviceFlags = 161i32,
    RemoteProcConnectBaselineCpu = 162i32,
    RemoteProcDomainGetJobInfo = 163i32,
    RemoteProcDomainAbortJob = 164i32,
    RemoteProcStorageVolWipe = 165i32,
    RemoteProcDomainMigrateSetMaxDowntime = 166i32,
    RemoteProcConnectDomainEventRegisterAny = 167i32,
    RemoteProcConnectDomainEventDeregisterAny = 168i32,
    RemoteProcDomainEventReboot = 169i32,
    RemoteProcDomainEventRtcChange = 170i32,
    RemoteProcDomainEventWatchdog = 171i32,
    RemoteProcDomainEventIoError = 172i32,
    RemoteProcDomainEventGraphics = 173i32,
    RemoteProcDomainUpdateDeviceFlags = 174i32,
    RemoteProcNwfilterLookupByName = 175i32,
    RemoteProcNwfilterLookupByUuid = 176i32,
    RemoteProcNwfilterGetXmlDesc = 177i32,
    RemoteProcConnectNumOfNwfilters = 178i32,
    RemoteProcConnectListNwfilters = 179i32,
    RemoteProcNwfilterDefineXml = 180i32,
    RemoteProcNwfilterUndefine = 181i32,
    RemoteProcDomainManagedSave = 182i32,
    RemoteProcDomainHasManagedSaveImage = 183i32,
    RemoteProcDomainManagedSaveRemove = 184i32,
    RemoteProcDomainSnapshotCreateXml = 185i32,
    RemoteProcDomainSnapshotGetXmlDesc = 186i32,
    RemoteProcDomainSnapshotNum = 187i32,
    RemoteProcDomainSnapshotListNames = 188i32,
    RemoteProcDomainSnapshotLookupByName = 189i32,
    RemoteProcDomainHasCurrentSnapshot = 190i32,
    RemoteProcDomainSnapshotCurrent = 191i32,
    RemoteProcDomainRevertToSnapshot = 192i32,
    RemoteProcDomainSnapshotDelete = 193i32,
    RemoteProcDomainGetBlockInfo = 194i32,
    RemoteProcDomainEventIoErrorReason = 195i32,
    RemoteProcDomainCreateWithFlags = 196i32,
    RemoteProcDomainSetMemoryParameters = 197i32,
    RemoteProcDomainGetMemoryParameters = 198i32,
    RemoteProcDomainSetVcpusFlags = 199i32,
    RemoteProcDomainGetVcpusFlags = 200i32,
    RemoteProcDomainOpenConsole = 201i32,
    RemoteProcDomainIsUpdated = 202i32,
    RemoteProcConnectGetSysinfo = 203i32,
    RemoteProcDomainSetMemoryFlags = 204i32,
    RemoteProcDomainSetBlkioParameters = 205i32,
    RemoteProcDomainGetBlkioParameters = 206i32,
    RemoteProcDomainMigrateSetMaxSpeed = 207i32,
    RemoteProcStorageVolUpload = 208i32,
    RemoteProcStorageVolDownload = 209i32,
    RemoteProcDomainInjectNmi = 210i32,
    RemoteProcDomainScreenshot = 211i32,
    RemoteProcDomainGetState = 212i32,
    RemoteProcDomainMigrateBegin3 = 213i32,
    RemoteProcDomainMigratePrepare3 = 214i32,
    RemoteProcDomainMigratePrepareTunnel3 = 215i32,
    RemoteProcDomainMigratePerform3 = 216i32,
    RemoteProcDomainMigrateFinish3 = 217i32,
    RemoteProcDomainMigrateConfirm3 = 218i32,
    RemoteProcDomainSetSchedulerParametersFlags = 219i32,
    RemoteProcInterfaceChangeBegin = 220i32,
    RemoteProcInterfaceChangeCommit = 221i32,
    RemoteProcInterfaceChangeRollback = 222i32,
    RemoteProcDomainGetSchedulerParametersFlags = 223i32,
    RemoteProcDomainEventControlError = 224i32,
    RemoteProcDomainPinVcpuFlags = 225i32,
    RemoteProcDomainSendKey = 226i32,
    RemoteProcNodeGetCpuStats = 227i32,
    RemoteProcNodeGetMemoryStats = 228i32,
    RemoteProcDomainGetControlInfo = 229i32,
    RemoteProcDomainGetVcpuPinInfo = 230i32,
    RemoteProcDomainUndefineFlags = 231i32,
    RemoteProcDomainSaveFlags = 232i32,
    RemoteProcDomainRestoreFlags = 233i32,
    RemoteProcDomainDestroyFlags = 234i32,
    RemoteProcDomainSaveImageGetXmlDesc = 235i32,
    RemoteProcDomainSaveImageDefineXml = 236i32,
    RemoteProcDomainBlockJobAbort = 237i32,
    RemoteProcDomainGetBlockJobInfo = 238i32,
    RemoteProcDomainBlockJobSetSpeed = 239i32,
    RemoteProcDomainBlockPull = 240i32,
    RemoteProcDomainEventBlockJob = 241i32,
    RemoteProcDomainMigrateGetMaxSpeed = 242i32,
    RemoteProcDomainBlockStatsFlags = 243i32,
    RemoteProcDomainSnapshotGetParent = 244i32,
    RemoteProcDomainReset = 245i32,
    RemoteProcDomainSnapshotNumChildren = 246i32,
    RemoteProcDomainSnapshotListChildrenNames = 247i32,
    RemoteProcDomainEventDiskChange = 248i32,
    RemoteProcDomainOpenGraphics = 249i32,
    RemoteProcNodeSuspendForDuration = 250i32,
    RemoteProcDomainBlockResize = 251i32,
    RemoteProcDomainSetBlockIoTune = 252i32,
    RemoteProcDomainGetBlockIoTune = 253i32,
    RemoteProcDomainSetNumaParameters = 254i32,
    RemoteProcDomainGetNumaParameters = 255i32,
    RemoteProcDomainSetInterfaceParameters = 256i32,
    RemoteProcDomainGetInterfaceParameters = 257i32,
    RemoteProcDomainShutdownFlags = 258i32,
    RemoteProcStorageVolWipePattern = 259i32,
    RemoteProcStorageVolResize = 260i32,
    RemoteProcDomainPmSuspendForDuration = 261i32,
    RemoteProcDomainGetCpuStats = 262i32,
    RemoteProcDomainGetDiskErrors = 263i32,
    RemoteProcDomainSetMetadata = 264i32,
    RemoteProcDomainGetMetadata = 265i32,
    RemoteProcDomainBlockRebase = 266i32,
    RemoteProcDomainPmWakeup = 267i32,
    RemoteProcDomainEventTrayChange = 268i32,
    RemoteProcDomainEventPmwakeup = 269i32,
    RemoteProcDomainEventPmsuspend = 270i32,
    RemoteProcDomainSnapshotIsCurrent = 271i32,
    RemoteProcDomainSnapshotHasMetadata = 272i32,
    RemoteProcConnectListAllDomains = 273i32,
    RemoteProcDomainListAllSnapshots = 274i32,
    RemoteProcDomainSnapshotListAllChildren = 275i32,
    RemoteProcDomainEventBalloonChange = 276i32,
    RemoteProcDomainGetHostname = 277i32,
    RemoteProcDomainGetSecurityLabelList = 278i32,
    RemoteProcDomainPinEmulator = 279i32,
    RemoteProcDomainGetEmulatorPinInfo = 280i32,
    RemoteProcConnectListAllStoragePools = 281i32,
    RemoteProcStoragePoolListAllVolumes = 282i32,
    RemoteProcConnectListAllNetworks = 283i32,
    RemoteProcConnectListAllInterfaces = 284i32,
    RemoteProcConnectListAllNodeDevices = 285i32,
    RemoteProcConnectListAllNwfilters = 286i32,
    RemoteProcConnectListAllSecrets = 287i32,
    RemoteProcNodeSetMemoryParameters = 288i32,
    RemoteProcNodeGetMemoryParameters = 289i32,
    RemoteProcDomainBlockCommit = 290i32,
    RemoteProcNetworkUpdate = 291i32,
    RemoteProcDomainEventPmsuspendDisk = 292i32,
    RemoteProcNodeGetCpuMap = 293i32,
    RemoteProcDomainFstrim = 294i32,
    RemoteProcDomainSendProcessSignal = 295i32,
    RemoteProcDomainOpenChannel = 296i32,
    RemoteProcNodeDeviceLookupScsiHostByWwn = 297i32,
    RemoteProcDomainGetJobStats = 298i32,
    RemoteProcDomainMigrateGetCompressionCache = 299i32,
    RemoteProcDomainMigrateSetCompressionCache = 300i32,
    RemoteProcNodeDeviceDetachFlags = 301i32,
    RemoteProcDomainMigrateBegin3Params = 302i32,
    RemoteProcDomainMigratePrepare3Params = 303i32,
    RemoteProcDomainMigratePrepareTunnel3Params = 304i32,
    RemoteProcDomainMigratePerform3Params = 305i32,
    RemoteProcDomainMigrateFinish3Params = 306i32,
    RemoteProcDomainMigrateConfirm3Params = 307i32,
    RemoteProcDomainSetMemoryStatsPeriod = 308i32,
    RemoteProcDomainCreateXmlWithFiles = 309i32,
    RemoteProcDomainCreateWithFiles = 310i32,
    RemoteProcDomainEventDeviceRemoved = 311i32,
    RemoteProcConnectGetCpuModelNames = 312i32,
    RemoteProcConnectNetworkEventRegisterAny = 313i32,
    RemoteProcConnectNetworkEventDeregisterAny = 314i32,
    RemoteProcNetworkEventLifecycle = 315i32,
    RemoteProcConnectDomainEventCallbackRegisterAny = 316i32,
    RemoteProcConnectDomainEventCallbackDeregisterAny = 317i32,
    RemoteProcDomainEventCallbackLifecycle = 318i32,
    RemoteProcDomainEventCallbackReboot = 319i32,
    RemoteProcDomainEventCallbackRtcChange = 320i32,
    RemoteProcDomainEventCallbackWatchdog = 321i32,
    RemoteProcDomainEventCallbackIoError = 322i32,
    RemoteProcDomainEventCallbackGraphics = 323i32,
    RemoteProcDomainEventCallbackIoErrorReason = 324i32,
    RemoteProcDomainEventCallbackControlError = 325i32,
    RemoteProcDomainEventCallbackBlockJob = 326i32,
    RemoteProcDomainEventCallbackDiskChange = 327i32,
    RemoteProcDomainEventCallbackTrayChange = 328i32,
    RemoteProcDomainEventCallbackPmwakeup = 329i32,
    RemoteProcDomainEventCallbackPmsuspend = 330i32,
    RemoteProcDomainEventCallbackBalloonChange = 331i32,
    RemoteProcDomainEventCallbackPmsuspendDisk = 332i32,
    RemoteProcDomainEventCallbackDeviceRemoved = 333i32,
    RemoteProcDomainCoreDumpWithFormat = 334i32,
    RemoteProcDomainFsfreeze = 335i32,
    RemoteProcDomainFsthaw = 336i32,
    RemoteProcDomainGetTime = 337i32,
    RemoteProcDomainSetTime = 338i32,
    RemoteProcDomainEventBlockJob2 = 339i32,
    RemoteProcNodeGetFreePages = 340i32,
    RemoteProcNetworkGetDhcpLeases = 341i32,
    RemoteProcConnectGetDomainCapabilities = 342i32,
    RemoteProcDomainOpenGraphicsFd = 343i32,
    RemoteProcConnectGetAllDomainStats = 344i32,
    RemoteProcDomainBlockCopy = 345i32,
    RemoteProcDomainEventCallbackTunable = 346i32,
    RemoteProcNodeAllocPages = 347i32,
    RemoteProcDomainEventCallbackAgentLifecycle = 348i32,
    RemoteProcDomainGetFsinfo = 349i32,
    RemoteProcDomainDefineXmlFlags = 350i32,
    RemoteProcDomainGetIothreadInfo = 351i32,
    RemoteProcDomainPinIothread = 352i32,
    RemoteProcDomainInterfaceAddresses = 353i32,
    RemoteProcDomainEventCallbackDeviceAdded = 354i32,
    RemoteProcDomainAddIothread = 355i32,
    RemoteProcDomainDelIothread = 356i32,
    RemoteProcDomainSetUserPassword = 357i32,
    RemoteProcDomainRename = 358i32,
    RemoteProcDomainEventCallbackMigrationIteration = 359i32,
    RemoteProcConnectRegisterCloseCallback = 360i32,
    RemoteProcConnectUnregisterCloseCallback = 361i32,
    RemoteProcConnectEventConnectionClosed = 362i32,
    RemoteProcDomainEventCallbackJobCompleted = 363i32,
    RemoteProcDomainMigrateStartPostCopy = 364i32,
    RemoteProcDomainGetPerfEvents = 365i32,
    RemoteProcDomainSetPerfEvents = 366i32,
    RemoteProcDomainEventCallbackDeviceRemovalFailed = 367i32,
    RemoteProcConnectStoragePoolEventRegisterAny = 368i32,
    RemoteProcConnectStoragePoolEventDeregisterAny = 369i32,
    RemoteProcStoragePoolEventLifecycle = 370i32,
    RemoteProcDomainGetGuestVcpus = 371i32,
    RemoteProcDomainSetGuestVcpus = 372i32,
    RemoteProcStoragePoolEventRefresh = 373i32,
    RemoteProcConnectNodeDeviceEventRegisterAny = 374i32,
    RemoteProcConnectNodeDeviceEventDeregisterAny = 375i32,
    RemoteProcNodeDeviceEventLifecycle = 376i32,
    RemoteProcNodeDeviceEventUpdate = 377i32,
    RemoteProcStorageVolGetInfoFlags = 378i32,
    RemoteProcDomainEventCallbackMetadataChange = 379i32,
    RemoteProcConnectSecretEventRegisterAny = 380i32,
    RemoteProcConnectSecretEventDeregisterAny = 381i32,
    RemoteProcSecretEventLifecycle = 382i32,
    RemoteProcSecretEventValueChanged = 383i32,
    RemoteProcDomainSetVcpu = 384i32,
    RemoteProcDomainEventBlockThreshold = 385i32,
    RemoteProcDomainSetBlockThreshold = 386i32,
    RemoteProcDomainMigrateGetMaxDowntime = 387i32,
    RemoteProcDomainManagedSaveGetXmlDesc = 388i32,
    RemoteProcDomainManagedSaveDefineXml = 389i32,
    RemoteProcDomainSetLifecycleAction = 390i32,
    RemoteProcStoragePoolLookupByTargetPath = 391i32,
    RemoteProcDomainDetachDeviceAlias = 392i32,
    RemoteProcConnectCompareHypervisorCpu = 393i32,
    RemoteProcConnectBaselineHypervisorCpu = 394i32,
    RemoteProcNodeGetSevInfo = 395i32,
    RemoteProcDomainGetLaunchSecurityInfo = 396i32,
    RemoteProcNwfilterBindingLookupByPortDev = 397i32,
    RemoteProcNwfilterBindingGetXmlDesc = 398i32,
    RemoteProcNwfilterBindingCreateXml = 399i32,
    RemoteProcNwfilterBindingDelete = 400i32,
    RemoteProcConnectListAllNwfilterBindings = 401i32,
    RemoteProcDomainSetIothreadParams = 402i32,
    RemoteProcConnectGetStoragePoolCapabilities = 403i32,
    RemoteProcNetworkListAllPorts = 404i32,
    RemoteProcNetworkPortLookupByUuid = 405i32,
    RemoteProcNetworkPortCreateXml = 406i32,
    RemoteProcNetworkPortGetParameters = 407i32,
    RemoteProcNetworkPortSetParameters = 408i32,
    RemoteProcNetworkPortGetXmlDesc = 409i32,
    RemoteProcNetworkPortDelete = 410i32,
    RemoteProcDomainCheckpointCreateXml = 411i32,
    RemoteProcDomainCheckpointGetXmlDesc = 412i32,
    RemoteProcDomainListAllCheckpoints = 413i32,
    RemoteProcDomainCheckpointListAllChildren = 414i32,
    RemoteProcDomainCheckpointLookupByName = 415i32,
    RemoteProcDomainCheckpointGetParent = 416i32,
    RemoteProcDomainCheckpointDelete = 417i32,
    RemoteProcDomainGetGuestInfo = 418i32,
    RemoteProcConnectSetIdentity = 419i32,
    RemoteProcDomainAgentSetResponseTimeout = 420i32,
    RemoteProcDomainBackupBegin = 421i32,
    RemoteProcDomainBackupGetXmlDesc = 422i32,
    RemoteProcDomainEventMemoryFailure = 423i32,
    RemoteProcDomainAuthorizedSshKeysGet = 424i32,
    RemoteProcDomainAuthorizedSshKeysSet = 425i32,
    RemoteProcDomainGetMessages = 426i32,
    RemoteProcDomainStartDirtyRateCalc = 427i32,
    RemoteProcNodeDeviceDefineXml = 428i32,
    RemoteProcNodeDeviceUndefine = 429i32,
    RemoteProcNodeDeviceCreate = 430i32,
    RemoteProcNwfilterDefineXmlFlags = 431i32,
    RemoteProcNetworkDefineXmlFlags = 432i32,
    RemoteProcNodeDeviceGetAutostart = 433i32,
    RemoteProcNodeDeviceSetAutostart = 434i32,
    RemoteProcNodeDeviceIsPersistent = 435i32,
    RemoteProcNodeDeviceIsActive = 436i32,
    RemoteProcNetworkCreateXmlFlags = 437i32,
    RemoteProcDomainEventMemoryDeviceSizeChange = 438i32,
    RemoteProcDomainSetLaunchSecurityState = 439i32,
}
