use crate::binding;
use crate::error::Error;
use crate::protocol;
use log::trace;
use serde::{de::DeserializeOwned, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
#[cfg(target_family = "unix")]
use std::os::unix::net::UnixStream;
pub trait ReadWrite: Read + Write {}
impl ReadWrite for TcpStream {}
#[cfg(target_family = "unix")]
impl ReadWrite for UnixStream {}
pub struct Client {
    inner: Box<dyn ReadWrite>,
    serial: u32,
}
impl Client {
    pub fn new(socket: impl ReadWrite + 'static) -> Self {
        Client {
            inner: Box::new(socket),
            serial: 0,
        }
    }
}
impl Libvirt for Client {
    fn inner(&mut self) -> &mut Box<dyn ReadWrite> {
        &mut self.inner
    }
    fn serial(&self) -> u32 {
        self.serial
    }
    fn serial_add(&mut self, value: u32) {
        self.serial += value;
    }
}
pub trait Libvirt {
    fn inner(&mut self) -> &mut Box<dyn ReadWrite>;
    fn serial(&self) -> u32;
    fn serial_add(&mut self, value: u32);
    fn connect_open(&mut self, args: binding::RemoteConnectOpenArgs) -> Result<(), Error> {
        trace!("{}", stringify!(connect_open));
        let req: Option<binding::RemoteConnectOpenArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcConnectOpen, req)?;
        Ok(())
    }
    fn connect_close(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_close));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcConnectClose, req)?;
        Ok(())
    }
    fn connect_get_type(&mut self) -> Result<binding::RemoteConnectGetTypeRet, Error> {
        trace!("{}", stringify!(connect_get_type));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectGetTypeRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetType,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_get_version(&mut self) -> Result<binding::RemoteConnectGetVersionRet, Error> {
        trace!("{}", stringify!(connect_get_version));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectGetVersionRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetVersion,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_get_max_vcpus(
        &mut self,
        args: binding::RemoteConnectGetMaxVcpusArgs,
    ) -> Result<binding::RemoteConnectGetMaxVcpusRet, Error> {
        trace!("{}", stringify!(connect_get_max_vcpus));
        let req: Option<binding::RemoteConnectGetMaxVcpusArgs> = Some(args);
        let res: Option<binding::RemoteConnectGetMaxVcpusRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetMaxVcpus,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_get_info(&mut self) -> Result<binding::RemoteNodeGetInfoRet, Error> {
        trace!("{}", stringify!(node_get_info));
        let req: Option<()> = None;
        let res: Option<binding::RemoteNodeGetInfoRet> =
            call(self, binding::RemoteProcedure::RemoteProcNodeGetInfo, req)?;
        Ok(res.unwrap())
    }
    fn connect_get_capabilities(
        &mut self,
    ) -> Result<binding::RemoteConnectGetCapabilitiesRet, Error> {
        trace!("{}", stringify!(connect_get_capabilities));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectGetCapabilitiesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetCapabilities,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_attach_device(
        &mut self,
        args: binding::RemoteDomainAttachDeviceArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_attach_device));
        let req: Option<binding::RemoteDomainAttachDeviceArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainAttachDevice,
            req,
        )?;
        Ok(())
    }
    fn domain_create(&mut self, args: binding::RemoteDomainCreateArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_create));
        let req: Option<binding::RemoteDomainCreateArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainCreate, req)?;
        Ok(())
    }
    fn domain_create_xml(
        &mut self,
        args: binding::RemoteDomainCreateXmlArgs,
    ) -> Result<binding::RemoteDomainCreateXmlRet, Error> {
        trace!("{}", stringify!(domain_create_xml));
        let req: Option<binding::RemoteDomainCreateXmlArgs> = Some(args);
        let res: Option<binding::RemoteDomainCreateXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCreateXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_define_xml(
        &mut self,
        args: binding::RemoteDomainDefineXmlArgs,
    ) -> Result<binding::RemoteDomainDefineXmlRet, Error> {
        trace!("{}", stringify!(domain_define_xml));
        let req: Option<binding::RemoteDomainDefineXmlArgs> = Some(args);
        let res: Option<binding::RemoteDomainDefineXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainDefineXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_destroy(&mut self, args: binding::RemoteDomainDestroyArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_destroy));
        let req: Option<binding::RemoteDomainDestroyArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainDestroy, req)?;
        Ok(())
    }
    fn domain_detach_device(
        &mut self,
        args: binding::RemoteDomainDetachDeviceArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_detach_device));
        let req: Option<binding::RemoteDomainDetachDeviceArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainDetachDevice,
            req,
        )?;
        Ok(())
    }
    fn domain_get_xml_desc(
        &mut self,
        args: binding::RemoteDomainGetXmlDescArgs,
    ) -> Result<binding::RemoteDomainGetXmlDescRet, Error> {
        trace!("{}", stringify!(domain_get_xml_desc));
        let req: Option<binding::RemoteDomainGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_autostart(
        &mut self,
        args: binding::RemoteDomainGetAutostartArgs,
    ) -> Result<binding::RemoteDomainGetAutostartRet, Error> {
        trace!("{}", stringify!(domain_get_autostart));
        let req: Option<binding::RemoteDomainGetAutostartArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetAutostartRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetAutostart,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_info(
        &mut self,
        args: binding::RemoteDomainGetInfoArgs,
    ) -> Result<binding::RemoteDomainGetInfoRet, Error> {
        trace!("{}", stringify!(domain_get_info));
        let req: Option<binding::RemoteDomainGetInfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetInfoRet> =
            call(self, binding::RemoteProcedure::RemoteProcDomainGetInfo, req)?;
        Ok(res.unwrap())
    }
    fn domain_get_max_memory(
        &mut self,
        args: binding::RemoteDomainGetMaxMemoryArgs,
    ) -> Result<binding::RemoteDomainGetMaxMemoryRet, Error> {
        trace!("{}", stringify!(domain_get_max_memory));
        let req: Option<binding::RemoteDomainGetMaxMemoryArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetMaxMemoryRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetMaxMemory,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_max_vcpus(
        &mut self,
        args: binding::RemoteDomainGetMaxVcpusArgs,
    ) -> Result<binding::RemoteDomainGetMaxVcpusRet, Error> {
        trace!("{}", stringify!(domain_get_max_vcpus));
        let req: Option<binding::RemoteDomainGetMaxVcpusArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetMaxVcpusRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetMaxVcpus,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_os_type(
        &mut self,
        args: binding::RemoteDomainGetOsTypeArgs,
    ) -> Result<binding::RemoteDomainGetOsTypeRet, Error> {
        trace!("{}", stringify!(domain_get_os_type));
        let req: Option<binding::RemoteDomainGetOsTypeArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetOsTypeRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetOsType,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_vcpus(
        &mut self,
        args: binding::RemoteDomainGetVcpusArgs,
    ) -> Result<binding::RemoteDomainGetVcpusRet, Error> {
        trace!("{}", stringify!(domain_get_vcpus));
        let req: Option<binding::RemoteDomainGetVcpusArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetVcpusRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetVcpus,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_defined_domains(
        &mut self,
        args: binding::RemoteConnectListDefinedDomainsArgs,
    ) -> Result<binding::RemoteConnectListDefinedDomainsRet, Error> {
        trace!("{}", stringify!(connect_list_defined_domains));
        let req: Option<binding::RemoteConnectListDefinedDomainsArgs> = Some(args);
        let res: Option<binding::RemoteConnectListDefinedDomainsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListDefinedDomains,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_lookup_by_id(
        &mut self,
        args: binding::RemoteDomainLookupByIdArgs,
    ) -> Result<binding::RemoteDomainLookupByIdRet, Error> {
        trace!("{}", stringify!(domain_lookup_by_id));
        let req: Option<binding::RemoteDomainLookupByIdArgs> = Some(args);
        let res: Option<binding::RemoteDomainLookupByIdRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainLookupById,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_lookup_by_name(
        &mut self,
        args: binding::RemoteDomainLookupByNameArgs,
    ) -> Result<binding::RemoteDomainLookupByNameRet, Error> {
        trace!("{}", stringify!(domain_lookup_by_name));
        let req: Option<binding::RemoteDomainLookupByNameArgs> = Some(args);
        let res: Option<binding::RemoteDomainLookupByNameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainLookupByName,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_lookup_by_uuid(
        &mut self,
        args: binding::RemoteDomainLookupByUuidArgs,
    ) -> Result<binding::RemoteDomainLookupByUuidRet, Error> {
        trace!("{}", stringify!(domain_lookup_by_uuid));
        let req: Option<binding::RemoteDomainLookupByUuidArgs> = Some(args);
        let res: Option<binding::RemoteDomainLookupByUuidRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainLookupByUuid,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_num_of_defined_domains(
        &mut self,
    ) -> Result<binding::RemoteConnectNumOfDefinedDomainsRet, Error> {
        trace!("{}", stringify!(connect_num_of_defined_domains));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectNumOfDefinedDomainsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNumOfDefinedDomains,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_pin_vcpu(&mut self, args: binding::RemoteDomainPinVcpuArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pin_vcpu));
        let req: Option<binding::RemoteDomainPinVcpuArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainPinVcpu, req)?;
        Ok(())
    }
    fn domain_reboot(&mut self, args: binding::RemoteDomainRebootArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_reboot));
        let req: Option<binding::RemoteDomainRebootArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainReboot, req)?;
        Ok(())
    }
    fn domain_resume(&mut self, args: binding::RemoteDomainResumeArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_resume));
        let req: Option<binding::RemoteDomainResumeArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainResume, req)?;
        Ok(())
    }
    fn domain_set_autostart(
        &mut self,
        args: binding::RemoteDomainSetAutostartArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_autostart));
        let req: Option<binding::RemoteDomainSetAutostartArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetAutostart,
            req,
        )?;
        Ok(())
    }
    fn domain_set_max_memory(
        &mut self,
        args: binding::RemoteDomainSetMaxMemoryArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_max_memory));
        let req: Option<binding::RemoteDomainSetMaxMemoryArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetMaxMemory,
            req,
        )?;
        Ok(())
    }
    fn domain_set_memory(&mut self, args: binding::RemoteDomainSetMemoryArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_memory));
        let req: Option<binding::RemoteDomainSetMemoryArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetMemory,
            req,
        )?;
        Ok(())
    }
    fn domain_set_vcpus(&mut self, args: binding::RemoteDomainSetVcpusArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_vcpus));
        let req: Option<binding::RemoteDomainSetVcpusArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetVcpus,
            req,
        )?;
        Ok(())
    }
    fn domain_shutdown(&mut self, args: binding::RemoteDomainShutdownArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_shutdown));
        let req: Option<binding::RemoteDomainShutdownArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainShutdown,
            req,
        )?;
        Ok(())
    }
    fn domain_suspend(&mut self, args: binding::RemoteDomainSuspendArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_suspend));
        let req: Option<binding::RemoteDomainSuspendArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainSuspend, req)?;
        Ok(())
    }
    fn domain_undefine(&mut self, args: binding::RemoteDomainUndefineArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_undefine));
        let req: Option<binding::RemoteDomainUndefineArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainUndefine,
            req,
        )?;
        Ok(())
    }
    fn connect_list_defined_networks(
        &mut self,
        args: binding::RemoteConnectListDefinedNetworksArgs,
    ) -> Result<binding::RemoteConnectListDefinedNetworksRet, Error> {
        trace!("{}", stringify!(connect_list_defined_networks));
        let req: Option<binding::RemoteConnectListDefinedNetworksArgs> = Some(args);
        let res: Option<binding::RemoteConnectListDefinedNetworksRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListDefinedNetworks,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_domains(
        &mut self,
        args: binding::RemoteConnectListDomainsArgs,
    ) -> Result<binding::RemoteConnectListDomainsRet, Error> {
        trace!("{}", stringify!(connect_list_domains));
        let req: Option<binding::RemoteConnectListDomainsArgs> = Some(args);
        let res: Option<binding::RemoteConnectListDomainsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListDomains,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_networks(
        &mut self,
        args: binding::RemoteConnectListNetworksArgs,
    ) -> Result<binding::RemoteConnectListNetworksRet, Error> {
        trace!("{}", stringify!(connect_list_networks));
        let req: Option<binding::RemoteConnectListNetworksArgs> = Some(args);
        let res: Option<binding::RemoteConnectListNetworksRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListNetworks,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_create(&mut self, args: binding::RemoteNetworkCreateArgs) -> Result<(), Error> {
        trace!("{}", stringify!(network_create));
        let req: Option<binding::RemoteNetworkCreateArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcNetworkCreate, req)?;
        Ok(())
    }
    fn network_create_xml(
        &mut self,
        args: binding::RemoteNetworkCreateXmlArgs,
    ) -> Result<binding::RemoteNetworkCreateXmlRet, Error> {
        trace!("{}", stringify!(network_create_xml));
        let req: Option<binding::RemoteNetworkCreateXmlArgs> = Some(args);
        let res: Option<binding::RemoteNetworkCreateXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkCreateXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_define_xml(
        &mut self,
        args: binding::RemoteNetworkDefineXmlArgs,
    ) -> Result<binding::RemoteNetworkDefineXmlRet, Error> {
        trace!("{}", stringify!(network_define_xml));
        let req: Option<binding::RemoteNetworkDefineXmlArgs> = Some(args);
        let res: Option<binding::RemoteNetworkDefineXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkDefineXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_destroy(&mut self, args: binding::RemoteNetworkDestroyArgs) -> Result<(), Error> {
        trace!("{}", stringify!(network_destroy));
        let req: Option<binding::RemoteNetworkDestroyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkDestroy,
            req,
        )?;
        Ok(())
    }
    fn network_get_xml_desc(
        &mut self,
        args: binding::RemoteNetworkGetXmlDescArgs,
    ) -> Result<binding::RemoteNetworkGetXmlDescRet, Error> {
        trace!("{}", stringify!(network_get_xml_desc));
        let req: Option<binding::RemoteNetworkGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteNetworkGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_get_autostart(
        &mut self,
        args: binding::RemoteNetworkGetAutostartArgs,
    ) -> Result<binding::RemoteNetworkGetAutostartRet, Error> {
        trace!("{}", stringify!(network_get_autostart));
        let req: Option<binding::RemoteNetworkGetAutostartArgs> = Some(args);
        let res: Option<binding::RemoteNetworkGetAutostartRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkGetAutostart,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_get_bridge_name(
        &mut self,
        args: binding::RemoteNetworkGetBridgeNameArgs,
    ) -> Result<binding::RemoteNetworkGetBridgeNameRet, Error> {
        trace!("{}", stringify!(network_get_bridge_name));
        let req: Option<binding::RemoteNetworkGetBridgeNameArgs> = Some(args);
        let res: Option<binding::RemoteNetworkGetBridgeNameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkGetBridgeName,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_lookup_by_name(
        &mut self,
        args: binding::RemoteNetworkLookupByNameArgs,
    ) -> Result<binding::RemoteNetworkLookupByNameRet, Error> {
        trace!("{}", stringify!(network_lookup_by_name));
        let req: Option<binding::RemoteNetworkLookupByNameArgs> = Some(args);
        let res: Option<binding::RemoteNetworkLookupByNameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkLookupByName,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_lookup_by_uuid(
        &mut self,
        args: binding::RemoteNetworkLookupByUuidArgs,
    ) -> Result<binding::RemoteNetworkLookupByUuidRet, Error> {
        trace!("{}", stringify!(network_lookup_by_uuid));
        let req: Option<binding::RemoteNetworkLookupByUuidArgs> = Some(args);
        let res: Option<binding::RemoteNetworkLookupByUuidRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkLookupByUuid,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_set_autostart(
        &mut self,
        args: binding::RemoteNetworkSetAutostartArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(network_set_autostart));
        let req: Option<binding::RemoteNetworkSetAutostartArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkSetAutostart,
            req,
        )?;
        Ok(())
    }
    fn network_undefine(&mut self, args: binding::RemoteNetworkUndefineArgs) -> Result<(), Error> {
        trace!("{}", stringify!(network_undefine));
        let req: Option<binding::RemoteNetworkUndefineArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkUndefine,
            req,
        )?;
        Ok(())
    }
    fn connect_num_of_defined_networks(
        &mut self,
    ) -> Result<binding::RemoteConnectNumOfDefinedNetworksRet, Error> {
        trace!("{}", stringify!(connect_num_of_defined_networks));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectNumOfDefinedNetworksRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNumOfDefinedNetworks,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_num_of_domains(&mut self) -> Result<binding::RemoteConnectNumOfDomainsRet, Error> {
        trace!("{}", stringify!(connect_num_of_domains));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectNumOfDomainsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNumOfDomains,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_num_of_networks(&mut self) -> Result<binding::RemoteConnectNumOfNetworksRet, Error> {
        trace!("{}", stringify!(connect_num_of_networks));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectNumOfNetworksRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNumOfNetworks,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_core_dump(&mut self, args: binding::RemoteDomainCoreDumpArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_core_dump));
        let req: Option<binding::RemoteDomainCoreDumpArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCoreDump,
            req,
        )?;
        Ok(())
    }
    fn domain_restore(&mut self, args: binding::RemoteDomainRestoreArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_restore));
        let req: Option<binding::RemoteDomainRestoreArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainRestore, req)?;
        Ok(())
    }
    fn domain_save(&mut self, args: binding::RemoteDomainSaveArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_save));
        let req: Option<binding::RemoteDomainSaveArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainSave, req)?;
        Ok(())
    }
    fn domain_get_scheduler_type(
        &mut self,
        args: binding::RemoteDomainGetSchedulerTypeArgs,
    ) -> Result<binding::RemoteDomainGetSchedulerTypeRet, Error> {
        trace!("{}", stringify!(domain_get_scheduler_type));
        let req: Option<binding::RemoteDomainGetSchedulerTypeArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetSchedulerTypeRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetSchedulerType,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_scheduler_parameters(
        &mut self,
        args: binding::RemoteDomainGetSchedulerParametersArgs,
    ) -> Result<binding::RemoteDomainGetSchedulerParametersRet, Error> {
        trace!("{}", stringify!(domain_get_scheduler_parameters));
        let req: Option<binding::RemoteDomainGetSchedulerParametersArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetSchedulerParametersRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetSchedulerParameters,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_set_scheduler_parameters(
        &mut self,
        args: binding::RemoteDomainSetSchedulerParametersArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_scheduler_parameters));
        let req: Option<binding::RemoteDomainSetSchedulerParametersArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetSchedulerParameters,
            req,
        )?;
        Ok(())
    }
    fn connect_get_hostname(&mut self) -> Result<binding::RemoteConnectGetHostnameRet, Error> {
        trace!("{}", stringify!(connect_get_hostname));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectGetHostnameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetHostname,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_supports_feature(
        &mut self,
        args: binding::RemoteConnectSupportsFeatureArgs,
    ) -> Result<binding::RemoteConnectSupportsFeatureRet, Error> {
        trace!("{}", stringify!(connect_supports_feature));
        let req: Option<binding::RemoteConnectSupportsFeatureArgs> = Some(args);
        let res: Option<binding::RemoteConnectSupportsFeatureRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectSupportsFeature,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_prepare(
        &mut self,
        args: binding::RemoteDomainMigratePrepareArgs,
    ) -> Result<binding::RemoteDomainMigratePrepareRet, Error> {
        trace!("{}", stringify!(domain_migrate_prepare));
        let req: Option<binding::RemoteDomainMigratePrepareArgs> = Some(args);
        let res: Option<binding::RemoteDomainMigratePrepareRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigratePrepare,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_perform(
        &mut self,
        args: binding::RemoteDomainMigratePerformArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_perform));
        let req: Option<binding::RemoteDomainMigratePerformArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigratePerform,
            req,
        )?;
        Ok(())
    }
    fn domain_migrate_finish(
        &mut self,
        args: binding::RemoteDomainMigrateFinishArgs,
    ) -> Result<binding::RemoteDomainMigrateFinishRet, Error> {
        trace!("{}", stringify!(domain_migrate_finish));
        let req: Option<binding::RemoteDomainMigrateFinishArgs> = Some(args);
        let res: Option<binding::RemoteDomainMigrateFinishRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateFinish,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_block_stats(
        &mut self,
        args: binding::RemoteDomainBlockStatsArgs,
    ) -> Result<binding::RemoteDomainBlockStatsRet, Error> {
        trace!("{}", stringify!(domain_block_stats));
        let req: Option<binding::RemoteDomainBlockStatsArgs> = Some(args);
        let res: Option<binding::RemoteDomainBlockStatsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBlockStats,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_interface_stats(
        &mut self,
        args: binding::RemoteDomainInterfaceStatsArgs,
    ) -> Result<binding::RemoteDomainInterfaceStatsRet, Error> {
        trace!("{}", stringify!(domain_interface_stats));
        let req: Option<binding::RemoteDomainInterfaceStatsArgs> = Some(args);
        let res: Option<binding::RemoteDomainInterfaceStatsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainInterfaceStats,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn auth_list(&mut self) -> Result<binding::RemoteAuthListRet, Error> {
        trace!("{}", stringify!(auth_list));
        let req: Option<()> = None;
        let res: Option<binding::RemoteAuthListRet> =
            call(self, binding::RemoteProcedure::RemoteProcAuthList, req)?;
        Ok(res.unwrap())
    }
    fn auth_sasl_init(&mut self) -> Result<binding::RemoteAuthSaslInitRet, Error> {
        trace!("{}", stringify!(auth_sasl_init));
        let req: Option<()> = None;
        let res: Option<binding::RemoteAuthSaslInitRet> =
            call(self, binding::RemoteProcedure::RemoteProcAuthSaslInit, req)?;
        Ok(res.unwrap())
    }
    fn auth_sasl_start(
        &mut self,
        args: binding::RemoteAuthSaslStartArgs,
    ) -> Result<binding::RemoteAuthSaslStartRet, Error> {
        trace!("{}", stringify!(auth_sasl_start));
        let req: Option<binding::RemoteAuthSaslStartArgs> = Some(args);
        let res: Option<binding::RemoteAuthSaslStartRet> =
            call(self, binding::RemoteProcedure::RemoteProcAuthSaslStart, req)?;
        Ok(res.unwrap())
    }
    fn auth_sasl_step(
        &mut self,
        args: binding::RemoteAuthSaslStepArgs,
    ) -> Result<binding::RemoteAuthSaslStepRet, Error> {
        trace!("{}", stringify!(auth_sasl_step));
        let req: Option<binding::RemoteAuthSaslStepArgs> = Some(args);
        let res: Option<binding::RemoteAuthSaslStepRet> =
            call(self, binding::RemoteProcedure::RemoteProcAuthSaslStep, req)?;
        Ok(res.unwrap())
    }
    fn auth_polkit(&mut self) -> Result<binding::RemoteAuthPolkitRet, Error> {
        trace!("{}", stringify!(auth_polkit));
        let req: Option<()> = None;
        let res: Option<binding::RemoteAuthPolkitRet> =
            call(self, binding::RemoteProcedure::RemoteProcAuthPolkit, req)?;
        Ok(res.unwrap())
    }
    fn connect_num_of_storage_pools(
        &mut self,
    ) -> Result<binding::RemoteConnectNumOfStoragePoolsRet, Error> {
        trace!("{}", stringify!(connect_num_of_storage_pools));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectNumOfStoragePoolsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNumOfStoragePools,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_storage_pools(
        &mut self,
        args: binding::RemoteConnectListStoragePoolsArgs,
    ) -> Result<binding::RemoteConnectListStoragePoolsRet, Error> {
        trace!("{}", stringify!(connect_list_storage_pools));
        let req: Option<binding::RemoteConnectListStoragePoolsArgs> = Some(args);
        let res: Option<binding::RemoteConnectListStoragePoolsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListStoragePools,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_num_of_defined_storage_pools(
        &mut self,
    ) -> Result<binding::RemoteConnectNumOfDefinedStoragePoolsRet, Error> {
        trace!("{}", stringify!(connect_num_of_defined_storage_pools));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectNumOfDefinedStoragePoolsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNumOfDefinedStoragePools,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_defined_storage_pools(
        &mut self,
        args: binding::RemoteConnectListDefinedStoragePoolsArgs,
    ) -> Result<binding::RemoteConnectListDefinedStoragePoolsRet, Error> {
        trace!("{}", stringify!(connect_list_defined_storage_pools));
        let req: Option<binding::RemoteConnectListDefinedStoragePoolsArgs> = Some(args);
        let res: Option<binding::RemoteConnectListDefinedStoragePoolsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListDefinedStoragePools,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_find_storage_pool_sources(
        &mut self,
        args: binding::RemoteConnectFindStoragePoolSourcesArgs,
    ) -> Result<binding::RemoteConnectFindStoragePoolSourcesRet, Error> {
        trace!("{}", stringify!(connect_find_storage_pool_sources));
        let req: Option<binding::RemoteConnectFindStoragePoolSourcesArgs> = Some(args);
        let res: Option<binding::RemoteConnectFindStoragePoolSourcesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectFindStoragePoolSources,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_create_xml(
        &mut self,
        args: binding::RemoteStoragePoolCreateXmlArgs,
    ) -> Result<binding::RemoteStoragePoolCreateXmlRet, Error> {
        trace!("{}", stringify!(storage_pool_create_xml));
        let req: Option<binding::RemoteStoragePoolCreateXmlArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolCreateXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolCreateXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_define_xml(
        &mut self,
        args: binding::RemoteStoragePoolDefineXmlArgs,
    ) -> Result<binding::RemoteStoragePoolDefineXmlRet, Error> {
        trace!("{}", stringify!(storage_pool_define_xml));
        let req: Option<binding::RemoteStoragePoolDefineXmlArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolDefineXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolDefineXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_create(
        &mut self,
        args: binding::RemoteStoragePoolCreateArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_create));
        let req: Option<binding::RemoteStoragePoolCreateArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolCreate,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_build(
        &mut self,
        args: binding::RemoteStoragePoolBuildArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_build));
        let req: Option<binding::RemoteStoragePoolBuildArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolBuild,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_destroy(
        &mut self,
        args: binding::RemoteStoragePoolDestroyArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_destroy));
        let req: Option<binding::RemoteStoragePoolDestroyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolDestroy,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_delete(
        &mut self,
        args: binding::RemoteStoragePoolDeleteArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_delete));
        let req: Option<binding::RemoteStoragePoolDeleteArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolDelete,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_undefine(
        &mut self,
        args: binding::RemoteStoragePoolUndefineArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_undefine));
        let req: Option<binding::RemoteStoragePoolUndefineArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolUndefine,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_refresh(
        &mut self,
        args: binding::RemoteStoragePoolRefreshArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_refresh));
        let req: Option<binding::RemoteStoragePoolRefreshArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolRefresh,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_lookup_by_name(
        &mut self,
        args: binding::RemoteStoragePoolLookupByNameArgs,
    ) -> Result<binding::RemoteStoragePoolLookupByNameRet, Error> {
        trace!("{}", stringify!(storage_pool_lookup_by_name));
        let req: Option<binding::RemoteStoragePoolLookupByNameArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolLookupByNameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolLookupByName,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_lookup_by_uuid(
        &mut self,
        args: binding::RemoteStoragePoolLookupByUuidArgs,
    ) -> Result<binding::RemoteStoragePoolLookupByUuidRet, Error> {
        trace!("{}", stringify!(storage_pool_lookup_by_uuid));
        let req: Option<binding::RemoteStoragePoolLookupByUuidArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolLookupByUuidRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolLookupByUuid,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_lookup_by_volume(
        &mut self,
        args: binding::RemoteStoragePoolLookupByVolumeArgs,
    ) -> Result<binding::RemoteStoragePoolLookupByVolumeRet, Error> {
        trace!("{}", stringify!(storage_pool_lookup_by_volume));
        let req: Option<binding::RemoteStoragePoolLookupByVolumeArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolLookupByVolumeRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolLookupByVolume,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_get_info(
        &mut self,
        args: binding::RemoteStoragePoolGetInfoArgs,
    ) -> Result<binding::RemoteStoragePoolGetInfoRet, Error> {
        trace!("{}", stringify!(storage_pool_get_info));
        let req: Option<binding::RemoteStoragePoolGetInfoArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolGetInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolGetInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_get_xml_desc(
        &mut self,
        args: binding::RemoteStoragePoolGetXmlDescArgs,
    ) -> Result<binding::RemoteStoragePoolGetXmlDescRet, Error> {
        trace!("{}", stringify!(storage_pool_get_xml_desc));
        let req: Option<binding::RemoteStoragePoolGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_get_autostart(
        &mut self,
        args: binding::RemoteStoragePoolGetAutostartArgs,
    ) -> Result<binding::RemoteStoragePoolGetAutostartRet, Error> {
        trace!("{}", stringify!(storage_pool_get_autostart));
        let req: Option<binding::RemoteStoragePoolGetAutostartArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolGetAutostartRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolGetAutostart,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_set_autostart(
        &mut self,
        args: binding::RemoteStoragePoolSetAutostartArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_set_autostart));
        let req: Option<binding::RemoteStoragePoolSetAutostartArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolSetAutostart,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_num_of_volumes(
        &mut self,
        args: binding::RemoteStoragePoolNumOfVolumesArgs,
    ) -> Result<binding::RemoteStoragePoolNumOfVolumesRet, Error> {
        trace!("{}", stringify!(storage_pool_num_of_volumes));
        let req: Option<binding::RemoteStoragePoolNumOfVolumesArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolNumOfVolumesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolNumOfVolumes,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_list_volumes(
        &mut self,
        args: binding::RemoteStoragePoolListVolumesArgs,
    ) -> Result<binding::RemoteStoragePoolListVolumesRet, Error> {
        trace!("{}", stringify!(storage_pool_list_volumes));
        let req: Option<binding::RemoteStoragePoolListVolumesArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolListVolumesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolListVolumes,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_vol_create_xml(
        &mut self,
        args: binding::RemoteStorageVolCreateXmlArgs,
    ) -> Result<binding::RemoteStorageVolCreateXmlRet, Error> {
        trace!("{}", stringify!(storage_vol_create_xml));
        let req: Option<binding::RemoteStorageVolCreateXmlArgs> = Some(args);
        let res: Option<binding::RemoteStorageVolCreateXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolCreateXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_vol_delete(
        &mut self,
        args: binding::RemoteStorageVolDeleteArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_delete));
        let req: Option<binding::RemoteStorageVolDeleteArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolDelete,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_lookup_by_name(
        &mut self,
        args: binding::RemoteStorageVolLookupByNameArgs,
    ) -> Result<binding::RemoteStorageVolLookupByNameRet, Error> {
        trace!("{}", stringify!(storage_vol_lookup_by_name));
        let req: Option<binding::RemoteStorageVolLookupByNameArgs> = Some(args);
        let res: Option<binding::RemoteStorageVolLookupByNameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolLookupByName,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_vol_lookup_by_key(
        &mut self,
        args: binding::RemoteStorageVolLookupByKeyArgs,
    ) -> Result<binding::RemoteStorageVolLookupByKeyRet, Error> {
        trace!("{}", stringify!(storage_vol_lookup_by_key));
        let req: Option<binding::RemoteStorageVolLookupByKeyArgs> = Some(args);
        let res: Option<binding::RemoteStorageVolLookupByKeyRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolLookupByKey,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_vol_lookup_by_path(
        &mut self,
        args: binding::RemoteStorageVolLookupByPathArgs,
    ) -> Result<binding::RemoteStorageVolLookupByPathRet, Error> {
        trace!("{}", stringify!(storage_vol_lookup_by_path));
        let req: Option<binding::RemoteStorageVolLookupByPathArgs> = Some(args);
        let res: Option<binding::RemoteStorageVolLookupByPathRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolLookupByPath,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_vol_get_info(
        &mut self,
        args: binding::RemoteStorageVolGetInfoArgs,
    ) -> Result<binding::RemoteStorageVolGetInfoRet, Error> {
        trace!("{}", stringify!(storage_vol_get_info));
        let req: Option<binding::RemoteStorageVolGetInfoArgs> = Some(args);
        let res: Option<binding::RemoteStorageVolGetInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolGetInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_vol_get_xml_desc(
        &mut self,
        args: binding::RemoteStorageVolGetXmlDescArgs,
    ) -> Result<binding::RemoteStorageVolGetXmlDescRet, Error> {
        trace!("{}", stringify!(storage_vol_get_xml_desc));
        let req: Option<binding::RemoteStorageVolGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteStorageVolGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_vol_get_path(
        &mut self,
        args: binding::RemoteStorageVolGetPathArgs,
    ) -> Result<binding::RemoteStorageVolGetPathRet, Error> {
        trace!("{}", stringify!(storage_vol_get_path));
        let req: Option<binding::RemoteStorageVolGetPathArgs> = Some(args);
        let res: Option<binding::RemoteStorageVolGetPathRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolGetPath,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_get_cells_free_memory(
        &mut self,
        args: binding::RemoteNodeGetCellsFreeMemoryArgs,
    ) -> Result<binding::RemoteNodeGetCellsFreeMemoryRet, Error> {
        trace!("{}", stringify!(node_get_cells_free_memory));
        let req: Option<binding::RemoteNodeGetCellsFreeMemoryArgs> = Some(args);
        let res: Option<binding::RemoteNodeGetCellsFreeMemoryRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeGetCellsFreeMemory,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_get_free_memory(&mut self) -> Result<binding::RemoteNodeGetFreeMemoryRet, Error> {
        trace!("{}", stringify!(node_get_free_memory));
        let req: Option<()> = None;
        let res: Option<binding::RemoteNodeGetFreeMemoryRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeGetFreeMemory,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_block_peek(
        &mut self,
        args: binding::RemoteDomainBlockPeekArgs,
    ) -> Result<binding::RemoteDomainBlockPeekRet, Error> {
        trace!("{}", stringify!(domain_block_peek));
        let req: Option<binding::RemoteDomainBlockPeekArgs> = Some(args);
        let res: Option<binding::RemoteDomainBlockPeekRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBlockPeek,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_memory_peek(
        &mut self,
        args: binding::RemoteDomainMemoryPeekArgs,
    ) -> Result<binding::RemoteDomainMemoryPeekRet, Error> {
        trace!("{}", stringify!(domain_memory_peek));
        let req: Option<binding::RemoteDomainMemoryPeekArgs> = Some(args);
        let res: Option<binding::RemoteDomainMemoryPeekRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMemoryPeek,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_domain_event_register(
        &mut self,
    ) -> Result<binding::RemoteConnectDomainEventRegisterRet, Error> {
        trace!("{}", stringify!(connect_domain_event_register));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectDomainEventRegisterRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectDomainEventRegister,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_domain_event_deregister(
        &mut self,
    ) -> Result<binding::RemoteConnectDomainEventDeregisterRet, Error> {
        trace!("{}", stringify!(connect_domain_event_deregister));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectDomainEventDeregisterRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectDomainEventDeregister,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventLifecycle,
            req,
        )?;
        Ok(())
    }
    fn domain_migrate_prepare2(
        &mut self,
        args: binding::RemoteDomainMigratePrepare2Args,
    ) -> Result<binding::RemoteDomainMigratePrepare2Ret, Error> {
        trace!("{}", stringify!(domain_migrate_prepare2));
        let req: Option<binding::RemoteDomainMigratePrepare2Args> = Some(args);
        let res: Option<binding::RemoteDomainMigratePrepare2Ret> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigratePrepare2,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_finish2(
        &mut self,
        args: binding::RemoteDomainMigrateFinish2Args,
    ) -> Result<binding::RemoteDomainMigrateFinish2Ret, Error> {
        trace!("{}", stringify!(domain_migrate_finish2));
        let req: Option<binding::RemoteDomainMigrateFinish2Args> = Some(args);
        let res: Option<binding::RemoteDomainMigrateFinish2Ret> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateFinish2,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_get_uri(&mut self) -> Result<binding::RemoteConnectGetUriRet, Error> {
        trace!("{}", stringify!(connect_get_uri));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectGetUriRet> =
            call(self, binding::RemoteProcedure::RemoteProcConnectGetUri, req)?;
        Ok(res.unwrap())
    }
    fn node_num_of_devices(
        &mut self,
        args: binding::RemoteNodeNumOfDevicesArgs,
    ) -> Result<binding::RemoteNodeNumOfDevicesRet, Error> {
        trace!("{}", stringify!(node_num_of_devices));
        let req: Option<binding::RemoteNodeNumOfDevicesArgs> = Some(args);
        let res: Option<binding::RemoteNodeNumOfDevicesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeNumOfDevices,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_list_devices(
        &mut self,
        args: binding::RemoteNodeListDevicesArgs,
    ) -> Result<binding::RemoteNodeListDevicesRet, Error> {
        trace!("{}", stringify!(node_list_devices));
        let req: Option<binding::RemoteNodeListDevicesArgs> = Some(args);
        let res: Option<binding::RemoteNodeListDevicesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeListDevices,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_lookup_by_name(
        &mut self,
        args: binding::RemoteNodeDeviceLookupByNameArgs,
    ) -> Result<binding::RemoteNodeDeviceLookupByNameRet, Error> {
        trace!("{}", stringify!(node_device_lookup_by_name));
        let req: Option<binding::RemoteNodeDeviceLookupByNameArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceLookupByNameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceLookupByName,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_get_xml_desc(
        &mut self,
        args: binding::RemoteNodeDeviceGetXmlDescArgs,
    ) -> Result<binding::RemoteNodeDeviceGetXmlDescRet, Error> {
        trace!("{}", stringify!(node_device_get_xml_desc));
        let req: Option<binding::RemoteNodeDeviceGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_get_parent(
        &mut self,
        args: binding::RemoteNodeDeviceGetParentArgs,
    ) -> Result<binding::RemoteNodeDeviceGetParentRet, Error> {
        trace!("{}", stringify!(node_device_get_parent));
        let req: Option<binding::RemoteNodeDeviceGetParentArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceGetParentRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceGetParent,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_num_of_caps(
        &mut self,
        args: binding::RemoteNodeDeviceNumOfCapsArgs,
    ) -> Result<binding::RemoteNodeDeviceNumOfCapsRet, Error> {
        trace!("{}", stringify!(node_device_num_of_caps));
        let req: Option<binding::RemoteNodeDeviceNumOfCapsArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceNumOfCapsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceNumOfCaps,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_list_caps(
        &mut self,
        args: binding::RemoteNodeDeviceListCapsArgs,
    ) -> Result<binding::RemoteNodeDeviceListCapsRet, Error> {
        trace!("{}", stringify!(node_device_list_caps));
        let req: Option<binding::RemoteNodeDeviceListCapsArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceListCapsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceListCaps,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_dettach(
        &mut self,
        args: binding::RemoteNodeDeviceDettachArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_dettach));
        let req: Option<binding::RemoteNodeDeviceDettachArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceDettach,
            req,
        )?;
        Ok(())
    }
    fn node_device_re_attach(
        &mut self,
        args: binding::RemoteNodeDeviceReAttachArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_re_attach));
        let req: Option<binding::RemoteNodeDeviceReAttachArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceReAttach,
            req,
        )?;
        Ok(())
    }
    fn node_device_reset(&mut self, args: binding::RemoteNodeDeviceResetArgs) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_reset));
        let req: Option<binding::RemoteNodeDeviceResetArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceReset,
            req,
        )?;
        Ok(())
    }
    fn domain_get_security_label(
        &mut self,
        args: binding::RemoteDomainGetSecurityLabelArgs,
    ) -> Result<binding::RemoteDomainGetSecurityLabelRet, Error> {
        trace!("{}", stringify!(domain_get_security_label));
        let req: Option<binding::RemoteDomainGetSecurityLabelArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetSecurityLabelRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetSecurityLabel,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_get_security_model(&mut self) -> Result<binding::RemoteNodeGetSecurityModelRet, Error> {
        trace!("{}", stringify!(node_get_security_model));
        let req: Option<()> = None;
        let res: Option<binding::RemoteNodeGetSecurityModelRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeGetSecurityModel,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_create_xml(
        &mut self,
        args: binding::RemoteNodeDeviceCreateXmlArgs,
    ) -> Result<binding::RemoteNodeDeviceCreateXmlRet, Error> {
        trace!("{}", stringify!(node_device_create_xml));
        let req: Option<binding::RemoteNodeDeviceCreateXmlArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceCreateXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceCreateXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_destroy(
        &mut self,
        args: binding::RemoteNodeDeviceDestroyArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_destroy));
        let req: Option<binding::RemoteNodeDeviceDestroyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceDestroy,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_create_xml_from(
        &mut self,
        args: binding::RemoteStorageVolCreateXmlFromArgs,
    ) -> Result<binding::RemoteStorageVolCreateXmlFromRet, Error> {
        trace!("{}", stringify!(storage_vol_create_xml_from));
        let req: Option<binding::RemoteStorageVolCreateXmlFromArgs> = Some(args);
        let res: Option<binding::RemoteStorageVolCreateXmlFromRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolCreateXmlFrom,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_num_of_interfaces(
        &mut self,
    ) -> Result<binding::RemoteConnectNumOfInterfacesRet, Error> {
        trace!("{}", stringify!(connect_num_of_interfaces));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectNumOfInterfacesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNumOfInterfaces,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_interfaces(
        &mut self,
        args: binding::RemoteConnectListInterfacesArgs,
    ) -> Result<binding::RemoteConnectListInterfacesRet, Error> {
        trace!("{}", stringify!(connect_list_interfaces));
        let req: Option<binding::RemoteConnectListInterfacesArgs> = Some(args);
        let res: Option<binding::RemoteConnectListInterfacesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListInterfaces,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn interface_lookup_by_name(
        &mut self,
        args: binding::RemoteInterfaceLookupByNameArgs,
    ) -> Result<binding::RemoteInterfaceLookupByNameRet, Error> {
        trace!("{}", stringify!(interface_lookup_by_name));
        let req: Option<binding::RemoteInterfaceLookupByNameArgs> = Some(args);
        let res: Option<binding::RemoteInterfaceLookupByNameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceLookupByName,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn interface_lookup_by_mac_string(
        &mut self,
        args: binding::RemoteInterfaceLookupByMacStringArgs,
    ) -> Result<binding::RemoteInterfaceLookupByMacStringRet, Error> {
        trace!("{}", stringify!(interface_lookup_by_mac_string));
        let req: Option<binding::RemoteInterfaceLookupByMacStringArgs> = Some(args);
        let res: Option<binding::RemoteInterfaceLookupByMacStringRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceLookupByMacString,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn interface_get_xml_desc(
        &mut self,
        args: binding::RemoteInterfaceGetXmlDescArgs,
    ) -> Result<binding::RemoteInterfaceGetXmlDescRet, Error> {
        trace!("{}", stringify!(interface_get_xml_desc));
        let req: Option<binding::RemoteInterfaceGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteInterfaceGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn interface_define_xml(
        &mut self,
        args: binding::RemoteInterfaceDefineXmlArgs,
    ) -> Result<binding::RemoteInterfaceDefineXmlRet, Error> {
        trace!("{}", stringify!(interface_define_xml));
        let req: Option<binding::RemoteInterfaceDefineXmlArgs> = Some(args);
        let res: Option<binding::RemoteInterfaceDefineXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceDefineXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn interface_undefine(
        &mut self,
        args: binding::RemoteInterfaceUndefineArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(interface_undefine));
        let req: Option<binding::RemoteInterfaceUndefineArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceUndefine,
            req,
        )?;
        Ok(())
    }
    fn interface_create(&mut self, args: binding::RemoteInterfaceCreateArgs) -> Result<(), Error> {
        trace!("{}", stringify!(interface_create));
        let req: Option<binding::RemoteInterfaceCreateArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceCreate,
            req,
        )?;
        Ok(())
    }
    fn interface_destroy(
        &mut self,
        args: binding::RemoteInterfaceDestroyArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(interface_destroy));
        let req: Option<binding::RemoteInterfaceDestroyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceDestroy,
            req,
        )?;
        Ok(())
    }
    fn connect_domain_xml_from_native(
        &mut self,
        args: binding::RemoteConnectDomainXmlFromNativeArgs,
    ) -> Result<binding::RemoteConnectDomainXmlFromNativeRet, Error> {
        trace!("{}", stringify!(connect_domain_xml_from_native));
        let req: Option<binding::RemoteConnectDomainXmlFromNativeArgs> = Some(args);
        let res: Option<binding::RemoteConnectDomainXmlFromNativeRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectDomainXmlFromNative,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_domain_xml_to_native(
        &mut self,
        args: binding::RemoteConnectDomainXmlToNativeArgs,
    ) -> Result<binding::RemoteConnectDomainXmlToNativeRet, Error> {
        trace!("{}", stringify!(connect_domain_xml_to_native));
        let req: Option<binding::RemoteConnectDomainXmlToNativeArgs> = Some(args);
        let res: Option<binding::RemoteConnectDomainXmlToNativeRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectDomainXmlToNative,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_num_of_defined_interfaces(
        &mut self,
    ) -> Result<binding::RemoteConnectNumOfDefinedInterfacesRet, Error> {
        trace!("{}", stringify!(connect_num_of_defined_interfaces));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectNumOfDefinedInterfacesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNumOfDefinedInterfaces,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_defined_interfaces(
        &mut self,
        args: binding::RemoteConnectListDefinedInterfacesArgs,
    ) -> Result<binding::RemoteConnectListDefinedInterfacesRet, Error> {
        trace!("{}", stringify!(connect_list_defined_interfaces));
        let req: Option<binding::RemoteConnectListDefinedInterfacesArgs> = Some(args);
        let res: Option<binding::RemoteConnectListDefinedInterfacesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListDefinedInterfaces,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_num_of_secrets(&mut self) -> Result<binding::RemoteConnectNumOfSecretsRet, Error> {
        trace!("{}", stringify!(connect_num_of_secrets));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectNumOfSecretsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNumOfSecrets,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_secrets(
        &mut self,
        args: binding::RemoteConnectListSecretsArgs,
    ) -> Result<binding::RemoteConnectListSecretsRet, Error> {
        trace!("{}", stringify!(connect_list_secrets));
        let req: Option<binding::RemoteConnectListSecretsArgs> = Some(args);
        let res: Option<binding::RemoteConnectListSecretsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListSecrets,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn secret_lookup_by_uuid(
        &mut self,
        args: binding::RemoteSecretLookupByUuidArgs,
    ) -> Result<binding::RemoteSecretLookupByUuidRet, Error> {
        trace!("{}", stringify!(secret_lookup_by_uuid));
        let req: Option<binding::RemoteSecretLookupByUuidArgs> = Some(args);
        let res: Option<binding::RemoteSecretLookupByUuidRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcSecretLookupByUuid,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn secret_define_xml(
        &mut self,
        args: binding::RemoteSecretDefineXmlArgs,
    ) -> Result<binding::RemoteSecretDefineXmlRet, Error> {
        trace!("{}", stringify!(secret_define_xml));
        let req: Option<binding::RemoteSecretDefineXmlArgs> = Some(args);
        let res: Option<binding::RemoteSecretDefineXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcSecretDefineXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn secret_get_xml_desc(
        &mut self,
        args: binding::RemoteSecretGetXmlDescArgs,
    ) -> Result<binding::RemoteSecretGetXmlDescRet, Error> {
        trace!("{}", stringify!(secret_get_xml_desc));
        let req: Option<binding::RemoteSecretGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteSecretGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcSecretGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn secret_set_value(&mut self, args: binding::RemoteSecretSetValueArgs) -> Result<(), Error> {
        trace!("{}", stringify!(secret_set_value));
        let req: Option<binding::RemoteSecretSetValueArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcSecretSetValue,
            req,
        )?;
        Ok(())
    }
    fn secret_get_value(
        &mut self,
        args: binding::RemoteSecretGetValueArgs,
    ) -> Result<binding::RemoteSecretGetValueRet, Error> {
        trace!("{}", stringify!(secret_get_value));
        let req: Option<binding::RemoteSecretGetValueArgs> = Some(args);
        let res: Option<binding::RemoteSecretGetValueRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcSecretGetValue,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn secret_undefine(&mut self, args: binding::RemoteSecretUndefineArgs) -> Result<(), Error> {
        trace!("{}", stringify!(secret_undefine));
        let req: Option<binding::RemoteSecretUndefineArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcSecretUndefine,
            req,
        )?;
        Ok(())
    }
    fn secret_lookup_by_usage(
        &mut self,
        args: binding::RemoteSecretLookupByUsageArgs,
    ) -> Result<binding::RemoteSecretLookupByUsageRet, Error> {
        trace!("{}", stringify!(secret_lookup_by_usage));
        let req: Option<binding::RemoteSecretLookupByUsageArgs> = Some(args);
        let res: Option<binding::RemoteSecretLookupByUsageRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcSecretLookupByUsage,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_prepare_tunnel(
        &mut self,
        args: binding::RemoteDomainMigratePrepareTunnelArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_prepare_tunnel));
        let req: Option<binding::RemoteDomainMigratePrepareTunnelArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigratePrepareTunnel,
            req,
        )?;
        Ok(())
    }
    fn connect_is_secure(&mut self) -> Result<binding::RemoteConnectIsSecureRet, Error> {
        trace!("{}", stringify!(connect_is_secure));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectIsSecureRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectIsSecure,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_is_active(
        &mut self,
        args: binding::RemoteDomainIsActiveArgs,
    ) -> Result<binding::RemoteDomainIsActiveRet, Error> {
        trace!("{}", stringify!(domain_is_active));
        let req: Option<binding::RemoteDomainIsActiveArgs> = Some(args);
        let res: Option<binding::RemoteDomainIsActiveRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainIsActive,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_is_persistent(
        &mut self,
        args: binding::RemoteDomainIsPersistentArgs,
    ) -> Result<binding::RemoteDomainIsPersistentRet, Error> {
        trace!("{}", stringify!(domain_is_persistent));
        let req: Option<binding::RemoteDomainIsPersistentArgs> = Some(args);
        let res: Option<binding::RemoteDomainIsPersistentRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainIsPersistent,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_is_active(
        &mut self,
        args: binding::RemoteNetworkIsActiveArgs,
    ) -> Result<binding::RemoteNetworkIsActiveRet, Error> {
        trace!("{}", stringify!(network_is_active));
        let req: Option<binding::RemoteNetworkIsActiveArgs> = Some(args);
        let res: Option<binding::RemoteNetworkIsActiveRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkIsActive,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_is_persistent(
        &mut self,
        args: binding::RemoteNetworkIsPersistentArgs,
    ) -> Result<binding::RemoteNetworkIsPersistentRet, Error> {
        trace!("{}", stringify!(network_is_persistent));
        let req: Option<binding::RemoteNetworkIsPersistentArgs> = Some(args);
        let res: Option<binding::RemoteNetworkIsPersistentRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkIsPersistent,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_is_active(
        &mut self,
        args: binding::RemoteStoragePoolIsActiveArgs,
    ) -> Result<binding::RemoteStoragePoolIsActiveRet, Error> {
        trace!("{}", stringify!(storage_pool_is_active));
        let req: Option<binding::RemoteStoragePoolIsActiveArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolIsActiveRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolIsActive,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_is_persistent(
        &mut self,
        args: binding::RemoteStoragePoolIsPersistentArgs,
    ) -> Result<binding::RemoteStoragePoolIsPersistentRet, Error> {
        trace!("{}", stringify!(storage_pool_is_persistent));
        let req: Option<binding::RemoteStoragePoolIsPersistentArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolIsPersistentRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolIsPersistent,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn interface_is_active(
        &mut self,
        args: binding::RemoteInterfaceIsActiveArgs,
    ) -> Result<binding::RemoteInterfaceIsActiveRet, Error> {
        trace!("{}", stringify!(interface_is_active));
        let req: Option<binding::RemoteInterfaceIsActiveArgs> = Some(args);
        let res: Option<binding::RemoteInterfaceIsActiveRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceIsActive,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_get_lib_version(&mut self) -> Result<binding::RemoteConnectGetLibVersionRet, Error> {
        trace!("{}", stringify!(connect_get_lib_version));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectGetLibVersionRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetLibVersion,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_compare_cpu(
        &mut self,
        args: binding::RemoteConnectCompareCpuArgs,
    ) -> Result<binding::RemoteConnectCompareCpuRet, Error> {
        trace!("{}", stringify!(connect_compare_cpu));
        let req: Option<binding::RemoteConnectCompareCpuArgs> = Some(args);
        let res: Option<binding::RemoteConnectCompareCpuRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectCompareCpu,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_memory_stats(
        &mut self,
        args: binding::RemoteDomainMemoryStatsArgs,
    ) -> Result<binding::RemoteDomainMemoryStatsRet, Error> {
        trace!("{}", stringify!(domain_memory_stats));
        let req: Option<binding::RemoteDomainMemoryStatsArgs> = Some(args);
        let res: Option<binding::RemoteDomainMemoryStatsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMemoryStats,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_attach_device_flags(
        &mut self,
        args: binding::RemoteDomainAttachDeviceFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_attach_device_flags));
        let req: Option<binding::RemoteDomainAttachDeviceFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainAttachDeviceFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_detach_device_flags(
        &mut self,
        args: binding::RemoteDomainDetachDeviceFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_detach_device_flags));
        let req: Option<binding::RemoteDomainDetachDeviceFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainDetachDeviceFlags,
            req,
        )?;
        Ok(())
    }
    fn connect_baseline_cpu(
        &mut self,
        args: binding::RemoteConnectBaselineCpuArgs,
    ) -> Result<binding::RemoteConnectBaselineCpuRet, Error> {
        trace!("{}", stringify!(connect_baseline_cpu));
        let req: Option<binding::RemoteConnectBaselineCpuArgs> = Some(args);
        let res: Option<binding::RemoteConnectBaselineCpuRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectBaselineCpu,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_job_info(
        &mut self,
        args: binding::RemoteDomainGetJobInfoArgs,
    ) -> Result<binding::RemoteDomainGetJobInfoRet, Error> {
        trace!("{}", stringify!(domain_get_job_info));
        let req: Option<binding::RemoteDomainGetJobInfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetJobInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetJobInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_abort_job(&mut self, args: binding::RemoteDomainAbortJobArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_abort_job));
        let req: Option<binding::RemoteDomainAbortJobArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainAbortJob,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_wipe(&mut self, args: binding::RemoteStorageVolWipeArgs) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_wipe));
        let req: Option<binding::RemoteStorageVolWipeArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolWipe,
            req,
        )?;
        Ok(())
    }
    fn domain_migrate_set_max_downtime(
        &mut self,
        args: binding::RemoteDomainMigrateSetMaxDowntimeArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_set_max_downtime));
        let req: Option<binding::RemoteDomainMigrateSetMaxDowntimeArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateSetMaxDowntime,
            req,
        )?;
        Ok(())
    }
    fn connect_domain_event_register_any(
        &mut self,
        args: binding::RemoteConnectDomainEventRegisterAnyArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(connect_domain_event_register_any));
        let req: Option<binding::RemoteConnectDomainEventRegisterAnyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectDomainEventRegisterAny,
            req,
        )?;
        Ok(())
    }
    fn connect_domain_event_deregister_any(
        &mut self,
        args: binding::RemoteConnectDomainEventDeregisterAnyArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(connect_domain_event_deregister_any));
        let req: Option<binding::RemoteConnectDomainEventDeregisterAnyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectDomainEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn domain_event_reboot(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_reboot));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventReboot,
            req,
        )?;
        Ok(())
    }
    fn domain_event_rtc_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_rtc_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventRtcChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_watchdog(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_watchdog));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventWatchdog,
            req,
        )?;
        Ok(())
    }
    fn domain_event_io_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_io_error));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventIoError,
            req,
        )?;
        Ok(())
    }
    fn domain_event_graphics(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_graphics));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventGraphics,
            req,
        )?;
        Ok(())
    }
    fn domain_update_device_flags(
        &mut self,
        args: binding::RemoteDomainUpdateDeviceFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_update_device_flags));
        let req: Option<binding::RemoteDomainUpdateDeviceFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainUpdateDeviceFlags,
            req,
        )?;
        Ok(())
    }
    fn nwfilter_lookup_by_name(
        &mut self,
        args: binding::RemoteNwfilterLookupByNameArgs,
    ) -> Result<binding::RemoteNwfilterLookupByNameRet, Error> {
        trace!("{}", stringify!(nwfilter_lookup_by_name));
        let req: Option<binding::RemoteNwfilterLookupByNameArgs> = Some(args);
        let res: Option<binding::RemoteNwfilterLookupByNameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNwfilterLookupByName,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn nwfilter_lookup_by_uuid(
        &mut self,
        args: binding::RemoteNwfilterLookupByUuidArgs,
    ) -> Result<binding::RemoteNwfilterLookupByUuidRet, Error> {
        trace!("{}", stringify!(nwfilter_lookup_by_uuid));
        let req: Option<binding::RemoteNwfilterLookupByUuidArgs> = Some(args);
        let res: Option<binding::RemoteNwfilterLookupByUuidRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNwfilterLookupByUuid,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn nwfilter_get_xml_desc(
        &mut self,
        args: binding::RemoteNwfilterGetXmlDescArgs,
    ) -> Result<binding::RemoteNwfilterGetXmlDescRet, Error> {
        trace!("{}", stringify!(nwfilter_get_xml_desc));
        let req: Option<binding::RemoteNwfilterGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteNwfilterGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNwfilterGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_num_of_nwfilters(
        &mut self,
    ) -> Result<binding::RemoteConnectNumOfNwfiltersRet, Error> {
        trace!("{}", stringify!(connect_num_of_nwfilters));
        let req: Option<()> = None;
        let res: Option<binding::RemoteConnectNumOfNwfiltersRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNumOfNwfilters,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_nwfilters(
        &mut self,
        args: binding::RemoteConnectListNwfiltersArgs,
    ) -> Result<binding::RemoteConnectListNwfiltersRet, Error> {
        trace!("{}", stringify!(connect_list_nwfilters));
        let req: Option<binding::RemoteConnectListNwfiltersArgs> = Some(args);
        let res: Option<binding::RemoteConnectListNwfiltersRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListNwfilters,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn nwfilter_define_xml(
        &mut self,
        args: binding::RemoteNwfilterDefineXmlArgs,
    ) -> Result<binding::RemoteNwfilterDefineXmlRet, Error> {
        trace!("{}", stringify!(nwfilter_define_xml));
        let req: Option<binding::RemoteNwfilterDefineXmlArgs> = Some(args);
        let res: Option<binding::RemoteNwfilterDefineXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNwfilterDefineXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn nwfilter_undefine(
        &mut self,
        args: binding::RemoteNwfilterUndefineArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(nwfilter_undefine));
        let req: Option<binding::RemoteNwfilterUndefineArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNwfilterUndefine,
            req,
        )?;
        Ok(())
    }
    fn domain_managed_save(
        &mut self,
        args: binding::RemoteDomainManagedSaveArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_managed_save));
        let req: Option<binding::RemoteDomainManagedSaveArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainManagedSave,
            req,
        )?;
        Ok(())
    }
    fn domain_has_managed_save_image(
        &mut self,
        args: binding::RemoteDomainHasManagedSaveImageArgs,
    ) -> Result<binding::RemoteDomainHasManagedSaveImageRet, Error> {
        trace!("{}", stringify!(domain_has_managed_save_image));
        let req: Option<binding::RemoteDomainHasManagedSaveImageArgs> = Some(args);
        let res: Option<binding::RemoteDomainHasManagedSaveImageRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainHasManagedSaveImage,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_managed_save_remove(
        &mut self,
        args: binding::RemoteDomainManagedSaveRemoveArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_managed_save_remove));
        let req: Option<binding::RemoteDomainManagedSaveRemoveArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainManagedSaveRemove,
            req,
        )?;
        Ok(())
    }
    fn domain_snapshot_create_xml(
        &mut self,
        args: binding::RemoteDomainSnapshotCreateXmlArgs,
    ) -> Result<binding::RemoteDomainSnapshotCreateXmlRet, Error> {
        trace!("{}", stringify!(domain_snapshot_create_xml));
        let req: Option<binding::RemoteDomainSnapshotCreateXmlArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotCreateXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotCreateXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_snapshot_get_xml_desc(
        &mut self,
        args: binding::RemoteDomainSnapshotGetXmlDescArgs,
    ) -> Result<binding::RemoteDomainSnapshotGetXmlDescRet, Error> {
        trace!("{}", stringify!(domain_snapshot_get_xml_desc));
        let req: Option<binding::RemoteDomainSnapshotGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_snapshot_num(
        &mut self,
        args: binding::RemoteDomainSnapshotNumArgs,
    ) -> Result<binding::RemoteDomainSnapshotNumRet, Error> {
        trace!("{}", stringify!(domain_snapshot_num));
        let req: Option<binding::RemoteDomainSnapshotNumArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotNumRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotNum,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_snapshot_list_names(
        &mut self,
        args: binding::RemoteDomainSnapshotListNamesArgs,
    ) -> Result<binding::RemoteDomainSnapshotListNamesRet, Error> {
        trace!("{}", stringify!(domain_snapshot_list_names));
        let req: Option<binding::RemoteDomainSnapshotListNamesArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotListNamesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotListNames,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_snapshot_lookup_by_name(
        &mut self,
        args: binding::RemoteDomainSnapshotLookupByNameArgs,
    ) -> Result<binding::RemoteDomainSnapshotLookupByNameRet, Error> {
        trace!("{}", stringify!(domain_snapshot_lookup_by_name));
        let req: Option<binding::RemoteDomainSnapshotLookupByNameArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotLookupByNameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotLookupByName,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_has_current_snapshot(
        &mut self,
        args: binding::RemoteDomainHasCurrentSnapshotArgs,
    ) -> Result<binding::RemoteDomainHasCurrentSnapshotRet, Error> {
        trace!("{}", stringify!(domain_has_current_snapshot));
        let req: Option<binding::RemoteDomainHasCurrentSnapshotArgs> = Some(args);
        let res: Option<binding::RemoteDomainHasCurrentSnapshotRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainHasCurrentSnapshot,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_snapshot_current(
        &mut self,
        args: binding::RemoteDomainSnapshotCurrentArgs,
    ) -> Result<binding::RemoteDomainSnapshotCurrentRet, Error> {
        trace!("{}", stringify!(domain_snapshot_current));
        let req: Option<binding::RemoteDomainSnapshotCurrentArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotCurrentRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotCurrent,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_revert_to_snapshot(
        &mut self,
        args: binding::RemoteDomainRevertToSnapshotArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_revert_to_snapshot));
        let req: Option<binding::RemoteDomainRevertToSnapshotArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainRevertToSnapshot,
            req,
        )?;
        Ok(())
    }
    fn domain_snapshot_delete(
        &mut self,
        args: binding::RemoteDomainSnapshotDeleteArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_snapshot_delete));
        let req: Option<binding::RemoteDomainSnapshotDeleteArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotDelete,
            req,
        )?;
        Ok(())
    }
    fn domain_get_block_info(
        &mut self,
        args: binding::RemoteDomainGetBlockInfoArgs,
    ) -> Result<binding::RemoteDomainGetBlockInfoRet, Error> {
        trace!("{}", stringify!(domain_get_block_info));
        let req: Option<binding::RemoteDomainGetBlockInfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetBlockInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetBlockInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_io_error_reason(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_io_error_reason));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventIoErrorReason,
            req,
        )?;
        Ok(())
    }
    fn domain_create_with_flags(
        &mut self,
        args: binding::RemoteDomainCreateWithFlagsArgs,
    ) -> Result<binding::RemoteDomainCreateWithFlagsRet, Error> {
        trace!("{}", stringify!(domain_create_with_flags));
        let req: Option<binding::RemoteDomainCreateWithFlagsArgs> = Some(args);
        let res: Option<binding::RemoteDomainCreateWithFlagsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCreateWithFlags,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_set_memory_parameters(
        &mut self,
        args: binding::RemoteDomainSetMemoryParametersArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_memory_parameters));
        let req: Option<binding::RemoteDomainSetMemoryParametersArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetMemoryParameters,
            req,
        )?;
        Ok(())
    }
    fn domain_get_memory_parameters(
        &mut self,
        args: binding::RemoteDomainGetMemoryParametersArgs,
    ) -> Result<binding::RemoteDomainGetMemoryParametersRet, Error> {
        trace!("{}", stringify!(domain_get_memory_parameters));
        let req: Option<binding::RemoteDomainGetMemoryParametersArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetMemoryParametersRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetMemoryParameters,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_set_vcpus_flags(
        &mut self,
        args: binding::RemoteDomainSetVcpusFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_vcpus_flags));
        let req: Option<binding::RemoteDomainSetVcpusFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetVcpusFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_get_vcpus_flags(
        &mut self,
        args: binding::RemoteDomainGetVcpusFlagsArgs,
    ) -> Result<binding::RemoteDomainGetVcpusFlagsRet, Error> {
        trace!("{}", stringify!(domain_get_vcpus_flags));
        let req: Option<binding::RemoteDomainGetVcpusFlagsArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetVcpusFlagsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetVcpusFlags,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_open_console(
        &mut self,
        args: binding::RemoteDomainOpenConsoleArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_open_console));
        let req: Option<binding::RemoteDomainOpenConsoleArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainOpenConsole,
            req,
        )?;
        Ok(())
    }
    fn domain_is_updated(
        &mut self,
        args: binding::RemoteDomainIsUpdatedArgs,
    ) -> Result<binding::RemoteDomainIsUpdatedRet, Error> {
        trace!("{}", stringify!(domain_is_updated));
        let req: Option<binding::RemoteDomainIsUpdatedArgs> = Some(args);
        let res: Option<binding::RemoteDomainIsUpdatedRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainIsUpdated,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_get_sysinfo(
        &mut self,
        args: binding::RemoteConnectGetSysinfoArgs,
    ) -> Result<binding::RemoteConnectGetSysinfoRet, Error> {
        trace!("{}", stringify!(connect_get_sysinfo));
        let req: Option<binding::RemoteConnectGetSysinfoArgs> = Some(args);
        let res: Option<binding::RemoteConnectGetSysinfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetSysinfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_set_memory_flags(
        &mut self,
        args: binding::RemoteDomainSetMemoryFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_memory_flags));
        let req: Option<binding::RemoteDomainSetMemoryFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetMemoryFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_set_blkio_parameters(
        &mut self,
        args: binding::RemoteDomainSetBlkioParametersArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_blkio_parameters));
        let req: Option<binding::RemoteDomainSetBlkioParametersArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetBlkioParameters,
            req,
        )?;
        Ok(())
    }
    fn domain_get_blkio_parameters(
        &mut self,
        args: binding::RemoteDomainGetBlkioParametersArgs,
    ) -> Result<binding::RemoteDomainGetBlkioParametersRet, Error> {
        trace!("{}", stringify!(domain_get_blkio_parameters));
        let req: Option<binding::RemoteDomainGetBlkioParametersArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetBlkioParametersRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetBlkioParameters,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_set_max_speed(
        &mut self,
        args: binding::RemoteDomainMigrateSetMaxSpeedArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_set_max_speed));
        let req: Option<binding::RemoteDomainMigrateSetMaxSpeedArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateSetMaxSpeed,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_upload(
        &mut self,
        args: binding::RemoteStorageVolUploadArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_upload));
        let req: Option<binding::RemoteStorageVolUploadArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolUpload,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_download(
        &mut self,
        args: binding::RemoteStorageVolDownloadArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_download));
        let req: Option<binding::RemoteStorageVolDownloadArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolDownload,
            req,
        )?;
        Ok(())
    }
    fn domain_inject_nmi(&mut self, args: binding::RemoteDomainInjectNmiArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_inject_nmi));
        let req: Option<binding::RemoteDomainInjectNmiArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainInjectNmi,
            req,
        )?;
        Ok(())
    }
    fn domain_screenshot(
        &mut self,
        args: binding::RemoteDomainScreenshotArgs,
    ) -> Result<binding::RemoteDomainScreenshotRet, Error> {
        trace!("{}", stringify!(domain_screenshot));
        let req: Option<binding::RemoteDomainScreenshotArgs> = Some(args);
        let res: Option<binding::RemoteDomainScreenshotRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainScreenshot,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_state(
        &mut self,
        args: binding::RemoteDomainGetStateArgs,
    ) -> Result<binding::RemoteDomainGetStateRet, Error> {
        trace!("{}", stringify!(domain_get_state));
        let req: Option<binding::RemoteDomainGetStateArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetStateRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetState,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_begin3(
        &mut self,
        args: binding::RemoteDomainMigrateBegin3Args,
    ) -> Result<binding::RemoteDomainMigrateBegin3Ret, Error> {
        trace!("{}", stringify!(domain_migrate_begin3));
        let req: Option<binding::RemoteDomainMigrateBegin3Args> = Some(args);
        let res: Option<binding::RemoteDomainMigrateBegin3Ret> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateBegin3,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_prepare3(
        &mut self,
        args: binding::RemoteDomainMigratePrepare3Args,
    ) -> Result<binding::RemoteDomainMigratePrepare3Ret, Error> {
        trace!("{}", stringify!(domain_migrate_prepare3));
        let req: Option<binding::RemoteDomainMigratePrepare3Args> = Some(args);
        let res: Option<binding::RemoteDomainMigratePrepare3Ret> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigratePrepare3,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_prepare_tunnel3(
        &mut self,
        args: binding::RemoteDomainMigratePrepareTunnel3Args,
    ) -> Result<binding::RemoteDomainMigratePrepareTunnel3Ret, Error> {
        trace!("{}", stringify!(domain_migrate_prepare_tunnel3));
        let req: Option<binding::RemoteDomainMigratePrepareTunnel3Args> = Some(args);
        let res: Option<binding::RemoteDomainMigratePrepareTunnel3Ret> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigratePrepareTunnel3,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_perform3(
        &mut self,
        args: binding::RemoteDomainMigratePerform3Args,
    ) -> Result<binding::RemoteDomainMigratePerform3Ret, Error> {
        trace!("{}", stringify!(domain_migrate_perform3));
        let req: Option<binding::RemoteDomainMigratePerform3Args> = Some(args);
        let res: Option<binding::RemoteDomainMigratePerform3Ret> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigratePerform3,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_finish3(
        &mut self,
        args: binding::RemoteDomainMigrateFinish3Args,
    ) -> Result<binding::RemoteDomainMigrateFinish3Ret, Error> {
        trace!("{}", stringify!(domain_migrate_finish3));
        let req: Option<binding::RemoteDomainMigrateFinish3Args> = Some(args);
        let res: Option<binding::RemoteDomainMigrateFinish3Ret> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateFinish3,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_confirm3(
        &mut self,
        args: binding::RemoteDomainMigrateConfirm3Args,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_confirm3));
        let req: Option<binding::RemoteDomainMigrateConfirm3Args> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateConfirm3,
            req,
        )?;
        Ok(())
    }
    fn domain_set_scheduler_parameters_flags(
        &mut self,
        args: binding::RemoteDomainSetSchedulerParametersFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_scheduler_parameters_flags));
        let req: Option<binding::RemoteDomainSetSchedulerParametersFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetSchedulerParametersFlags,
            req,
        )?;
        Ok(())
    }
    fn interface_change_begin(
        &mut self,
        args: binding::RemoteInterfaceChangeBeginArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_begin));
        let req: Option<binding::RemoteInterfaceChangeBeginArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceChangeBegin,
            req,
        )?;
        Ok(())
    }
    fn interface_change_commit(
        &mut self,
        args: binding::RemoteInterfaceChangeCommitArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_commit));
        let req: Option<binding::RemoteInterfaceChangeCommitArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceChangeCommit,
            req,
        )?;
        Ok(())
    }
    fn interface_change_rollback(
        &mut self,
        args: binding::RemoteInterfaceChangeRollbackArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_rollback));
        let req: Option<binding::RemoteInterfaceChangeRollbackArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcInterfaceChangeRollback,
            req,
        )?;
        Ok(())
    }
    fn domain_get_scheduler_parameters_flags(
        &mut self,
        args: binding::RemoteDomainGetSchedulerParametersFlagsArgs,
    ) -> Result<binding::RemoteDomainGetSchedulerParametersFlagsRet, Error> {
        trace!("{}", stringify!(domain_get_scheduler_parameters_flags));
        let req: Option<binding::RemoteDomainGetSchedulerParametersFlagsArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetSchedulerParametersFlagsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetSchedulerParametersFlags,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_control_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_control_error));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventControlError,
            req,
        )?;
        Ok(())
    }
    fn domain_pin_vcpu_flags(
        &mut self,
        args: binding::RemoteDomainPinVcpuFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pin_vcpu_flags));
        let req: Option<binding::RemoteDomainPinVcpuFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainPinVcpuFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_send_key(&mut self, args: binding::RemoteDomainSendKeyArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_send_key));
        let req: Option<binding::RemoteDomainSendKeyArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainSendKey, req)?;
        Ok(())
    }
    fn node_get_cpu_stats(
        &mut self,
        args: binding::RemoteNodeGetCpuStatsArgs,
    ) -> Result<binding::RemoteNodeGetCpuStatsRet, Error> {
        trace!("{}", stringify!(node_get_cpu_stats));
        let req: Option<binding::RemoteNodeGetCpuStatsArgs> = Some(args);
        let res: Option<binding::RemoteNodeGetCpuStatsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeGetCpuStats,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_get_memory_stats(
        &mut self,
        args: binding::RemoteNodeGetMemoryStatsArgs,
    ) -> Result<binding::RemoteNodeGetMemoryStatsRet, Error> {
        trace!("{}", stringify!(node_get_memory_stats));
        let req: Option<binding::RemoteNodeGetMemoryStatsArgs> = Some(args);
        let res: Option<binding::RemoteNodeGetMemoryStatsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeGetMemoryStats,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_control_info(
        &mut self,
        args: binding::RemoteDomainGetControlInfoArgs,
    ) -> Result<binding::RemoteDomainGetControlInfoRet, Error> {
        trace!("{}", stringify!(domain_get_control_info));
        let req: Option<binding::RemoteDomainGetControlInfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetControlInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetControlInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_vcpu_pin_info(
        &mut self,
        args: binding::RemoteDomainGetVcpuPinInfoArgs,
    ) -> Result<binding::RemoteDomainGetVcpuPinInfoRet, Error> {
        trace!("{}", stringify!(domain_get_vcpu_pin_info));
        let req: Option<binding::RemoteDomainGetVcpuPinInfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetVcpuPinInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetVcpuPinInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_undefine_flags(
        &mut self,
        args: binding::RemoteDomainUndefineFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_undefine_flags));
        let req: Option<binding::RemoteDomainUndefineFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainUndefineFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_save_flags(&mut self, args: binding::RemoteDomainSaveFlagsArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_save_flags));
        let req: Option<binding::RemoteDomainSaveFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSaveFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_restore_flags(
        &mut self,
        args: binding::RemoteDomainRestoreFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_restore_flags));
        let req: Option<binding::RemoteDomainRestoreFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainRestoreFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_destroy_flags(
        &mut self,
        args: binding::RemoteDomainDestroyFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_destroy_flags));
        let req: Option<binding::RemoteDomainDestroyFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainDestroyFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_save_image_get_xml_desc(
        &mut self,
        args: binding::RemoteDomainSaveImageGetXmlDescArgs,
    ) -> Result<binding::RemoteDomainSaveImageGetXmlDescRet, Error> {
        trace!("{}", stringify!(domain_save_image_get_xml_desc));
        let req: Option<binding::RemoteDomainSaveImageGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteDomainSaveImageGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSaveImageGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_save_image_define_xml(
        &mut self,
        args: binding::RemoteDomainSaveImageDefineXmlArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_save_image_define_xml));
        let req: Option<binding::RemoteDomainSaveImageDefineXmlArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSaveImageDefineXml,
            req,
        )?;
        Ok(())
    }
    fn domain_block_job_abort(
        &mut self,
        args: binding::RemoteDomainBlockJobAbortArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_job_abort));
        let req: Option<binding::RemoteDomainBlockJobAbortArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBlockJobAbort,
            req,
        )?;
        Ok(())
    }
    fn domain_get_block_job_info(
        &mut self,
        args: binding::RemoteDomainGetBlockJobInfoArgs,
    ) -> Result<binding::RemoteDomainGetBlockJobInfoRet, Error> {
        trace!("{}", stringify!(domain_get_block_job_info));
        let req: Option<binding::RemoteDomainGetBlockJobInfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetBlockJobInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetBlockJobInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_block_job_set_speed(
        &mut self,
        args: binding::RemoteDomainBlockJobSetSpeedArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_job_set_speed));
        let req: Option<binding::RemoteDomainBlockJobSetSpeedArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBlockJobSetSpeed,
            req,
        )?;
        Ok(())
    }
    fn domain_block_pull(&mut self, args: binding::RemoteDomainBlockPullArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_pull));
        let req: Option<binding::RemoteDomainBlockPullArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBlockPull,
            req,
        )?;
        Ok(())
    }
    fn domain_event_block_job(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_job));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventBlockJob,
            req,
        )?;
        Ok(())
    }
    fn domain_migrate_get_max_speed(
        &mut self,
        args: binding::RemoteDomainMigrateGetMaxSpeedArgs,
    ) -> Result<binding::RemoteDomainMigrateGetMaxSpeedRet, Error> {
        trace!("{}", stringify!(domain_migrate_get_max_speed));
        let req: Option<binding::RemoteDomainMigrateGetMaxSpeedArgs> = Some(args);
        let res: Option<binding::RemoteDomainMigrateGetMaxSpeedRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateGetMaxSpeed,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_block_stats_flags(
        &mut self,
        args: binding::RemoteDomainBlockStatsFlagsArgs,
    ) -> Result<binding::RemoteDomainBlockStatsFlagsRet, Error> {
        trace!("{}", stringify!(domain_block_stats_flags));
        let req: Option<binding::RemoteDomainBlockStatsFlagsArgs> = Some(args);
        let res: Option<binding::RemoteDomainBlockStatsFlagsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBlockStatsFlags,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_snapshot_get_parent(
        &mut self,
        args: binding::RemoteDomainSnapshotGetParentArgs,
    ) -> Result<binding::RemoteDomainSnapshotGetParentRet, Error> {
        trace!("{}", stringify!(domain_snapshot_get_parent));
        let req: Option<binding::RemoteDomainSnapshotGetParentArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotGetParentRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotGetParent,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_reset(&mut self, args: binding::RemoteDomainResetArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_reset));
        let req: Option<binding::RemoteDomainResetArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainReset, req)?;
        Ok(())
    }
    fn domain_snapshot_num_children(
        &mut self,
        args: binding::RemoteDomainSnapshotNumChildrenArgs,
    ) -> Result<binding::RemoteDomainSnapshotNumChildrenRet, Error> {
        trace!("{}", stringify!(domain_snapshot_num_children));
        let req: Option<binding::RemoteDomainSnapshotNumChildrenArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotNumChildrenRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotNumChildren,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_snapshot_list_children_names(
        &mut self,
        args: binding::RemoteDomainSnapshotListChildrenNamesArgs,
    ) -> Result<binding::RemoteDomainSnapshotListChildrenNamesRet, Error> {
        trace!("{}", stringify!(domain_snapshot_list_children_names));
        let req: Option<binding::RemoteDomainSnapshotListChildrenNamesArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotListChildrenNamesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotListChildrenNames,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_disk_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_disk_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventDiskChange,
            req,
        )?;
        Ok(())
    }
    fn domain_open_graphics(
        &mut self,
        args: binding::RemoteDomainOpenGraphicsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_open_graphics));
        let req: Option<binding::RemoteDomainOpenGraphicsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainOpenGraphics,
            req,
        )?;
        Ok(())
    }
    fn node_suspend_for_duration(
        &mut self,
        args: binding::RemoteNodeSuspendForDurationArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_suspend_for_duration));
        let req: Option<binding::RemoteNodeSuspendForDurationArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeSuspendForDuration,
            req,
        )?;
        Ok(())
    }
    fn domain_block_resize(
        &mut self,
        args: binding::RemoteDomainBlockResizeArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_resize));
        let req: Option<binding::RemoteDomainBlockResizeArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBlockResize,
            req,
        )?;
        Ok(())
    }
    fn domain_set_block_io_tune(
        &mut self,
        args: binding::RemoteDomainSetBlockIoTuneArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_block_io_tune));
        let req: Option<binding::RemoteDomainSetBlockIoTuneArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetBlockIoTune,
            req,
        )?;
        Ok(())
    }
    fn domain_get_block_io_tune(
        &mut self,
        args: binding::RemoteDomainGetBlockIoTuneArgs,
    ) -> Result<binding::RemoteDomainGetBlockIoTuneRet, Error> {
        trace!("{}", stringify!(domain_get_block_io_tune));
        let req: Option<binding::RemoteDomainGetBlockIoTuneArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetBlockIoTuneRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetBlockIoTune,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_set_numa_parameters(
        &mut self,
        args: binding::RemoteDomainSetNumaParametersArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_numa_parameters));
        let req: Option<binding::RemoteDomainSetNumaParametersArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetNumaParameters,
            req,
        )?;
        Ok(())
    }
    fn domain_get_numa_parameters(
        &mut self,
        args: binding::RemoteDomainGetNumaParametersArgs,
    ) -> Result<binding::RemoteDomainGetNumaParametersRet, Error> {
        trace!("{}", stringify!(domain_get_numa_parameters));
        let req: Option<binding::RemoteDomainGetNumaParametersArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetNumaParametersRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetNumaParameters,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_set_interface_parameters(
        &mut self,
        args: binding::RemoteDomainSetInterfaceParametersArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_interface_parameters));
        let req: Option<binding::RemoteDomainSetInterfaceParametersArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetInterfaceParameters,
            req,
        )?;
        Ok(())
    }
    fn domain_get_interface_parameters(
        &mut self,
        args: binding::RemoteDomainGetInterfaceParametersArgs,
    ) -> Result<binding::RemoteDomainGetInterfaceParametersRet, Error> {
        trace!("{}", stringify!(domain_get_interface_parameters));
        let req: Option<binding::RemoteDomainGetInterfaceParametersArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetInterfaceParametersRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetInterfaceParameters,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_shutdown_flags(
        &mut self,
        args: binding::RemoteDomainShutdownFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_shutdown_flags));
        let req: Option<binding::RemoteDomainShutdownFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainShutdownFlags,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_wipe_pattern(
        &mut self,
        args: binding::RemoteStorageVolWipePatternArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_wipe_pattern));
        let req: Option<binding::RemoteStorageVolWipePatternArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolWipePattern,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_resize(
        &mut self,
        args: binding::RemoteStorageVolResizeArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_resize));
        let req: Option<binding::RemoteStorageVolResizeArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolResize,
            req,
        )?;
        Ok(())
    }
    fn domain_pm_suspend_for_duration(
        &mut self,
        args: binding::RemoteDomainPmSuspendForDurationArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pm_suspend_for_duration));
        let req: Option<binding::RemoteDomainPmSuspendForDurationArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainPmSuspendForDuration,
            req,
        )?;
        Ok(())
    }
    fn domain_get_cpu_stats(
        &mut self,
        args: binding::RemoteDomainGetCpuStatsArgs,
    ) -> Result<binding::RemoteDomainGetCpuStatsRet, Error> {
        trace!("{}", stringify!(domain_get_cpu_stats));
        let req: Option<binding::RemoteDomainGetCpuStatsArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetCpuStatsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetCpuStats,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_disk_errors(
        &mut self,
        args: binding::RemoteDomainGetDiskErrorsArgs,
    ) -> Result<binding::RemoteDomainGetDiskErrorsRet, Error> {
        trace!("{}", stringify!(domain_get_disk_errors));
        let req: Option<binding::RemoteDomainGetDiskErrorsArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetDiskErrorsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetDiskErrors,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_set_metadata(
        &mut self,
        args: binding::RemoteDomainSetMetadataArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_metadata));
        let req: Option<binding::RemoteDomainSetMetadataArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetMetadata,
            req,
        )?;
        Ok(())
    }
    fn domain_get_metadata(
        &mut self,
        args: binding::RemoteDomainGetMetadataArgs,
    ) -> Result<binding::RemoteDomainGetMetadataRet, Error> {
        trace!("{}", stringify!(domain_get_metadata));
        let req: Option<binding::RemoteDomainGetMetadataArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetMetadataRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetMetadata,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_block_rebase(
        &mut self,
        args: binding::RemoteDomainBlockRebaseArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_rebase));
        let req: Option<binding::RemoteDomainBlockRebaseArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBlockRebase,
            req,
        )?;
        Ok(())
    }
    fn domain_pm_wakeup(&mut self, args: binding::RemoteDomainPmWakeupArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pm_wakeup));
        let req: Option<binding::RemoteDomainPmWakeupArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainPmWakeup,
            req,
        )?;
        Ok(())
    }
    fn domain_event_tray_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_tray_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventTrayChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_pmwakeup(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmwakeup));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventPmwakeup,
            req,
        )?;
        Ok(())
    }
    fn domain_event_pmsuspend(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmsuspend));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventPmsuspend,
            req,
        )?;
        Ok(())
    }
    fn domain_snapshot_is_current(
        &mut self,
        args: binding::RemoteDomainSnapshotIsCurrentArgs,
    ) -> Result<binding::RemoteDomainSnapshotIsCurrentRet, Error> {
        trace!("{}", stringify!(domain_snapshot_is_current));
        let req: Option<binding::RemoteDomainSnapshotIsCurrentArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotIsCurrentRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotIsCurrent,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_snapshot_has_metadata(
        &mut self,
        args: binding::RemoteDomainSnapshotHasMetadataArgs,
    ) -> Result<binding::RemoteDomainSnapshotHasMetadataRet, Error> {
        trace!("{}", stringify!(domain_snapshot_has_metadata));
        let req: Option<binding::RemoteDomainSnapshotHasMetadataArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotHasMetadataRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotHasMetadata,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_all_domains(
        &mut self,
        args: binding::RemoteConnectListAllDomainsArgs,
    ) -> Result<binding::RemoteConnectListAllDomainsRet, Error> {
        trace!("{}", stringify!(connect_list_all_domains));
        let req: Option<binding::RemoteConnectListAllDomainsArgs> = Some(args);
        let res: Option<binding::RemoteConnectListAllDomainsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListAllDomains,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_list_all_snapshots(
        &mut self,
        args: binding::RemoteDomainListAllSnapshotsArgs,
    ) -> Result<binding::RemoteDomainListAllSnapshotsRet, Error> {
        trace!("{}", stringify!(domain_list_all_snapshots));
        let req: Option<binding::RemoteDomainListAllSnapshotsArgs> = Some(args);
        let res: Option<binding::RemoteDomainListAllSnapshotsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainListAllSnapshots,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_snapshot_list_all_children(
        &mut self,
        args: binding::RemoteDomainSnapshotListAllChildrenArgs,
    ) -> Result<binding::RemoteDomainSnapshotListAllChildrenRet, Error> {
        trace!("{}", stringify!(domain_snapshot_list_all_children));
        let req: Option<binding::RemoteDomainSnapshotListAllChildrenArgs> = Some(args);
        let res: Option<binding::RemoteDomainSnapshotListAllChildrenRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSnapshotListAllChildren,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_balloon_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_balloon_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventBalloonChange,
            req,
        )?;
        Ok(())
    }
    fn domain_get_hostname(
        &mut self,
        args: binding::RemoteDomainGetHostnameArgs,
    ) -> Result<binding::RemoteDomainGetHostnameRet, Error> {
        trace!("{}", stringify!(domain_get_hostname));
        let req: Option<binding::RemoteDomainGetHostnameArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetHostnameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetHostname,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_security_label_list(
        &mut self,
        args: binding::RemoteDomainGetSecurityLabelListArgs,
    ) -> Result<binding::RemoteDomainGetSecurityLabelListRet, Error> {
        trace!("{}", stringify!(domain_get_security_label_list));
        let req: Option<binding::RemoteDomainGetSecurityLabelListArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetSecurityLabelListRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetSecurityLabelList,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_pin_emulator(
        &mut self,
        args: binding::RemoteDomainPinEmulatorArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pin_emulator));
        let req: Option<binding::RemoteDomainPinEmulatorArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainPinEmulator,
            req,
        )?;
        Ok(())
    }
    fn domain_get_emulator_pin_info(
        &mut self,
        args: binding::RemoteDomainGetEmulatorPinInfoArgs,
    ) -> Result<binding::RemoteDomainGetEmulatorPinInfoRet, Error> {
        trace!("{}", stringify!(domain_get_emulator_pin_info));
        let req: Option<binding::RemoteDomainGetEmulatorPinInfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetEmulatorPinInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetEmulatorPinInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_all_storage_pools(
        &mut self,
        args: binding::RemoteConnectListAllStoragePoolsArgs,
    ) -> Result<binding::RemoteConnectListAllStoragePoolsRet, Error> {
        trace!("{}", stringify!(connect_list_all_storage_pools));
        let req: Option<binding::RemoteConnectListAllStoragePoolsArgs> = Some(args);
        let res: Option<binding::RemoteConnectListAllStoragePoolsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListAllStoragePools,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn storage_pool_list_all_volumes(
        &mut self,
        args: binding::RemoteStoragePoolListAllVolumesArgs,
    ) -> Result<binding::RemoteStoragePoolListAllVolumesRet, Error> {
        trace!("{}", stringify!(storage_pool_list_all_volumes));
        let req: Option<binding::RemoteStoragePoolListAllVolumesArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolListAllVolumesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolListAllVolumes,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_all_networks(
        &mut self,
        args: binding::RemoteConnectListAllNetworksArgs,
    ) -> Result<binding::RemoteConnectListAllNetworksRet, Error> {
        trace!("{}", stringify!(connect_list_all_networks));
        let req: Option<binding::RemoteConnectListAllNetworksArgs> = Some(args);
        let res: Option<binding::RemoteConnectListAllNetworksRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListAllNetworks,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_all_interfaces(
        &mut self,
        args: binding::RemoteConnectListAllInterfacesArgs,
    ) -> Result<binding::RemoteConnectListAllInterfacesRet, Error> {
        trace!("{}", stringify!(connect_list_all_interfaces));
        let req: Option<binding::RemoteConnectListAllInterfacesArgs> = Some(args);
        let res: Option<binding::RemoteConnectListAllInterfacesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListAllInterfaces,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_all_node_devices(
        &mut self,
        args: binding::RemoteConnectListAllNodeDevicesArgs,
    ) -> Result<binding::RemoteConnectListAllNodeDevicesRet, Error> {
        trace!("{}", stringify!(connect_list_all_node_devices));
        let req: Option<binding::RemoteConnectListAllNodeDevicesArgs> = Some(args);
        let res: Option<binding::RemoteConnectListAllNodeDevicesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListAllNodeDevices,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_all_nwfilters(
        &mut self,
        args: binding::RemoteConnectListAllNwfiltersArgs,
    ) -> Result<binding::RemoteConnectListAllNwfiltersRet, Error> {
        trace!("{}", stringify!(connect_list_all_nwfilters));
        let req: Option<binding::RemoteConnectListAllNwfiltersArgs> = Some(args);
        let res: Option<binding::RemoteConnectListAllNwfiltersRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListAllNwfilters,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_list_all_secrets(
        &mut self,
        args: binding::RemoteConnectListAllSecretsArgs,
    ) -> Result<binding::RemoteConnectListAllSecretsRet, Error> {
        trace!("{}", stringify!(connect_list_all_secrets));
        let req: Option<binding::RemoteConnectListAllSecretsArgs> = Some(args);
        let res: Option<binding::RemoteConnectListAllSecretsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListAllSecrets,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_set_memory_parameters(
        &mut self,
        args: binding::RemoteNodeSetMemoryParametersArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_set_memory_parameters));
        let req: Option<binding::RemoteNodeSetMemoryParametersArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeSetMemoryParameters,
            req,
        )?;
        Ok(())
    }
    fn node_get_memory_parameters(
        &mut self,
        args: binding::RemoteNodeGetMemoryParametersArgs,
    ) -> Result<binding::RemoteNodeGetMemoryParametersRet, Error> {
        trace!("{}", stringify!(node_get_memory_parameters));
        let req: Option<binding::RemoteNodeGetMemoryParametersArgs> = Some(args);
        let res: Option<binding::RemoteNodeGetMemoryParametersRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeGetMemoryParameters,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_block_commit(
        &mut self,
        args: binding::RemoteDomainBlockCommitArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_commit));
        let req: Option<binding::RemoteDomainBlockCommitArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBlockCommit,
            req,
        )?;
        Ok(())
    }
    fn network_update(&mut self, args: binding::RemoteNetworkUpdateArgs) -> Result<(), Error> {
        trace!("{}", stringify!(network_update));
        let req: Option<binding::RemoteNetworkUpdateArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcNetworkUpdate, req)?;
        Ok(())
    }
    fn domain_event_pmsuspend_disk(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmsuspend_disk));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventPmsuspendDisk,
            req,
        )?;
        Ok(())
    }
    fn node_get_cpu_map(
        &mut self,
        args: binding::RemoteNodeGetCpuMapArgs,
    ) -> Result<binding::RemoteNodeGetCpuMapRet, Error> {
        trace!("{}", stringify!(node_get_cpu_map));
        let req: Option<binding::RemoteNodeGetCpuMapArgs> = Some(args);
        let res: Option<binding::RemoteNodeGetCpuMapRet> =
            call(self, binding::RemoteProcedure::RemoteProcNodeGetCpuMap, req)?;
        Ok(res.unwrap())
    }
    fn domain_fstrim(&mut self, args: binding::RemoteDomainFstrimArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_fstrim));
        let req: Option<binding::RemoteDomainFstrimArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainFstrim, req)?;
        Ok(())
    }
    fn domain_send_process_signal(
        &mut self,
        args: binding::RemoteDomainSendProcessSignalArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_send_process_signal));
        let req: Option<binding::RemoteDomainSendProcessSignalArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSendProcessSignal,
            req,
        )?;
        Ok(())
    }
    fn domain_open_channel(
        &mut self,
        args: binding::RemoteDomainOpenChannelArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_open_channel));
        let req: Option<binding::RemoteDomainOpenChannelArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainOpenChannel,
            req,
        )?;
        Ok(())
    }
    fn node_device_lookup_scsi_host_by_wwn(
        &mut self,
        args: binding::RemoteNodeDeviceLookupScsiHostByWwnArgs,
    ) -> Result<binding::RemoteNodeDeviceLookupScsiHostByWwnRet, Error> {
        trace!("{}", stringify!(node_device_lookup_scsi_host_by_wwn));
        let req: Option<binding::RemoteNodeDeviceLookupScsiHostByWwnArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceLookupScsiHostByWwnRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceLookupScsiHostByWwn,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_job_stats(
        &mut self,
        args: binding::RemoteDomainGetJobStatsArgs,
    ) -> Result<binding::RemoteDomainGetJobStatsRet, Error> {
        trace!("{}", stringify!(domain_get_job_stats));
        let req: Option<binding::RemoteDomainGetJobStatsArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetJobStatsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetJobStats,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_get_compression_cache(
        &mut self,
        args: binding::RemoteDomainMigrateGetCompressionCacheArgs,
    ) -> Result<binding::RemoteDomainMigrateGetCompressionCacheRet, Error> {
        trace!("{}", stringify!(domain_migrate_get_compression_cache));
        let req: Option<binding::RemoteDomainMigrateGetCompressionCacheArgs> = Some(args);
        let res: Option<binding::RemoteDomainMigrateGetCompressionCacheRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateGetCompressionCache,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_set_compression_cache(
        &mut self,
        args: binding::RemoteDomainMigrateSetCompressionCacheArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_set_compression_cache));
        let req: Option<binding::RemoteDomainMigrateSetCompressionCacheArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateSetCompressionCache,
            req,
        )?;
        Ok(())
    }
    fn node_device_detach_flags(
        &mut self,
        args: binding::RemoteNodeDeviceDetachFlagsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_detach_flags));
        let req: Option<binding::RemoteNodeDeviceDetachFlagsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceDetachFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_migrate_begin3params(
        &mut self,
        args: binding::RemoteDomainMigrateBegin3ParamsArgs,
    ) -> Result<binding::RemoteDomainMigrateBegin3ParamsRet, Error> {
        trace!("{}", stringify!(domain_migrate_begin3params));
        let req: Option<binding::RemoteDomainMigrateBegin3ParamsArgs> = Some(args);
        let res: Option<binding::RemoteDomainMigrateBegin3ParamsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateBegin3Params,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_prepare3params(
        &mut self,
        args: binding::RemoteDomainMigratePrepare3ParamsArgs,
    ) -> Result<binding::RemoteDomainMigratePrepare3ParamsRet, Error> {
        trace!("{}", stringify!(domain_migrate_prepare3params));
        let req: Option<binding::RemoteDomainMigratePrepare3ParamsArgs> = Some(args);
        let res: Option<binding::RemoteDomainMigratePrepare3ParamsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigratePrepare3Params,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_prepare_tunnel3params(
        &mut self,
        args: binding::RemoteDomainMigratePrepareTunnel3ParamsArgs,
    ) -> Result<binding::RemoteDomainMigratePrepareTunnel3ParamsRet, Error> {
        trace!("{}", stringify!(domain_migrate_prepare_tunnel3params));
        let req: Option<binding::RemoteDomainMigratePrepareTunnel3ParamsArgs> = Some(args);
        let res: Option<binding::RemoteDomainMigratePrepareTunnel3ParamsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigratePrepareTunnel3Params,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_perform3params(
        &mut self,
        args: binding::RemoteDomainMigratePerform3ParamsArgs,
    ) -> Result<binding::RemoteDomainMigratePerform3ParamsRet, Error> {
        trace!("{}", stringify!(domain_migrate_perform3params));
        let req: Option<binding::RemoteDomainMigratePerform3ParamsArgs> = Some(args);
        let res: Option<binding::RemoteDomainMigratePerform3ParamsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigratePerform3Params,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_finish3params(
        &mut self,
        args: binding::RemoteDomainMigrateFinish3ParamsArgs,
    ) -> Result<binding::RemoteDomainMigrateFinish3ParamsRet, Error> {
        trace!("{}", stringify!(domain_migrate_finish3params));
        let req: Option<binding::RemoteDomainMigrateFinish3ParamsArgs> = Some(args);
        let res: Option<binding::RemoteDomainMigrateFinish3ParamsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateFinish3Params,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_migrate_confirm3params(
        &mut self,
        args: binding::RemoteDomainMigrateConfirm3ParamsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_confirm3params));
        let req: Option<binding::RemoteDomainMigrateConfirm3ParamsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateConfirm3Params,
            req,
        )?;
        Ok(())
    }
    fn domain_set_memory_stats_period(
        &mut self,
        args: binding::RemoteDomainSetMemoryStatsPeriodArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_memory_stats_period));
        let req: Option<binding::RemoteDomainSetMemoryStatsPeriodArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetMemoryStatsPeriod,
            req,
        )?;
        Ok(())
    }
    fn domain_create_xml_with_files(
        &mut self,
        args: binding::RemoteDomainCreateXmlWithFilesArgs,
    ) -> Result<binding::RemoteDomainCreateXmlWithFilesRet, Error> {
        trace!("{}", stringify!(domain_create_xml_with_files));
        let req: Option<binding::RemoteDomainCreateXmlWithFilesArgs> = Some(args);
        let res: Option<binding::RemoteDomainCreateXmlWithFilesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCreateXmlWithFiles,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_create_with_files(
        &mut self,
        args: binding::RemoteDomainCreateWithFilesArgs,
    ) -> Result<binding::RemoteDomainCreateWithFilesRet, Error> {
        trace!("{}", stringify!(domain_create_with_files));
        let req: Option<binding::RemoteDomainCreateWithFilesArgs> = Some(args);
        let res: Option<binding::RemoteDomainCreateWithFilesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCreateWithFiles,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_device_removed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_device_removed));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventDeviceRemoved,
            req,
        )?;
        Ok(())
    }
    fn connect_get_cpu_model_names(
        &mut self,
        args: binding::RemoteConnectGetCpuModelNamesArgs,
    ) -> Result<binding::RemoteConnectGetCpuModelNamesRet, Error> {
        trace!("{}", stringify!(connect_get_cpu_model_names));
        let req: Option<binding::RemoteConnectGetCpuModelNamesArgs> = Some(args);
        let res: Option<binding::RemoteConnectGetCpuModelNamesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetCpuModelNames,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_network_event_register_any(
        &mut self,
        args: binding::RemoteConnectNetworkEventRegisterAnyArgs,
    ) -> Result<binding::RemoteConnectNetworkEventRegisterAnyRet, Error> {
        trace!("{}", stringify!(connect_network_event_register_any));
        let req: Option<binding::RemoteConnectNetworkEventRegisterAnyArgs> = Some(args);
        let res: Option<binding::RemoteConnectNetworkEventRegisterAnyRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNetworkEventRegisterAny,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_network_event_deregister_any(
        &mut self,
        args: binding::RemoteConnectNetworkEventDeregisterAnyArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(connect_network_event_deregister_any));
        let req: Option<binding::RemoteConnectNetworkEventDeregisterAnyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNetworkEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn network_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(network_event_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkEventLifecycle,
            req,
        )?;
        Ok(())
    }
    fn connect_domain_event_callback_register_any(
        &mut self,
        args: binding::RemoteConnectDomainEventCallbackRegisterAnyArgs,
    ) -> Result<binding::RemoteConnectDomainEventCallbackRegisterAnyRet, Error> {
        trace!("{}", stringify!(connect_domain_event_callback_register_any));
        let req: Option<binding::RemoteConnectDomainEventCallbackRegisterAnyArgs> = Some(args);
        let res: Option<binding::RemoteConnectDomainEventCallbackRegisterAnyRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectDomainEventCallbackRegisterAny,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_domain_event_callback_deregister_any(
        &mut self,
        args: binding::RemoteConnectDomainEventCallbackDeregisterAnyArgs,
    ) -> Result<(), Error> {
        trace!(
            "{}",
            stringify!(connect_domain_event_callback_deregister_any)
        );
        let req: Option<binding::RemoteConnectDomainEventCallbackDeregisterAnyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectDomainEventCallbackDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackLifecycle,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_reboot(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_reboot));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackReboot,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_rtc_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_rtc_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackRtcChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_watchdog(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_watchdog));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackWatchdog,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_io_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackIoError,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_graphics(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_graphics));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackGraphics,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_io_error_reason(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error_reason));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackIoErrorReason,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_control_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_control_error));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackControlError,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_block_job(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_block_job));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackBlockJob,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_disk_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_disk_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackDiskChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_tray_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_tray_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackTrayChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmwakeup(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmwakeup));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackPmwakeup,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmsuspend(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackPmsuspend,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_balloon_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_balloon_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackBalloonChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmsuspend_disk(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend_disk));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackPmsuspendDisk,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_device_removed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_device_removed));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackDeviceRemoved,
            req,
        )?;
        Ok(())
    }
    fn domain_core_dump_with_format(
        &mut self,
        args: binding::RemoteDomainCoreDumpWithFormatArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_core_dump_with_format));
        let req: Option<binding::RemoteDomainCoreDumpWithFormatArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCoreDumpWithFormat,
            req,
        )?;
        Ok(())
    }
    fn domain_fsfreeze(
        &mut self,
        args: binding::RemoteDomainFsfreezeArgs,
    ) -> Result<binding::RemoteDomainFsfreezeRet, Error> {
        trace!("{}", stringify!(domain_fsfreeze));
        let req: Option<binding::RemoteDomainFsfreezeArgs> = Some(args);
        let res: Option<binding::RemoteDomainFsfreezeRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainFsfreeze,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_fsthaw(
        &mut self,
        args: binding::RemoteDomainFsthawArgs,
    ) -> Result<binding::RemoteDomainFsthawRet, Error> {
        trace!("{}", stringify!(domain_fsthaw));
        let req: Option<binding::RemoteDomainFsthawArgs> = Some(args);
        let res: Option<binding::RemoteDomainFsthawRet> =
            call(self, binding::RemoteProcedure::RemoteProcDomainFsthaw, req)?;
        Ok(res.unwrap())
    }
    fn domain_get_time(
        &mut self,
        args: binding::RemoteDomainGetTimeArgs,
    ) -> Result<binding::RemoteDomainGetTimeRet, Error> {
        trace!("{}", stringify!(domain_get_time));
        let req: Option<binding::RemoteDomainGetTimeArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetTimeRet> =
            call(self, binding::RemoteProcedure::RemoteProcDomainGetTime, req)?;
        Ok(res.unwrap())
    }
    fn domain_set_time(&mut self, args: binding::RemoteDomainSetTimeArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_time));
        let req: Option<binding::RemoteDomainSetTimeArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainSetTime, req)?;
        Ok(())
    }
    fn domain_event_block_job2(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_job2));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventBlockJob2,
            req,
        )?;
        Ok(())
    }
    fn node_get_free_pages(
        &mut self,
        args: binding::RemoteNodeGetFreePagesArgs,
    ) -> Result<binding::RemoteNodeGetFreePagesRet, Error> {
        trace!("{}", stringify!(node_get_free_pages));
        let req: Option<binding::RemoteNodeGetFreePagesArgs> = Some(args);
        let res: Option<binding::RemoteNodeGetFreePagesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeGetFreePages,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_get_dhcp_leases(
        &mut self,
        args: binding::RemoteNetworkGetDhcpLeasesArgs,
    ) -> Result<binding::RemoteNetworkGetDhcpLeasesRet, Error> {
        trace!("{}", stringify!(network_get_dhcp_leases));
        let req: Option<binding::RemoteNetworkGetDhcpLeasesArgs> = Some(args);
        let res: Option<binding::RemoteNetworkGetDhcpLeasesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkGetDhcpLeases,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_get_domain_capabilities(
        &mut self,
        args: binding::RemoteConnectGetDomainCapabilitiesArgs,
    ) -> Result<binding::RemoteConnectGetDomainCapabilitiesRet, Error> {
        trace!("{}", stringify!(connect_get_domain_capabilities));
        let req: Option<binding::RemoteConnectGetDomainCapabilitiesArgs> = Some(args);
        let res: Option<binding::RemoteConnectGetDomainCapabilitiesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetDomainCapabilities,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_open_graphics_fd(
        &mut self,
        args: binding::RemoteDomainOpenGraphicsFdArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_open_graphics_fd));
        let req: Option<binding::RemoteDomainOpenGraphicsFdArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainOpenGraphicsFd,
            req,
        )?;
        Ok(())
    }
    fn connect_get_all_domain_stats(
        &mut self,
        args: binding::RemoteConnectGetAllDomainStatsArgs,
    ) -> Result<binding::RemoteConnectGetAllDomainStatsRet, Error> {
        trace!("{}", stringify!(connect_get_all_domain_stats));
        let req: Option<binding::RemoteConnectGetAllDomainStatsArgs> = Some(args);
        let res: Option<binding::RemoteConnectGetAllDomainStatsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetAllDomainStats,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_block_copy(&mut self, args: binding::RemoteDomainBlockCopyArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_copy));
        let req: Option<binding::RemoteDomainBlockCopyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBlockCopy,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_tunable(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_tunable));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackTunable,
            req,
        )?;
        Ok(())
    }
    fn node_alloc_pages(
        &mut self,
        args: binding::RemoteNodeAllocPagesArgs,
    ) -> Result<binding::RemoteNodeAllocPagesRet, Error> {
        trace!("{}", stringify!(node_alloc_pages));
        let req: Option<binding::RemoteNodeAllocPagesArgs> = Some(args);
        let res: Option<binding::RemoteNodeAllocPagesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeAllocPages,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_agent_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_agent_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackAgentLifecycle,
            req,
        )?;
        Ok(())
    }
    fn domain_get_fsinfo(
        &mut self,
        args: binding::RemoteDomainGetFsinfoArgs,
    ) -> Result<binding::RemoteDomainGetFsinfoRet, Error> {
        trace!("{}", stringify!(domain_get_fsinfo));
        let req: Option<binding::RemoteDomainGetFsinfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetFsinfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetFsinfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_define_xml_flags(
        &mut self,
        args: binding::RemoteDomainDefineXmlFlagsArgs,
    ) -> Result<binding::RemoteDomainDefineXmlFlagsRet, Error> {
        trace!("{}", stringify!(domain_define_xml_flags));
        let req: Option<binding::RemoteDomainDefineXmlFlagsArgs> = Some(args);
        let res: Option<binding::RemoteDomainDefineXmlFlagsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainDefineXmlFlags,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_iothread_info(
        &mut self,
        args: binding::RemoteDomainGetIothreadInfoArgs,
    ) -> Result<binding::RemoteDomainGetIothreadInfoRet, Error> {
        trace!("{}", stringify!(domain_get_iothread_info));
        let req: Option<binding::RemoteDomainGetIothreadInfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetIothreadInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetIothreadInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_pin_iothread(
        &mut self,
        args: binding::RemoteDomainPinIothreadArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pin_iothread));
        let req: Option<binding::RemoteDomainPinIothreadArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainPinIothread,
            req,
        )?;
        Ok(())
    }
    fn domain_interface_addresses(
        &mut self,
        args: binding::RemoteDomainInterfaceAddressesArgs,
    ) -> Result<binding::RemoteDomainInterfaceAddressesRet, Error> {
        trace!("{}", stringify!(domain_interface_addresses));
        let req: Option<binding::RemoteDomainInterfaceAddressesArgs> = Some(args);
        let res: Option<binding::RemoteDomainInterfaceAddressesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainInterfaceAddresses,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_device_added(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_device_added));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackDeviceAdded,
            req,
        )?;
        Ok(())
    }
    fn domain_add_iothread(
        &mut self,
        args: binding::RemoteDomainAddIothreadArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_add_iothread));
        let req: Option<binding::RemoteDomainAddIothreadArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainAddIothread,
            req,
        )?;
        Ok(())
    }
    fn domain_del_iothread(
        &mut self,
        args: binding::RemoteDomainDelIothreadArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_del_iothread));
        let req: Option<binding::RemoteDomainDelIothreadArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainDelIothread,
            req,
        )?;
        Ok(())
    }
    fn domain_set_user_password(
        &mut self,
        args: binding::RemoteDomainSetUserPasswordArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_user_password));
        let req: Option<binding::RemoteDomainSetUserPasswordArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetUserPassword,
            req,
        )?;
        Ok(())
    }
    fn domain_rename(
        &mut self,
        args: binding::RemoteDomainRenameArgs,
    ) -> Result<binding::RemoteDomainRenameRet, Error> {
        trace!("{}", stringify!(domain_rename));
        let req: Option<binding::RemoteDomainRenameArgs> = Some(args);
        let res: Option<binding::RemoteDomainRenameRet> =
            call(self, binding::RemoteProcedure::RemoteProcDomainRename, req)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_migration_iteration(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_migration_iteration));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackMigrationIteration,
            req,
        )?;
        Ok(())
    }
    fn connect_register_close_callback(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_register_close_callback));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectRegisterCloseCallback,
            req,
        )?;
        Ok(())
    }
    fn connect_unregister_close_callback(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_unregister_close_callback));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectUnregisterCloseCallback,
            req,
        )?;
        Ok(())
    }
    fn connect_event_connection_closed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_event_connection_closed));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectEventConnectionClosed,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_job_completed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_job_completed));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackJobCompleted,
            req,
        )?;
        Ok(())
    }
    fn domain_migrate_start_post_copy(
        &mut self,
        args: binding::RemoteDomainMigrateStartPostCopyArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_start_post_copy));
        let req: Option<binding::RemoteDomainMigrateStartPostCopyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateStartPostCopy,
            req,
        )?;
        Ok(())
    }
    fn domain_get_perf_events(
        &mut self,
        args: binding::RemoteDomainGetPerfEventsArgs,
    ) -> Result<binding::RemoteDomainGetPerfEventsRet, Error> {
        trace!("{}", stringify!(domain_get_perf_events));
        let req: Option<binding::RemoteDomainGetPerfEventsArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetPerfEventsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetPerfEvents,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_set_perf_events(
        &mut self,
        args: binding::RemoteDomainSetPerfEventsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_perf_events));
        let req: Option<binding::RemoteDomainSetPerfEventsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetPerfEvents,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_device_removal_failed(&mut self) -> Result<(), Error> {
        trace!(
            "{}",
            stringify!(domain_event_callback_device_removal_failed)
        );
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackDeviceRemovalFailed,
            req,
        )?;
        Ok(())
    }
    fn connect_storage_pool_event_register_any(
        &mut self,
        args: binding::RemoteConnectStoragePoolEventRegisterAnyArgs,
    ) -> Result<binding::RemoteConnectStoragePoolEventRegisterAnyRet, Error> {
        trace!("{}", stringify!(connect_storage_pool_event_register_any));
        let req: Option<binding::RemoteConnectStoragePoolEventRegisterAnyArgs> = Some(args);
        let res: Option<binding::RemoteConnectStoragePoolEventRegisterAnyRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectStoragePoolEventRegisterAny,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_storage_pool_event_deregister_any(
        &mut self,
        args: binding::RemoteConnectStoragePoolEventDeregisterAnyArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(connect_storage_pool_event_deregister_any));
        let req: Option<binding::RemoteConnectStoragePoolEventDeregisterAnyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectStoragePoolEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_event_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolEventLifecycle,
            req,
        )?;
        Ok(())
    }
    fn domain_get_guest_vcpus(
        &mut self,
        args: binding::RemoteDomainGetGuestVcpusArgs,
    ) -> Result<binding::RemoteDomainGetGuestVcpusRet, Error> {
        trace!("{}", stringify!(domain_get_guest_vcpus));
        let req: Option<binding::RemoteDomainGetGuestVcpusArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetGuestVcpusRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetGuestVcpus,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_set_guest_vcpus(
        &mut self,
        args: binding::RemoteDomainSetGuestVcpusArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_guest_vcpus));
        let req: Option<binding::RemoteDomainSetGuestVcpusArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetGuestVcpus,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_event_refresh(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_event_refresh));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolEventRefresh,
            req,
        )?;
        Ok(())
    }
    fn connect_node_device_event_register_any(
        &mut self,
        args: binding::RemoteConnectNodeDeviceEventRegisterAnyArgs,
    ) -> Result<binding::RemoteConnectNodeDeviceEventRegisterAnyRet, Error> {
        trace!("{}", stringify!(connect_node_device_event_register_any));
        let req: Option<binding::RemoteConnectNodeDeviceEventRegisterAnyArgs> = Some(args);
        let res: Option<binding::RemoteConnectNodeDeviceEventRegisterAnyRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNodeDeviceEventRegisterAny,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_node_device_event_deregister_any(
        &mut self,
        args: binding::RemoteConnectNodeDeviceEventDeregisterAnyArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(connect_node_device_event_deregister_any));
        let req: Option<binding::RemoteConnectNodeDeviceEventDeregisterAnyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectNodeDeviceEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn node_device_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_event_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceEventLifecycle,
            req,
        )?;
        Ok(())
    }
    fn node_device_event_update(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_event_update));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceEventUpdate,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_get_info_flags(
        &mut self,
        args: binding::RemoteStorageVolGetInfoFlagsArgs,
    ) -> Result<binding::RemoteStorageVolGetInfoFlagsRet, Error> {
        trace!("{}", stringify!(storage_vol_get_info_flags));
        let req: Option<binding::RemoteStorageVolGetInfoFlagsArgs> = Some(args);
        let res: Option<binding::RemoteStorageVolGetInfoFlagsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStorageVolGetInfoFlags,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_metadata_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_metadata_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventCallbackMetadataChange,
            req,
        )?;
        Ok(())
    }
    fn connect_secret_event_register_any(
        &mut self,
        args: binding::RemoteConnectSecretEventRegisterAnyArgs,
    ) -> Result<binding::RemoteConnectSecretEventRegisterAnyRet, Error> {
        trace!("{}", stringify!(connect_secret_event_register_any));
        let req: Option<binding::RemoteConnectSecretEventRegisterAnyArgs> = Some(args);
        let res: Option<binding::RemoteConnectSecretEventRegisterAnyRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectSecretEventRegisterAny,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_secret_event_deregister_any(
        &mut self,
        args: binding::RemoteConnectSecretEventDeregisterAnyArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(connect_secret_event_deregister_any));
        let req: Option<binding::RemoteConnectSecretEventDeregisterAnyArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectSecretEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn secret_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(secret_event_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcSecretEventLifecycle,
            req,
        )?;
        Ok(())
    }
    fn secret_event_value_changed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(secret_event_value_changed));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcSecretEventValueChanged,
            req,
        )?;
        Ok(())
    }
    fn domain_set_vcpu(&mut self, args: binding::RemoteDomainSetVcpuArgs) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_vcpu));
        let req: Option<binding::RemoteDomainSetVcpuArgs> = Some(args);
        let _res: Option<()> = call(self, binding::RemoteProcedure::RemoteProcDomainSetVcpu, req)?;
        Ok(())
    }
    fn domain_event_block_threshold(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_threshold));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventBlockThreshold,
            req,
        )?;
        Ok(())
    }
    fn domain_set_block_threshold(
        &mut self,
        args: binding::RemoteDomainSetBlockThresholdArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_block_threshold));
        let req: Option<binding::RemoteDomainSetBlockThresholdArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetBlockThreshold,
            req,
        )?;
        Ok(())
    }
    fn domain_migrate_get_max_downtime(
        &mut self,
        args: binding::RemoteDomainMigrateGetMaxDowntimeArgs,
    ) -> Result<binding::RemoteDomainMigrateGetMaxDowntimeRet, Error> {
        trace!("{}", stringify!(domain_migrate_get_max_downtime));
        let req: Option<binding::RemoteDomainMigrateGetMaxDowntimeArgs> = Some(args);
        let res: Option<binding::RemoteDomainMigrateGetMaxDowntimeRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainMigrateGetMaxDowntime,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_managed_save_get_xml_desc(
        &mut self,
        args: binding::RemoteDomainManagedSaveGetXmlDescArgs,
    ) -> Result<binding::RemoteDomainManagedSaveGetXmlDescRet, Error> {
        trace!("{}", stringify!(domain_managed_save_get_xml_desc));
        let req: Option<binding::RemoteDomainManagedSaveGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteDomainManagedSaveGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainManagedSaveGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_managed_save_define_xml(
        &mut self,
        args: binding::RemoteDomainManagedSaveDefineXmlArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_managed_save_define_xml));
        let req: Option<binding::RemoteDomainManagedSaveDefineXmlArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainManagedSaveDefineXml,
            req,
        )?;
        Ok(())
    }
    fn domain_set_lifecycle_action(
        &mut self,
        args: binding::RemoteDomainSetLifecycleActionArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_lifecycle_action));
        let req: Option<binding::RemoteDomainSetLifecycleActionArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetLifecycleAction,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_lookup_by_target_path(
        &mut self,
        args: binding::RemoteStoragePoolLookupByTargetPathArgs,
    ) -> Result<binding::RemoteStoragePoolLookupByTargetPathRet, Error> {
        trace!("{}", stringify!(storage_pool_lookup_by_target_path));
        let req: Option<binding::RemoteStoragePoolLookupByTargetPathArgs> = Some(args);
        let res: Option<binding::RemoteStoragePoolLookupByTargetPathRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcStoragePoolLookupByTargetPath,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_detach_device_alias(
        &mut self,
        args: binding::RemoteDomainDetachDeviceAliasArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_detach_device_alias));
        let req: Option<binding::RemoteDomainDetachDeviceAliasArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainDetachDeviceAlias,
            req,
        )?;
        Ok(())
    }
    fn connect_compare_hypervisor_cpu(
        &mut self,
        args: binding::RemoteConnectCompareHypervisorCpuArgs,
    ) -> Result<binding::RemoteConnectCompareHypervisorCpuRet, Error> {
        trace!("{}", stringify!(connect_compare_hypervisor_cpu));
        let req: Option<binding::RemoteConnectCompareHypervisorCpuArgs> = Some(args);
        let res: Option<binding::RemoteConnectCompareHypervisorCpuRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectCompareHypervisorCpu,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_baseline_hypervisor_cpu(
        &mut self,
        args: binding::RemoteConnectBaselineHypervisorCpuArgs,
    ) -> Result<binding::RemoteConnectBaselineHypervisorCpuRet, Error> {
        trace!("{}", stringify!(connect_baseline_hypervisor_cpu));
        let req: Option<binding::RemoteConnectBaselineHypervisorCpuArgs> = Some(args);
        let res: Option<binding::RemoteConnectBaselineHypervisorCpuRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectBaselineHypervisorCpu,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_get_sev_info(
        &mut self,
        args: binding::RemoteNodeGetSevInfoArgs,
    ) -> Result<binding::RemoteNodeGetSevInfoRet, Error> {
        trace!("{}", stringify!(node_get_sev_info));
        let req: Option<binding::RemoteNodeGetSevInfoArgs> = Some(args);
        let res: Option<binding::RemoteNodeGetSevInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeGetSevInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_get_launch_security_info(
        &mut self,
        args: binding::RemoteDomainGetLaunchSecurityInfoArgs,
    ) -> Result<binding::RemoteDomainGetLaunchSecurityInfoRet, Error> {
        trace!("{}", stringify!(domain_get_launch_security_info));
        let req: Option<binding::RemoteDomainGetLaunchSecurityInfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetLaunchSecurityInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetLaunchSecurityInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn nwfilter_binding_lookup_by_port_dev(
        &mut self,
        args: binding::RemoteNwfilterBindingLookupByPortDevArgs,
    ) -> Result<binding::RemoteNwfilterBindingLookupByPortDevRet, Error> {
        trace!("{}", stringify!(nwfilter_binding_lookup_by_port_dev));
        let req: Option<binding::RemoteNwfilterBindingLookupByPortDevArgs> = Some(args);
        let res: Option<binding::RemoteNwfilterBindingLookupByPortDevRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNwfilterBindingLookupByPortDev,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn nwfilter_binding_get_xml_desc(
        &mut self,
        args: binding::RemoteNwfilterBindingGetXmlDescArgs,
    ) -> Result<binding::RemoteNwfilterBindingGetXmlDescRet, Error> {
        trace!("{}", stringify!(nwfilter_binding_get_xml_desc));
        let req: Option<binding::RemoteNwfilterBindingGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteNwfilterBindingGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNwfilterBindingGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn nwfilter_binding_create_xml(
        &mut self,
        args: binding::RemoteNwfilterBindingCreateXmlArgs,
    ) -> Result<binding::RemoteNwfilterBindingCreateXmlRet, Error> {
        trace!("{}", stringify!(nwfilter_binding_create_xml));
        let req: Option<binding::RemoteNwfilterBindingCreateXmlArgs> = Some(args);
        let res: Option<binding::RemoteNwfilterBindingCreateXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNwfilterBindingCreateXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn nwfilter_binding_delete(
        &mut self,
        args: binding::RemoteNwfilterBindingDeleteArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(nwfilter_binding_delete));
        let req: Option<binding::RemoteNwfilterBindingDeleteArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNwfilterBindingDelete,
            req,
        )?;
        Ok(())
    }
    fn connect_list_all_nwfilter_bindings(
        &mut self,
        args: binding::RemoteConnectListAllNwfilterBindingsArgs,
    ) -> Result<binding::RemoteConnectListAllNwfilterBindingsRet, Error> {
        trace!("{}", stringify!(connect_list_all_nwfilter_bindings));
        let req: Option<binding::RemoteConnectListAllNwfilterBindingsArgs> = Some(args);
        let res: Option<binding::RemoteConnectListAllNwfilterBindingsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectListAllNwfilterBindings,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_set_iothread_params(
        &mut self,
        args: binding::RemoteDomainSetIothreadParamsArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_iothread_params));
        let req: Option<binding::RemoteDomainSetIothreadParamsArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetIothreadParams,
            req,
        )?;
        Ok(())
    }
    fn connect_get_storage_pool_capabilities(
        &mut self,
        args: binding::RemoteConnectGetStoragePoolCapabilitiesArgs,
    ) -> Result<binding::RemoteConnectGetStoragePoolCapabilitiesRet, Error> {
        trace!("{}", stringify!(connect_get_storage_pool_capabilities));
        let req: Option<binding::RemoteConnectGetStoragePoolCapabilitiesArgs> = Some(args);
        let res: Option<binding::RemoteConnectGetStoragePoolCapabilitiesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectGetStoragePoolCapabilities,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_list_all_ports(
        &mut self,
        args: binding::RemoteNetworkListAllPortsArgs,
    ) -> Result<binding::RemoteNetworkListAllPortsRet, Error> {
        trace!("{}", stringify!(network_list_all_ports));
        let req: Option<binding::RemoteNetworkListAllPortsArgs> = Some(args);
        let res: Option<binding::RemoteNetworkListAllPortsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkListAllPorts,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_port_lookup_by_uuid(
        &mut self,
        args: binding::RemoteNetworkPortLookupByUuidArgs,
    ) -> Result<binding::RemoteNetworkPortLookupByUuidRet, Error> {
        trace!("{}", stringify!(network_port_lookup_by_uuid));
        let req: Option<binding::RemoteNetworkPortLookupByUuidArgs> = Some(args);
        let res: Option<binding::RemoteNetworkPortLookupByUuidRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkPortLookupByUuid,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_port_create_xml(
        &mut self,
        args: binding::RemoteNetworkPortCreateXmlArgs,
    ) -> Result<binding::RemoteNetworkPortCreateXmlRet, Error> {
        trace!("{}", stringify!(network_port_create_xml));
        let req: Option<binding::RemoteNetworkPortCreateXmlArgs> = Some(args);
        let res: Option<binding::RemoteNetworkPortCreateXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkPortCreateXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_port_get_parameters(
        &mut self,
        args: binding::RemoteNetworkPortGetParametersArgs,
    ) -> Result<binding::RemoteNetworkPortGetParametersRet, Error> {
        trace!("{}", stringify!(network_port_get_parameters));
        let req: Option<binding::RemoteNetworkPortGetParametersArgs> = Some(args);
        let res: Option<binding::RemoteNetworkPortGetParametersRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkPortGetParameters,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_port_set_parameters(
        &mut self,
        args: binding::RemoteNetworkPortSetParametersArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(network_port_set_parameters));
        let req: Option<binding::RemoteNetworkPortSetParametersArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkPortSetParameters,
            req,
        )?;
        Ok(())
    }
    fn network_port_get_xml_desc(
        &mut self,
        args: binding::RemoteNetworkPortGetXmlDescArgs,
    ) -> Result<binding::RemoteNetworkPortGetXmlDescRet, Error> {
        trace!("{}", stringify!(network_port_get_xml_desc));
        let req: Option<binding::RemoteNetworkPortGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteNetworkPortGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkPortGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_port_delete(
        &mut self,
        args: binding::RemoteNetworkPortDeleteArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(network_port_delete));
        let req: Option<binding::RemoteNetworkPortDeleteArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkPortDelete,
            req,
        )?;
        Ok(())
    }
    fn domain_checkpoint_create_xml(
        &mut self,
        args: binding::RemoteDomainCheckpointCreateXmlArgs,
    ) -> Result<binding::RemoteDomainCheckpointCreateXmlRet, Error> {
        trace!("{}", stringify!(domain_checkpoint_create_xml));
        let req: Option<binding::RemoteDomainCheckpointCreateXmlArgs> = Some(args);
        let res: Option<binding::RemoteDomainCheckpointCreateXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCheckpointCreateXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_checkpoint_get_xml_desc(
        &mut self,
        args: binding::RemoteDomainCheckpointGetXmlDescArgs,
    ) -> Result<binding::RemoteDomainCheckpointGetXmlDescRet, Error> {
        trace!("{}", stringify!(domain_checkpoint_get_xml_desc));
        let req: Option<binding::RemoteDomainCheckpointGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteDomainCheckpointGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCheckpointGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_list_all_checkpoints(
        &mut self,
        args: binding::RemoteDomainListAllCheckpointsArgs,
    ) -> Result<binding::RemoteDomainListAllCheckpointsRet, Error> {
        trace!("{}", stringify!(domain_list_all_checkpoints));
        let req: Option<binding::RemoteDomainListAllCheckpointsArgs> = Some(args);
        let res: Option<binding::RemoteDomainListAllCheckpointsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainListAllCheckpoints,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_checkpoint_list_all_children(
        &mut self,
        args: binding::RemoteDomainCheckpointListAllChildrenArgs,
    ) -> Result<binding::RemoteDomainCheckpointListAllChildrenRet, Error> {
        trace!("{}", stringify!(domain_checkpoint_list_all_children));
        let req: Option<binding::RemoteDomainCheckpointListAllChildrenArgs> = Some(args);
        let res: Option<binding::RemoteDomainCheckpointListAllChildrenRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCheckpointListAllChildren,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_checkpoint_lookup_by_name(
        &mut self,
        args: binding::RemoteDomainCheckpointLookupByNameArgs,
    ) -> Result<binding::RemoteDomainCheckpointLookupByNameRet, Error> {
        trace!("{}", stringify!(domain_checkpoint_lookup_by_name));
        let req: Option<binding::RemoteDomainCheckpointLookupByNameArgs> = Some(args);
        let res: Option<binding::RemoteDomainCheckpointLookupByNameRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCheckpointLookupByName,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_checkpoint_get_parent(
        &mut self,
        args: binding::RemoteDomainCheckpointGetParentArgs,
    ) -> Result<binding::RemoteDomainCheckpointGetParentRet, Error> {
        trace!("{}", stringify!(domain_checkpoint_get_parent));
        let req: Option<binding::RemoteDomainCheckpointGetParentArgs> = Some(args);
        let res: Option<binding::RemoteDomainCheckpointGetParentRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCheckpointGetParent,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_checkpoint_delete(
        &mut self,
        args: binding::RemoteDomainCheckpointDeleteArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_checkpoint_delete));
        let req: Option<binding::RemoteDomainCheckpointDeleteArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainCheckpointDelete,
            req,
        )?;
        Ok(())
    }
    fn domain_get_guest_info(
        &mut self,
        args: binding::RemoteDomainGetGuestInfoArgs,
    ) -> Result<binding::RemoteDomainGetGuestInfoRet, Error> {
        trace!("{}", stringify!(domain_get_guest_info));
        let req: Option<binding::RemoteDomainGetGuestInfoArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetGuestInfoRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetGuestInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn connect_set_identity(
        &mut self,
        args: binding::RemoteConnectSetIdentityArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(connect_set_identity));
        let req: Option<binding::RemoteConnectSetIdentityArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcConnectSetIdentity,
            req,
        )?;
        Ok(())
    }
    fn domain_agent_set_response_timeout(
        &mut self,
        args: binding::RemoteDomainAgentSetResponseTimeoutArgs,
    ) -> Result<binding::RemoteDomainAgentSetResponseTimeoutRet, Error> {
        trace!("{}", stringify!(domain_agent_set_response_timeout));
        let req: Option<binding::RemoteDomainAgentSetResponseTimeoutArgs> = Some(args);
        let res: Option<binding::RemoteDomainAgentSetResponseTimeoutRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainAgentSetResponseTimeout,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_backup_begin(
        &mut self,
        args: binding::RemoteDomainBackupBeginArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_backup_begin));
        let req: Option<binding::RemoteDomainBackupBeginArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBackupBegin,
            req,
        )?;
        Ok(())
    }
    fn domain_backup_get_xml_desc(
        &mut self,
        args: binding::RemoteDomainBackupGetXmlDescArgs,
    ) -> Result<binding::RemoteDomainBackupGetXmlDescRet, Error> {
        trace!("{}", stringify!(domain_backup_get_xml_desc));
        let req: Option<binding::RemoteDomainBackupGetXmlDescArgs> = Some(args);
        let res: Option<binding::RemoteDomainBackupGetXmlDescRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainBackupGetXmlDesc,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_memory_failure(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_memory_failure));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventMemoryFailure,
            req,
        )?;
        Ok(())
    }
    fn domain_authorized_ssh_keys_get(
        &mut self,
        args: binding::RemoteDomainAuthorizedSshKeysGetArgs,
    ) -> Result<binding::RemoteDomainAuthorizedSshKeysGetRet, Error> {
        trace!("{}", stringify!(domain_authorized_ssh_keys_get));
        let req: Option<binding::RemoteDomainAuthorizedSshKeysGetArgs> = Some(args);
        let res: Option<binding::RemoteDomainAuthorizedSshKeysGetRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainAuthorizedSshKeysGet,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_authorized_ssh_keys_set(
        &mut self,
        args: binding::RemoteDomainAuthorizedSshKeysSetArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_authorized_ssh_keys_set));
        let req: Option<binding::RemoteDomainAuthorizedSshKeysSetArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainAuthorizedSshKeysSet,
            req,
        )?;
        Ok(())
    }
    fn domain_get_messages(
        &mut self,
        args: binding::RemoteDomainGetMessagesArgs,
    ) -> Result<binding::RemoteDomainGetMessagesRet, Error> {
        trace!("{}", stringify!(domain_get_messages));
        let req: Option<binding::RemoteDomainGetMessagesArgs> = Some(args);
        let res: Option<binding::RemoteDomainGetMessagesRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainGetMessages,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_start_dirty_rate_calc(
        &mut self,
        args: binding::RemoteDomainStartDirtyRateCalcArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_start_dirty_rate_calc));
        let req: Option<binding::RemoteDomainStartDirtyRateCalcArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainStartDirtyRateCalc,
            req,
        )?;
        Ok(())
    }
    fn node_device_define_xml(
        &mut self,
        args: binding::RemoteNodeDeviceDefineXmlArgs,
    ) -> Result<binding::RemoteNodeDeviceDefineXmlRet, Error> {
        trace!("{}", stringify!(node_device_define_xml));
        let req: Option<binding::RemoteNodeDeviceDefineXmlArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceDefineXmlRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceDefineXml,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_undefine(
        &mut self,
        args: binding::RemoteNodeDeviceUndefineArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_undefine));
        let req: Option<binding::RemoteNodeDeviceUndefineArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceUndefine,
            req,
        )?;
        Ok(())
    }
    fn node_device_create(
        &mut self,
        args: binding::RemoteNodeDeviceCreateArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_create));
        let req: Option<binding::RemoteNodeDeviceCreateArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceCreate,
            req,
        )?;
        Ok(())
    }
    fn nwfilter_define_xml_flags(
        &mut self,
        args: binding::RemoteNwfilterDefineXmlFlagsArgs,
    ) -> Result<binding::RemoteNwfilterDefineXmlFlagsRet, Error> {
        trace!("{}", stringify!(nwfilter_define_xml_flags));
        let req: Option<binding::RemoteNwfilterDefineXmlFlagsArgs> = Some(args);
        let res: Option<binding::RemoteNwfilterDefineXmlFlagsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNwfilterDefineXmlFlags,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_define_xml_flags(
        &mut self,
        args: binding::RemoteNetworkDefineXmlFlagsArgs,
    ) -> Result<binding::RemoteNetworkDefineXmlFlagsRet, Error> {
        trace!("{}", stringify!(network_define_xml_flags));
        let req: Option<binding::RemoteNetworkDefineXmlFlagsArgs> = Some(args);
        let res: Option<binding::RemoteNetworkDefineXmlFlagsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkDefineXmlFlags,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_get_autostart(
        &mut self,
        args: binding::RemoteNodeDeviceGetAutostartArgs,
    ) -> Result<binding::RemoteNodeDeviceGetAutostartRet, Error> {
        trace!("{}", stringify!(node_device_get_autostart));
        let req: Option<binding::RemoteNodeDeviceGetAutostartArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceGetAutostartRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceGetAutostart,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_set_autostart(
        &mut self,
        args: binding::RemoteNodeDeviceSetAutostartArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_set_autostart));
        let req: Option<binding::RemoteNodeDeviceSetAutostartArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceSetAutostart,
            req,
        )?;
        Ok(())
    }
    fn node_device_is_persistent(
        &mut self,
        args: binding::RemoteNodeDeviceIsPersistentArgs,
    ) -> Result<binding::RemoteNodeDeviceIsPersistentRet, Error> {
        trace!("{}", stringify!(node_device_is_persistent));
        let req: Option<binding::RemoteNodeDeviceIsPersistentArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceIsPersistentRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceIsPersistent,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn node_device_is_active(
        &mut self,
        args: binding::RemoteNodeDeviceIsActiveArgs,
    ) -> Result<binding::RemoteNodeDeviceIsActiveRet, Error> {
        trace!("{}", stringify!(node_device_is_active));
        let req: Option<binding::RemoteNodeDeviceIsActiveArgs> = Some(args);
        let res: Option<binding::RemoteNodeDeviceIsActiveRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNodeDeviceIsActive,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn network_create_xml_flags(
        &mut self,
        args: binding::RemoteNetworkCreateXmlFlagsArgs,
    ) -> Result<binding::RemoteNetworkCreateXmlFlagsRet, Error> {
        trace!("{}", stringify!(network_create_xml_flags));
        let req: Option<binding::RemoteNetworkCreateXmlFlagsArgs> = Some(args);
        let res: Option<binding::RemoteNetworkCreateXmlFlagsRet> = call(
            self,
            binding::RemoteProcedure::RemoteProcNetworkCreateXmlFlags,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_event_memory_device_size_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_memory_device_size_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainEventMemoryDeviceSizeChange,
            req,
        )?;
        Ok(())
    }
    fn domain_set_launch_security_state(
        &mut self,
        args: binding::RemoteDomainSetLaunchSecurityStateArgs,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_launch_security_state));
        let req: Option<binding::RemoteDomainSetLaunchSecurityStateArgs> = Some(args);
        let _res: Option<()> = call(
            self,
            binding::RemoteProcedure::RemoteProcDomainSetLaunchSecurityState,
            req,
        )?;
        Ok(())
    }
    fn domain_event_lifecycle_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventLifecycleMsg, Error> {
        trace!("{}", stringify!(domain_event_lifecycle_msg));
        let res: Option<binding::RemoteDomainEventLifecycleMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_lifecycle_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackLifecycleMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_lifecycle_msg));
        let res: Option<binding::RemoteDomainEventCallbackLifecycleMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_reboot_msg(&mut self) -> Result<binding::RemoteDomainEventRebootMsg, Error> {
        trace!("{}", stringify!(domain_event_reboot_msg));
        let res: Option<binding::RemoteDomainEventRebootMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_reboot_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackRebootMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_reboot_msg));
        let res: Option<binding::RemoteDomainEventCallbackRebootMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_rtc_change_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventRtcChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_rtc_change_msg));
        let res: Option<binding::RemoteDomainEventRtcChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_rtc_change_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackRtcChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_rtc_change_msg));
        let res: Option<binding::RemoteDomainEventCallbackRtcChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_watchdog_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventWatchdogMsg, Error> {
        trace!("{}", stringify!(domain_event_watchdog_msg));
        let res: Option<binding::RemoteDomainEventWatchdogMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_watchdog_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackWatchdogMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_watchdog_msg));
        let res: Option<binding::RemoteDomainEventCallbackWatchdogMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_io_error_msg(&mut self) -> Result<binding::RemoteDomainEventIoErrorMsg, Error> {
        trace!("{}", stringify!(domain_event_io_error_msg));
        let res: Option<binding::RemoteDomainEventIoErrorMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_io_error_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackIoErrorMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_io_error_msg));
        let res: Option<binding::RemoteDomainEventCallbackIoErrorMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_io_error_reason_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventIoErrorReasonMsg, Error> {
        trace!("{}", stringify!(domain_event_io_error_reason_msg));
        let res: Option<binding::RemoteDomainEventIoErrorReasonMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_io_error_reason_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackIoErrorReasonMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_io_error_reason_msg));
        let res: Option<binding::RemoteDomainEventCallbackIoErrorReasonMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_graphics_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventGraphicsMsg, Error> {
        trace!("{}", stringify!(domain_event_graphics_msg));
        let res: Option<binding::RemoteDomainEventGraphicsMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_graphics_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackGraphicsMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_graphics_msg));
        let res: Option<binding::RemoteDomainEventCallbackGraphicsMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_block_job_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventBlockJobMsg, Error> {
        trace!("{}", stringify!(domain_event_block_job_msg));
        let res: Option<binding::RemoteDomainEventBlockJobMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_block_job_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackBlockJobMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_block_job_msg));
        let res: Option<binding::RemoteDomainEventCallbackBlockJobMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_disk_change_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventDiskChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_disk_change_msg));
        let res: Option<binding::RemoteDomainEventDiskChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_disk_change_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackDiskChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_disk_change_msg));
        let res: Option<binding::RemoteDomainEventCallbackDiskChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_tray_change_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventTrayChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_tray_change_msg));
        let res: Option<binding::RemoteDomainEventTrayChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_tray_change_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackTrayChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_tray_change_msg));
        let res: Option<binding::RemoteDomainEventCallbackTrayChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_pmwakeup_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventPmwakeupMsg, Error> {
        trace!("{}", stringify!(domain_event_pmwakeup_msg));
        let res: Option<binding::RemoteDomainEventPmwakeupMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_pmwakeup_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackPmwakeupMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_pmwakeup_msg));
        let res: Option<binding::RemoteDomainEventCallbackPmwakeupMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_pmsuspend_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventPmsuspendMsg, Error> {
        trace!("{}", stringify!(domain_event_pmsuspend_msg));
        let res: Option<binding::RemoteDomainEventPmsuspendMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_pmsuspend_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackPmsuspendMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend_msg));
        let res: Option<binding::RemoteDomainEventCallbackPmsuspendMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_balloon_change_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventBalloonChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_balloon_change_msg));
        let res: Option<binding::RemoteDomainEventBalloonChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_balloon_change_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackBalloonChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_balloon_change_msg));
        let res: Option<binding::RemoteDomainEventCallbackBalloonChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_pmsuspend_disk_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventPmsuspendDiskMsg, Error> {
        trace!("{}", stringify!(domain_event_pmsuspend_disk_msg));
        let res: Option<binding::RemoteDomainEventPmsuspendDiskMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_pmsuspend_disk_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackPmsuspendDiskMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend_disk_msg));
        let res: Option<binding::RemoteDomainEventCallbackPmsuspendDiskMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_control_error_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventControlErrorMsg, Error> {
        trace!("{}", stringify!(domain_event_control_error_msg));
        let res: Option<binding::RemoteDomainEventControlErrorMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_control_error_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackControlErrorMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_control_error_msg));
        let res: Option<binding::RemoteDomainEventCallbackControlErrorMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_device_removed_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventDeviceRemovedMsg, Error> {
        trace!("{}", stringify!(domain_event_device_removed_msg));
        let res: Option<binding::RemoteDomainEventDeviceRemovedMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_device_removed_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackDeviceRemovedMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_device_removed_msg));
        let res: Option<binding::RemoteDomainEventCallbackDeviceRemovedMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_block_job2msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventBlockJob2Msg, Error> {
        trace!("{}", stringify!(domain_event_block_job2msg));
        let res: Option<binding::RemoteDomainEventBlockJob2Msg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_block_threshold_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventBlockThresholdMsg, Error> {
        trace!("{}", stringify!(domain_event_block_threshold_msg));
        let res: Option<binding::RemoteDomainEventBlockThresholdMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_tunable_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackTunableMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_tunable_msg));
        let res: Option<binding::RemoteDomainEventCallbackTunableMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_device_added_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackDeviceAddedMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_device_added_msg));
        let res: Option<binding::RemoteDomainEventCallbackDeviceAddedMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn connect_event_connection_closed_msg(
        &mut self,
    ) -> Result<binding::RemoteConnectEventConnectionClosedMsg, Error> {
        trace!("{}", stringify!(connect_event_connection_closed_msg));
        let res: Option<binding::RemoteConnectEventConnectionClosedMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn network_event_lifecycle_msg(
        &mut self,
    ) -> Result<binding::RemoteNetworkEventLifecycleMsg, Error> {
        trace!("{}", stringify!(network_event_lifecycle_msg));
        let res: Option<binding::RemoteNetworkEventLifecycleMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn storage_pool_event_lifecycle_msg(
        &mut self,
    ) -> Result<binding::RemoteStoragePoolEventLifecycleMsg, Error> {
        trace!("{}", stringify!(storage_pool_event_lifecycle_msg));
        let res: Option<binding::RemoteStoragePoolEventLifecycleMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn storage_pool_event_refresh_msg(
        &mut self,
    ) -> Result<binding::RemoteStoragePoolEventRefreshMsg, Error> {
        trace!("{}", stringify!(storage_pool_event_refresh_msg));
        let res: Option<binding::RemoteStoragePoolEventRefreshMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn node_device_event_lifecycle_msg(
        &mut self,
    ) -> Result<binding::RemoteNodeDeviceEventLifecycleMsg, Error> {
        trace!("{}", stringify!(node_device_event_lifecycle_msg));
        let res: Option<binding::RemoteNodeDeviceEventLifecycleMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn node_device_event_update_msg(
        &mut self,
    ) -> Result<binding::RemoteNodeDeviceEventUpdateMsg, Error> {
        trace!("{}", stringify!(node_device_event_update_msg));
        let res: Option<binding::RemoteNodeDeviceEventUpdateMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_agent_lifecycle_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackAgentLifecycleMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_agent_lifecycle_msg));
        let res: Option<binding::RemoteDomainEventCallbackAgentLifecycleMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_migration_iteration_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackMigrationIterationMsg, Error> {
        trace!(
            "{}",
            stringify!(domain_event_callback_migration_iteration_msg)
        );
        let res: Option<binding::RemoteDomainEventCallbackMigrationIterationMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_job_completed_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackJobCompletedMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_job_completed_msg));
        let res: Option<binding::RemoteDomainEventCallbackJobCompletedMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_device_removal_failed_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackDeviceRemovalFailedMsg, Error> {
        trace!(
            "{}",
            stringify!(domain_event_callback_device_removal_failed_msg)
        );
        let res: Option<binding::RemoteDomainEventCallbackDeviceRemovalFailedMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_metadata_change_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventCallbackMetadataChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_callback_metadata_change_msg));
        let res: Option<binding::RemoteDomainEventCallbackMetadataChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_memory_failure_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventMemoryFailureMsg, Error> {
        trace!("{}", stringify!(domain_event_memory_failure_msg));
        let res: Option<binding::RemoteDomainEventMemoryFailureMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn secret_event_lifecycle_msg(
        &mut self,
    ) -> Result<binding::RemoteSecretEventLifecycleMsg, Error> {
        trace!("{}", stringify!(secret_event_lifecycle_msg));
        let res: Option<binding::RemoteSecretEventLifecycleMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn secret_event_value_changed_msg(
        &mut self,
    ) -> Result<binding::RemoteSecretEventValueChangedMsg, Error> {
        trace!("{}", stringify!(secret_event_value_changed_msg));
        let res: Option<binding::RemoteSecretEventValueChangedMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_memory_device_size_change_msg(
        &mut self,
    ) -> Result<binding::RemoteDomainEventMemoryDeviceSizeChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_memory_device_size_change_msg));
        let res: Option<binding::RemoteDomainEventMemoryDeviceSizeChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
}
fn call<S, D>(
    client: &mut (impl Libvirt + ?Sized),
    procedure: binding::RemoteProcedure,
    args: Option<S>,
) -> Result<Option<D>, Error>
where
    S: Serialize,
    D: DeserializeOwned,
{
    client.serial_add(1);
    let mut req_len: u32 = 4;
    let req_header = protocol::Virnetmessageheader {
        prog: binding::REMOTE_PROGRAM,
        vers: binding::REMOTE_PROTOCOL_VERSION,
        proc: procedure as i32,
        r#type: protocol::Virnetmessagetype::VirNetCall,
        serial: client.serial(),
        status: protocol::Virnetmessagestatus::VirNetOk,
    };
    let req_header_bytes = serde_xdr::to_bytes(&req_header).map_err(Error::SerializeError)?;
    req_len += req_header_bytes.len() as u32;
    let mut args_bytes = None;
    if let Some(args) = &args {
        let body = serde_xdr::to_bytes(args).map_err(Error::SerializeError)?;
        req_len += body.len() as u32;
        args_bytes = Some(body);
    }
    client
        .inner()
        .write_all(&req_len.to_be_bytes())
        .map_err(Error::SendError)?;
    client
        .inner()
        .write_all(&req_header_bytes)
        .map_err(Error::SendError)?;
    if let Some(args_bytes) = &args_bytes {
        client
            .inner()
            .write_all(args_bytes)
            .map_err(Error::SendError)?;
    }
    let mut res_len_bytes = [0; 4];
    client
        .inner()
        .read_exact(&mut res_len_bytes)
        .map_err(Error::ReceiveError)?;
    let res_len = u32::from_be_bytes(res_len_bytes) as usize;
    let mut res_header_bytes = [0; 24];
    client
        .inner()
        .read_exact(&mut res_header_bytes)
        .map_err(Error::ReceiveError)?;
    let res_header = serde_xdr::from_bytes::<protocol::Virnetmessageheader>(&res_header_bytes)
        .map_err(Error::DeserializeError)?;
    if res_len == (4 + res_header_bytes.len()) {
        return Ok(None);
    }
    let mut res_body_bytes = vec![0u8; res_len - 4 - res_header_bytes.len()];
    client
        .inner()
        .read_exact(&mut res_body_bytes)
        .map_err(Error::ReceiveError)?;
    if res_header.status == protocol::Virnetmessagestatus::VirNetError {
        let res = serde_xdr::from_bytes::<protocol::Virnetmessageerror>(&res_body_bytes)
            .map_err(Error::DeserializeError)?;
        Err(Error::ProtocolError(res))
    } else {
        let res = serde_xdr::from_bytes::<D>(&res_body_bytes).map_err(Error::DeserializeError)?;
        Ok(Some(res))
    }
}
fn msg<D>(client: &mut (impl Libvirt + ?Sized)) -> Result<Option<D>, Error>
where
    D: DeserializeOwned,
{
    let mut res_len_bytes = [0; 4];
    client
        .inner()
        .read_exact(&mut res_len_bytes)
        .map_err(Error::ReceiveError)?;
    let res_len = u32::from_be_bytes(res_len_bytes) as usize;
    let mut res_header_bytes = [0; 24];
    client
        .inner()
        .read_exact(&mut res_header_bytes)
        .map_err(Error::ReceiveError)?;
    let res_header = serde_xdr::from_bytes::<protocol::Virnetmessageheader>(&res_header_bytes)
        .map_err(Error::DeserializeError)?;
    if res_len == (4 + res_header_bytes.len()) {
        return Ok(None);
    }
    let mut res_body_bytes = vec![0u8; res_len - 4 - res_header_bytes.len()];
    client
        .inner()
        .read_exact(&mut res_body_bytes)
        .map_err(Error::ReceiveError)?;
    if res_header.status == protocol::Virnetmessagestatus::VirNetError {
        let res = serde_xdr::from_bytes::<protocol::Virnetmessageerror>(&res_body_bytes)
            .map_err(Error::DeserializeError)?;
        Err(Error::ProtocolError(res))
    } else {
        let res = serde_xdr::from_bytes::<D>(&res_body_bytes).map_err(Error::DeserializeError)?;
        Ok(Some(res))
    }
}
