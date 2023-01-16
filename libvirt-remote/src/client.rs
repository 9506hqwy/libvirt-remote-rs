use crate::binding::*;
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
pub enum VirNetRequest<S>
where
    S: Serialize,
{
    Data(S),
    Stream(VirNetStream),
}
pub enum VirNetResponse<D>
where
    D: DeserializeOwned,
{
    Data(D),
    Stream(VirNetStream),
}
pub enum VirNetStream {
    Hole(protocol::VirNetStreamHole),
    Raw(Vec<u8>),
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
    fn download(&mut self) -> Result<Option<VirNetStream>, Error> {
        download(self)
    }
    fn connect_open(&mut self, name: Option<String>, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_open));
        let req: Option<RemoteConnectOpenArgs> = Some(RemoteConnectOpenArgs { name, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcConnectOpen, req)?;
        Ok(())
    }
    fn connect_close(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_close));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcConnectClose, req)?;
        Ok(())
    }
    fn connect_get_type(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_type));
        let req: Option<()> = None;
        let res: Option<RemoteConnectGetTypeRet> =
            call(self, RemoteProcedure::RemoteProcConnectGetType, req)?;
        let res = res.unwrap();
        let RemoteConnectGetTypeRet { r#type } = res;
        Ok(r#type)
    }
    fn connect_get_version(&mut self) -> Result<u64, Error> {
        trace!("{}", stringify!(connect_get_version));
        let req: Option<()> = None;
        let res: Option<RemoteConnectGetVersionRet> =
            call(self, RemoteProcedure::RemoteProcConnectGetVersion, req)?;
        let res = res.unwrap();
        let RemoteConnectGetVersionRet { hv_ver } = res;
        Ok(hv_ver)
    }
    fn connect_get_max_vcpus(&mut self, r#type: Option<String>) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_get_max_vcpus));
        let req: Option<RemoteConnectGetMaxVcpusArgs> =
            Some(RemoteConnectGetMaxVcpusArgs { r#type });
        let res: Option<RemoteConnectGetMaxVcpusRet> =
            call(self, RemoteProcedure::RemoteProcConnectGetMaxVcpus, req)?;
        let res = res.unwrap();
        let RemoteConnectGetMaxVcpusRet { max_vcpus } = res;
        Ok(max_vcpus)
    }
    fn node_get_info(&mut self) -> Result<RemoteNodeGetInfoRet, Error> {
        trace!("{}", stringify!(node_get_info));
        let req: Option<()> = None;
        let res: Option<RemoteNodeGetInfoRet> =
            call(self, RemoteProcedure::RemoteProcNodeGetInfo, req)?;
        Ok(res.unwrap())
    }
    fn connect_get_capabilities(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_capabilities));
        let req: Option<()> = None;
        let res: Option<RemoteConnectGetCapabilitiesRet> =
            call(self, RemoteProcedure::RemoteProcConnectGetCapabilities, req)?;
        let res = res.unwrap();
        let RemoteConnectGetCapabilitiesRet { capabilities } = res;
        Ok(capabilities)
    }
    fn domain_attach_device(&mut self, dom: RemoteNonnullDomain, xml: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_attach_device));
        let req: Option<RemoteDomainAttachDeviceArgs> =
            Some(RemoteDomainAttachDeviceArgs { dom, xml });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainAttachDevice, req)?;
        Ok(())
    }
    fn domain_create(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_create));
        let req: Option<RemoteDomainCreateArgs> = Some(RemoteDomainCreateArgs { dom });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainCreate, req)?;
        Ok(())
    }
    fn domain_create_xml(
        &mut self,
        xml_desc: String,
        flags: u32,
    ) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_create_xml));
        let req: Option<RemoteDomainCreateXmlArgs> =
            Some(RemoteDomainCreateXmlArgs { xml_desc, flags });
        let res: Option<RemoteDomainCreateXmlRet> =
            call(self, RemoteProcedure::RemoteProcDomainCreateXml, req)?;
        let res = res.unwrap();
        let RemoteDomainCreateXmlRet { dom } = res;
        Ok(dom)
    }
    fn domain_define_xml(&mut self, xml: String) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_define_xml));
        let req: Option<RemoteDomainDefineXmlArgs> = Some(RemoteDomainDefineXmlArgs { xml });
        let res: Option<RemoteDomainDefineXmlRet> =
            call(self, RemoteProcedure::RemoteProcDomainDefineXml, req)?;
        let res = res.unwrap();
        let RemoteDomainDefineXmlRet { dom } = res;
        Ok(dom)
    }
    fn domain_destroy(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_destroy));
        let req: Option<RemoteDomainDestroyArgs> = Some(RemoteDomainDestroyArgs { dom });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainDestroy, req)?;
        Ok(())
    }
    fn domain_detach_device(&mut self, dom: RemoteNonnullDomain, xml: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_detach_device));
        let req: Option<RemoteDomainDetachDeviceArgs> =
            Some(RemoteDomainDetachDeviceArgs { dom, xml });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainDetachDevice, req)?;
        Ok(())
    }
    fn domain_get_xml_desc(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(domain_get_xml_desc));
        let req: Option<RemoteDomainGetXmlDescArgs> =
            Some(RemoteDomainGetXmlDescArgs { dom, flags });
        let res: Option<RemoteDomainGetXmlDescRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetXmlDesc, req)?;
        let res = res.unwrap();
        let RemoteDomainGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_get_autostart(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_get_autostart));
        let req: Option<RemoteDomainGetAutostartArgs> = Some(RemoteDomainGetAutostartArgs { dom });
        let res: Option<RemoteDomainGetAutostartRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetAutostart, req)?;
        let res = res.unwrap();
        let RemoteDomainGetAutostartRet { autostart } = res;
        Ok(autostart)
    }
    fn domain_get_info(
        &mut self,
        dom: RemoteNonnullDomain,
    ) -> Result<(u8, u64, u64, u16, u64), Error> {
        trace!("{}", stringify!(domain_get_info));
        let req: Option<RemoteDomainGetInfoArgs> = Some(RemoteDomainGetInfoArgs { dom });
        let res: Option<RemoteDomainGetInfoRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetInfo, req)?;
        let res = res.unwrap();
        let RemoteDomainGetInfoRet {
            state,
            max_mem,
            memory,
            nr_virt_cpu,
            cpu_time,
        } = res;
        Ok((state, max_mem, memory, nr_virt_cpu, cpu_time))
    }
    fn domain_get_max_memory(&mut self, dom: RemoteNonnullDomain) -> Result<u64, Error> {
        trace!("{}", stringify!(domain_get_max_memory));
        let req: Option<RemoteDomainGetMaxMemoryArgs> = Some(RemoteDomainGetMaxMemoryArgs { dom });
        let res: Option<RemoteDomainGetMaxMemoryRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetMaxMemory, req)?;
        let res = res.unwrap();
        let RemoteDomainGetMaxMemoryRet { memory } = res;
        Ok(memory)
    }
    fn domain_get_max_vcpus(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_get_max_vcpus));
        let req: Option<RemoteDomainGetMaxVcpusArgs> = Some(RemoteDomainGetMaxVcpusArgs { dom });
        let res: Option<RemoteDomainGetMaxVcpusRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetMaxVcpus, req)?;
        let res = res.unwrap();
        let RemoteDomainGetMaxVcpusRet { num } = res;
        Ok(num)
    }
    fn domain_get_os_type(&mut self, dom: RemoteNonnullDomain) -> Result<String, Error> {
        trace!("{}", stringify!(domain_get_os_type));
        let req: Option<RemoteDomainGetOsTypeArgs> = Some(RemoteDomainGetOsTypeArgs { dom });
        let res: Option<RemoteDomainGetOsTypeRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetOsType, req)?;
        let res = res.unwrap();
        let RemoteDomainGetOsTypeRet { r#type } = res;
        Ok(r#type)
    }
    fn domain_get_vcpus(
        &mut self,
        dom: RemoteNonnullDomain,
        maxinfo: i32,
        maplen: i32,
    ) -> Result<(Vec<RemoteVcpuInfo>, Vec<u8>), Error> {
        trace!("{}", stringify!(domain_get_vcpus));
        let req: Option<RemoteDomainGetVcpusArgs> = Some(RemoteDomainGetVcpusArgs {
            dom,
            maxinfo,
            maplen,
        });
        let res: Option<RemoteDomainGetVcpusRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetVcpus, req)?;
        let res = res.unwrap();
        let RemoteDomainGetVcpusRet { info, cpumaps } = res;
        Ok((info, cpumaps))
    }
    fn connect_list_defined_domains(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_defined_domains));
        let req: Option<RemoteConnectListDefinedDomainsArgs> =
            Some(RemoteConnectListDefinedDomainsArgs { maxnames });
        let res: Option<RemoteConnectListDefinedDomainsRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectListDefinedDomains,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListDefinedDomainsRet { names } = res;
        Ok(names)
    }
    fn domain_lookup_by_id(&mut self, id: i32) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_lookup_by_id));
        let req: Option<RemoteDomainLookupByIdArgs> = Some(RemoteDomainLookupByIdArgs { id });
        let res: Option<RemoteDomainLookupByIdRet> =
            call(self, RemoteProcedure::RemoteProcDomainLookupById, req)?;
        let res = res.unwrap();
        let RemoteDomainLookupByIdRet { dom } = res;
        Ok(dom)
    }
    fn domain_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_lookup_by_name));
        let req: Option<RemoteDomainLookupByNameArgs> = Some(RemoteDomainLookupByNameArgs { name });
        let res: Option<RemoteDomainLookupByNameRet> =
            call(self, RemoteProcedure::RemoteProcDomainLookupByName, req)?;
        let res = res.unwrap();
        let RemoteDomainLookupByNameRet { dom } = res;
        Ok(dom)
    }
    fn domain_lookup_by_uuid(
        &mut self,
        uuid: [u8; VIR_UUID_BUFLEN as usize],
    ) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_lookup_by_uuid));
        let req: Option<RemoteDomainLookupByUuidArgs> = Some(RemoteDomainLookupByUuidArgs { uuid });
        let res: Option<RemoteDomainLookupByUuidRet> =
            call(self, RemoteProcedure::RemoteProcDomainLookupByUuid, req)?;
        let res = res.unwrap();
        let RemoteDomainLookupByUuidRet { dom } = res;
        Ok(dom)
    }
    fn connect_num_of_defined_domains(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_defined_domains));
        let req: Option<()> = None;
        let res: Option<RemoteConnectNumOfDefinedDomainsRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectNumOfDefinedDomains,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNumOfDefinedDomainsRet { num } = res;
        Ok(num)
    }
    fn domain_pin_vcpu(
        &mut self,
        dom: RemoteNonnullDomain,
        vcpu: u32,
        cpumap: Vec<u8>,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pin_vcpu));
        let req: Option<RemoteDomainPinVcpuArgs> =
            Some(RemoteDomainPinVcpuArgs { dom, vcpu, cpumap });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainPinVcpu, req)?;
        Ok(())
    }
    fn domain_reboot(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_reboot));
        let req: Option<RemoteDomainRebootArgs> = Some(RemoteDomainRebootArgs { dom, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainReboot, req)?;
        Ok(())
    }
    fn domain_resume(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_resume));
        let req: Option<RemoteDomainResumeArgs> = Some(RemoteDomainResumeArgs { dom });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainResume, req)?;
        Ok(())
    }
    fn domain_set_autostart(
        &mut self,
        dom: RemoteNonnullDomain,
        autostart: i32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_autostart));
        let req: Option<RemoteDomainSetAutostartArgs> =
            Some(RemoteDomainSetAutostartArgs { dom, autostart });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetAutostart, req)?;
        Ok(())
    }
    fn domain_set_max_memory(
        &mut self,
        dom: RemoteNonnullDomain,
        memory: u64,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_max_memory));
        let req: Option<RemoteDomainSetMaxMemoryArgs> =
            Some(RemoteDomainSetMaxMemoryArgs { dom, memory });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetMaxMemory, req)?;
        Ok(())
    }
    fn domain_set_memory(&mut self, dom: RemoteNonnullDomain, memory: u64) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_memory));
        let req: Option<RemoteDomainSetMemoryArgs> =
            Some(RemoteDomainSetMemoryArgs { dom, memory });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetMemory, req)?;
        Ok(())
    }
    fn domain_set_vcpus(&mut self, dom: RemoteNonnullDomain, nvcpus: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_vcpus));
        let req: Option<RemoteDomainSetVcpusArgs> = Some(RemoteDomainSetVcpusArgs { dom, nvcpus });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetVcpus, req)?;
        Ok(())
    }
    fn domain_shutdown(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_shutdown));
        let req: Option<RemoteDomainShutdownArgs> = Some(RemoteDomainShutdownArgs { dom });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainShutdown, req)?;
        Ok(())
    }
    fn domain_suspend(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_suspend));
        let req: Option<RemoteDomainSuspendArgs> = Some(RemoteDomainSuspendArgs { dom });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSuspend, req)?;
        Ok(())
    }
    fn domain_undefine(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_undefine));
        let req: Option<RemoteDomainUndefineArgs> = Some(RemoteDomainUndefineArgs { dom });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainUndefine, req)?;
        Ok(())
    }
    fn connect_list_defined_networks(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_defined_networks));
        let req: Option<RemoteConnectListDefinedNetworksArgs> =
            Some(RemoteConnectListDefinedNetworksArgs { maxnames });
        let res: Option<RemoteConnectListDefinedNetworksRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectListDefinedNetworks,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListDefinedNetworksRet { names } = res;
        Ok(names)
    }
    fn connect_list_domains(&mut self, maxids: i32) -> Result<Vec<i32>, Error> {
        trace!("{}", stringify!(connect_list_domains));
        let req: Option<RemoteConnectListDomainsArgs> =
            Some(RemoteConnectListDomainsArgs { maxids });
        let res: Option<RemoteConnectListDomainsRet> =
            call(self, RemoteProcedure::RemoteProcConnectListDomains, req)?;
        let res = res.unwrap();
        let RemoteConnectListDomainsRet { ids } = res;
        Ok(ids)
    }
    fn connect_list_networks(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_networks));
        let req: Option<RemoteConnectListNetworksArgs> =
            Some(RemoteConnectListNetworksArgs { maxnames });
        let res: Option<RemoteConnectListNetworksRet> =
            call(self, RemoteProcedure::RemoteProcConnectListNetworks, req)?;
        let res = res.unwrap();
        let RemoteConnectListNetworksRet { names } = res;
        Ok(names)
    }
    fn network_create(&mut self, net: RemoteNonnullNetwork) -> Result<(), Error> {
        trace!("{}", stringify!(network_create));
        let req: Option<RemoteNetworkCreateArgs> = Some(RemoteNetworkCreateArgs { net });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNetworkCreate, req)?;
        Ok(())
    }
    fn network_create_xml(&mut self, xml: String) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_create_xml));
        let req: Option<RemoteNetworkCreateXmlArgs> = Some(RemoteNetworkCreateXmlArgs { xml });
        let res: Option<RemoteNetworkCreateXmlRet> =
            call(self, RemoteProcedure::RemoteProcNetworkCreateXml, req)?;
        let res = res.unwrap();
        let RemoteNetworkCreateXmlRet { net } = res;
        Ok(net)
    }
    fn network_define_xml(&mut self, xml: String) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_define_xml));
        let req: Option<RemoteNetworkDefineXmlArgs> = Some(RemoteNetworkDefineXmlArgs { xml });
        let res: Option<RemoteNetworkDefineXmlRet> =
            call(self, RemoteProcedure::RemoteProcNetworkDefineXml, req)?;
        let res = res.unwrap();
        let RemoteNetworkDefineXmlRet { net } = res;
        Ok(net)
    }
    fn network_destroy(&mut self, net: RemoteNonnullNetwork) -> Result<(), Error> {
        trace!("{}", stringify!(network_destroy));
        let req: Option<RemoteNetworkDestroyArgs> = Some(RemoteNetworkDestroyArgs { net });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNetworkDestroy, req)?;
        Ok(())
    }
    fn network_get_xml_desc(
        &mut self,
        net: RemoteNonnullNetwork,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(network_get_xml_desc));
        let req: Option<RemoteNetworkGetXmlDescArgs> =
            Some(RemoteNetworkGetXmlDescArgs { net, flags });
        let res: Option<RemoteNetworkGetXmlDescRet> =
            call(self, RemoteProcedure::RemoteProcNetworkGetXmlDesc, req)?;
        let res = res.unwrap();
        let RemoteNetworkGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn network_get_autostart(&mut self, net: RemoteNonnullNetwork) -> Result<i32, Error> {
        trace!("{}", stringify!(network_get_autostart));
        let req: Option<RemoteNetworkGetAutostartArgs> =
            Some(RemoteNetworkGetAutostartArgs { net });
        let res: Option<RemoteNetworkGetAutostartRet> =
            call(self, RemoteProcedure::RemoteProcNetworkGetAutostart, req)?;
        let res = res.unwrap();
        let RemoteNetworkGetAutostartRet { autostart } = res;
        Ok(autostart)
    }
    fn network_get_bridge_name(&mut self, net: RemoteNonnullNetwork) -> Result<String, Error> {
        trace!("{}", stringify!(network_get_bridge_name));
        let req: Option<RemoteNetworkGetBridgeNameArgs> =
            Some(RemoteNetworkGetBridgeNameArgs { net });
        let res: Option<RemoteNetworkGetBridgeNameRet> =
            call(self, RemoteProcedure::RemoteProcNetworkGetBridgeName, req)?;
        let res = res.unwrap();
        let RemoteNetworkGetBridgeNameRet { name } = res;
        Ok(name)
    }
    fn network_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_lookup_by_name));
        let req: Option<RemoteNetworkLookupByNameArgs> =
            Some(RemoteNetworkLookupByNameArgs { name });
        let res: Option<RemoteNetworkLookupByNameRet> =
            call(self, RemoteProcedure::RemoteProcNetworkLookupByName, req)?;
        let res = res.unwrap();
        let RemoteNetworkLookupByNameRet { net } = res;
        Ok(net)
    }
    fn network_lookup_by_uuid(
        &mut self,
        uuid: [u8; VIR_UUID_BUFLEN as usize],
    ) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_lookup_by_uuid));
        let req: Option<RemoteNetworkLookupByUuidArgs> =
            Some(RemoteNetworkLookupByUuidArgs { uuid });
        let res: Option<RemoteNetworkLookupByUuidRet> =
            call(self, RemoteProcedure::RemoteProcNetworkLookupByUuid, req)?;
        let res = res.unwrap();
        let RemoteNetworkLookupByUuidRet { net } = res;
        Ok(net)
    }
    fn network_set_autostart(
        &mut self,
        net: RemoteNonnullNetwork,
        autostart: i32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(network_set_autostart));
        let req: Option<RemoteNetworkSetAutostartArgs> =
            Some(RemoteNetworkSetAutostartArgs { net, autostart });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNetworkSetAutostart, req)?;
        Ok(())
    }
    fn network_undefine(&mut self, net: RemoteNonnullNetwork) -> Result<(), Error> {
        trace!("{}", stringify!(network_undefine));
        let req: Option<RemoteNetworkUndefineArgs> = Some(RemoteNetworkUndefineArgs { net });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNetworkUndefine, req)?;
        Ok(())
    }
    fn connect_num_of_defined_networks(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_defined_networks));
        let req: Option<()> = None;
        let res: Option<RemoteConnectNumOfDefinedNetworksRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectNumOfDefinedNetworks,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNumOfDefinedNetworksRet { num } = res;
        Ok(num)
    }
    fn connect_num_of_domains(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_domains));
        let req: Option<()> = None;
        let res: Option<RemoteConnectNumOfDomainsRet> =
            call(self, RemoteProcedure::RemoteProcConnectNumOfDomains, req)?;
        let res = res.unwrap();
        let RemoteConnectNumOfDomainsRet { num } = res;
        Ok(num)
    }
    fn connect_num_of_networks(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_networks));
        let req: Option<()> = None;
        let res: Option<RemoteConnectNumOfNetworksRet> =
            call(self, RemoteProcedure::RemoteProcConnectNumOfNetworks, req)?;
        let res = res.unwrap();
        let RemoteConnectNumOfNetworksRet { num } = res;
        Ok(num)
    }
    fn domain_core_dump(
        &mut self,
        dom: RemoteNonnullDomain,
        to: String,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_core_dump));
        let req: Option<RemoteDomainCoreDumpArgs> =
            Some(RemoteDomainCoreDumpArgs { dom, to, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainCoreDump, req)?;
        Ok(())
    }
    fn domain_restore(&mut self, from: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_restore));
        let req: Option<RemoteDomainRestoreArgs> = Some(RemoteDomainRestoreArgs { from });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainRestore, req)?;
        Ok(())
    }
    fn domain_save(&mut self, dom: RemoteNonnullDomain, to: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_save));
        let req: Option<RemoteDomainSaveArgs> = Some(RemoteDomainSaveArgs { dom, to });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSave, req)?;
        Ok(())
    }
    fn domain_get_scheduler_type(
        &mut self,
        dom: RemoteNonnullDomain,
    ) -> Result<(String, i32), Error> {
        trace!("{}", stringify!(domain_get_scheduler_type));
        let req: Option<RemoteDomainGetSchedulerTypeArgs> =
            Some(RemoteDomainGetSchedulerTypeArgs { dom });
        let res: Option<RemoteDomainGetSchedulerTypeRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetSchedulerType, req)?;
        let res = res.unwrap();
        let RemoteDomainGetSchedulerTypeRet { r#type, nparams } = res;
        Ok((r#type, nparams))
    }
    fn domain_get_scheduler_parameters(
        &mut self,
        dom: RemoteNonnullDomain,
        nparams: i32,
    ) -> Result<Vec<RemoteTypedParam>, Error> {
        trace!("{}", stringify!(domain_get_scheduler_parameters));
        let req: Option<RemoteDomainGetSchedulerParametersArgs> =
            Some(RemoteDomainGetSchedulerParametersArgs { dom, nparams });
        let res: Option<RemoteDomainGetSchedulerParametersRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainGetSchedulerParameters,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetSchedulerParametersRet { params } = res;
        Ok(params)
    }
    fn domain_set_scheduler_parameters(
        &mut self,
        dom: RemoteNonnullDomain,
        params: Vec<RemoteTypedParam>,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_scheduler_parameters));
        let req: Option<RemoteDomainSetSchedulerParametersArgs> =
            Some(RemoteDomainSetSchedulerParametersArgs { dom, params });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetSchedulerParameters,
            req,
        )?;
        Ok(())
    }
    fn connect_get_hostname(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_hostname));
        let req: Option<()> = None;
        let res: Option<RemoteConnectGetHostnameRet> =
            call(self, RemoteProcedure::RemoteProcConnectGetHostname, req)?;
        let res = res.unwrap();
        let RemoteConnectGetHostnameRet { hostname } = res;
        Ok(hostname)
    }
    fn connect_supports_feature(&mut self, feature: i32) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_supports_feature));
        let req: Option<RemoteConnectSupportsFeatureArgs> =
            Some(RemoteConnectSupportsFeatureArgs { feature });
        let res: Option<RemoteConnectSupportsFeatureRet> =
            call(self, RemoteProcedure::RemoteProcConnectSupportsFeature, req)?;
        let res = res.unwrap();
        let RemoteConnectSupportsFeatureRet { supported } = res;
        Ok(supported)
    }
    fn domain_migrate_prepare(
        &mut self,
        uri_in: Option<String>,
        flags: u64,
        dname: Option<String>,
        resource: u64,
    ) -> Result<(Vec<u8>, Option<String>), Error> {
        trace!("{}", stringify!(domain_migrate_prepare));
        let req: Option<RemoteDomainMigratePrepareArgs> = Some(RemoteDomainMigratePrepareArgs {
            uri_in,
            flags,
            dname,
            resource,
        });
        let res: Option<RemoteDomainMigratePrepareRet> =
            call(self, RemoteProcedure::RemoteProcDomainMigratePrepare, req)?;
        let res = res.unwrap();
        let RemoteDomainMigratePrepareRet { cookie, uri_out } = res;
        Ok((cookie, uri_out))
    }
    fn domain_migrate_perform(
        &mut self,
        dom: RemoteNonnullDomain,
        cookie: Vec<u8>,
        uri: String,
        flags: u64,
        dname: Option<String>,
        resource: u64,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_perform));
        let req: Option<RemoteDomainMigratePerformArgs> = Some(RemoteDomainMigratePerformArgs {
            dom,
            cookie,
            uri,
            flags,
            dname,
            resource,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainMigratePerform, req)?;
        Ok(())
    }
    fn domain_migrate_finish(
        &mut self,
        dname: String,
        cookie: Vec<u8>,
        uri: String,
        flags: u64,
    ) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_migrate_finish));
        let req: Option<RemoteDomainMigrateFinishArgs> = Some(RemoteDomainMigrateFinishArgs {
            dname,
            cookie,
            uri,
            flags,
        });
        let res: Option<RemoteDomainMigrateFinishRet> =
            call(self, RemoteProcedure::RemoteProcDomainMigrateFinish, req)?;
        let res = res.unwrap();
        let RemoteDomainMigrateFinishRet { ddom } = res;
        Ok(ddom)
    }
    fn domain_block_stats(
        &mut self,
        dom: RemoteNonnullDomain,
        path: String,
    ) -> Result<(i64, i64, i64, i64, i64), Error> {
        trace!("{}", stringify!(domain_block_stats));
        let req: Option<RemoteDomainBlockStatsArgs> =
            Some(RemoteDomainBlockStatsArgs { dom, path });
        let res: Option<RemoteDomainBlockStatsRet> =
            call(self, RemoteProcedure::RemoteProcDomainBlockStats, req)?;
        let res = res.unwrap();
        let RemoteDomainBlockStatsRet {
            rd_req,
            rd_bytes,
            wr_req,
            wr_bytes,
            errs,
        } = res;
        Ok((rd_req, rd_bytes, wr_req, wr_bytes, errs))
    }
    fn domain_interface_stats(
        &mut self,
        dom: RemoteNonnullDomain,
        device: String,
    ) -> Result<RemoteDomainInterfaceStatsRet, Error> {
        trace!("{}", stringify!(domain_interface_stats));
        let req: Option<RemoteDomainInterfaceStatsArgs> =
            Some(RemoteDomainInterfaceStatsArgs { dom, device });
        let res: Option<RemoteDomainInterfaceStatsRet> =
            call(self, RemoteProcedure::RemoteProcDomainInterfaceStats, req)?;
        Ok(res.unwrap())
    }
    fn auth_list(&mut self) -> Result<Vec<RemoteAuthType>, Error> {
        trace!("{}", stringify!(auth_list));
        let req: Option<()> = None;
        let res: Option<RemoteAuthListRet> = call(self, RemoteProcedure::RemoteProcAuthList, req)?;
        let res = res.unwrap();
        let RemoteAuthListRet { types } = res;
        Ok(types)
    }
    fn auth_sasl_init(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(auth_sasl_init));
        let req: Option<()> = None;
        let res: Option<RemoteAuthSaslInitRet> =
            call(self, RemoteProcedure::RemoteProcAuthSaslInit, req)?;
        let res = res.unwrap();
        let RemoteAuthSaslInitRet { mechlist } = res;
        Ok(mechlist)
    }
    fn auth_sasl_start(
        &mut self,
        mech: String,
        nil: i32,
        data: Vec<i8>,
    ) -> Result<(i32, i32, Vec<i8>), Error> {
        trace!("{}", stringify!(auth_sasl_start));
        let req: Option<RemoteAuthSaslStartArgs> =
            Some(RemoteAuthSaslStartArgs { mech, nil, data });
        let res: Option<RemoteAuthSaslStartRet> =
            call(self, RemoteProcedure::RemoteProcAuthSaslStart, req)?;
        let res = res.unwrap();
        let RemoteAuthSaslStartRet {
            complete,
            nil,
            data,
        } = res;
        Ok((complete, nil, data))
    }
    fn auth_sasl_step(&mut self, nil: i32, data: Vec<i8>) -> Result<(i32, i32, Vec<i8>), Error> {
        trace!("{}", stringify!(auth_sasl_step));
        let req: Option<RemoteAuthSaslStepArgs> = Some(RemoteAuthSaslStepArgs { nil, data });
        let res: Option<RemoteAuthSaslStepRet> =
            call(self, RemoteProcedure::RemoteProcAuthSaslStep, req)?;
        let res = res.unwrap();
        let RemoteAuthSaslStepRet {
            complete,
            nil,
            data,
        } = res;
        Ok((complete, nil, data))
    }
    fn auth_polkit(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(auth_polkit));
        let req: Option<()> = None;
        let res: Option<RemoteAuthPolkitRet> =
            call(self, RemoteProcedure::RemoteProcAuthPolkit, req)?;
        let res = res.unwrap();
        let RemoteAuthPolkitRet { complete } = res;
        Ok(complete)
    }
    fn connect_num_of_storage_pools(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_storage_pools));
        let req: Option<()> = None;
        let res: Option<RemoteConnectNumOfStoragePoolsRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectNumOfStoragePools,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNumOfStoragePoolsRet { num } = res;
        Ok(num)
    }
    fn connect_list_storage_pools(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_storage_pools));
        let req: Option<RemoteConnectListStoragePoolsArgs> =
            Some(RemoteConnectListStoragePoolsArgs { maxnames });
        let res: Option<RemoteConnectListStoragePoolsRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectListStoragePools,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListStoragePoolsRet { names } = res;
        Ok(names)
    }
    fn connect_num_of_defined_storage_pools(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_defined_storage_pools));
        let req: Option<()> = None;
        let res: Option<RemoteConnectNumOfDefinedStoragePoolsRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectNumOfDefinedStoragePools,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNumOfDefinedStoragePoolsRet { num } = res;
        Ok(num)
    }
    fn connect_list_defined_storage_pools(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_defined_storage_pools));
        let req: Option<RemoteConnectListDefinedStoragePoolsArgs> =
            Some(RemoteConnectListDefinedStoragePoolsArgs { maxnames });
        let res: Option<RemoteConnectListDefinedStoragePoolsRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectListDefinedStoragePools,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListDefinedStoragePoolsRet { names } = res;
        Ok(names)
    }
    fn connect_find_storage_pool_sources(
        &mut self,
        r#type: String,
        src_spec: Option<String>,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(connect_find_storage_pool_sources));
        let req: Option<RemoteConnectFindStoragePoolSourcesArgs> =
            Some(RemoteConnectFindStoragePoolSourcesArgs {
                r#type,
                src_spec,
                flags,
            });
        let res: Option<RemoteConnectFindStoragePoolSourcesRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectFindStoragePoolSources,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectFindStoragePoolSourcesRet { xml } = res;
        Ok(xml)
    }
    fn storage_pool_create_xml(
        &mut self,
        xml: String,
        flags: u32,
    ) -> Result<RemoteNonnullStoragePool, Error> {
        trace!("{}", stringify!(storage_pool_create_xml));
        let req: Option<RemoteStoragePoolCreateXmlArgs> =
            Some(RemoteStoragePoolCreateXmlArgs { xml, flags });
        let res: Option<RemoteStoragePoolCreateXmlRet> =
            call(self, RemoteProcedure::RemoteProcStoragePoolCreateXml, req)?;
        let res = res.unwrap();
        let RemoteStoragePoolCreateXmlRet { pool } = res;
        Ok(pool)
    }
    fn storage_pool_define_xml(
        &mut self,
        xml: String,
        flags: u32,
    ) -> Result<RemoteNonnullStoragePool, Error> {
        trace!("{}", stringify!(storage_pool_define_xml));
        let req: Option<RemoteStoragePoolDefineXmlArgs> =
            Some(RemoteStoragePoolDefineXmlArgs { xml, flags });
        let res: Option<RemoteStoragePoolDefineXmlRet> =
            call(self, RemoteProcedure::RemoteProcStoragePoolDefineXml, req)?;
        let res = res.unwrap();
        let RemoteStoragePoolDefineXmlRet { pool } = res;
        Ok(pool)
    }
    fn storage_pool_create(
        &mut self,
        pool: RemoteNonnullStoragePool,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_create));
        let req: Option<RemoteStoragePoolCreateArgs> =
            Some(RemoteStoragePoolCreateArgs { pool, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStoragePoolCreate, req)?;
        Ok(())
    }
    fn storage_pool_build(
        &mut self,
        pool: RemoteNonnullStoragePool,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_build));
        let req: Option<RemoteStoragePoolBuildArgs> =
            Some(RemoteStoragePoolBuildArgs { pool, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStoragePoolBuild, req)?;
        Ok(())
    }
    fn storage_pool_destroy(&mut self, pool: RemoteNonnullStoragePool) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_destroy));
        let req: Option<RemoteStoragePoolDestroyArgs> = Some(RemoteStoragePoolDestroyArgs { pool });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStoragePoolDestroy, req)?;
        Ok(())
    }
    fn storage_pool_delete(
        &mut self,
        pool: RemoteNonnullStoragePool,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_delete));
        let req: Option<RemoteStoragePoolDeleteArgs> =
            Some(RemoteStoragePoolDeleteArgs { pool, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStoragePoolDelete, req)?;
        Ok(())
    }
    fn storage_pool_undefine(&mut self, pool: RemoteNonnullStoragePool) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_undefine));
        let req: Option<RemoteStoragePoolUndefineArgs> =
            Some(RemoteStoragePoolUndefineArgs { pool });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStoragePoolUndefine, req)?;
        Ok(())
    }
    fn storage_pool_refresh(
        &mut self,
        pool: RemoteNonnullStoragePool,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_refresh));
        let req: Option<RemoteStoragePoolRefreshArgs> =
            Some(RemoteStoragePoolRefreshArgs { pool, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStoragePoolRefresh, req)?;
        Ok(())
    }
    fn storage_pool_lookup_by_name(
        &mut self,
        name: String,
    ) -> Result<RemoteNonnullStoragePool, Error> {
        trace!("{}", stringify!(storage_pool_lookup_by_name));
        let req: Option<RemoteStoragePoolLookupByNameArgs> =
            Some(RemoteStoragePoolLookupByNameArgs { name });
        let res: Option<RemoteStoragePoolLookupByNameRet> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolLookupByName,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStoragePoolLookupByNameRet { pool } = res;
        Ok(pool)
    }
    fn storage_pool_lookup_by_uuid(
        &mut self,
        uuid: [u8; VIR_UUID_BUFLEN as usize],
    ) -> Result<RemoteNonnullStoragePool, Error> {
        trace!("{}", stringify!(storage_pool_lookup_by_uuid));
        let req: Option<RemoteStoragePoolLookupByUuidArgs> =
            Some(RemoteStoragePoolLookupByUuidArgs { uuid });
        let res: Option<RemoteStoragePoolLookupByUuidRet> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolLookupByUuid,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStoragePoolLookupByUuidRet { pool } = res;
        Ok(pool)
    }
    fn storage_pool_lookup_by_volume(
        &mut self,
        vol: RemoteNonnullStorageVol,
    ) -> Result<RemoteNonnullStoragePool, Error> {
        trace!("{}", stringify!(storage_pool_lookup_by_volume));
        let req: Option<RemoteStoragePoolLookupByVolumeArgs> =
            Some(RemoteStoragePoolLookupByVolumeArgs { vol });
        let res: Option<RemoteStoragePoolLookupByVolumeRet> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolLookupByVolume,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStoragePoolLookupByVolumeRet { pool } = res;
        Ok(pool)
    }
    fn storage_pool_get_info(
        &mut self,
        pool: RemoteNonnullStoragePool,
    ) -> Result<(u8, u64, u64, u64), Error> {
        trace!("{}", stringify!(storage_pool_get_info));
        let req: Option<RemoteStoragePoolGetInfoArgs> = Some(RemoteStoragePoolGetInfoArgs { pool });
        let res: Option<RemoteStoragePoolGetInfoRet> =
            call(self, RemoteProcedure::RemoteProcStoragePoolGetInfo, req)?;
        let res = res.unwrap();
        let RemoteStoragePoolGetInfoRet {
            state,
            capacity,
            allocation,
            available,
        } = res;
        Ok((state, capacity, allocation, available))
    }
    fn storage_pool_get_xml_desc(
        &mut self,
        pool: RemoteNonnullStoragePool,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(storage_pool_get_xml_desc));
        let req: Option<RemoteStoragePoolGetXmlDescArgs> =
            Some(RemoteStoragePoolGetXmlDescArgs { pool, flags });
        let res: Option<RemoteStoragePoolGetXmlDescRet> =
            call(self, RemoteProcedure::RemoteProcStoragePoolGetXmlDesc, req)?;
        let res = res.unwrap();
        let RemoteStoragePoolGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn storage_pool_get_autostart(&mut self, pool: RemoteNonnullStoragePool) -> Result<i32, Error> {
        trace!("{}", stringify!(storage_pool_get_autostart));
        let req: Option<RemoteStoragePoolGetAutostartArgs> =
            Some(RemoteStoragePoolGetAutostartArgs { pool });
        let res: Option<RemoteStoragePoolGetAutostartRet> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolGetAutostart,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStoragePoolGetAutostartRet { autostart } = res;
        Ok(autostart)
    }
    fn storage_pool_set_autostart(
        &mut self,
        pool: RemoteNonnullStoragePool,
        autostart: i32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_set_autostart));
        let req: Option<RemoteStoragePoolSetAutostartArgs> =
            Some(RemoteStoragePoolSetAutostartArgs { pool, autostart });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolSetAutostart,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_num_of_volumes(
        &mut self,
        pool: RemoteNonnullStoragePool,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(storage_pool_num_of_volumes));
        let req: Option<RemoteStoragePoolNumOfVolumesArgs> =
            Some(RemoteStoragePoolNumOfVolumesArgs { pool });
        let res: Option<RemoteStoragePoolNumOfVolumesRet> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolNumOfVolumes,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStoragePoolNumOfVolumesRet { num } = res;
        Ok(num)
    }
    fn storage_pool_list_volumes(
        &mut self,
        pool: RemoteNonnullStoragePool,
        maxnames: i32,
    ) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(storage_pool_list_volumes));
        let req: Option<RemoteStoragePoolListVolumesArgs> =
            Some(RemoteStoragePoolListVolumesArgs { pool, maxnames });
        let res: Option<RemoteStoragePoolListVolumesRet> =
            call(self, RemoteProcedure::RemoteProcStoragePoolListVolumes, req)?;
        let res = res.unwrap();
        let RemoteStoragePoolListVolumesRet { names } = res;
        Ok(names)
    }
    fn storage_vol_create_xml(
        &mut self,
        pool: RemoteNonnullStoragePool,
        xml: String,
        flags: u32,
    ) -> Result<RemoteNonnullStorageVol, Error> {
        trace!("{}", stringify!(storage_vol_create_xml));
        let req: Option<RemoteStorageVolCreateXmlArgs> =
            Some(RemoteStorageVolCreateXmlArgs { pool, xml, flags });
        let res: Option<RemoteStorageVolCreateXmlRet> =
            call(self, RemoteProcedure::RemoteProcStorageVolCreateXml, req)?;
        let res = res.unwrap();
        let RemoteStorageVolCreateXmlRet { vol } = res;
        Ok(vol)
    }
    fn storage_vol_delete(
        &mut self,
        vol: RemoteNonnullStorageVol,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_delete));
        let req: Option<RemoteStorageVolDeleteArgs> =
            Some(RemoteStorageVolDeleteArgs { vol, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStorageVolDelete, req)?;
        Ok(())
    }
    fn storage_vol_lookup_by_name(
        &mut self,
        pool: RemoteNonnullStoragePool,
        name: String,
    ) -> Result<RemoteNonnullStorageVol, Error> {
        trace!("{}", stringify!(storage_vol_lookup_by_name));
        let req: Option<RemoteStorageVolLookupByNameArgs> =
            Some(RemoteStorageVolLookupByNameArgs { pool, name });
        let res: Option<RemoteStorageVolLookupByNameRet> =
            call(self, RemoteProcedure::RemoteProcStorageVolLookupByName, req)?;
        let res = res.unwrap();
        let RemoteStorageVolLookupByNameRet { vol } = res;
        Ok(vol)
    }
    fn storage_vol_lookup_by_key(&mut self, key: String) -> Result<RemoteNonnullStorageVol, Error> {
        trace!("{}", stringify!(storage_vol_lookup_by_key));
        let req: Option<RemoteStorageVolLookupByKeyArgs> =
            Some(RemoteStorageVolLookupByKeyArgs { key });
        let res: Option<RemoteStorageVolLookupByKeyRet> =
            call(self, RemoteProcedure::RemoteProcStorageVolLookupByKey, req)?;
        let res = res.unwrap();
        let RemoteStorageVolLookupByKeyRet { vol } = res;
        Ok(vol)
    }
    fn storage_vol_lookup_by_path(
        &mut self,
        path: String,
    ) -> Result<RemoteNonnullStorageVol, Error> {
        trace!("{}", stringify!(storage_vol_lookup_by_path));
        let req: Option<RemoteStorageVolLookupByPathArgs> =
            Some(RemoteStorageVolLookupByPathArgs { path });
        let res: Option<RemoteStorageVolLookupByPathRet> =
            call(self, RemoteProcedure::RemoteProcStorageVolLookupByPath, req)?;
        let res = res.unwrap();
        let RemoteStorageVolLookupByPathRet { vol } = res;
        Ok(vol)
    }
    fn storage_vol_get_info(
        &mut self,
        vol: RemoteNonnullStorageVol,
    ) -> Result<(i8, u64, u64), Error> {
        trace!("{}", stringify!(storage_vol_get_info));
        let req: Option<RemoteStorageVolGetInfoArgs> = Some(RemoteStorageVolGetInfoArgs { vol });
        let res: Option<RemoteStorageVolGetInfoRet> =
            call(self, RemoteProcedure::RemoteProcStorageVolGetInfo, req)?;
        let res = res.unwrap();
        let RemoteStorageVolGetInfoRet {
            r#type,
            capacity,
            allocation,
        } = res;
        Ok((r#type, capacity, allocation))
    }
    fn storage_vol_get_xml_desc(
        &mut self,
        vol: RemoteNonnullStorageVol,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(storage_vol_get_xml_desc));
        let req: Option<RemoteStorageVolGetXmlDescArgs> =
            Some(RemoteStorageVolGetXmlDescArgs { vol, flags });
        let res: Option<RemoteStorageVolGetXmlDescRet> =
            call(self, RemoteProcedure::RemoteProcStorageVolGetXmlDesc, req)?;
        let res = res.unwrap();
        let RemoteStorageVolGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn storage_vol_get_path(&mut self, vol: RemoteNonnullStorageVol) -> Result<String, Error> {
        trace!("{}", stringify!(storage_vol_get_path));
        let req: Option<RemoteStorageVolGetPathArgs> = Some(RemoteStorageVolGetPathArgs { vol });
        let res: Option<RemoteStorageVolGetPathRet> =
            call(self, RemoteProcedure::RemoteProcStorageVolGetPath, req)?;
        let res = res.unwrap();
        let RemoteStorageVolGetPathRet { name } = res;
        Ok(name)
    }
    fn node_get_cells_free_memory(
        &mut self,
        start_cell: i32,
        maxcells: i32,
    ) -> Result<Vec<u64>, Error> {
        trace!("{}", stringify!(node_get_cells_free_memory));
        let req: Option<RemoteNodeGetCellsFreeMemoryArgs> =
            Some(RemoteNodeGetCellsFreeMemoryArgs {
                start_cell,
                maxcells,
            });
        let res: Option<RemoteNodeGetCellsFreeMemoryRet> =
            call(self, RemoteProcedure::RemoteProcNodeGetCellsFreeMemory, req)?;
        let res = res.unwrap();
        let RemoteNodeGetCellsFreeMemoryRet { cells } = res;
        Ok(cells)
    }
    fn node_get_free_memory(&mut self) -> Result<u64, Error> {
        trace!("{}", stringify!(node_get_free_memory));
        let req: Option<()> = None;
        let res: Option<RemoteNodeGetFreeMemoryRet> =
            call(self, RemoteProcedure::RemoteProcNodeGetFreeMemory, req)?;
        let res = res.unwrap();
        let RemoteNodeGetFreeMemoryRet { free_mem } = res;
        Ok(free_mem)
    }
    fn domain_block_peek(
        &mut self,
        dom: RemoteNonnullDomain,
        path: String,
        offset: u64,
        size: u32,
        flags: u32,
    ) -> Result<Vec<u8>, Error> {
        trace!("{}", stringify!(domain_block_peek));
        let req: Option<RemoteDomainBlockPeekArgs> = Some(RemoteDomainBlockPeekArgs {
            dom,
            path,
            offset,
            size,
            flags,
        });
        let res: Option<RemoteDomainBlockPeekRet> =
            call(self, RemoteProcedure::RemoteProcDomainBlockPeek, req)?;
        let res = res.unwrap();
        let RemoteDomainBlockPeekRet { buffer } = res;
        Ok(buffer)
    }
    fn domain_memory_peek(
        &mut self,
        dom: RemoteNonnullDomain,
        offset: u64,
        size: u32,
        flags: u32,
    ) -> Result<Vec<u8>, Error> {
        trace!("{}", stringify!(domain_memory_peek));
        let req: Option<RemoteDomainMemoryPeekArgs> = Some(RemoteDomainMemoryPeekArgs {
            dom,
            offset,
            size,
            flags,
        });
        let res: Option<RemoteDomainMemoryPeekRet> =
            call(self, RemoteProcedure::RemoteProcDomainMemoryPeek, req)?;
        let res = res.unwrap();
        let RemoteDomainMemoryPeekRet { buffer } = res;
        Ok(buffer)
    }
    fn connect_domain_event_register(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_domain_event_register));
        let req: Option<()> = None;
        let res: Option<RemoteConnectDomainEventRegisterRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectDomainEventRegister,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectDomainEventRegisterRet { cb_registered } = res;
        Ok(cb_registered)
    }
    fn connect_domain_event_deregister(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_domain_event_deregister));
        let req: Option<()> = None;
        let res: Option<RemoteConnectDomainEventDeregisterRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectDomainEventDeregister,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectDomainEventDeregisterRet { cb_registered } = res;
        Ok(cb_registered)
    }
    fn domain_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventLifecycle, req)?;
        Ok(())
    }
    fn domain_migrate_prepare2(
        &mut self,
        uri_in: Option<String>,
        flags: u64,
        dname: Option<String>,
        resource: u64,
        dom_xml: String,
    ) -> Result<(Vec<u8>, Option<String>), Error> {
        trace!("{}", stringify!(domain_migrate_prepare2));
        let req: Option<RemoteDomainMigratePrepare2Args> = Some(RemoteDomainMigratePrepare2Args {
            uri_in,
            flags,
            dname,
            resource,
            dom_xml,
        });
        let res: Option<RemoteDomainMigratePrepare2Ret> =
            call(self, RemoteProcedure::RemoteProcDomainMigratePrepare2, req)?;
        let res = res.unwrap();
        let RemoteDomainMigratePrepare2Ret { cookie, uri_out } = res;
        Ok((cookie, uri_out))
    }
    fn domain_migrate_finish2(
        &mut self,
        dname: String,
        cookie: Vec<u8>,
        uri: String,
        flags: u64,
        retcode: i32,
    ) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_migrate_finish2));
        let req: Option<RemoteDomainMigrateFinish2Args> = Some(RemoteDomainMigrateFinish2Args {
            dname,
            cookie,
            uri,
            flags,
            retcode,
        });
        let res: Option<RemoteDomainMigrateFinish2Ret> =
            call(self, RemoteProcedure::RemoteProcDomainMigrateFinish2, req)?;
        let res = res.unwrap();
        let RemoteDomainMigrateFinish2Ret { ddom } = res;
        Ok(ddom)
    }
    fn connect_get_uri(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_uri));
        let req: Option<()> = None;
        let res: Option<RemoteConnectGetUriRet> =
            call(self, RemoteProcedure::RemoteProcConnectGetUri, req)?;
        let res = res.unwrap();
        let RemoteConnectGetUriRet { uri } = res;
        Ok(uri)
    }
    fn node_num_of_devices(&mut self, cap: Option<String>, flags: u32) -> Result<i32, Error> {
        trace!("{}", stringify!(node_num_of_devices));
        let req: Option<RemoteNodeNumOfDevicesArgs> =
            Some(RemoteNodeNumOfDevicesArgs { cap, flags });
        let res: Option<RemoteNodeNumOfDevicesRet> =
            call(self, RemoteProcedure::RemoteProcNodeNumOfDevices, req)?;
        let res = res.unwrap();
        let RemoteNodeNumOfDevicesRet { num } = res;
        Ok(num)
    }
    fn node_list_devices(
        &mut self,
        cap: Option<String>,
        maxnames: i32,
        flags: u32,
    ) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(node_list_devices));
        let req: Option<RemoteNodeListDevicesArgs> = Some(RemoteNodeListDevicesArgs {
            cap,
            maxnames,
            flags,
        });
        let res: Option<RemoteNodeListDevicesRet> =
            call(self, RemoteProcedure::RemoteProcNodeListDevices, req)?;
        let res = res.unwrap();
        let RemoteNodeListDevicesRet { names } = res;
        Ok(names)
    }
    fn node_device_lookup_by_name(
        &mut self,
        name: String,
    ) -> Result<RemoteNonnullNodeDevice, Error> {
        trace!("{}", stringify!(node_device_lookup_by_name));
        let req: Option<RemoteNodeDeviceLookupByNameArgs> =
            Some(RemoteNodeDeviceLookupByNameArgs { name });
        let res: Option<RemoteNodeDeviceLookupByNameRet> =
            call(self, RemoteProcedure::RemoteProcNodeDeviceLookupByName, req)?;
        let res = res.unwrap();
        let RemoteNodeDeviceLookupByNameRet { dev } = res;
        Ok(dev)
    }
    fn node_device_get_xml_desc(&mut self, name: String, flags: u32) -> Result<String, Error> {
        trace!("{}", stringify!(node_device_get_xml_desc));
        let req: Option<RemoteNodeDeviceGetXmlDescArgs> =
            Some(RemoteNodeDeviceGetXmlDescArgs { name, flags });
        let res: Option<RemoteNodeDeviceGetXmlDescRet> =
            call(self, RemoteProcedure::RemoteProcNodeDeviceGetXmlDesc, req)?;
        let res = res.unwrap();
        let RemoteNodeDeviceGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn node_device_get_parent(&mut self, name: String) -> Result<Option<String>, Error> {
        trace!("{}", stringify!(node_device_get_parent));
        let req: Option<RemoteNodeDeviceGetParentArgs> =
            Some(RemoteNodeDeviceGetParentArgs { name });
        let res: Option<RemoteNodeDeviceGetParentRet> =
            call(self, RemoteProcedure::RemoteProcNodeDeviceGetParent, req)?;
        let res = res.unwrap();
        let RemoteNodeDeviceGetParentRet { parent_name } = res;
        Ok(parent_name)
    }
    fn node_device_num_of_caps(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_num_of_caps));
        let req: Option<RemoteNodeDeviceNumOfCapsArgs> =
            Some(RemoteNodeDeviceNumOfCapsArgs { name });
        let res: Option<RemoteNodeDeviceNumOfCapsRet> =
            call(self, RemoteProcedure::RemoteProcNodeDeviceNumOfCaps, req)?;
        let res = res.unwrap();
        let RemoteNodeDeviceNumOfCapsRet { num } = res;
        Ok(num)
    }
    fn node_device_list_caps(&mut self, name: String, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(node_device_list_caps));
        let req: Option<RemoteNodeDeviceListCapsArgs> =
            Some(RemoteNodeDeviceListCapsArgs { name, maxnames });
        let res: Option<RemoteNodeDeviceListCapsRet> =
            call(self, RemoteProcedure::RemoteProcNodeDeviceListCaps, req)?;
        let res = res.unwrap();
        let RemoteNodeDeviceListCapsRet { names } = res;
        Ok(names)
    }
    fn node_device_dettach(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_dettach));
        let req: Option<RemoteNodeDeviceDettachArgs> = Some(RemoteNodeDeviceDettachArgs { name });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNodeDeviceDettach, req)?;
        Ok(())
    }
    fn node_device_re_attach(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_re_attach));
        let req: Option<RemoteNodeDeviceReAttachArgs> = Some(RemoteNodeDeviceReAttachArgs { name });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNodeDeviceReAttach, req)?;
        Ok(())
    }
    fn node_device_reset(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_reset));
        let req: Option<RemoteNodeDeviceResetArgs> = Some(RemoteNodeDeviceResetArgs { name });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNodeDeviceReset, req)?;
        Ok(())
    }
    fn domain_get_security_label(
        &mut self,
        dom: RemoteNonnullDomain,
    ) -> Result<(Vec<i8>, i32), Error> {
        trace!("{}", stringify!(domain_get_security_label));
        let req: Option<RemoteDomainGetSecurityLabelArgs> =
            Some(RemoteDomainGetSecurityLabelArgs { dom });
        let res: Option<RemoteDomainGetSecurityLabelRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetSecurityLabel, req)?;
        let res = res.unwrap();
        let RemoteDomainGetSecurityLabelRet { label, enforcing } = res;
        Ok((label, enforcing))
    }
    fn node_get_security_model(&mut self) -> Result<(Vec<i8>, Vec<i8>), Error> {
        trace!("{}", stringify!(node_get_security_model));
        let req: Option<()> = None;
        let res: Option<RemoteNodeGetSecurityModelRet> =
            call(self, RemoteProcedure::RemoteProcNodeGetSecurityModel, req)?;
        let res = res.unwrap();
        let RemoteNodeGetSecurityModelRet { model, doi } = res;
        Ok((model, doi))
    }
    fn node_device_create_xml(
        &mut self,
        xml_desc: String,
        flags: u32,
    ) -> Result<RemoteNonnullNodeDevice, Error> {
        trace!("{}", stringify!(node_device_create_xml));
        let req: Option<RemoteNodeDeviceCreateXmlArgs> =
            Some(RemoteNodeDeviceCreateXmlArgs { xml_desc, flags });
        let res: Option<RemoteNodeDeviceCreateXmlRet> =
            call(self, RemoteProcedure::RemoteProcNodeDeviceCreateXml, req)?;
        let res = res.unwrap();
        let RemoteNodeDeviceCreateXmlRet { dev } = res;
        Ok(dev)
    }
    fn node_device_destroy(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_destroy));
        let req: Option<RemoteNodeDeviceDestroyArgs> = Some(RemoteNodeDeviceDestroyArgs { name });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNodeDeviceDestroy, req)?;
        Ok(())
    }
    fn storage_vol_create_xml_from(
        &mut self,
        pool: RemoteNonnullStoragePool,
        xml: String,
        clonevol: RemoteNonnullStorageVol,
        flags: u32,
    ) -> Result<RemoteNonnullStorageVol, Error> {
        trace!("{}", stringify!(storage_vol_create_xml_from));
        let req: Option<RemoteStorageVolCreateXmlFromArgs> =
            Some(RemoteStorageVolCreateXmlFromArgs {
                pool,
                xml,
                clonevol,
                flags,
            });
        let res: Option<RemoteStorageVolCreateXmlFromRet> = call(
            self,
            RemoteProcedure::RemoteProcStorageVolCreateXmlFrom,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStorageVolCreateXmlFromRet { vol } = res;
        Ok(vol)
    }
    fn connect_num_of_interfaces(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_interfaces));
        let req: Option<()> = None;
        let res: Option<RemoteConnectNumOfInterfacesRet> =
            call(self, RemoteProcedure::RemoteProcConnectNumOfInterfaces, req)?;
        let res = res.unwrap();
        let RemoteConnectNumOfInterfacesRet { num } = res;
        Ok(num)
    }
    fn connect_list_interfaces(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_interfaces));
        let req: Option<RemoteConnectListInterfacesArgs> =
            Some(RemoteConnectListInterfacesArgs { maxnames });
        let res: Option<RemoteConnectListInterfacesRet> =
            call(self, RemoteProcedure::RemoteProcConnectListInterfaces, req)?;
        let res = res.unwrap();
        let RemoteConnectListInterfacesRet { names } = res;
        Ok(names)
    }
    fn interface_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullInterface, Error> {
        trace!("{}", stringify!(interface_lookup_by_name));
        let req: Option<RemoteInterfaceLookupByNameArgs> =
            Some(RemoteInterfaceLookupByNameArgs { name });
        let res: Option<RemoteInterfaceLookupByNameRet> =
            call(self, RemoteProcedure::RemoteProcInterfaceLookupByName, req)?;
        let res = res.unwrap();
        let RemoteInterfaceLookupByNameRet { iface } = res;
        Ok(iface)
    }
    fn interface_lookup_by_mac_string(
        &mut self,
        mac: String,
    ) -> Result<RemoteNonnullInterface, Error> {
        trace!("{}", stringify!(interface_lookup_by_mac_string));
        let req: Option<RemoteInterfaceLookupByMacStringArgs> =
            Some(RemoteInterfaceLookupByMacStringArgs { mac });
        let res: Option<RemoteInterfaceLookupByMacStringRet> = call(
            self,
            RemoteProcedure::RemoteProcInterfaceLookupByMacString,
            req,
        )?;
        let res = res.unwrap();
        let RemoteInterfaceLookupByMacStringRet { iface } = res;
        Ok(iface)
    }
    fn interface_get_xml_desc(
        &mut self,
        iface: RemoteNonnullInterface,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(interface_get_xml_desc));
        let req: Option<RemoteInterfaceGetXmlDescArgs> =
            Some(RemoteInterfaceGetXmlDescArgs { iface, flags });
        let res: Option<RemoteInterfaceGetXmlDescRet> =
            call(self, RemoteProcedure::RemoteProcInterfaceGetXmlDesc, req)?;
        let res = res.unwrap();
        let RemoteInterfaceGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn interface_define_xml(
        &mut self,
        xml: String,
        flags: u32,
    ) -> Result<RemoteNonnullInterface, Error> {
        trace!("{}", stringify!(interface_define_xml));
        let req: Option<RemoteInterfaceDefineXmlArgs> =
            Some(RemoteInterfaceDefineXmlArgs { xml, flags });
        let res: Option<RemoteInterfaceDefineXmlRet> =
            call(self, RemoteProcedure::RemoteProcInterfaceDefineXml, req)?;
        let res = res.unwrap();
        let RemoteInterfaceDefineXmlRet { iface } = res;
        Ok(iface)
    }
    fn interface_undefine(&mut self, iface: RemoteNonnullInterface) -> Result<(), Error> {
        trace!("{}", stringify!(interface_undefine));
        let req: Option<RemoteInterfaceUndefineArgs> = Some(RemoteInterfaceUndefineArgs { iface });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcInterfaceUndefine, req)?;
        Ok(())
    }
    fn interface_create(&mut self, iface: RemoteNonnullInterface, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_create));
        let req: Option<RemoteInterfaceCreateArgs> =
            Some(RemoteInterfaceCreateArgs { iface, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcInterfaceCreate, req)?;
        Ok(())
    }
    fn interface_destroy(
        &mut self,
        iface: RemoteNonnullInterface,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(interface_destroy));
        let req: Option<RemoteInterfaceDestroyArgs> =
            Some(RemoteInterfaceDestroyArgs { iface, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcInterfaceDestroy, req)?;
        Ok(())
    }
    fn connect_domain_xml_from_native(
        &mut self,
        native_format: String,
        native_config: String,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(connect_domain_xml_from_native));
        let req: Option<RemoteConnectDomainXmlFromNativeArgs> =
            Some(RemoteConnectDomainXmlFromNativeArgs {
                native_format,
                native_config,
                flags,
            });
        let res: Option<RemoteConnectDomainXmlFromNativeRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectDomainXmlFromNative,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectDomainXmlFromNativeRet { domain_xml } = res;
        Ok(domain_xml)
    }
    fn connect_domain_xml_to_native(
        &mut self,
        native_format: String,
        domain_xml: String,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(connect_domain_xml_to_native));
        let req: Option<RemoteConnectDomainXmlToNativeArgs> =
            Some(RemoteConnectDomainXmlToNativeArgs {
                native_format,
                domain_xml,
                flags,
            });
        let res: Option<RemoteConnectDomainXmlToNativeRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectDomainXmlToNative,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectDomainXmlToNativeRet { native_config } = res;
        Ok(native_config)
    }
    fn connect_num_of_defined_interfaces(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_defined_interfaces));
        let req: Option<()> = None;
        let res: Option<RemoteConnectNumOfDefinedInterfacesRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectNumOfDefinedInterfaces,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNumOfDefinedInterfacesRet { num } = res;
        Ok(num)
    }
    fn connect_list_defined_interfaces(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_defined_interfaces));
        let req: Option<RemoteConnectListDefinedInterfacesArgs> =
            Some(RemoteConnectListDefinedInterfacesArgs { maxnames });
        let res: Option<RemoteConnectListDefinedInterfacesRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectListDefinedInterfaces,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListDefinedInterfacesRet { names } = res;
        Ok(names)
    }
    fn connect_num_of_secrets(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_secrets));
        let req: Option<()> = None;
        let res: Option<RemoteConnectNumOfSecretsRet> =
            call(self, RemoteProcedure::RemoteProcConnectNumOfSecrets, req)?;
        let res = res.unwrap();
        let RemoteConnectNumOfSecretsRet { num } = res;
        Ok(num)
    }
    fn connect_list_secrets(&mut self, maxuuids: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_secrets));
        let req: Option<RemoteConnectListSecretsArgs> =
            Some(RemoteConnectListSecretsArgs { maxuuids });
        let res: Option<RemoteConnectListSecretsRet> =
            call(self, RemoteProcedure::RemoteProcConnectListSecrets, req)?;
        let res = res.unwrap();
        let RemoteConnectListSecretsRet { uuids } = res;
        Ok(uuids)
    }
    fn secret_lookup_by_uuid(
        &mut self,
        uuid: [u8; VIR_UUID_BUFLEN as usize],
    ) -> Result<RemoteNonnullSecret, Error> {
        trace!("{}", stringify!(secret_lookup_by_uuid));
        let req: Option<RemoteSecretLookupByUuidArgs> = Some(RemoteSecretLookupByUuidArgs { uuid });
        let res: Option<RemoteSecretLookupByUuidRet> =
            call(self, RemoteProcedure::RemoteProcSecretLookupByUuid, req)?;
        let res = res.unwrap();
        let RemoteSecretLookupByUuidRet { secret } = res;
        Ok(secret)
    }
    fn secret_define_xml(&mut self, xml: String, flags: u32) -> Result<RemoteNonnullSecret, Error> {
        trace!("{}", stringify!(secret_define_xml));
        let req: Option<RemoteSecretDefineXmlArgs> = Some(RemoteSecretDefineXmlArgs { xml, flags });
        let res: Option<RemoteSecretDefineXmlRet> =
            call(self, RemoteProcedure::RemoteProcSecretDefineXml, req)?;
        let res = res.unwrap();
        let RemoteSecretDefineXmlRet { secret } = res;
        Ok(secret)
    }
    fn secret_get_xml_desc(
        &mut self,
        secret: RemoteNonnullSecret,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(secret_get_xml_desc));
        let req: Option<RemoteSecretGetXmlDescArgs> =
            Some(RemoteSecretGetXmlDescArgs { secret, flags });
        let res: Option<RemoteSecretGetXmlDescRet> =
            call(self, RemoteProcedure::RemoteProcSecretGetXmlDesc, req)?;
        let res = res.unwrap();
        let RemoteSecretGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn secret_set_value(
        &mut self,
        secret: RemoteNonnullSecret,
        value: Vec<u8>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(secret_set_value));
        let req: Option<RemoteSecretSetValueArgs> = Some(RemoteSecretSetValueArgs {
            secret,
            value,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcSecretSetValue, req)?;
        Ok(())
    }
    fn secret_get_value(
        &mut self,
        secret: RemoteNonnullSecret,
        flags: u32,
    ) -> Result<Vec<u8>, Error> {
        trace!("{}", stringify!(secret_get_value));
        let req: Option<RemoteSecretGetValueArgs> =
            Some(RemoteSecretGetValueArgs { secret, flags });
        let res: Option<RemoteSecretGetValueRet> =
            call(self, RemoteProcedure::RemoteProcSecretGetValue, req)?;
        let res = res.unwrap();
        let RemoteSecretGetValueRet { value } = res;
        Ok(value)
    }
    fn secret_undefine(&mut self, secret: RemoteNonnullSecret) -> Result<(), Error> {
        trace!("{}", stringify!(secret_undefine));
        let req: Option<RemoteSecretUndefineArgs> = Some(RemoteSecretUndefineArgs { secret });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcSecretUndefine, req)?;
        Ok(())
    }
    fn secret_lookup_by_usage(
        &mut self,
        usage_type: i32,
        usage_id: String,
    ) -> Result<RemoteNonnullSecret, Error> {
        trace!("{}", stringify!(secret_lookup_by_usage));
        let req: Option<RemoteSecretLookupByUsageArgs> = Some(RemoteSecretLookupByUsageArgs {
            usage_type,
            usage_id,
        });
        let res: Option<RemoteSecretLookupByUsageRet> =
            call(self, RemoteProcedure::RemoteProcSecretLookupByUsage, req)?;
        let res = res.unwrap();
        let RemoteSecretLookupByUsageRet { secret } = res;
        Ok(secret)
    }
    fn domain_migrate_prepare_tunnel(
        &mut self,
        flags: u64,
        dname: Option<String>,
        resource: u64,
        dom_xml: String,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_prepare_tunnel));
        let req: Option<RemoteDomainMigratePrepareTunnelArgs> =
            Some(RemoteDomainMigratePrepareTunnelArgs {
                flags,
                dname,
                resource,
                dom_xml,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigratePrepareTunnel,
            req,
        )?;
        Ok(())
    }
    fn connect_is_secure(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_is_secure));
        let req: Option<()> = None;
        let res: Option<RemoteConnectIsSecureRet> =
            call(self, RemoteProcedure::RemoteProcConnectIsSecure, req)?;
        let res = res.unwrap();
        let RemoteConnectIsSecureRet { secure } = res;
        Ok(secure)
    }
    fn domain_is_active(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_is_active));
        let req: Option<RemoteDomainIsActiveArgs> = Some(RemoteDomainIsActiveArgs { dom });
        let res: Option<RemoteDomainIsActiveRet> =
            call(self, RemoteProcedure::RemoteProcDomainIsActive, req)?;
        let res = res.unwrap();
        let RemoteDomainIsActiveRet { active } = res;
        Ok(active)
    }
    fn domain_is_persistent(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_is_persistent));
        let req: Option<RemoteDomainIsPersistentArgs> = Some(RemoteDomainIsPersistentArgs { dom });
        let res: Option<RemoteDomainIsPersistentRet> =
            call(self, RemoteProcedure::RemoteProcDomainIsPersistent, req)?;
        let res = res.unwrap();
        let RemoteDomainIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn network_is_active(&mut self, net: RemoteNonnullNetwork) -> Result<i32, Error> {
        trace!("{}", stringify!(network_is_active));
        let req: Option<RemoteNetworkIsActiveArgs> = Some(RemoteNetworkIsActiveArgs { net });
        let res: Option<RemoteNetworkIsActiveRet> =
            call(self, RemoteProcedure::RemoteProcNetworkIsActive, req)?;
        let res = res.unwrap();
        let RemoteNetworkIsActiveRet { active } = res;
        Ok(active)
    }
    fn network_is_persistent(&mut self, net: RemoteNonnullNetwork) -> Result<i32, Error> {
        trace!("{}", stringify!(network_is_persistent));
        let req: Option<RemoteNetworkIsPersistentArgs> =
            Some(RemoteNetworkIsPersistentArgs { net });
        let res: Option<RemoteNetworkIsPersistentRet> =
            call(self, RemoteProcedure::RemoteProcNetworkIsPersistent, req)?;
        let res = res.unwrap();
        let RemoteNetworkIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn storage_pool_is_active(&mut self, pool: RemoteNonnullStoragePool) -> Result<i32, Error> {
        trace!("{}", stringify!(storage_pool_is_active));
        let req: Option<RemoteStoragePoolIsActiveArgs> =
            Some(RemoteStoragePoolIsActiveArgs { pool });
        let res: Option<RemoteStoragePoolIsActiveRet> =
            call(self, RemoteProcedure::RemoteProcStoragePoolIsActive, req)?;
        let res = res.unwrap();
        let RemoteStoragePoolIsActiveRet { active } = res;
        Ok(active)
    }
    fn storage_pool_is_persistent(&mut self, pool: RemoteNonnullStoragePool) -> Result<i32, Error> {
        trace!("{}", stringify!(storage_pool_is_persistent));
        let req: Option<RemoteStoragePoolIsPersistentArgs> =
            Some(RemoteStoragePoolIsPersistentArgs { pool });
        let res: Option<RemoteStoragePoolIsPersistentRet> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolIsPersistent,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStoragePoolIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn interface_is_active(&mut self, iface: RemoteNonnullInterface) -> Result<i32, Error> {
        trace!("{}", stringify!(interface_is_active));
        let req: Option<RemoteInterfaceIsActiveArgs> = Some(RemoteInterfaceIsActiveArgs { iface });
        let res: Option<RemoteInterfaceIsActiveRet> =
            call(self, RemoteProcedure::RemoteProcInterfaceIsActive, req)?;
        let res = res.unwrap();
        let RemoteInterfaceIsActiveRet { active } = res;
        Ok(active)
    }
    fn connect_get_lib_version(&mut self) -> Result<u64, Error> {
        trace!("{}", stringify!(connect_get_lib_version));
        let req: Option<()> = None;
        let res: Option<RemoteConnectGetLibVersionRet> =
            call(self, RemoteProcedure::RemoteProcConnectGetLibVersion, req)?;
        let res = res.unwrap();
        let RemoteConnectGetLibVersionRet { lib_ver } = res;
        Ok(lib_ver)
    }
    fn connect_compare_cpu(&mut self, xml: String, flags: u32) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_compare_cpu));
        let req: Option<RemoteConnectCompareCpuArgs> =
            Some(RemoteConnectCompareCpuArgs { xml, flags });
        let res: Option<RemoteConnectCompareCpuRet> =
            call(self, RemoteProcedure::RemoteProcConnectCompareCpu, req)?;
        let res = res.unwrap();
        let RemoteConnectCompareCpuRet { result } = res;
        Ok(result)
    }
    fn domain_memory_stats(
        &mut self,
        dom: RemoteNonnullDomain,
        max_stats: u32,
        flags: u32,
    ) -> Result<Vec<RemoteDomainMemoryStat>, Error> {
        trace!("{}", stringify!(domain_memory_stats));
        let req: Option<RemoteDomainMemoryStatsArgs> = Some(RemoteDomainMemoryStatsArgs {
            dom,
            max_stats,
            flags,
        });
        let res: Option<RemoteDomainMemoryStatsRet> =
            call(self, RemoteProcedure::RemoteProcDomainMemoryStats, req)?;
        let res = res.unwrap();
        let RemoteDomainMemoryStatsRet { stats } = res;
        Ok(stats)
    }
    fn domain_attach_device_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        xml: String,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_attach_device_flags));
        let req: Option<RemoteDomainAttachDeviceFlagsArgs> =
            Some(RemoteDomainAttachDeviceFlagsArgs { dom, xml, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainAttachDeviceFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_detach_device_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        xml: String,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_detach_device_flags));
        let req: Option<RemoteDomainDetachDeviceFlagsArgs> =
            Some(RemoteDomainDetachDeviceFlagsArgs { dom, xml, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainDetachDeviceFlags,
            req,
        )?;
        Ok(())
    }
    fn connect_baseline_cpu(&mut self, xml_cpus: Vec<String>, flags: u32) -> Result<String, Error> {
        trace!("{}", stringify!(connect_baseline_cpu));
        let req: Option<RemoteConnectBaselineCpuArgs> =
            Some(RemoteConnectBaselineCpuArgs { xml_cpus, flags });
        let res: Option<RemoteConnectBaselineCpuRet> =
            call(self, RemoteProcedure::RemoteProcConnectBaselineCpu, req)?;
        let res = res.unwrap();
        let RemoteConnectBaselineCpuRet { cpu } = res;
        Ok(cpu)
    }
    fn domain_get_job_info(
        &mut self,
        dom: RemoteNonnullDomain,
    ) -> Result<RemoteDomainGetJobInfoRet, Error> {
        trace!("{}", stringify!(domain_get_job_info));
        let req: Option<RemoteDomainGetJobInfoArgs> = Some(RemoteDomainGetJobInfoArgs { dom });
        let res: Option<RemoteDomainGetJobInfoRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetJobInfo, req)?;
        Ok(res.unwrap())
    }
    fn domain_abort_job(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_abort_job));
        let req: Option<RemoteDomainAbortJobArgs> = Some(RemoteDomainAbortJobArgs { dom });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainAbortJob, req)?;
        Ok(())
    }
    fn storage_vol_wipe(&mut self, vol: RemoteNonnullStorageVol, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_wipe));
        let req: Option<RemoteStorageVolWipeArgs> = Some(RemoteStorageVolWipeArgs { vol, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStorageVolWipe, req)?;
        Ok(())
    }
    fn domain_migrate_set_max_downtime(
        &mut self,
        dom: RemoteNonnullDomain,
        downtime: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_set_max_downtime));
        let req: Option<RemoteDomainMigrateSetMaxDowntimeArgs> =
            Some(RemoteDomainMigrateSetMaxDowntimeArgs {
                dom,
                downtime,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigrateSetMaxDowntime,
            req,
        )?;
        Ok(())
    }
    fn connect_domain_event_register_any(&mut self, event_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_domain_event_register_any));
        let req: Option<RemoteConnectDomainEventRegisterAnyArgs> =
            Some(RemoteConnectDomainEventRegisterAnyArgs { event_id });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcConnectDomainEventRegisterAny,
            req,
        )?;
        Ok(())
    }
    fn connect_domain_event_deregister_any(&mut self, event_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_domain_event_deregister_any));
        let req: Option<RemoteConnectDomainEventDeregisterAnyArgs> =
            Some(RemoteConnectDomainEventDeregisterAnyArgs { event_id });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcConnectDomainEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn domain_event_reboot(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_reboot));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventReboot, req)?;
        Ok(())
    }
    fn domain_event_rtc_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_rtc_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventRtcChange, req)?;
        Ok(())
    }
    fn domain_event_watchdog(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_watchdog));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventWatchdog, req)?;
        Ok(())
    }
    fn domain_event_io_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_io_error));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventIoError, req)?;
        Ok(())
    }
    fn domain_event_graphics(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_graphics));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventGraphics, req)?;
        Ok(())
    }
    fn domain_update_device_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        xml: String,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_update_device_flags));
        let req: Option<RemoteDomainUpdateDeviceFlagsArgs> =
            Some(RemoteDomainUpdateDeviceFlagsArgs { dom, xml, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainUpdateDeviceFlags,
            req,
        )?;
        Ok(())
    }
    fn nwfilter_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullNwfilter, Error> {
        trace!("{}", stringify!(nwfilter_lookup_by_name));
        let req: Option<RemoteNwfilterLookupByNameArgs> =
            Some(RemoteNwfilterLookupByNameArgs { name });
        let res: Option<RemoteNwfilterLookupByNameRet> =
            call(self, RemoteProcedure::RemoteProcNwfilterLookupByName, req)?;
        let res = res.unwrap();
        let RemoteNwfilterLookupByNameRet { nwfilter } = res;
        Ok(nwfilter)
    }
    fn nwfilter_lookup_by_uuid(
        &mut self,
        uuid: [u8; VIR_UUID_BUFLEN as usize],
    ) -> Result<RemoteNonnullNwfilter, Error> {
        trace!("{}", stringify!(nwfilter_lookup_by_uuid));
        let req: Option<RemoteNwfilterLookupByUuidArgs> =
            Some(RemoteNwfilterLookupByUuidArgs { uuid });
        let res: Option<RemoteNwfilterLookupByUuidRet> =
            call(self, RemoteProcedure::RemoteProcNwfilterLookupByUuid, req)?;
        let res = res.unwrap();
        let RemoteNwfilterLookupByUuidRet { nwfilter } = res;
        Ok(nwfilter)
    }
    fn nwfilter_get_xml_desc(
        &mut self,
        nwfilter: RemoteNonnullNwfilter,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(nwfilter_get_xml_desc));
        let req: Option<RemoteNwfilterGetXmlDescArgs> =
            Some(RemoteNwfilterGetXmlDescArgs { nwfilter, flags });
        let res: Option<RemoteNwfilterGetXmlDescRet> =
            call(self, RemoteProcedure::RemoteProcNwfilterGetXmlDesc, req)?;
        let res = res.unwrap();
        let RemoteNwfilterGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn connect_num_of_nwfilters(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_nwfilters));
        let req: Option<()> = None;
        let res: Option<RemoteConnectNumOfNwfiltersRet> =
            call(self, RemoteProcedure::RemoteProcConnectNumOfNwfilters, req)?;
        let res = res.unwrap();
        let RemoteConnectNumOfNwfiltersRet { num } = res;
        Ok(num)
    }
    fn connect_list_nwfilters(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_nwfilters));
        let req: Option<RemoteConnectListNwfiltersArgs> =
            Some(RemoteConnectListNwfiltersArgs { maxnames });
        let res: Option<RemoteConnectListNwfiltersRet> =
            call(self, RemoteProcedure::RemoteProcConnectListNwfilters, req)?;
        let res = res.unwrap();
        let RemoteConnectListNwfiltersRet { names } = res;
        Ok(names)
    }
    fn nwfilter_define_xml(&mut self, xml: String) -> Result<RemoteNonnullNwfilter, Error> {
        trace!("{}", stringify!(nwfilter_define_xml));
        let req: Option<RemoteNwfilterDefineXmlArgs> = Some(RemoteNwfilterDefineXmlArgs { xml });
        let res: Option<RemoteNwfilterDefineXmlRet> =
            call(self, RemoteProcedure::RemoteProcNwfilterDefineXml, req)?;
        let res = res.unwrap();
        let RemoteNwfilterDefineXmlRet { nwfilter } = res;
        Ok(nwfilter)
    }
    fn nwfilter_undefine(&mut self, nwfilter: RemoteNonnullNwfilter) -> Result<(), Error> {
        trace!("{}", stringify!(nwfilter_undefine));
        let req: Option<RemoteNwfilterUndefineArgs> = Some(RemoteNwfilterUndefineArgs { nwfilter });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNwfilterUndefine, req)?;
        Ok(())
    }
    fn domain_managed_save(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_managed_save));
        let req: Option<RemoteDomainManagedSaveArgs> =
            Some(RemoteDomainManagedSaveArgs { dom, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainManagedSave, req)?;
        Ok(())
    }
    fn domain_has_managed_save_image(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_has_managed_save_image));
        let req: Option<RemoteDomainHasManagedSaveImageArgs> =
            Some(RemoteDomainHasManagedSaveImageArgs { dom, flags });
        let res: Option<RemoteDomainHasManagedSaveImageRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainHasManagedSaveImage,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainHasManagedSaveImageRet { result } = res;
        Ok(result)
    }
    fn domain_managed_save_remove(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_managed_save_remove));
        let req: Option<RemoteDomainManagedSaveRemoveArgs> =
            Some(RemoteDomainManagedSaveRemoveArgs { dom, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainManagedSaveRemove,
            req,
        )?;
        Ok(())
    }
    fn domain_snapshot_create_xml(
        &mut self,
        dom: RemoteNonnullDomain,
        xml_desc: String,
        flags: u32,
    ) -> Result<RemoteNonnullDomainSnapshot, Error> {
        trace!("{}", stringify!(domain_snapshot_create_xml));
        let req: Option<RemoteDomainSnapshotCreateXmlArgs> =
            Some(RemoteDomainSnapshotCreateXmlArgs {
                dom,
                xml_desc,
                flags,
            });
        let res: Option<RemoteDomainSnapshotCreateXmlRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotCreateXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSnapshotCreateXmlRet { snap } = res;
        Ok(snap)
    }
    fn domain_snapshot_get_xml_desc(
        &mut self,
        snap: RemoteNonnullDomainSnapshot,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(domain_snapshot_get_xml_desc));
        let req: Option<RemoteDomainSnapshotGetXmlDescArgs> =
            Some(RemoteDomainSnapshotGetXmlDescArgs { snap, flags });
        let res: Option<RemoteDomainSnapshotGetXmlDescRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSnapshotGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_snapshot_num(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_snapshot_num));
        let req: Option<RemoteDomainSnapshotNumArgs> =
            Some(RemoteDomainSnapshotNumArgs { dom, flags });
        let res: Option<RemoteDomainSnapshotNumRet> =
            call(self, RemoteProcedure::RemoteProcDomainSnapshotNum, req)?;
        let res = res.unwrap();
        let RemoteDomainSnapshotNumRet { num } = res;
        Ok(num)
    }
    fn domain_snapshot_list_names(
        &mut self,
        dom: RemoteNonnullDomain,
        maxnames: i32,
        flags: u32,
    ) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(domain_snapshot_list_names));
        let req: Option<RemoteDomainSnapshotListNamesArgs> =
            Some(RemoteDomainSnapshotListNamesArgs {
                dom,
                maxnames,
                flags,
            });
        let res: Option<RemoteDomainSnapshotListNamesRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotListNames,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSnapshotListNamesRet { names } = res;
        Ok(names)
    }
    fn domain_snapshot_lookup_by_name(
        &mut self,
        dom: RemoteNonnullDomain,
        name: String,
        flags: u32,
    ) -> Result<RemoteNonnullDomainSnapshot, Error> {
        trace!("{}", stringify!(domain_snapshot_lookup_by_name));
        let req: Option<RemoteDomainSnapshotLookupByNameArgs> =
            Some(RemoteDomainSnapshotLookupByNameArgs { dom, name, flags });
        let res: Option<RemoteDomainSnapshotLookupByNameRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotLookupByName,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSnapshotLookupByNameRet { snap } = res;
        Ok(snap)
    }
    fn domain_has_current_snapshot(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_has_current_snapshot));
        let req: Option<RemoteDomainHasCurrentSnapshotArgs> =
            Some(RemoteDomainHasCurrentSnapshotArgs { dom, flags });
        let res: Option<RemoteDomainHasCurrentSnapshotRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainHasCurrentSnapshot,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainHasCurrentSnapshotRet { result } = res;
        Ok(result)
    }
    fn domain_snapshot_current(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<RemoteNonnullDomainSnapshot, Error> {
        trace!("{}", stringify!(domain_snapshot_current));
        let req: Option<RemoteDomainSnapshotCurrentArgs> =
            Some(RemoteDomainSnapshotCurrentArgs { dom, flags });
        let res: Option<RemoteDomainSnapshotCurrentRet> =
            call(self, RemoteProcedure::RemoteProcDomainSnapshotCurrent, req)?;
        let res = res.unwrap();
        let RemoteDomainSnapshotCurrentRet { snap } = res;
        Ok(snap)
    }
    fn domain_revert_to_snapshot(
        &mut self,
        snap: RemoteNonnullDomainSnapshot,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_revert_to_snapshot));
        let req: Option<RemoteDomainRevertToSnapshotArgs> =
            Some(RemoteDomainRevertToSnapshotArgs { snap, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainRevertToSnapshot, req)?;
        Ok(())
    }
    fn domain_snapshot_delete(
        &mut self,
        snap: RemoteNonnullDomainSnapshot,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_snapshot_delete));
        let req: Option<RemoteDomainSnapshotDeleteArgs> =
            Some(RemoteDomainSnapshotDeleteArgs { snap, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSnapshotDelete, req)?;
        Ok(())
    }
    fn domain_get_block_info(
        &mut self,
        dom: RemoteNonnullDomain,
        path: String,
        flags: u32,
    ) -> Result<(u64, u64, u64), Error> {
        trace!("{}", stringify!(domain_get_block_info));
        let req: Option<RemoteDomainGetBlockInfoArgs> =
            Some(RemoteDomainGetBlockInfoArgs { dom, path, flags });
        let res: Option<RemoteDomainGetBlockInfoRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetBlockInfo, req)?;
        let res = res.unwrap();
        let RemoteDomainGetBlockInfoRet {
            allocation,
            capacity,
            physical,
        } = res;
        Ok((allocation, capacity, physical))
    }
    fn domain_event_io_error_reason(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_io_error_reason));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventIoErrorReason,
            req,
        )?;
        Ok(())
    }
    fn domain_create_with_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_create_with_flags));
        let req: Option<RemoteDomainCreateWithFlagsArgs> =
            Some(RemoteDomainCreateWithFlagsArgs { dom, flags });
        let res: Option<RemoteDomainCreateWithFlagsRet> =
            call(self, RemoteProcedure::RemoteProcDomainCreateWithFlags, req)?;
        let res = res.unwrap();
        let RemoteDomainCreateWithFlagsRet { dom } = res;
        Ok(dom)
    }
    fn domain_set_memory_parameters(
        &mut self,
        dom: RemoteNonnullDomain,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_memory_parameters));
        let req: Option<RemoteDomainSetMemoryParametersArgs> =
            Some(RemoteDomainSetMemoryParametersArgs { dom, params, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetMemoryParameters,
            req,
        )?;
        Ok(())
    }
    fn domain_get_memory_parameters(
        &mut self,
        dom: RemoteNonnullDomain,
        nparams: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteTypedParam>, i32), Error> {
        trace!("{}", stringify!(domain_get_memory_parameters));
        let req: Option<RemoteDomainGetMemoryParametersArgs> =
            Some(RemoteDomainGetMemoryParametersArgs {
                dom,
                nparams,
                flags,
            });
        let res: Option<RemoteDomainGetMemoryParametersRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainGetMemoryParameters,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetMemoryParametersRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_set_vcpus_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        nvcpus: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_vcpus_flags));
        let req: Option<RemoteDomainSetVcpusFlagsArgs> =
            Some(RemoteDomainSetVcpusFlagsArgs { dom, nvcpus, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetVcpusFlags, req)?;
        Ok(())
    }
    fn domain_get_vcpus_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_get_vcpus_flags));
        let req: Option<RemoteDomainGetVcpusFlagsArgs> =
            Some(RemoteDomainGetVcpusFlagsArgs { dom, flags });
        let res: Option<RemoteDomainGetVcpusFlagsRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetVcpusFlags, req)?;
        let res = res.unwrap();
        let RemoteDomainGetVcpusFlagsRet { num } = res;
        Ok(num)
    }
    fn domain_open_console(
        &mut self,
        dom: RemoteNonnullDomain,
        dev_name: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_open_console));
        let req: Option<RemoteDomainOpenConsoleArgs> = Some(RemoteDomainOpenConsoleArgs {
            dom,
            dev_name,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainOpenConsole, req)?;
        Ok(())
    }
    fn domain_is_updated(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_is_updated));
        let req: Option<RemoteDomainIsUpdatedArgs> = Some(RemoteDomainIsUpdatedArgs { dom });
        let res: Option<RemoteDomainIsUpdatedRet> =
            call(self, RemoteProcedure::RemoteProcDomainIsUpdated, req)?;
        let res = res.unwrap();
        let RemoteDomainIsUpdatedRet { updated } = res;
        Ok(updated)
    }
    fn connect_get_sysinfo(&mut self, flags: u32) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_sysinfo));
        let req: Option<RemoteConnectGetSysinfoArgs> = Some(RemoteConnectGetSysinfoArgs { flags });
        let res: Option<RemoteConnectGetSysinfoRet> =
            call(self, RemoteProcedure::RemoteProcConnectGetSysinfo, req)?;
        let res = res.unwrap();
        let RemoteConnectGetSysinfoRet { sysinfo } = res;
        Ok(sysinfo)
    }
    fn domain_set_memory_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        memory: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_memory_flags));
        let req: Option<RemoteDomainSetMemoryFlagsArgs> =
            Some(RemoteDomainSetMemoryFlagsArgs { dom, memory, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetMemoryFlags, req)?;
        Ok(())
    }
    fn domain_set_blkio_parameters(
        &mut self,
        dom: RemoteNonnullDomain,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_blkio_parameters));
        let req: Option<RemoteDomainSetBlkioParametersArgs> =
            Some(RemoteDomainSetBlkioParametersArgs { dom, params, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetBlkioParameters,
            req,
        )?;
        Ok(())
    }
    fn domain_get_blkio_parameters(
        &mut self,
        dom: RemoteNonnullDomain,
        nparams: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteTypedParam>, i32), Error> {
        trace!("{}", stringify!(domain_get_blkio_parameters));
        let req: Option<RemoteDomainGetBlkioParametersArgs> =
            Some(RemoteDomainGetBlkioParametersArgs {
                dom,
                nparams,
                flags,
            });
        let res: Option<RemoteDomainGetBlkioParametersRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainGetBlkioParameters,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetBlkioParametersRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_migrate_set_max_speed(
        &mut self,
        dom: RemoteNonnullDomain,
        bandwidth: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_set_max_speed));
        let req: Option<RemoteDomainMigrateSetMaxSpeedArgs> =
            Some(RemoteDomainMigrateSetMaxSpeedArgs {
                dom,
                bandwidth,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigrateSetMaxSpeed,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_upload(
        &mut self,
        vol: RemoteNonnullStorageVol,
        offset: u64,
        length: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_upload));
        let req: Option<RemoteStorageVolUploadArgs> = Some(RemoteStorageVolUploadArgs {
            vol,
            offset,
            length,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStorageVolUpload, req)?;
        Ok(())
    }
    fn storage_vol_download(
        &mut self,
        vol: RemoteNonnullStorageVol,
        offset: u64,
        length: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_download));
        let req: Option<RemoteStorageVolDownloadArgs> = Some(RemoteStorageVolDownloadArgs {
            vol,
            offset,
            length,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStorageVolDownload, req)?;
        Ok(())
    }
    fn domain_inject_nmi(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_inject_nmi));
        let req: Option<RemoteDomainInjectNmiArgs> = Some(RemoteDomainInjectNmiArgs { dom, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainInjectNmi, req)?;
        Ok(())
    }
    fn domain_screenshot(
        &mut self,
        dom: RemoteNonnullDomain,
        screen: u32,
        flags: u32,
    ) -> Result<Option<String>, Error> {
        trace!("{}", stringify!(domain_screenshot));
        let req: Option<RemoteDomainScreenshotArgs> =
            Some(RemoteDomainScreenshotArgs { dom, screen, flags });
        let res: Option<RemoteDomainScreenshotRet> =
            call(self, RemoteProcedure::RemoteProcDomainScreenshot, req)?;
        let res = res.unwrap();
        let RemoteDomainScreenshotRet { mime } = res;
        Ok(mime)
    }
    fn domain_get_state(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<(i32, i32), Error> {
        trace!("{}", stringify!(domain_get_state));
        let req: Option<RemoteDomainGetStateArgs> = Some(RemoteDomainGetStateArgs { dom, flags });
        let res: Option<RemoteDomainGetStateRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetState, req)?;
        let res = res.unwrap();
        let RemoteDomainGetStateRet { state, reason } = res;
        Ok((state, reason))
    }
    fn domain_migrate_begin3(
        &mut self,
        dom: RemoteNonnullDomain,
        xmlin: Option<String>,
        flags: u64,
        dname: Option<String>,
        resource: u64,
    ) -> Result<(Vec<u8>, String), Error> {
        trace!("{}", stringify!(domain_migrate_begin3));
        let req: Option<RemoteDomainMigrateBegin3Args> = Some(RemoteDomainMigrateBegin3Args {
            dom,
            xmlin,
            flags,
            dname,
            resource,
        });
        let res: Option<RemoteDomainMigrateBegin3Ret> =
            call(self, RemoteProcedure::RemoteProcDomainMigrateBegin3, req)?;
        let res = res.unwrap();
        let RemoteDomainMigrateBegin3Ret { cookie_out, xml } = res;
        Ok((cookie_out, xml))
    }
    fn domain_migrate_prepare3(
        &mut self,
        cookie_in: Vec<u8>,
        uri_in: Option<String>,
        flags: u64,
        dname: Option<String>,
        resource: u64,
        dom_xml: String,
    ) -> Result<(Vec<u8>, Option<String>), Error> {
        trace!("{}", stringify!(domain_migrate_prepare3));
        let req: Option<RemoteDomainMigratePrepare3Args> = Some(RemoteDomainMigratePrepare3Args {
            cookie_in,
            uri_in,
            flags,
            dname,
            resource,
            dom_xml,
        });
        let res: Option<RemoteDomainMigratePrepare3Ret> =
            call(self, RemoteProcedure::RemoteProcDomainMigratePrepare3, req)?;
        let res = res.unwrap();
        let RemoteDomainMigratePrepare3Ret {
            cookie_out,
            uri_out,
        } = res;
        Ok((cookie_out, uri_out))
    }
    fn domain_migrate_prepare_tunnel3(
        &mut self,
        cookie_in: Vec<u8>,
        flags: u64,
        dname: Option<String>,
        resource: u64,
        dom_xml: String,
    ) -> Result<Vec<u8>, Error> {
        trace!("{}", stringify!(domain_migrate_prepare_tunnel3));
        let req: Option<RemoteDomainMigratePrepareTunnel3Args> =
            Some(RemoteDomainMigratePrepareTunnel3Args {
                cookie_in,
                flags,
                dname,
                resource,
                dom_xml,
            });
        let res: Option<RemoteDomainMigratePrepareTunnel3Ret> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigratePrepareTunnel3,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMigratePrepareTunnel3Ret { cookie_out } = res;
        Ok(cookie_out)
    }
    fn domain_migrate_perform3(
        &mut self,
        args: RemoteDomainMigratePerform3Args,
    ) -> Result<Vec<u8>, Error> {
        trace!("{}", stringify!(domain_migrate_perform3));
        let req: Option<RemoteDomainMigratePerform3Args> = Some(args);
        let res: Option<RemoteDomainMigratePerform3Ret> =
            call(self, RemoteProcedure::RemoteProcDomainMigratePerform3, req)?;
        let res = res.unwrap();
        let RemoteDomainMigratePerform3Ret { cookie_out } = res;
        Ok(cookie_out)
    }
    fn domain_migrate_finish3(
        &mut self,
        dname: String,
        cookie_in: Vec<u8>,
        dconnuri: Option<String>,
        uri: Option<String>,
        flags: u64,
        cancelled: i32,
    ) -> Result<(RemoteNonnullDomain, Vec<u8>), Error> {
        trace!("{}", stringify!(domain_migrate_finish3));
        let req: Option<RemoteDomainMigrateFinish3Args> = Some(RemoteDomainMigrateFinish3Args {
            dname,
            cookie_in,
            dconnuri,
            uri,
            flags,
            cancelled,
        });
        let res: Option<RemoteDomainMigrateFinish3Ret> =
            call(self, RemoteProcedure::RemoteProcDomainMigrateFinish3, req)?;
        let res = res.unwrap();
        let RemoteDomainMigrateFinish3Ret { dom, cookie_out } = res;
        Ok((dom, cookie_out))
    }
    fn domain_migrate_confirm3(
        &mut self,
        dom: RemoteNonnullDomain,
        cookie_in: Vec<u8>,
        flags: u64,
        cancelled: i32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_confirm3));
        let req: Option<RemoteDomainMigrateConfirm3Args> = Some(RemoteDomainMigrateConfirm3Args {
            dom,
            cookie_in,
            flags,
            cancelled,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainMigrateConfirm3, req)?;
        Ok(())
    }
    fn domain_set_scheduler_parameters_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_scheduler_parameters_flags));
        let req: Option<RemoteDomainSetSchedulerParametersFlagsArgs> =
            Some(RemoteDomainSetSchedulerParametersFlagsArgs { dom, params, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetSchedulerParametersFlags,
            req,
        )?;
        Ok(())
    }
    fn interface_change_begin(&mut self, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_begin));
        let req: Option<RemoteInterfaceChangeBeginArgs> =
            Some(RemoteInterfaceChangeBeginArgs { flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcInterfaceChangeBegin, req)?;
        Ok(())
    }
    fn interface_change_commit(&mut self, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_commit));
        let req: Option<RemoteInterfaceChangeCommitArgs> =
            Some(RemoteInterfaceChangeCommitArgs { flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcInterfaceChangeCommit, req)?;
        Ok(())
    }
    fn interface_change_rollback(&mut self, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_rollback));
        let req: Option<RemoteInterfaceChangeRollbackArgs> =
            Some(RemoteInterfaceChangeRollbackArgs { flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcInterfaceChangeRollback,
            req,
        )?;
        Ok(())
    }
    fn domain_get_scheduler_parameters_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        nparams: i32,
        flags: u32,
    ) -> Result<Vec<RemoteTypedParam>, Error> {
        trace!("{}", stringify!(domain_get_scheduler_parameters_flags));
        let req: Option<RemoteDomainGetSchedulerParametersFlagsArgs> =
            Some(RemoteDomainGetSchedulerParametersFlagsArgs {
                dom,
                nparams,
                flags,
            });
        let res: Option<RemoteDomainGetSchedulerParametersFlagsRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainGetSchedulerParametersFlags,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetSchedulerParametersFlagsRet { params } = res;
        Ok(params)
    }
    fn domain_event_control_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_control_error));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventControlError,
            req,
        )?;
        Ok(())
    }
    fn domain_pin_vcpu_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        vcpu: u32,
        cpumap: Vec<u8>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pin_vcpu_flags));
        let req: Option<RemoteDomainPinVcpuFlagsArgs> = Some(RemoteDomainPinVcpuFlagsArgs {
            dom,
            vcpu,
            cpumap,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainPinVcpuFlags, req)?;
        Ok(())
    }
    fn domain_send_key(
        &mut self,
        dom: RemoteNonnullDomain,
        codeset: u32,
        holdtime: u32,
        keycodes: Vec<u32>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_send_key));
        let req: Option<RemoteDomainSendKeyArgs> = Some(RemoteDomainSendKeyArgs {
            dom,
            codeset,
            holdtime,
            keycodes,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSendKey, req)?;
        Ok(())
    }
    fn node_get_cpu_stats(
        &mut self,
        cpu_num: i32,
        nparams: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNodeGetCpuStats>, i32), Error> {
        trace!("{}", stringify!(node_get_cpu_stats));
        let req: Option<RemoteNodeGetCpuStatsArgs> = Some(RemoteNodeGetCpuStatsArgs {
            cpu_num,
            nparams,
            flags,
        });
        let res: Option<RemoteNodeGetCpuStatsRet> =
            call(self, RemoteProcedure::RemoteProcNodeGetCpuStats, req)?;
        let res = res.unwrap();
        let RemoteNodeGetCpuStatsRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn node_get_memory_stats(
        &mut self,
        nparams: i32,
        cell_num: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNodeGetMemoryStats>, i32), Error> {
        trace!("{}", stringify!(node_get_memory_stats));
        let req: Option<RemoteNodeGetMemoryStatsArgs> = Some(RemoteNodeGetMemoryStatsArgs {
            nparams,
            cell_num,
            flags,
        });
        let res: Option<RemoteNodeGetMemoryStatsRet> =
            call(self, RemoteProcedure::RemoteProcNodeGetMemoryStats, req)?;
        let res = res.unwrap();
        let RemoteNodeGetMemoryStatsRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_get_control_info(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<(u32, u32, u64), Error> {
        trace!("{}", stringify!(domain_get_control_info));
        let req: Option<RemoteDomainGetControlInfoArgs> =
            Some(RemoteDomainGetControlInfoArgs { dom, flags });
        let res: Option<RemoteDomainGetControlInfoRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetControlInfo, req)?;
        let res = res.unwrap();
        let RemoteDomainGetControlInfoRet {
            state,
            details,
            state_time,
        } = res;
        Ok((state, details, state_time))
    }
    fn domain_get_vcpu_pin_info(
        &mut self,
        dom: RemoteNonnullDomain,
        ncpumaps: i32,
        maplen: i32,
        flags: u32,
    ) -> Result<(Vec<u8>, i32), Error> {
        trace!("{}", stringify!(domain_get_vcpu_pin_info));
        let req: Option<RemoteDomainGetVcpuPinInfoArgs> = Some(RemoteDomainGetVcpuPinInfoArgs {
            dom,
            ncpumaps,
            maplen,
            flags,
        });
        let res: Option<RemoteDomainGetVcpuPinInfoRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetVcpuPinInfo, req)?;
        let res = res.unwrap();
        let RemoteDomainGetVcpuPinInfoRet { cpumaps, num } = res;
        Ok((cpumaps, num))
    }
    fn domain_undefine_flags(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_undefine_flags));
        let req: Option<RemoteDomainUndefineFlagsArgs> =
            Some(RemoteDomainUndefineFlagsArgs { dom, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainUndefineFlags, req)?;
        Ok(())
    }
    fn domain_save_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        to: String,
        dxml: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_save_flags));
        let req: Option<RemoteDomainSaveFlagsArgs> = Some(RemoteDomainSaveFlagsArgs {
            dom,
            to,
            dxml,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSaveFlags, req)?;
        Ok(())
    }
    fn domain_restore_flags(
        &mut self,
        from: String,
        dxml: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_restore_flags));
        let req: Option<RemoteDomainRestoreFlagsArgs> =
            Some(RemoteDomainRestoreFlagsArgs { from, dxml, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainRestoreFlags, req)?;
        Ok(())
    }
    fn domain_destroy_flags(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_destroy_flags));
        let req: Option<RemoteDomainDestroyFlagsArgs> =
            Some(RemoteDomainDestroyFlagsArgs { dom, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainDestroyFlags, req)?;
        Ok(())
    }
    fn domain_save_image_get_xml_desc(
        &mut self,
        file: String,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(domain_save_image_get_xml_desc));
        let req: Option<RemoteDomainSaveImageGetXmlDescArgs> =
            Some(RemoteDomainSaveImageGetXmlDescArgs { file, flags });
        let res: Option<RemoteDomainSaveImageGetXmlDescRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSaveImageGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSaveImageGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_save_image_define_xml(
        &mut self,
        file: String,
        dxml: String,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_save_image_define_xml));
        let req: Option<RemoteDomainSaveImageDefineXmlArgs> =
            Some(RemoteDomainSaveImageDefineXmlArgs { file, dxml, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSaveImageDefineXml,
            req,
        )?;
        Ok(())
    }
    fn domain_block_job_abort(
        &mut self,
        dom: RemoteNonnullDomain,
        path: String,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_job_abort));
        let req: Option<RemoteDomainBlockJobAbortArgs> =
            Some(RemoteDomainBlockJobAbortArgs { dom, path, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainBlockJobAbort, req)?;
        Ok(())
    }
    fn domain_get_block_job_info(
        &mut self,
        dom: RemoteNonnullDomain,
        path: String,
        flags: u32,
    ) -> Result<(i32, i32, u64, u64, u64), Error> {
        trace!("{}", stringify!(domain_get_block_job_info));
        let req: Option<RemoteDomainGetBlockJobInfoArgs> =
            Some(RemoteDomainGetBlockJobInfoArgs { dom, path, flags });
        let res: Option<RemoteDomainGetBlockJobInfoRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetBlockJobInfo, req)?;
        let res = res.unwrap();
        let RemoteDomainGetBlockJobInfoRet {
            found,
            r#type,
            bandwidth,
            cur,
            end,
        } = res;
        Ok((found, r#type, bandwidth, cur, end))
    }
    fn domain_block_job_set_speed(
        &mut self,
        dom: RemoteNonnullDomain,
        path: String,
        bandwidth: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_job_set_speed));
        let req: Option<RemoteDomainBlockJobSetSpeedArgs> =
            Some(RemoteDomainBlockJobSetSpeedArgs {
                dom,
                path,
                bandwidth,
                flags,
            });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainBlockJobSetSpeed, req)?;
        Ok(())
    }
    fn domain_block_pull(
        &mut self,
        dom: RemoteNonnullDomain,
        path: String,
        bandwidth: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_pull));
        let req: Option<RemoteDomainBlockPullArgs> = Some(RemoteDomainBlockPullArgs {
            dom,
            path,
            bandwidth,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainBlockPull, req)?;
        Ok(())
    }
    fn domain_event_block_job(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_job));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventBlockJob, req)?;
        Ok(())
    }
    fn domain_migrate_get_max_speed(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<u64, Error> {
        trace!("{}", stringify!(domain_migrate_get_max_speed));
        let req: Option<RemoteDomainMigrateGetMaxSpeedArgs> =
            Some(RemoteDomainMigrateGetMaxSpeedArgs { dom, flags });
        let res: Option<RemoteDomainMigrateGetMaxSpeedRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigrateGetMaxSpeed,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMigrateGetMaxSpeedRet { bandwidth } = res;
        Ok(bandwidth)
    }
    fn domain_block_stats_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        path: String,
        nparams: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteTypedParam>, i32), Error> {
        trace!("{}", stringify!(domain_block_stats_flags));
        let req: Option<RemoteDomainBlockStatsFlagsArgs> = Some(RemoteDomainBlockStatsFlagsArgs {
            dom,
            path,
            nparams,
            flags,
        });
        let res: Option<RemoteDomainBlockStatsFlagsRet> =
            call(self, RemoteProcedure::RemoteProcDomainBlockStatsFlags, req)?;
        let res = res.unwrap();
        let RemoteDomainBlockStatsFlagsRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_snapshot_get_parent(
        &mut self,
        snap: RemoteNonnullDomainSnapshot,
        flags: u32,
    ) -> Result<RemoteNonnullDomainSnapshot, Error> {
        trace!("{}", stringify!(domain_snapshot_get_parent));
        let req: Option<RemoteDomainSnapshotGetParentArgs> =
            Some(RemoteDomainSnapshotGetParentArgs { snap, flags });
        let res: Option<RemoteDomainSnapshotGetParentRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotGetParent,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSnapshotGetParentRet { snap } = res;
        Ok(snap)
    }
    fn domain_reset(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_reset));
        let req: Option<RemoteDomainResetArgs> = Some(RemoteDomainResetArgs { dom, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainReset, req)?;
        Ok(())
    }
    fn domain_snapshot_num_children(
        &mut self,
        snap: RemoteNonnullDomainSnapshot,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_snapshot_num_children));
        let req: Option<RemoteDomainSnapshotNumChildrenArgs> =
            Some(RemoteDomainSnapshotNumChildrenArgs { snap, flags });
        let res: Option<RemoteDomainSnapshotNumChildrenRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotNumChildren,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSnapshotNumChildrenRet { num } = res;
        Ok(num)
    }
    fn domain_snapshot_list_children_names(
        &mut self,
        snap: RemoteNonnullDomainSnapshot,
        maxnames: i32,
        flags: u32,
    ) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(domain_snapshot_list_children_names));
        let req: Option<RemoteDomainSnapshotListChildrenNamesArgs> =
            Some(RemoteDomainSnapshotListChildrenNamesArgs {
                snap,
                maxnames,
                flags,
            });
        let res: Option<RemoteDomainSnapshotListChildrenNamesRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotListChildrenNames,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSnapshotListChildrenNamesRet { names } = res;
        Ok(names)
    }
    fn domain_event_disk_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_disk_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventDiskChange, req)?;
        Ok(())
    }
    fn domain_open_graphics(
        &mut self,
        dom: RemoteNonnullDomain,
        idx: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_open_graphics));
        let req: Option<RemoteDomainOpenGraphicsArgs> =
            Some(RemoteDomainOpenGraphicsArgs { dom, idx, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainOpenGraphics, req)?;
        Ok(())
    }
    fn node_suspend_for_duration(
        &mut self,
        target: u32,
        duration: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_suspend_for_duration));
        let req: Option<RemoteNodeSuspendForDurationArgs> =
            Some(RemoteNodeSuspendForDurationArgs {
                target,
                duration,
                flags,
            });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNodeSuspendForDuration, req)?;
        Ok(())
    }
    fn domain_block_resize(
        &mut self,
        dom: RemoteNonnullDomain,
        disk: String,
        size: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_resize));
        let req: Option<RemoteDomainBlockResizeArgs> = Some(RemoteDomainBlockResizeArgs {
            dom,
            disk,
            size,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainBlockResize, req)?;
        Ok(())
    }
    fn domain_set_block_io_tune(
        &mut self,
        dom: RemoteNonnullDomain,
        disk: String,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_block_io_tune));
        let req: Option<RemoteDomainSetBlockIoTuneArgs> = Some(RemoteDomainSetBlockIoTuneArgs {
            dom,
            disk,
            params,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetBlockIoTune, req)?;
        Ok(())
    }
    fn domain_get_block_io_tune(
        &mut self,
        dom: RemoteNonnullDomain,
        disk: Option<String>,
        nparams: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteTypedParam>, i32), Error> {
        trace!("{}", stringify!(domain_get_block_io_tune));
        let req: Option<RemoteDomainGetBlockIoTuneArgs> = Some(RemoteDomainGetBlockIoTuneArgs {
            dom,
            disk,
            nparams,
            flags,
        });
        let res: Option<RemoteDomainGetBlockIoTuneRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetBlockIoTune, req)?;
        let res = res.unwrap();
        let RemoteDomainGetBlockIoTuneRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_set_numa_parameters(
        &mut self,
        dom: RemoteNonnullDomain,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_numa_parameters));
        let req: Option<RemoteDomainSetNumaParametersArgs> =
            Some(RemoteDomainSetNumaParametersArgs { dom, params, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetNumaParameters,
            req,
        )?;
        Ok(())
    }
    fn domain_get_numa_parameters(
        &mut self,
        dom: RemoteNonnullDomain,
        nparams: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteTypedParam>, i32), Error> {
        trace!("{}", stringify!(domain_get_numa_parameters));
        let req: Option<RemoteDomainGetNumaParametersArgs> =
            Some(RemoteDomainGetNumaParametersArgs {
                dom,
                nparams,
                flags,
            });
        let res: Option<RemoteDomainGetNumaParametersRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainGetNumaParameters,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetNumaParametersRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_set_interface_parameters(
        &mut self,
        dom: RemoteNonnullDomain,
        device: String,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_interface_parameters));
        let req: Option<RemoteDomainSetInterfaceParametersArgs> =
            Some(RemoteDomainSetInterfaceParametersArgs {
                dom,
                device,
                params,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetInterfaceParameters,
            req,
        )?;
        Ok(())
    }
    fn domain_get_interface_parameters(
        &mut self,
        dom: RemoteNonnullDomain,
        device: String,
        nparams: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteTypedParam>, i32), Error> {
        trace!("{}", stringify!(domain_get_interface_parameters));
        let req: Option<RemoteDomainGetInterfaceParametersArgs> =
            Some(RemoteDomainGetInterfaceParametersArgs {
                dom,
                device,
                nparams,
                flags,
            });
        let res: Option<RemoteDomainGetInterfaceParametersRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainGetInterfaceParameters,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetInterfaceParametersRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_shutdown_flags(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_shutdown_flags));
        let req: Option<RemoteDomainShutdownFlagsArgs> =
            Some(RemoteDomainShutdownFlagsArgs { dom, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainShutdownFlags, req)?;
        Ok(())
    }
    fn storage_vol_wipe_pattern(
        &mut self,
        vol: RemoteNonnullStorageVol,
        algorithm: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_wipe_pattern));
        let req: Option<RemoteStorageVolWipePatternArgs> = Some(RemoteStorageVolWipePatternArgs {
            vol,
            algorithm,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStorageVolWipePattern, req)?;
        Ok(())
    }
    fn storage_vol_resize(
        &mut self,
        vol: RemoteNonnullStorageVol,
        capacity: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_resize));
        let req: Option<RemoteStorageVolResizeArgs> = Some(RemoteStorageVolResizeArgs {
            vol,
            capacity,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcStorageVolResize, req)?;
        Ok(())
    }
    fn domain_pm_suspend_for_duration(
        &mut self,
        dom: RemoteNonnullDomain,
        target: u32,
        duration: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pm_suspend_for_duration));
        let req: Option<RemoteDomainPmSuspendForDurationArgs> =
            Some(RemoteDomainPmSuspendForDurationArgs {
                dom,
                target,
                duration,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainPmSuspendForDuration,
            req,
        )?;
        Ok(())
    }
    fn domain_get_cpu_stats(
        &mut self,
        dom: RemoteNonnullDomain,
        nparams: u32,
        start_cpu: i32,
        ncpus: u32,
        flags: u32,
    ) -> Result<(Vec<RemoteTypedParam>, i32), Error> {
        trace!("{}", stringify!(domain_get_cpu_stats));
        let req: Option<RemoteDomainGetCpuStatsArgs> = Some(RemoteDomainGetCpuStatsArgs {
            dom,
            nparams,
            start_cpu,
            ncpus,
            flags,
        });
        let res: Option<RemoteDomainGetCpuStatsRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetCpuStats, req)?;
        let res = res.unwrap();
        let RemoteDomainGetCpuStatsRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_get_disk_errors(
        &mut self,
        dom: RemoteNonnullDomain,
        maxerrors: u32,
        flags: u32,
    ) -> Result<(Vec<RemoteDomainDiskError>, i32), Error> {
        trace!("{}", stringify!(domain_get_disk_errors));
        let req: Option<RemoteDomainGetDiskErrorsArgs> = Some(RemoteDomainGetDiskErrorsArgs {
            dom,
            maxerrors,
            flags,
        });
        let res: Option<RemoteDomainGetDiskErrorsRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetDiskErrors, req)?;
        let res = res.unwrap();
        let RemoteDomainGetDiskErrorsRet { errors, nerrors } = res;
        Ok((errors, nerrors))
    }
    fn domain_set_metadata(
        &mut self,
        dom: RemoteNonnullDomain,
        r#type: i32,
        metadata: Option<String>,
        key: Option<String>,
        uri: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_metadata));
        let req: Option<RemoteDomainSetMetadataArgs> = Some(RemoteDomainSetMetadataArgs {
            dom,
            r#type,
            metadata,
            key,
            uri,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetMetadata, req)?;
        Ok(())
    }
    fn domain_get_metadata(
        &mut self,
        dom: RemoteNonnullDomain,
        r#type: i32,
        uri: Option<String>,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(domain_get_metadata));
        let req: Option<RemoteDomainGetMetadataArgs> = Some(RemoteDomainGetMetadataArgs {
            dom,
            r#type,
            uri,
            flags,
        });
        let res: Option<RemoteDomainGetMetadataRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetMetadata, req)?;
        let res = res.unwrap();
        let RemoteDomainGetMetadataRet { metadata } = res;
        Ok(metadata)
    }
    fn domain_block_rebase(
        &mut self,
        dom: RemoteNonnullDomain,
        path: String,
        base: Option<String>,
        bandwidth: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_rebase));
        let req: Option<RemoteDomainBlockRebaseArgs> = Some(RemoteDomainBlockRebaseArgs {
            dom,
            path,
            base,
            bandwidth,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainBlockRebase, req)?;
        Ok(())
    }
    fn domain_pm_wakeup(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pm_wakeup));
        let req: Option<RemoteDomainPmWakeupArgs> = Some(RemoteDomainPmWakeupArgs { dom, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainPmWakeup, req)?;
        Ok(())
    }
    fn domain_event_tray_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_tray_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventTrayChange, req)?;
        Ok(())
    }
    fn domain_event_pmwakeup(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmwakeup));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventPmwakeup, req)?;
        Ok(())
    }
    fn domain_event_pmsuspend(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmsuspend));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventPmsuspend, req)?;
        Ok(())
    }
    fn domain_snapshot_is_current(
        &mut self,
        snap: RemoteNonnullDomainSnapshot,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_snapshot_is_current));
        let req: Option<RemoteDomainSnapshotIsCurrentArgs> =
            Some(RemoteDomainSnapshotIsCurrentArgs { snap, flags });
        let res: Option<RemoteDomainSnapshotIsCurrentRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotIsCurrent,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSnapshotIsCurrentRet { current } = res;
        Ok(current)
    }
    fn domain_snapshot_has_metadata(
        &mut self,
        snap: RemoteNonnullDomainSnapshot,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_snapshot_has_metadata));
        let req: Option<RemoteDomainSnapshotHasMetadataArgs> =
            Some(RemoteDomainSnapshotHasMetadataArgs { snap, flags });
        let res: Option<RemoteDomainSnapshotHasMetadataRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotHasMetadata,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSnapshotHasMetadataRet { metadata } = res;
        Ok(metadata)
    }
    fn connect_list_all_domains(
        &mut self,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullDomain>, u32), Error> {
        trace!("{}", stringify!(connect_list_all_domains));
        let req: Option<RemoteConnectListAllDomainsArgs> = Some(RemoteConnectListAllDomainsArgs {
            need_results,
            flags,
        });
        let res: Option<RemoteConnectListAllDomainsRet> =
            call(self, RemoteProcedure::RemoteProcConnectListAllDomains, req)?;
        let res = res.unwrap();
        let RemoteConnectListAllDomainsRet { domains, ret } = res;
        Ok((domains, ret))
    }
    fn domain_list_all_snapshots(
        &mut self,
        dom: RemoteNonnullDomain,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullDomainSnapshot>, i32), Error> {
        trace!("{}", stringify!(domain_list_all_snapshots));
        let req: Option<RemoteDomainListAllSnapshotsArgs> =
            Some(RemoteDomainListAllSnapshotsArgs {
                dom,
                need_results,
                flags,
            });
        let res: Option<RemoteDomainListAllSnapshotsRet> =
            call(self, RemoteProcedure::RemoteProcDomainListAllSnapshots, req)?;
        let res = res.unwrap();
        let RemoteDomainListAllSnapshotsRet { snapshots, ret } = res;
        Ok((snapshots, ret))
    }
    fn domain_snapshot_list_all_children(
        &mut self,
        snapshot: RemoteNonnullDomainSnapshot,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullDomainSnapshot>, i32), Error> {
        trace!("{}", stringify!(domain_snapshot_list_all_children));
        let req: Option<RemoteDomainSnapshotListAllChildrenArgs> =
            Some(RemoteDomainSnapshotListAllChildrenArgs {
                snapshot,
                need_results,
                flags,
            });
        let res: Option<RemoteDomainSnapshotListAllChildrenRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotListAllChildren,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainSnapshotListAllChildrenRet { snapshots, ret } = res;
        Ok((snapshots, ret))
    }
    fn domain_event_balloon_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_balloon_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventBalloonChange,
            req,
        )?;
        Ok(())
    }
    fn domain_get_hostname(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(domain_get_hostname));
        let req: Option<RemoteDomainGetHostnameArgs> =
            Some(RemoteDomainGetHostnameArgs { dom, flags });
        let res: Option<RemoteDomainGetHostnameRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetHostname, req)?;
        let res = res.unwrap();
        let RemoteDomainGetHostnameRet { hostname } = res;
        Ok(hostname)
    }
    fn domain_get_security_label_list(
        &mut self,
        dom: RemoteNonnullDomain,
    ) -> Result<(Vec<RemoteDomainGetSecurityLabelRet>, i32), Error> {
        trace!("{}", stringify!(domain_get_security_label_list));
        let req: Option<RemoteDomainGetSecurityLabelListArgs> =
            Some(RemoteDomainGetSecurityLabelListArgs { dom });
        let res: Option<RemoteDomainGetSecurityLabelListRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainGetSecurityLabelList,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetSecurityLabelListRet { labels, ret } = res;
        Ok((labels, ret))
    }
    fn domain_pin_emulator(
        &mut self,
        dom: RemoteNonnullDomain,
        cpumap: Vec<u8>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pin_emulator));
        let req: Option<RemoteDomainPinEmulatorArgs> =
            Some(RemoteDomainPinEmulatorArgs { dom, cpumap, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainPinEmulator, req)?;
        Ok(())
    }
    fn domain_get_emulator_pin_info(
        &mut self,
        dom: RemoteNonnullDomain,
        maplen: i32,
        flags: u32,
    ) -> Result<(Vec<u8>, i32), Error> {
        trace!("{}", stringify!(domain_get_emulator_pin_info));
        let req: Option<RemoteDomainGetEmulatorPinInfoArgs> =
            Some(RemoteDomainGetEmulatorPinInfoArgs { dom, maplen, flags });
        let res: Option<RemoteDomainGetEmulatorPinInfoRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainGetEmulatorPinInfo,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetEmulatorPinInfoRet { cpumaps, ret } = res;
        Ok((cpumaps, ret))
    }
    fn connect_list_all_storage_pools(
        &mut self,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullStoragePool>, u32), Error> {
        trace!("{}", stringify!(connect_list_all_storage_pools));
        let req: Option<RemoteConnectListAllStoragePoolsArgs> =
            Some(RemoteConnectListAllStoragePoolsArgs {
                need_results,
                flags,
            });
        let res: Option<RemoteConnectListAllStoragePoolsRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectListAllStoragePools,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListAllStoragePoolsRet { pools, ret } = res;
        Ok((pools, ret))
    }
    fn storage_pool_list_all_volumes(
        &mut self,
        pool: RemoteNonnullStoragePool,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullStorageVol>, u32), Error> {
        trace!("{}", stringify!(storage_pool_list_all_volumes));
        let req: Option<RemoteStoragePoolListAllVolumesArgs> =
            Some(RemoteStoragePoolListAllVolumesArgs {
                pool,
                need_results,
                flags,
            });
        let res: Option<RemoteStoragePoolListAllVolumesRet> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolListAllVolumes,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStoragePoolListAllVolumesRet { vols, ret } = res;
        Ok((vols, ret))
    }
    fn connect_list_all_networks(
        &mut self,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullNetwork>, u32), Error> {
        trace!("{}", stringify!(connect_list_all_networks));
        let req: Option<RemoteConnectListAllNetworksArgs> =
            Some(RemoteConnectListAllNetworksArgs {
                need_results,
                flags,
            });
        let res: Option<RemoteConnectListAllNetworksRet> =
            call(self, RemoteProcedure::RemoteProcConnectListAllNetworks, req)?;
        let res = res.unwrap();
        let RemoteConnectListAllNetworksRet { nets, ret } = res;
        Ok((nets, ret))
    }
    fn connect_list_all_interfaces(
        &mut self,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullInterface>, u32), Error> {
        trace!("{}", stringify!(connect_list_all_interfaces));
        let req: Option<RemoteConnectListAllInterfacesArgs> =
            Some(RemoteConnectListAllInterfacesArgs {
                need_results,
                flags,
            });
        let res: Option<RemoteConnectListAllInterfacesRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectListAllInterfaces,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListAllInterfacesRet { ifaces, ret } = res;
        Ok((ifaces, ret))
    }
    fn connect_list_all_node_devices(
        &mut self,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullNodeDevice>, u32), Error> {
        trace!("{}", stringify!(connect_list_all_node_devices));
        let req: Option<RemoteConnectListAllNodeDevicesArgs> =
            Some(RemoteConnectListAllNodeDevicesArgs {
                need_results,
                flags,
            });
        let res: Option<RemoteConnectListAllNodeDevicesRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectListAllNodeDevices,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListAllNodeDevicesRet { devices, ret } = res;
        Ok((devices, ret))
    }
    fn connect_list_all_nwfilters(
        &mut self,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullNwfilter>, u32), Error> {
        trace!("{}", stringify!(connect_list_all_nwfilters));
        let req: Option<RemoteConnectListAllNwfiltersArgs> =
            Some(RemoteConnectListAllNwfiltersArgs {
                need_results,
                flags,
            });
        let res: Option<RemoteConnectListAllNwfiltersRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectListAllNwfilters,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListAllNwfiltersRet { filters, ret } = res;
        Ok((filters, ret))
    }
    fn connect_list_all_secrets(
        &mut self,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullSecret>, u32), Error> {
        trace!("{}", stringify!(connect_list_all_secrets));
        let req: Option<RemoteConnectListAllSecretsArgs> = Some(RemoteConnectListAllSecretsArgs {
            need_results,
            flags,
        });
        let res: Option<RemoteConnectListAllSecretsRet> =
            call(self, RemoteProcedure::RemoteProcConnectListAllSecrets, req)?;
        let res = res.unwrap();
        let RemoteConnectListAllSecretsRet { secrets, ret } = res;
        Ok((secrets, ret))
    }
    fn node_set_memory_parameters(
        &mut self,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_set_memory_parameters));
        let req: Option<RemoteNodeSetMemoryParametersArgs> =
            Some(RemoteNodeSetMemoryParametersArgs { params, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcNodeSetMemoryParameters,
            req,
        )?;
        Ok(())
    }
    fn node_get_memory_parameters(
        &mut self,
        nparams: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteTypedParam>, i32), Error> {
        trace!("{}", stringify!(node_get_memory_parameters));
        let req: Option<RemoteNodeGetMemoryParametersArgs> =
            Some(RemoteNodeGetMemoryParametersArgs { nparams, flags });
        let res: Option<RemoteNodeGetMemoryParametersRet> = call(
            self,
            RemoteProcedure::RemoteProcNodeGetMemoryParameters,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNodeGetMemoryParametersRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_block_commit(
        &mut self,
        dom: RemoteNonnullDomain,
        disk: String,
        base: Option<String>,
        top: Option<String>,
        bandwidth: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_commit));
        let req: Option<RemoteDomainBlockCommitArgs> = Some(RemoteDomainBlockCommitArgs {
            dom,
            disk,
            base,
            top,
            bandwidth,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainBlockCommit, req)?;
        Ok(())
    }
    fn network_update(
        &mut self,
        net: RemoteNonnullNetwork,
        command: u32,
        section: u32,
        parent_index: i32,
        xml: String,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(network_update));
        let req: Option<RemoteNetworkUpdateArgs> = Some(RemoteNetworkUpdateArgs {
            net,
            command,
            section,
            parent_index,
            xml,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNetworkUpdate, req)?;
        Ok(())
    }
    fn domain_event_pmsuspend_disk(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmsuspend_disk));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventPmsuspendDisk,
            req,
        )?;
        Ok(())
    }
    fn node_get_cpu_map(
        &mut self,
        need_map: i32,
        need_online: i32,
        flags: u32,
    ) -> Result<(Vec<u8>, u32, i32), Error> {
        trace!("{}", stringify!(node_get_cpu_map));
        let req: Option<RemoteNodeGetCpuMapArgs> = Some(RemoteNodeGetCpuMapArgs {
            need_map,
            need_online,
            flags,
        });
        let res: Option<RemoteNodeGetCpuMapRet> =
            call(self, RemoteProcedure::RemoteProcNodeGetCpuMap, req)?;
        let res = res.unwrap();
        let RemoteNodeGetCpuMapRet {
            cpumap,
            online,
            ret,
        } = res;
        Ok((cpumap, online, ret))
    }
    fn domain_fstrim(
        &mut self,
        dom: RemoteNonnullDomain,
        mount_point: Option<String>,
        minimum: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_fstrim));
        let req: Option<RemoteDomainFstrimArgs> = Some(RemoteDomainFstrimArgs {
            dom,
            mount_point,
            minimum,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainFstrim, req)?;
        Ok(())
    }
    fn domain_send_process_signal(
        &mut self,
        dom: RemoteNonnullDomain,
        pid_value: i64,
        signum: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_send_process_signal));
        let req: Option<RemoteDomainSendProcessSignalArgs> =
            Some(RemoteDomainSendProcessSignalArgs {
                dom,
                pid_value,
                signum,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSendProcessSignal,
            req,
        )?;
        Ok(())
    }
    fn domain_open_channel(
        &mut self,
        dom: RemoteNonnullDomain,
        name: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_open_channel));
        let req: Option<RemoteDomainOpenChannelArgs> =
            Some(RemoteDomainOpenChannelArgs { dom, name, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainOpenChannel, req)?;
        Ok(())
    }
    fn node_device_lookup_scsi_host_by_wwn(
        &mut self,
        wwnn: String,
        wwpn: String,
        flags: u32,
    ) -> Result<RemoteNonnullNodeDevice, Error> {
        trace!("{}", stringify!(node_device_lookup_scsi_host_by_wwn));
        let req: Option<RemoteNodeDeviceLookupScsiHostByWwnArgs> =
            Some(RemoteNodeDeviceLookupScsiHostByWwnArgs { wwnn, wwpn, flags });
        let res: Option<RemoteNodeDeviceLookupScsiHostByWwnRet> = call(
            self,
            RemoteProcedure::RemoteProcNodeDeviceLookupScsiHostByWwn,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNodeDeviceLookupScsiHostByWwnRet { dev } = res;
        Ok(dev)
    }
    fn domain_get_job_stats(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<(i32, Vec<RemoteTypedParam>), Error> {
        trace!("{}", stringify!(domain_get_job_stats));
        let req: Option<RemoteDomainGetJobStatsArgs> =
            Some(RemoteDomainGetJobStatsArgs { dom, flags });
        let res: Option<RemoteDomainGetJobStatsRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetJobStats, req)?;
        let res = res.unwrap();
        let RemoteDomainGetJobStatsRet { r#type, params } = res;
        Ok((r#type, params))
    }
    fn domain_migrate_get_compression_cache(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<u64, Error> {
        trace!("{}", stringify!(domain_migrate_get_compression_cache));
        let req: Option<RemoteDomainMigrateGetCompressionCacheArgs> =
            Some(RemoteDomainMigrateGetCompressionCacheArgs { dom, flags });
        let res: Option<RemoteDomainMigrateGetCompressionCacheRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigrateGetCompressionCache,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMigrateGetCompressionCacheRet { cache_size } = res;
        Ok(cache_size)
    }
    fn domain_migrate_set_compression_cache(
        &mut self,
        dom: RemoteNonnullDomain,
        cache_size: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_set_compression_cache));
        let req: Option<RemoteDomainMigrateSetCompressionCacheArgs> =
            Some(RemoteDomainMigrateSetCompressionCacheArgs {
                dom,
                cache_size,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigrateSetCompressionCache,
            req,
        )?;
        Ok(())
    }
    fn node_device_detach_flags(
        &mut self,
        name: String,
        driver_name: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_detach_flags));
        let req: Option<RemoteNodeDeviceDetachFlagsArgs> = Some(RemoteNodeDeviceDetachFlagsArgs {
            name,
            driver_name,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNodeDeviceDetachFlags, req)?;
        Ok(())
    }
    fn domain_migrate_begin3_params(
        &mut self,
        dom: RemoteNonnullDomain,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(Vec<u8>, String), Error> {
        trace!("{}", stringify!(domain_migrate_begin3_params));
        let req: Option<RemoteDomainMigrateBegin3ParamsArgs> =
            Some(RemoteDomainMigrateBegin3ParamsArgs { dom, params, flags });
        let res: Option<RemoteDomainMigrateBegin3ParamsRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigrateBegin3Params,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMigrateBegin3ParamsRet { cookie_out, xml } = res;
        Ok((cookie_out, xml))
    }
    fn domain_migrate_prepare3_params(
        &mut self,
        params: Vec<RemoteTypedParam>,
        cookie_in: Vec<u8>,
        flags: u32,
    ) -> Result<(Vec<u8>, Option<String>), Error> {
        trace!("{}", stringify!(domain_migrate_prepare3_params));
        let req: Option<RemoteDomainMigratePrepare3ParamsArgs> =
            Some(RemoteDomainMigratePrepare3ParamsArgs {
                params,
                cookie_in,
                flags,
            });
        let res: Option<RemoteDomainMigratePrepare3ParamsRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigratePrepare3Params,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMigratePrepare3ParamsRet {
            cookie_out,
            uri_out,
        } = res;
        Ok((cookie_out, uri_out))
    }
    fn domain_migrate_prepare_tunnel3_params(
        &mut self,
        params: Vec<RemoteTypedParam>,
        cookie_in: Vec<u8>,
        flags: u32,
    ) -> Result<Vec<u8>, Error> {
        trace!("{}", stringify!(domain_migrate_prepare_tunnel3_params));
        let req: Option<RemoteDomainMigratePrepareTunnel3ParamsArgs> =
            Some(RemoteDomainMigratePrepareTunnel3ParamsArgs {
                params,
                cookie_in,
                flags,
            });
        let res: Option<RemoteDomainMigratePrepareTunnel3ParamsRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigratePrepareTunnel3Params,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMigratePrepareTunnel3ParamsRet { cookie_out } = res;
        Ok(cookie_out)
    }
    fn domain_migrate_perform3_params(
        &mut self,
        dom: RemoteNonnullDomain,
        dconnuri: Option<String>,
        params: Vec<RemoteTypedParam>,
        cookie_in: Vec<u8>,
        flags: u32,
    ) -> Result<Vec<u8>, Error> {
        trace!("{}", stringify!(domain_migrate_perform3_params));
        let req: Option<RemoteDomainMigratePerform3ParamsArgs> =
            Some(RemoteDomainMigratePerform3ParamsArgs {
                dom,
                dconnuri,
                params,
                cookie_in,
                flags,
            });
        let res: Option<RemoteDomainMigratePerform3ParamsRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigratePerform3Params,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMigratePerform3ParamsRet { cookie_out } = res;
        Ok(cookie_out)
    }
    fn domain_migrate_finish3_params(
        &mut self,
        params: Vec<RemoteTypedParam>,
        cookie_in: Vec<u8>,
        flags: u32,
        cancelled: i32,
    ) -> Result<(RemoteNonnullDomain, Vec<u8>), Error> {
        trace!("{}", stringify!(domain_migrate_finish3_params));
        let req: Option<RemoteDomainMigrateFinish3ParamsArgs> =
            Some(RemoteDomainMigrateFinish3ParamsArgs {
                params,
                cookie_in,
                flags,
                cancelled,
            });
        let res: Option<RemoteDomainMigrateFinish3ParamsRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigrateFinish3Params,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMigrateFinish3ParamsRet { dom, cookie_out } = res;
        Ok((dom, cookie_out))
    }
    fn domain_migrate_confirm3_params(
        &mut self,
        dom: RemoteNonnullDomain,
        params: Vec<RemoteTypedParam>,
        cookie_in: Vec<u8>,
        flags: u32,
        cancelled: i32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_confirm3_params));
        let req: Option<RemoteDomainMigrateConfirm3ParamsArgs> =
            Some(RemoteDomainMigrateConfirm3ParamsArgs {
                dom,
                params,
                cookie_in,
                flags,
                cancelled,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigrateConfirm3Params,
            req,
        )?;
        Ok(())
    }
    fn domain_set_memory_stats_period(
        &mut self,
        dom: RemoteNonnullDomain,
        period: i32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_memory_stats_period));
        let req: Option<RemoteDomainSetMemoryStatsPeriodArgs> =
            Some(RemoteDomainSetMemoryStatsPeriodArgs { dom, period, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetMemoryStatsPeriod,
            req,
        )?;
        Ok(())
    }
    fn domain_create_xml_with_files(
        &mut self,
        xml_desc: String,
        flags: u32,
    ) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_create_xml_with_files));
        let req: Option<RemoteDomainCreateXmlWithFilesArgs> =
            Some(RemoteDomainCreateXmlWithFilesArgs { xml_desc, flags });
        let res: Option<RemoteDomainCreateXmlWithFilesRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainCreateXmlWithFiles,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainCreateXmlWithFilesRet { dom } = res;
        Ok(dom)
    }
    fn domain_create_with_files(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_create_with_files));
        let req: Option<RemoteDomainCreateWithFilesArgs> =
            Some(RemoteDomainCreateWithFilesArgs { dom, flags });
        let res: Option<RemoteDomainCreateWithFilesRet> =
            call(self, RemoteProcedure::RemoteProcDomainCreateWithFiles, req)?;
        let res = res.unwrap();
        let RemoteDomainCreateWithFilesRet { dom } = res;
        Ok(dom)
    }
    fn domain_event_device_removed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_device_removed));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventDeviceRemoved,
            req,
        )?;
        Ok(())
    }
    fn connect_get_cpu_model_names(
        &mut self,
        arch: String,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<String>, i32), Error> {
        trace!("{}", stringify!(connect_get_cpu_model_names));
        let req: Option<RemoteConnectGetCpuModelNamesArgs> =
            Some(RemoteConnectGetCpuModelNamesArgs {
                arch,
                need_results,
                flags,
            });
        let res: Option<RemoteConnectGetCpuModelNamesRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectGetCpuModelNames,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetCpuModelNamesRet { models, ret } = res;
        Ok((models, ret))
    }
    fn connect_network_event_register_any(
        &mut self,
        event_id: i32,
        net: Option<RemoteNonnullNetwork>,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_network_event_register_any));
        let req: Option<RemoteConnectNetworkEventRegisterAnyArgs> =
            Some(RemoteConnectNetworkEventRegisterAnyArgs { event_id, net });
        let res: Option<RemoteConnectNetworkEventRegisterAnyRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectNetworkEventRegisterAny,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNetworkEventRegisterAnyRet { callback_id } = res;
        Ok(callback_id)
    }
    fn connect_network_event_deregister_any(&mut self, callback_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_network_event_deregister_any));
        let req: Option<RemoteConnectNetworkEventDeregisterAnyArgs> =
            Some(RemoteConnectNetworkEventDeregisterAnyArgs { callback_id });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcConnectNetworkEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn network_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(network_event_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNetworkEventLifecycle, req)?;
        Ok(())
    }
    fn connect_domain_event_callback_register_any(
        &mut self,
        event_id: i32,
        dom: Option<RemoteNonnullDomain>,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_domain_event_callback_register_any));
        let req: Option<RemoteConnectDomainEventCallbackRegisterAnyArgs> =
            Some(RemoteConnectDomainEventCallbackRegisterAnyArgs { event_id, dom });
        let res: Option<RemoteConnectDomainEventCallbackRegisterAnyRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectDomainEventCallbackRegisterAny,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectDomainEventCallbackRegisterAnyRet { callback_id } = res;
        Ok(callback_id)
    }
    fn connect_domain_event_callback_deregister_any(
        &mut self,
        callback_id: i32,
    ) -> Result<(), Error> {
        trace!(
            "{}",
            stringify!(connect_domain_event_callback_deregister_any)
        );
        let req: Option<RemoteConnectDomainEventCallbackDeregisterAnyArgs> =
            Some(RemoteConnectDomainEventCallbackDeregisterAnyArgs { callback_id });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcConnectDomainEventCallbackDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackLifecycle,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_reboot(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_reboot));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackReboot,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_rtc_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_rtc_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackRtcChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_watchdog(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_watchdog));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackWatchdog,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_io_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackIoError,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_graphics(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_graphics));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackGraphics,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_io_error_reason(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error_reason));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackIoErrorReason,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_control_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_control_error));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackControlError,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_block_job(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_block_job));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackBlockJob,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_disk_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_disk_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackDiskChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_tray_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_tray_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackTrayChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmwakeup(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmwakeup));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackPmwakeup,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmsuspend(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackPmsuspend,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_balloon_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_balloon_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackBalloonChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmsuspend_disk(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend_disk));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackPmsuspendDisk,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_device_removed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_device_removed));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackDeviceRemoved,
            req,
        )?;
        Ok(())
    }
    fn domain_core_dump_with_format(
        &mut self,
        dom: RemoteNonnullDomain,
        to: String,
        dumpformat: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_core_dump_with_format));
        let req: Option<RemoteDomainCoreDumpWithFormatArgs> =
            Some(RemoteDomainCoreDumpWithFormatArgs {
                dom,
                to,
                dumpformat,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainCoreDumpWithFormat,
            req,
        )?;
        Ok(())
    }
    fn domain_fsfreeze(
        &mut self,
        dom: RemoteNonnullDomain,
        mountpoints: Vec<String>,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_fsfreeze));
        let req: Option<RemoteDomainFsfreezeArgs> = Some(RemoteDomainFsfreezeArgs {
            dom,
            mountpoints,
            flags,
        });
        let res: Option<RemoteDomainFsfreezeRet> =
            call(self, RemoteProcedure::RemoteProcDomainFsfreeze, req)?;
        let res = res.unwrap();
        let RemoteDomainFsfreezeRet { filesystems } = res;
        Ok(filesystems)
    }
    fn domain_fsthaw(
        &mut self,
        dom: RemoteNonnullDomain,
        mountpoints: Vec<String>,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_fsthaw));
        let req: Option<RemoteDomainFsthawArgs> = Some(RemoteDomainFsthawArgs {
            dom,
            mountpoints,
            flags,
        });
        let res: Option<RemoteDomainFsthawRet> =
            call(self, RemoteProcedure::RemoteProcDomainFsthaw, req)?;
        let res = res.unwrap();
        let RemoteDomainFsthawRet { filesystems } = res;
        Ok(filesystems)
    }
    fn domain_get_time(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<(i64, u32), Error> {
        trace!("{}", stringify!(domain_get_time));
        let req: Option<RemoteDomainGetTimeArgs> = Some(RemoteDomainGetTimeArgs { dom, flags });
        let res: Option<RemoteDomainGetTimeRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetTime, req)?;
        let res = res.unwrap();
        let RemoteDomainGetTimeRet { seconds, nseconds } = res;
        Ok((seconds, nseconds))
    }
    fn domain_set_time(
        &mut self,
        dom: RemoteNonnullDomain,
        seconds: i64,
        nseconds: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_time));
        let req: Option<RemoteDomainSetTimeArgs> = Some(RemoteDomainSetTimeArgs {
            dom,
            seconds,
            nseconds,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetTime, req)?;
        Ok(())
    }
    fn domain_event_block_job2(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_job2));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainEventBlockJob2, req)?;
        Ok(())
    }
    fn node_get_free_pages(
        &mut self,
        pages: Vec<u32>,
        start_cell: i32,
        cell_count: u32,
        flags: u32,
    ) -> Result<Vec<u64>, Error> {
        trace!("{}", stringify!(node_get_free_pages));
        let req: Option<RemoteNodeGetFreePagesArgs> = Some(RemoteNodeGetFreePagesArgs {
            pages,
            start_cell,
            cell_count,
            flags,
        });
        let res: Option<RemoteNodeGetFreePagesRet> =
            call(self, RemoteProcedure::RemoteProcNodeGetFreePages, req)?;
        let res = res.unwrap();
        let RemoteNodeGetFreePagesRet { counts } = res;
        Ok(counts)
    }
    fn network_get_dhcp_leases(
        &mut self,
        net: RemoteNonnullNetwork,
        mac: Option<String>,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNetworkDhcpLease>, u32), Error> {
        trace!("{}", stringify!(network_get_dhcp_leases));
        let req: Option<RemoteNetworkGetDhcpLeasesArgs> = Some(RemoteNetworkGetDhcpLeasesArgs {
            net,
            mac,
            need_results,
            flags,
        });
        let res: Option<RemoteNetworkGetDhcpLeasesRet> =
            call(self, RemoteProcedure::RemoteProcNetworkGetDhcpLeases, req)?;
        let res = res.unwrap();
        let RemoteNetworkGetDhcpLeasesRet { leases, ret } = res;
        Ok((leases, ret))
    }
    fn connect_get_domain_capabilities(
        &mut self,
        emulatorbin: Option<String>,
        arch: Option<String>,
        machine: Option<String>,
        virttype: Option<String>,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_domain_capabilities));
        let req: Option<RemoteConnectGetDomainCapabilitiesArgs> =
            Some(RemoteConnectGetDomainCapabilitiesArgs {
                emulatorbin,
                arch,
                machine,
                virttype,
                flags,
            });
        let res: Option<RemoteConnectGetDomainCapabilitiesRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectGetDomainCapabilities,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetDomainCapabilitiesRet { capabilities } = res;
        Ok(capabilities)
    }
    fn domain_open_graphics_fd(
        &mut self,
        dom: RemoteNonnullDomain,
        idx: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_open_graphics_fd));
        let req: Option<RemoteDomainOpenGraphicsFdArgs> =
            Some(RemoteDomainOpenGraphicsFdArgs { dom, idx, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainOpenGraphicsFd, req)?;
        Ok(())
    }
    fn connect_get_all_domain_stats(
        &mut self,
        doms: Vec<RemoteNonnullDomain>,
        stats: u32,
        flags: u32,
    ) -> Result<Vec<RemoteDomainStatsRecord>, Error> {
        trace!("{}", stringify!(connect_get_all_domain_stats));
        let req: Option<RemoteConnectGetAllDomainStatsArgs> =
            Some(RemoteConnectGetAllDomainStatsArgs { doms, stats, flags });
        let res: Option<RemoteConnectGetAllDomainStatsRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectGetAllDomainStats,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetAllDomainStatsRet { ret_stats } = res;
        Ok(ret_stats)
    }
    fn domain_block_copy(
        &mut self,
        dom: RemoteNonnullDomain,
        path: String,
        destxml: String,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_block_copy));
        let req: Option<RemoteDomainBlockCopyArgs> = Some(RemoteDomainBlockCopyArgs {
            dom,
            path,
            destxml,
            params,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainBlockCopy, req)?;
        Ok(())
    }
    fn domain_event_callback_tunable(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_tunable));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackTunable,
            req,
        )?;
        Ok(())
    }
    fn node_alloc_pages(
        &mut self,
        page_sizes: Vec<u32>,
        page_counts: Vec<u64>,
        start_cell: i32,
        cell_count: u32,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(node_alloc_pages));
        let req: Option<RemoteNodeAllocPagesArgs> = Some(RemoteNodeAllocPagesArgs {
            page_sizes,
            page_counts,
            start_cell,
            cell_count,
            flags,
        });
        let res: Option<RemoteNodeAllocPagesRet> =
            call(self, RemoteProcedure::RemoteProcNodeAllocPages, req)?;
        let res = res.unwrap();
        let RemoteNodeAllocPagesRet { ret } = res;
        Ok(ret)
    }
    fn domain_event_callback_agent_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_agent_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackAgentLifecycle,
            req,
        )?;
        Ok(())
    }
    fn domain_get_fsinfo(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<(Vec<RemoteDomainFsinfo>, u32), Error> {
        trace!("{}", stringify!(domain_get_fsinfo));
        let req: Option<RemoteDomainGetFsinfoArgs> = Some(RemoteDomainGetFsinfoArgs { dom, flags });
        let res: Option<RemoteDomainGetFsinfoRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetFsinfo, req)?;
        let res = res.unwrap();
        let RemoteDomainGetFsinfoRet { info, ret } = res;
        Ok((info, ret))
    }
    fn domain_define_xml_flags(
        &mut self,
        xml: String,
        flags: u32,
    ) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_define_xml_flags));
        let req: Option<RemoteDomainDefineXmlFlagsArgs> =
            Some(RemoteDomainDefineXmlFlagsArgs { xml, flags });
        let res: Option<RemoteDomainDefineXmlFlagsRet> =
            call(self, RemoteProcedure::RemoteProcDomainDefineXmlFlags, req)?;
        let res = res.unwrap();
        let RemoteDomainDefineXmlFlagsRet { dom } = res;
        Ok(dom)
    }
    fn domain_get_iothread_info(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<(Vec<RemoteDomainIothreadInfo>, u32), Error> {
        trace!("{}", stringify!(domain_get_iothread_info));
        let req: Option<RemoteDomainGetIothreadInfoArgs> =
            Some(RemoteDomainGetIothreadInfoArgs { dom, flags });
        let res: Option<RemoteDomainGetIothreadInfoRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetIothreadInfo, req)?;
        let res = res.unwrap();
        let RemoteDomainGetIothreadInfoRet { info, ret } = res;
        Ok((info, ret))
    }
    fn domain_pin_iothread(
        &mut self,
        dom: RemoteNonnullDomain,
        iothreads_id: u32,
        cpumap: Vec<u8>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pin_iothread));
        let req: Option<RemoteDomainPinIothreadArgs> = Some(RemoteDomainPinIothreadArgs {
            dom,
            iothreads_id,
            cpumap,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainPinIothread, req)?;
        Ok(())
    }
    fn domain_interface_addresses(
        &mut self,
        dom: RemoteNonnullDomain,
        source: u32,
        flags: u32,
    ) -> Result<Vec<RemoteDomainInterface>, Error> {
        trace!("{}", stringify!(domain_interface_addresses));
        let req: Option<RemoteDomainInterfaceAddressesArgs> =
            Some(RemoteDomainInterfaceAddressesArgs { dom, source, flags });
        let res: Option<RemoteDomainInterfaceAddressesRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainInterfaceAddresses,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainInterfaceAddressesRet { ifaces } = res;
        Ok(ifaces)
    }
    fn domain_event_callback_device_added(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_device_added));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackDeviceAdded,
            req,
        )?;
        Ok(())
    }
    fn domain_add_iothread(
        &mut self,
        dom: RemoteNonnullDomain,
        iothread_id: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_add_iothread));
        let req: Option<RemoteDomainAddIothreadArgs> = Some(RemoteDomainAddIothreadArgs {
            dom,
            iothread_id,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainAddIothread, req)?;
        Ok(())
    }
    fn domain_del_iothread(
        &mut self,
        dom: RemoteNonnullDomain,
        iothread_id: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_del_iothread));
        let req: Option<RemoteDomainDelIothreadArgs> = Some(RemoteDomainDelIothreadArgs {
            dom,
            iothread_id,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainDelIothread, req)?;
        Ok(())
    }
    fn domain_set_user_password(
        &mut self,
        dom: RemoteNonnullDomain,
        user: Option<String>,
        password: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_user_password));
        let req: Option<RemoteDomainSetUserPasswordArgs> = Some(RemoteDomainSetUserPasswordArgs {
            dom,
            user,
            password,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetUserPassword, req)?;
        Ok(())
    }
    fn domain_rename(
        &mut self,
        dom: RemoteNonnullDomain,
        new_name: Option<String>,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_rename));
        let req: Option<RemoteDomainRenameArgs> = Some(RemoteDomainRenameArgs {
            dom,
            new_name,
            flags,
        });
        let res: Option<RemoteDomainRenameRet> =
            call(self, RemoteProcedure::RemoteProcDomainRename, req)?;
        let res = res.unwrap();
        let RemoteDomainRenameRet { retcode } = res;
        Ok(retcode)
    }
    fn domain_event_callback_migration_iteration(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_migration_iteration));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackMigrationIteration,
            req,
        )?;
        Ok(())
    }
    fn connect_register_close_callback(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_register_close_callback));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcConnectRegisterCloseCallback,
            req,
        )?;
        Ok(())
    }
    fn connect_unregister_close_callback(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_unregister_close_callback));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcConnectUnregisterCloseCallback,
            req,
        )?;
        Ok(())
    }
    fn connect_event_connection_closed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_event_connection_closed));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcConnectEventConnectionClosed,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_job_completed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_job_completed));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackJobCompleted,
            req,
        )?;
        Ok(())
    }
    fn domain_migrate_start_post_copy(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_start_post_copy));
        let req: Option<RemoteDomainMigrateStartPostCopyArgs> =
            Some(RemoteDomainMigrateStartPostCopyArgs { dom, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigrateStartPostCopy,
            req,
        )?;
        Ok(())
    }
    fn domain_get_perf_events(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<Vec<RemoteTypedParam>, Error> {
        trace!("{}", stringify!(domain_get_perf_events));
        let req: Option<RemoteDomainGetPerfEventsArgs> =
            Some(RemoteDomainGetPerfEventsArgs { dom, flags });
        let res: Option<RemoteDomainGetPerfEventsRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetPerfEvents, req)?;
        let res = res.unwrap();
        let RemoteDomainGetPerfEventsRet { params } = res;
        Ok(params)
    }
    fn domain_set_perf_events(
        &mut self,
        dom: RemoteNonnullDomain,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_perf_events));
        let req: Option<RemoteDomainSetPerfEventsArgs> =
            Some(RemoteDomainSetPerfEventsArgs { dom, params, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetPerfEvents, req)?;
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
            RemoteProcedure::RemoteProcDomainEventCallbackDeviceRemovalFailed,
            req,
        )?;
        Ok(())
    }
    fn connect_storage_pool_event_register_any(
        &mut self,
        event_id: i32,
        pool: Option<RemoteNonnullStoragePool>,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_storage_pool_event_register_any));
        let req: Option<RemoteConnectStoragePoolEventRegisterAnyArgs> =
            Some(RemoteConnectStoragePoolEventRegisterAnyArgs { event_id, pool });
        let res: Option<RemoteConnectStoragePoolEventRegisterAnyRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectStoragePoolEventRegisterAny,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectStoragePoolEventRegisterAnyRet { callback_id } = res;
        Ok(callback_id)
    }
    fn connect_storage_pool_event_deregister_any(&mut self, callback_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_storage_pool_event_deregister_any));
        let req: Option<RemoteConnectStoragePoolEventDeregisterAnyArgs> =
            Some(RemoteConnectStoragePoolEventDeregisterAnyArgs { callback_id });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcConnectStoragePoolEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_event_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolEventLifecycle,
            req,
        )?;
        Ok(())
    }
    fn domain_get_guest_vcpus(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<Vec<RemoteTypedParam>, Error> {
        trace!("{}", stringify!(domain_get_guest_vcpus));
        let req: Option<RemoteDomainGetGuestVcpusArgs> =
            Some(RemoteDomainGetGuestVcpusArgs { dom, flags });
        let res: Option<RemoteDomainGetGuestVcpusRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetGuestVcpus, req)?;
        let res = res.unwrap();
        let RemoteDomainGetGuestVcpusRet { params } = res;
        Ok(params)
    }
    fn domain_set_guest_vcpus(
        &mut self,
        dom: RemoteNonnullDomain,
        cpumap: String,
        state: i32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_guest_vcpus));
        let req: Option<RemoteDomainSetGuestVcpusArgs> = Some(RemoteDomainSetGuestVcpusArgs {
            dom,
            cpumap,
            state,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetGuestVcpus, req)?;
        Ok(())
    }
    fn storage_pool_event_refresh(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_event_refresh));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolEventRefresh,
            req,
        )?;
        Ok(())
    }
    fn connect_node_device_event_register_any(
        &mut self,
        event_id: i32,
        dev: Option<RemoteNonnullNodeDevice>,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_node_device_event_register_any));
        let req: Option<RemoteConnectNodeDeviceEventRegisterAnyArgs> =
            Some(RemoteConnectNodeDeviceEventRegisterAnyArgs { event_id, dev });
        let res: Option<RemoteConnectNodeDeviceEventRegisterAnyRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectNodeDeviceEventRegisterAny,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNodeDeviceEventRegisterAnyRet { callback_id } = res;
        Ok(callback_id)
    }
    fn connect_node_device_event_deregister_any(&mut self, callback_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_node_device_event_deregister_any));
        let req: Option<RemoteConnectNodeDeviceEventDeregisterAnyArgs> =
            Some(RemoteConnectNodeDeviceEventDeregisterAnyArgs { callback_id });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcConnectNodeDeviceEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn node_device_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_event_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcNodeDeviceEventLifecycle,
            req,
        )?;
        Ok(())
    }
    fn node_device_event_update(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_event_update));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNodeDeviceEventUpdate, req)?;
        Ok(())
    }
    fn storage_vol_get_info_flags(
        &mut self,
        vol: RemoteNonnullStorageVol,
        flags: u32,
    ) -> Result<(i8, u64, u64), Error> {
        trace!("{}", stringify!(storage_vol_get_info_flags));
        let req: Option<RemoteStorageVolGetInfoFlagsArgs> =
            Some(RemoteStorageVolGetInfoFlagsArgs { vol, flags });
        let res: Option<RemoteStorageVolGetInfoFlagsRet> =
            call(self, RemoteProcedure::RemoteProcStorageVolGetInfoFlags, req)?;
        let res = res.unwrap();
        let RemoteStorageVolGetInfoFlagsRet {
            r#type,
            capacity,
            allocation,
        } = res;
        Ok((r#type, capacity, allocation))
    }
    fn domain_event_callback_metadata_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_metadata_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackMetadataChange,
            req,
        )?;
        Ok(())
    }
    fn connect_secret_event_register_any(
        &mut self,
        event_id: i32,
        secret: Option<RemoteNonnullSecret>,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_secret_event_register_any));
        let req: Option<RemoteConnectSecretEventRegisterAnyArgs> =
            Some(RemoteConnectSecretEventRegisterAnyArgs { event_id, secret });
        let res: Option<RemoteConnectSecretEventRegisterAnyRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectSecretEventRegisterAny,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectSecretEventRegisterAnyRet { callback_id } = res;
        Ok(callback_id)
    }
    fn connect_secret_event_deregister_any(&mut self, callback_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_secret_event_deregister_any));
        let req: Option<RemoteConnectSecretEventDeregisterAnyArgs> =
            Some(RemoteConnectSecretEventDeregisterAnyArgs { callback_id });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcConnectSecretEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn secret_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(secret_event_lifecycle));
        let req: Option<()> = None;
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcSecretEventLifecycle, req)?;
        Ok(())
    }
    fn secret_event_value_changed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(secret_event_value_changed));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcSecretEventValueChanged,
            req,
        )?;
        Ok(())
    }
    fn domain_set_vcpu(
        &mut self,
        dom: RemoteNonnullDomain,
        cpumap: String,
        state: i32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_vcpu));
        let req: Option<RemoteDomainSetVcpuArgs> = Some(RemoteDomainSetVcpuArgs {
            dom,
            cpumap,
            state,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSetVcpu, req)?;
        Ok(())
    }
    fn domain_event_block_threshold(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_threshold));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventBlockThreshold,
            req,
        )?;
        Ok(())
    }
    fn domain_set_block_threshold(
        &mut self,
        dom: RemoteNonnullDomain,
        dev: String,
        threshold: u64,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_block_threshold));
        let req: Option<RemoteDomainSetBlockThresholdArgs> =
            Some(RemoteDomainSetBlockThresholdArgs {
                dom,
                dev,
                threshold,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetBlockThreshold,
            req,
        )?;
        Ok(())
    }
    fn domain_migrate_get_max_downtime(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<u64, Error> {
        trace!("{}", stringify!(domain_migrate_get_max_downtime));
        let req: Option<RemoteDomainMigrateGetMaxDowntimeArgs> =
            Some(RemoteDomainMigrateGetMaxDowntimeArgs { dom, flags });
        let res: Option<RemoteDomainMigrateGetMaxDowntimeRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainMigrateGetMaxDowntime,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMigrateGetMaxDowntimeRet { downtime } = res;
        Ok(downtime)
    }
    fn domain_managed_save_get_xml_desc(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(domain_managed_save_get_xml_desc));
        let req: Option<RemoteDomainManagedSaveGetXmlDescArgs> =
            Some(RemoteDomainManagedSaveGetXmlDescArgs { dom, flags });
        let res: Option<RemoteDomainManagedSaveGetXmlDescRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainManagedSaveGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainManagedSaveGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_managed_save_define_xml(
        &mut self,
        dom: RemoteNonnullDomain,
        dxml: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_managed_save_define_xml));
        let req: Option<RemoteDomainManagedSaveDefineXmlArgs> =
            Some(RemoteDomainManagedSaveDefineXmlArgs { dom, dxml, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainManagedSaveDefineXml,
            req,
        )?;
        Ok(())
    }
    fn domain_set_lifecycle_action(
        &mut self,
        dom: RemoteNonnullDomain,
        r#type: u32,
        action: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_lifecycle_action));
        let req: Option<RemoteDomainSetLifecycleActionArgs> =
            Some(RemoteDomainSetLifecycleActionArgs {
                dom,
                r#type,
                action,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetLifecycleAction,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_lookup_by_target_path(
        &mut self,
        path: String,
    ) -> Result<RemoteNonnullStoragePool, Error> {
        trace!("{}", stringify!(storage_pool_lookup_by_target_path));
        let req: Option<RemoteStoragePoolLookupByTargetPathArgs> =
            Some(RemoteStoragePoolLookupByTargetPathArgs { path });
        let res: Option<RemoteStoragePoolLookupByTargetPathRet> = call(
            self,
            RemoteProcedure::RemoteProcStoragePoolLookupByTargetPath,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStoragePoolLookupByTargetPathRet { pool } = res;
        Ok(pool)
    }
    fn domain_detach_device_alias(
        &mut self,
        dom: RemoteNonnullDomain,
        alias: String,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_detach_device_alias));
        let req: Option<RemoteDomainDetachDeviceAliasArgs> =
            Some(RemoteDomainDetachDeviceAliasArgs { dom, alias, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainDetachDeviceAlias,
            req,
        )?;
        Ok(())
    }
    fn connect_compare_hypervisor_cpu(
        &mut self,
        emulator: Option<String>,
        arch: Option<String>,
        machine: Option<String>,
        virttype: Option<String>,
        xml_cpu: String,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_compare_hypervisor_cpu));
        let req: Option<RemoteConnectCompareHypervisorCpuArgs> =
            Some(RemoteConnectCompareHypervisorCpuArgs {
                emulator,
                arch,
                machine,
                virttype,
                xml_cpu,
                flags,
            });
        let res: Option<RemoteConnectCompareHypervisorCpuRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectCompareHypervisorCpu,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectCompareHypervisorCpuRet { result } = res;
        Ok(result)
    }
    fn connect_baseline_hypervisor_cpu(
        &mut self,
        emulator: Option<String>,
        arch: Option<String>,
        machine: Option<String>,
        virttype: Option<String>,
        xml_cpus: Vec<String>,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(connect_baseline_hypervisor_cpu));
        let req: Option<RemoteConnectBaselineHypervisorCpuArgs> =
            Some(RemoteConnectBaselineHypervisorCpuArgs {
                emulator,
                arch,
                machine,
                virttype,
                xml_cpus,
                flags,
            });
        let res: Option<RemoteConnectBaselineHypervisorCpuRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectBaselineHypervisorCpu,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectBaselineHypervisorCpuRet { cpu } = res;
        Ok(cpu)
    }
    fn node_get_sev_info(
        &mut self,
        nparams: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteTypedParam>, i32), Error> {
        trace!("{}", stringify!(node_get_sev_info));
        let req: Option<RemoteNodeGetSevInfoArgs> =
            Some(RemoteNodeGetSevInfoArgs { nparams, flags });
        let res: Option<RemoteNodeGetSevInfoRet> =
            call(self, RemoteProcedure::RemoteProcNodeGetSevInfo, req)?;
        let res = res.unwrap();
        let RemoteNodeGetSevInfoRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_get_launch_security_info(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<Vec<RemoteTypedParam>, Error> {
        trace!("{}", stringify!(domain_get_launch_security_info));
        let req: Option<RemoteDomainGetLaunchSecurityInfoArgs> =
            Some(RemoteDomainGetLaunchSecurityInfoArgs { dom, flags });
        let res: Option<RemoteDomainGetLaunchSecurityInfoRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainGetLaunchSecurityInfo,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetLaunchSecurityInfoRet { params } = res;
        Ok(params)
    }
    fn nwfilter_binding_lookup_by_port_dev(
        &mut self,
        name: String,
    ) -> Result<RemoteNonnullNwfilterBinding, Error> {
        trace!("{}", stringify!(nwfilter_binding_lookup_by_port_dev));
        let req: Option<RemoteNwfilterBindingLookupByPortDevArgs> =
            Some(RemoteNwfilterBindingLookupByPortDevArgs { name });
        let res: Option<RemoteNwfilterBindingLookupByPortDevRet> = call(
            self,
            RemoteProcedure::RemoteProcNwfilterBindingLookupByPortDev,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNwfilterBindingLookupByPortDevRet { nwfilter } = res;
        Ok(nwfilter)
    }
    fn nwfilter_binding_get_xml_desc(
        &mut self,
        nwfilter: RemoteNonnullNwfilterBinding,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(nwfilter_binding_get_xml_desc));
        let req: Option<RemoteNwfilterBindingGetXmlDescArgs> =
            Some(RemoteNwfilterBindingGetXmlDescArgs { nwfilter, flags });
        let res: Option<RemoteNwfilterBindingGetXmlDescRet> = call(
            self,
            RemoteProcedure::RemoteProcNwfilterBindingGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNwfilterBindingGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn nwfilter_binding_create_xml(
        &mut self,
        xml: String,
        flags: u32,
    ) -> Result<RemoteNonnullNwfilterBinding, Error> {
        trace!("{}", stringify!(nwfilter_binding_create_xml));
        let req: Option<RemoteNwfilterBindingCreateXmlArgs> =
            Some(RemoteNwfilterBindingCreateXmlArgs { xml, flags });
        let res: Option<RemoteNwfilterBindingCreateXmlRet> = call(
            self,
            RemoteProcedure::RemoteProcNwfilterBindingCreateXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNwfilterBindingCreateXmlRet { nwfilter } = res;
        Ok(nwfilter)
    }
    fn nwfilter_binding_delete(
        &mut self,
        nwfilter: RemoteNonnullNwfilterBinding,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(nwfilter_binding_delete));
        let req: Option<RemoteNwfilterBindingDeleteArgs> =
            Some(RemoteNwfilterBindingDeleteArgs { nwfilter });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNwfilterBindingDelete, req)?;
        Ok(())
    }
    fn connect_list_all_nwfilter_bindings(
        &mut self,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullNwfilterBinding>, u32), Error> {
        trace!("{}", stringify!(connect_list_all_nwfilter_bindings));
        let req: Option<RemoteConnectListAllNwfilterBindingsArgs> =
            Some(RemoteConnectListAllNwfilterBindingsArgs {
                need_results,
                flags,
            });
        let res: Option<RemoteConnectListAllNwfilterBindingsRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectListAllNwfilterBindings,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListAllNwfilterBindingsRet { bindings, ret } = res;
        Ok((bindings, ret))
    }
    fn domain_set_iothread_params(
        &mut self,
        dom: RemoteNonnullDomain,
        iothread_id: u32,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_iothread_params));
        let req: Option<RemoteDomainSetIothreadParamsArgs> =
            Some(RemoteDomainSetIothreadParamsArgs {
                dom,
                iothread_id,
                params,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetIothreadParams,
            req,
        )?;
        Ok(())
    }
    fn connect_get_storage_pool_capabilities(&mut self, flags: u32) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_storage_pool_capabilities));
        let req: Option<RemoteConnectGetStoragePoolCapabilitiesArgs> =
            Some(RemoteConnectGetStoragePoolCapabilitiesArgs { flags });
        let res: Option<RemoteConnectGetStoragePoolCapabilitiesRet> = call(
            self,
            RemoteProcedure::RemoteProcConnectGetStoragePoolCapabilities,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetStoragePoolCapabilitiesRet { capabilities } = res;
        Ok(capabilities)
    }
    fn network_list_all_ports(
        &mut self,
        network: RemoteNonnullNetwork,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullNetworkPort>, u32), Error> {
        trace!("{}", stringify!(network_list_all_ports));
        let req: Option<RemoteNetworkListAllPortsArgs> = Some(RemoteNetworkListAllPortsArgs {
            network,
            need_results,
            flags,
        });
        let res: Option<RemoteNetworkListAllPortsRet> =
            call(self, RemoteProcedure::RemoteProcNetworkListAllPorts, req)?;
        let res = res.unwrap();
        let RemoteNetworkListAllPortsRet { ports, ret } = res;
        Ok((ports, ret))
    }
    fn network_port_lookup_by_uuid(
        &mut self,
        network: RemoteNonnullNetwork,
        uuid: [u8; VIR_UUID_BUFLEN as usize],
    ) -> Result<RemoteNonnullNetworkPort, Error> {
        trace!("{}", stringify!(network_port_lookup_by_uuid));
        let req: Option<RemoteNetworkPortLookupByUuidArgs> =
            Some(RemoteNetworkPortLookupByUuidArgs { network, uuid });
        let res: Option<RemoteNetworkPortLookupByUuidRet> = call(
            self,
            RemoteProcedure::RemoteProcNetworkPortLookupByUuid,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkPortLookupByUuidRet { port } = res;
        Ok(port)
    }
    fn network_port_create_xml(
        &mut self,
        network: RemoteNonnullNetwork,
        xml: String,
        flags: u32,
    ) -> Result<RemoteNonnullNetworkPort, Error> {
        trace!("{}", stringify!(network_port_create_xml));
        let req: Option<RemoteNetworkPortCreateXmlArgs> = Some(RemoteNetworkPortCreateXmlArgs {
            network,
            xml,
            flags,
        });
        let res: Option<RemoteNetworkPortCreateXmlRet> =
            call(self, RemoteProcedure::RemoteProcNetworkPortCreateXml, req)?;
        let res = res.unwrap();
        let RemoteNetworkPortCreateXmlRet { port } = res;
        Ok(port)
    }
    fn network_port_get_parameters(
        &mut self,
        port: RemoteNonnullNetworkPort,
        nparams: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteTypedParam>, i32), Error> {
        trace!("{}", stringify!(network_port_get_parameters));
        let req: Option<RemoteNetworkPortGetParametersArgs> =
            Some(RemoteNetworkPortGetParametersArgs {
                port,
                nparams,
                flags,
            });
        let res: Option<RemoteNetworkPortGetParametersRet> = call(
            self,
            RemoteProcedure::RemoteProcNetworkPortGetParameters,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkPortGetParametersRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn network_port_set_parameters(
        &mut self,
        port: RemoteNonnullNetworkPort,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(network_port_set_parameters));
        let req: Option<RemoteNetworkPortSetParametersArgs> =
            Some(RemoteNetworkPortSetParametersArgs {
                port,
                params,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcNetworkPortSetParameters,
            req,
        )?;
        Ok(())
    }
    fn network_port_get_xml_desc(
        &mut self,
        port: RemoteNonnullNetworkPort,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(network_port_get_xml_desc));
        let req: Option<RemoteNetworkPortGetXmlDescArgs> =
            Some(RemoteNetworkPortGetXmlDescArgs { port, flags });
        let res: Option<RemoteNetworkPortGetXmlDescRet> =
            call(self, RemoteProcedure::RemoteProcNetworkPortGetXmlDesc, req)?;
        let res = res.unwrap();
        let RemoteNetworkPortGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn network_port_delete(
        &mut self,
        port: RemoteNonnullNetworkPort,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(network_port_delete));
        let req: Option<RemoteNetworkPortDeleteArgs> =
            Some(RemoteNetworkPortDeleteArgs { port, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNetworkPortDelete, req)?;
        Ok(())
    }
    fn domain_checkpoint_create_xml(
        &mut self,
        dom: RemoteNonnullDomain,
        xml_desc: String,
        flags: u32,
    ) -> Result<RemoteNonnullDomainCheckpoint, Error> {
        trace!("{}", stringify!(domain_checkpoint_create_xml));
        let req: Option<RemoteDomainCheckpointCreateXmlArgs> =
            Some(RemoteDomainCheckpointCreateXmlArgs {
                dom,
                xml_desc,
                flags,
            });
        let res: Option<RemoteDomainCheckpointCreateXmlRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainCheckpointCreateXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainCheckpointCreateXmlRet { checkpoint } = res;
        Ok(checkpoint)
    }
    fn domain_checkpoint_get_xml_desc(
        &mut self,
        checkpoint: RemoteNonnullDomainCheckpoint,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(domain_checkpoint_get_xml_desc));
        let req: Option<RemoteDomainCheckpointGetXmlDescArgs> =
            Some(RemoteDomainCheckpointGetXmlDescArgs { checkpoint, flags });
        let res: Option<RemoteDomainCheckpointGetXmlDescRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainCheckpointGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainCheckpointGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_list_all_checkpoints(
        &mut self,
        dom: RemoteNonnullDomain,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullDomainCheckpoint>, i32), Error> {
        trace!("{}", stringify!(domain_list_all_checkpoints));
        let req: Option<RemoteDomainListAllCheckpointsArgs> =
            Some(RemoteDomainListAllCheckpointsArgs {
                dom,
                need_results,
                flags,
            });
        let res: Option<RemoteDomainListAllCheckpointsRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainListAllCheckpoints,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainListAllCheckpointsRet { checkpoints, ret } = res;
        Ok((checkpoints, ret))
    }
    fn domain_checkpoint_list_all_children(
        &mut self,
        checkpoint: RemoteNonnullDomainCheckpoint,
        need_results: i32,
        flags: u32,
    ) -> Result<(Vec<RemoteNonnullDomainCheckpoint>, i32), Error> {
        trace!("{}", stringify!(domain_checkpoint_list_all_children));
        let req: Option<RemoteDomainCheckpointListAllChildrenArgs> =
            Some(RemoteDomainCheckpointListAllChildrenArgs {
                checkpoint,
                need_results,
                flags,
            });
        let res: Option<RemoteDomainCheckpointListAllChildrenRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainCheckpointListAllChildren,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainCheckpointListAllChildrenRet { checkpoints, ret } = res;
        Ok((checkpoints, ret))
    }
    fn domain_checkpoint_lookup_by_name(
        &mut self,
        dom: RemoteNonnullDomain,
        name: String,
        flags: u32,
    ) -> Result<RemoteNonnullDomainCheckpoint, Error> {
        trace!("{}", stringify!(domain_checkpoint_lookup_by_name));
        let req: Option<RemoteDomainCheckpointLookupByNameArgs> =
            Some(RemoteDomainCheckpointLookupByNameArgs { dom, name, flags });
        let res: Option<RemoteDomainCheckpointLookupByNameRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainCheckpointLookupByName,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainCheckpointLookupByNameRet { checkpoint } = res;
        Ok(checkpoint)
    }
    fn domain_checkpoint_get_parent(
        &mut self,
        checkpoint: RemoteNonnullDomainCheckpoint,
        flags: u32,
    ) -> Result<RemoteNonnullDomainCheckpoint, Error> {
        trace!("{}", stringify!(domain_checkpoint_get_parent));
        let req: Option<RemoteDomainCheckpointGetParentArgs> =
            Some(RemoteDomainCheckpointGetParentArgs { checkpoint, flags });
        let res: Option<RemoteDomainCheckpointGetParentRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainCheckpointGetParent,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainCheckpointGetParentRet { parent } = res;
        Ok(parent)
    }
    fn domain_checkpoint_delete(
        &mut self,
        checkpoint: RemoteNonnullDomainCheckpoint,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_checkpoint_delete));
        let req: Option<RemoteDomainCheckpointDeleteArgs> =
            Some(RemoteDomainCheckpointDeleteArgs { checkpoint, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainCheckpointDelete, req)?;
        Ok(())
    }
    fn domain_get_guest_info(
        &mut self,
        dom: RemoteNonnullDomain,
        types: u32,
        flags: u32,
    ) -> Result<Vec<RemoteTypedParam>, Error> {
        trace!("{}", stringify!(domain_get_guest_info));
        let req: Option<RemoteDomainGetGuestInfoArgs> =
            Some(RemoteDomainGetGuestInfoArgs { dom, types, flags });
        let res: Option<RemoteDomainGetGuestInfoRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetGuestInfo, req)?;
        let res = res.unwrap();
        let RemoteDomainGetGuestInfoRet { params } = res;
        Ok(params)
    }
    fn connect_set_identity(
        &mut self,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(connect_set_identity));
        let req: Option<RemoteConnectSetIdentityArgs> =
            Some(RemoteConnectSetIdentityArgs { params, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcConnectSetIdentity, req)?;
        Ok(())
    }
    fn domain_agent_set_response_timeout(
        &mut self,
        dom: RemoteNonnullDomain,
        timeout: i32,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_agent_set_response_timeout));
        let req: Option<RemoteDomainAgentSetResponseTimeoutArgs> =
            Some(RemoteDomainAgentSetResponseTimeoutArgs {
                dom,
                timeout,
                flags,
            });
        let res: Option<RemoteDomainAgentSetResponseTimeoutRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainAgentSetResponseTimeout,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainAgentSetResponseTimeoutRet { result } = res;
        Ok(result)
    }
    fn domain_backup_begin(
        &mut self,
        dom: RemoteNonnullDomain,
        backup_xml: String,
        checkpoint_xml: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_backup_begin));
        let req: Option<RemoteDomainBackupBeginArgs> = Some(RemoteDomainBackupBeginArgs {
            dom,
            backup_xml,
            checkpoint_xml,
            flags,
        });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainBackupBegin, req)?;
        Ok(())
    }
    fn domain_backup_get_xml_desc(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(domain_backup_get_xml_desc));
        let req: Option<RemoteDomainBackupGetXmlDescArgs> =
            Some(RemoteDomainBackupGetXmlDescArgs { dom, flags });
        let res: Option<RemoteDomainBackupGetXmlDescRet> =
            call(self, RemoteProcedure::RemoteProcDomainBackupGetXmlDesc, req)?;
        let res = res.unwrap();
        let RemoteDomainBackupGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_event_memory_failure(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_memory_failure));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventMemoryFailure,
            req,
        )?;
        Ok(())
    }
    fn domain_authorized_ssh_keys_get(
        &mut self,
        dom: RemoteNonnullDomain,
        user: String,
        flags: u32,
    ) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(domain_authorized_ssh_keys_get));
        let req: Option<RemoteDomainAuthorizedSshKeysGetArgs> =
            Some(RemoteDomainAuthorizedSshKeysGetArgs { dom, user, flags });
        let res: Option<RemoteDomainAuthorizedSshKeysGetRet> = call(
            self,
            RemoteProcedure::RemoteProcDomainAuthorizedSshKeysGet,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainAuthorizedSshKeysGetRet { keys } = res;
        Ok(keys)
    }
    fn domain_authorized_ssh_keys_set(
        &mut self,
        dom: RemoteNonnullDomain,
        user: String,
        keys: Vec<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_authorized_ssh_keys_set));
        let req: Option<RemoteDomainAuthorizedSshKeysSetArgs> =
            Some(RemoteDomainAuthorizedSshKeysSetArgs {
                dom,
                user,
                keys,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainAuthorizedSshKeysSet,
            req,
        )?;
        Ok(())
    }
    fn domain_get_messages(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(domain_get_messages));
        let req: Option<RemoteDomainGetMessagesArgs> =
            Some(RemoteDomainGetMessagesArgs { dom, flags });
        let res: Option<RemoteDomainGetMessagesRet> =
            call(self, RemoteProcedure::RemoteProcDomainGetMessages, req)?;
        let res = res.unwrap();
        let RemoteDomainGetMessagesRet { msgs } = res;
        Ok(msgs)
    }
    fn domain_start_dirty_rate_calc(
        &mut self,
        dom: RemoteNonnullDomain,
        seconds: i32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_start_dirty_rate_calc));
        let req: Option<RemoteDomainStartDirtyRateCalcArgs> =
            Some(RemoteDomainStartDirtyRateCalcArgs {
                dom,
                seconds,
                flags,
            });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainStartDirtyRateCalc,
            req,
        )?;
        Ok(())
    }
    fn node_device_define_xml(
        &mut self,
        xml_desc: String,
        flags: u32,
    ) -> Result<RemoteNonnullNodeDevice, Error> {
        trace!("{}", stringify!(node_device_define_xml));
        let req: Option<RemoteNodeDeviceDefineXmlArgs> =
            Some(RemoteNodeDeviceDefineXmlArgs { xml_desc, flags });
        let res: Option<RemoteNodeDeviceDefineXmlRet> =
            call(self, RemoteProcedure::RemoteProcNodeDeviceDefineXml, req)?;
        let res = res.unwrap();
        let RemoteNodeDeviceDefineXmlRet { dev } = res;
        Ok(dev)
    }
    fn node_device_undefine(&mut self, name: String, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_undefine));
        let req: Option<RemoteNodeDeviceUndefineArgs> =
            Some(RemoteNodeDeviceUndefineArgs { name, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNodeDeviceUndefine, req)?;
        Ok(())
    }
    fn node_device_create(&mut self, name: String, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_create));
        let req: Option<RemoteNodeDeviceCreateArgs> =
            Some(RemoteNodeDeviceCreateArgs { name, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNodeDeviceCreate, req)?;
        Ok(())
    }
    fn nwfilter_define_xml_flags(
        &mut self,
        xml: String,
        flags: u32,
    ) -> Result<RemoteNonnullNwfilter, Error> {
        trace!("{}", stringify!(nwfilter_define_xml_flags));
        let req: Option<RemoteNwfilterDefineXmlFlagsArgs> =
            Some(RemoteNwfilterDefineXmlFlagsArgs { xml, flags });
        let res: Option<RemoteNwfilterDefineXmlFlagsRet> =
            call(self, RemoteProcedure::RemoteProcNwfilterDefineXmlFlags, req)?;
        let res = res.unwrap();
        let RemoteNwfilterDefineXmlFlagsRet { nwfilter } = res;
        Ok(nwfilter)
    }
    fn network_define_xml_flags(
        &mut self,
        xml: String,
        flags: u32,
    ) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_define_xml_flags));
        let req: Option<RemoteNetworkDefineXmlFlagsArgs> =
            Some(RemoteNetworkDefineXmlFlagsArgs { xml, flags });
        let res: Option<RemoteNetworkDefineXmlFlagsRet> =
            call(self, RemoteProcedure::RemoteProcNetworkDefineXmlFlags, req)?;
        let res = res.unwrap();
        let RemoteNetworkDefineXmlFlagsRet { net } = res;
        Ok(net)
    }
    fn node_device_get_autostart(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_get_autostart));
        let req: Option<RemoteNodeDeviceGetAutostartArgs> =
            Some(RemoteNodeDeviceGetAutostartArgs { name });
        let res: Option<RemoteNodeDeviceGetAutostartRet> =
            call(self, RemoteProcedure::RemoteProcNodeDeviceGetAutostart, req)?;
        let res = res.unwrap();
        let RemoteNodeDeviceGetAutostartRet { autostart } = res;
        Ok(autostart)
    }
    fn node_device_set_autostart(&mut self, name: String, autostart: i32) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_set_autostart));
        let req: Option<RemoteNodeDeviceSetAutostartArgs> =
            Some(RemoteNodeDeviceSetAutostartArgs { name, autostart });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcNodeDeviceSetAutostart, req)?;
        Ok(())
    }
    fn node_device_is_persistent(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_is_persistent));
        let req: Option<RemoteNodeDeviceIsPersistentArgs> =
            Some(RemoteNodeDeviceIsPersistentArgs { name });
        let res: Option<RemoteNodeDeviceIsPersistentRet> =
            call(self, RemoteProcedure::RemoteProcNodeDeviceIsPersistent, req)?;
        let res = res.unwrap();
        let RemoteNodeDeviceIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn node_device_is_active(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_is_active));
        let req: Option<RemoteNodeDeviceIsActiveArgs> = Some(RemoteNodeDeviceIsActiveArgs { name });
        let res: Option<RemoteNodeDeviceIsActiveRet> =
            call(self, RemoteProcedure::RemoteProcNodeDeviceIsActive, req)?;
        let res = res.unwrap();
        let RemoteNodeDeviceIsActiveRet { active } = res;
        Ok(active)
    }
    fn network_create_xml_flags(
        &mut self,
        xml: String,
        flags: u32,
    ) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_create_xml_flags));
        let req: Option<RemoteNetworkCreateXmlFlagsArgs> =
            Some(RemoteNetworkCreateXmlFlagsArgs { xml, flags });
        let res: Option<RemoteNetworkCreateXmlFlagsRet> =
            call(self, RemoteProcedure::RemoteProcNetworkCreateXmlFlags, req)?;
        let res = res.unwrap();
        let RemoteNetworkCreateXmlFlagsRet { net } = res;
        Ok(net)
    }
    fn domain_event_memory_device_size_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_memory_device_size_change));
        let req: Option<()> = None;
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainEventMemoryDeviceSizeChange,
            req,
        )?;
        Ok(())
    }
    fn domain_set_launch_security_state(
        &mut self,
        dom: RemoteNonnullDomain,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_launch_security_state));
        let req: Option<RemoteDomainSetLaunchSecurityStateArgs> =
            Some(RemoteDomainSetLaunchSecurityStateArgs { dom, params, flags });
        let _res: Option<()> = call(
            self,
            RemoteProcedure::RemoteProcDomainSetLaunchSecurityState,
            req,
        )?;
        Ok(())
    }
    fn domain_save_params(
        &mut self,
        dom: RemoteNonnullDomain,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_save_params));
        let req: Option<RemoteDomainSaveParamsArgs> =
            Some(RemoteDomainSaveParamsArgs { dom, params, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainSaveParams, req)?;
        Ok(())
    }
    fn domain_restore_params(
        &mut self,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_restore_params));
        let req: Option<RemoteDomainRestoreParamsArgs> =
            Some(RemoteDomainRestoreParamsArgs { params, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainRestoreParams, req)?;
        Ok(())
    }
    fn domain_abort_job_flags(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_abort_job_flags));
        let req: Option<RemoteDomainAbortJobFlagsArgs> =
            Some(RemoteDomainAbortJobFlagsArgs { dom, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainAbortJobFlags, req)?;
        Ok(())
    }
    fn domain_fd_associate(
        &mut self,
        dom: RemoteNonnullDomain,
        name: String,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_fd_associate));
        let req: Option<RemoteDomainFdAssociateArgs> =
            Some(RemoteDomainFdAssociateArgs { dom, name, flags });
        let _res: Option<()> = call(self, RemoteProcedure::RemoteProcDomainFdAssociate, req)?;
        Ok(())
    }
    fn connect_event_connection_closed_msg(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_event_connection_closed_msg));
        let res: Option<RemoteConnectEventConnectionClosedMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteConnectEventConnectionClosedMsg { reason } = res;
        Ok(reason)
    }
    fn domain_event_balloon_change_msg(&mut self) -> Result<(RemoteNonnullDomain, u64), Error> {
        trace!("{}", stringify!(domain_event_balloon_change_msg));
        let res: Option<RemoteDomainEventBalloonChangeMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventBalloonChangeMsg { dom, actual } = res;
        Ok((dom, actual))
    }
    fn domain_event_block_job2_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, String, i32, i32), Error> {
        trace!("{}", stringify!(domain_event_block_job2_msg));
        let res: Option<RemoteDomainEventBlockJob2Msg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventBlockJob2Msg {
            callback_id,
            dom,
            dst,
            r#type,
            status,
        } = res;
        Ok((callback_id, dom, dst, r#type, status))
    }
    fn domain_event_block_job_msg(
        &mut self,
    ) -> Result<(RemoteNonnullDomain, String, i32, i32), Error> {
        trace!("{}", stringify!(domain_event_block_job_msg));
        let res: Option<RemoteDomainEventBlockJobMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventBlockJobMsg {
            dom,
            path,
            r#type,
            status,
        } = res;
        Ok((dom, path, r#type, status))
    }
    fn domain_event_block_threshold_msg(
        &mut self,
    ) -> Result<RemoteDomainEventBlockThresholdMsg, Error> {
        trace!("{}", stringify!(domain_event_block_threshold_msg));
        let res: Option<RemoteDomainEventBlockThresholdMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_agent_lifecycle_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, i32, i32), Error> {
        trace!("{}", stringify!(domain_event_callback_agent_lifecycle_msg));
        let res: Option<RemoteDomainEventCallbackAgentLifecycleMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackAgentLifecycleMsg {
            callback_id,
            dom,
            state,
            reason,
        } = res;
        Ok((callback_id, dom, state, reason))
    }
    fn domain_event_callback_balloon_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventBalloonChangeMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_balloon_change_msg));
        let res: Option<RemoteDomainEventCallbackBalloonChangeMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackBalloonChangeMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_block_job_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventBlockJobMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_block_job_msg));
        let res: Option<RemoteDomainEventCallbackBlockJobMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackBlockJobMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_control_error_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventControlErrorMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_control_error_msg));
        let res: Option<RemoteDomainEventCallbackControlErrorMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackControlErrorMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_device_added_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, String), Error> {
        trace!("{}", stringify!(domain_event_callback_device_added_msg));
        let res: Option<RemoteDomainEventCallbackDeviceAddedMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackDeviceAddedMsg {
            callback_id,
            dom,
            dev_alias,
        } = res;
        Ok((callback_id, dom, dev_alias))
    }
    fn domain_event_callback_device_removal_failed_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, String), Error> {
        trace!(
            "{}",
            stringify!(domain_event_callback_device_removal_failed_msg)
        );
        let res: Option<RemoteDomainEventCallbackDeviceRemovalFailedMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackDeviceRemovalFailedMsg {
            callback_id,
            dom,
            dev_alias,
        } = res;
        Ok((callback_id, dom, dev_alias))
    }
    fn domain_event_callback_device_removed_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventDeviceRemovedMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_device_removed_msg));
        let res: Option<RemoteDomainEventCallbackDeviceRemovedMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackDeviceRemovedMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_disk_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventDiskChangeMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_disk_change_msg));
        let res: Option<RemoteDomainEventCallbackDiskChangeMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackDiskChangeMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_graphics_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventGraphicsMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_graphics_msg));
        let res: Option<RemoteDomainEventCallbackGraphicsMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackGraphicsMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_io_error_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventIoErrorMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error_msg));
        let res: Option<RemoteDomainEventCallbackIoErrorMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackIoErrorMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_io_error_reason_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventIoErrorReasonMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error_reason_msg));
        let res: Option<RemoteDomainEventCallbackIoErrorReasonMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackIoErrorReasonMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_job_completed_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, Vec<RemoteTypedParam>), Error> {
        trace!("{}", stringify!(domain_event_callback_job_completed_msg));
        let res: Option<RemoteDomainEventCallbackJobCompletedMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackJobCompletedMsg {
            callback_id,
            dom,
            params,
        } = res;
        Ok((callback_id, dom, params))
    }
    fn domain_event_callback_lifecycle_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventLifecycleMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_lifecycle_msg));
        let res: Option<RemoteDomainEventCallbackLifecycleMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackLifecycleMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_metadata_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, i32, Option<String>), Error> {
        trace!("{}", stringify!(domain_event_callback_metadata_change_msg));
        let res: Option<RemoteDomainEventCallbackMetadataChangeMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackMetadataChangeMsg {
            callback_id,
            dom,
            r#type,
            nsuri,
        } = res;
        Ok((callback_id, dom, r#type, nsuri))
    }
    fn domain_event_callback_migration_iteration_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, i32), Error> {
        trace!(
            "{}",
            stringify!(domain_event_callback_migration_iteration_msg)
        );
        let res: Option<RemoteDomainEventCallbackMigrationIterationMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackMigrationIterationMsg {
            callback_id,
            dom,
            iteration,
        } = res;
        Ok((callback_id, dom, iteration))
    }
    fn domain_event_callback_pmsuspend_disk_msg(
        &mut self,
    ) -> Result<(i32, i32, RemoteDomainEventPmsuspendDiskMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend_disk_msg));
        let res: Option<RemoteDomainEventCallbackPmsuspendDiskMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackPmsuspendDiskMsg {
            callback_id,
            reason,
            msg,
        } = res;
        Ok((callback_id, reason, msg))
    }
    fn domain_event_callback_pmsuspend_msg(
        &mut self,
    ) -> Result<(i32, i32, RemoteDomainEventPmsuspendMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend_msg));
        let res: Option<RemoteDomainEventCallbackPmsuspendMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackPmsuspendMsg {
            callback_id,
            reason,
            msg,
        } = res;
        Ok((callback_id, reason, msg))
    }
    fn domain_event_callback_pmwakeup_msg(
        &mut self,
    ) -> Result<(i32, i32, RemoteDomainEventPmwakeupMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_pmwakeup_msg));
        let res: Option<RemoteDomainEventCallbackPmwakeupMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackPmwakeupMsg {
            callback_id,
            reason,
            msg,
        } = res;
        Ok((callback_id, reason, msg))
    }
    fn domain_event_callback_reboot_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventRebootMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_reboot_msg));
        let res: Option<RemoteDomainEventCallbackRebootMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackRebootMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_rtc_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventRtcChangeMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_rtc_change_msg));
        let res: Option<RemoteDomainEventCallbackRtcChangeMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackRtcChangeMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_tray_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventTrayChangeMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_tray_change_msg));
        let res: Option<RemoteDomainEventCallbackTrayChangeMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackTrayChangeMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_tunable_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, Vec<RemoteTypedParam>), Error> {
        trace!("{}", stringify!(domain_event_callback_tunable_msg));
        let res: Option<RemoteDomainEventCallbackTunableMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackTunableMsg {
            callback_id,
            dom,
            params,
        } = res;
        Ok((callback_id, dom, params))
    }
    fn domain_event_callback_watchdog_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventWatchdogMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_watchdog_msg));
        let res: Option<RemoteDomainEventCallbackWatchdogMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackWatchdogMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_control_error_msg(&mut self) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_event_control_error_msg));
        let res: Option<RemoteDomainEventControlErrorMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventControlErrorMsg { dom } = res;
        Ok(dom)
    }
    fn domain_event_device_removed_msg(&mut self) -> Result<(RemoteNonnullDomain, String), Error> {
        trace!("{}", stringify!(domain_event_device_removed_msg));
        let res: Option<RemoteDomainEventDeviceRemovedMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventDeviceRemovedMsg { dom, dev_alias } = res;
        Ok((dom, dev_alias))
    }
    fn domain_event_disk_change_msg(&mut self) -> Result<RemoteDomainEventDiskChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_disk_change_msg));
        let res: Option<RemoteDomainEventDiskChangeMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_graphics_msg(&mut self) -> Result<RemoteDomainEventGraphicsMsg, Error> {
        trace!("{}", stringify!(domain_event_graphics_msg));
        let res: Option<RemoteDomainEventGraphicsMsg> = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_io_error_msg(
        &mut self,
    ) -> Result<(RemoteNonnullDomain, String, String, i32), Error> {
        trace!("{}", stringify!(domain_event_io_error_msg));
        let res: Option<RemoteDomainEventIoErrorMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventIoErrorMsg {
            dom,
            src_path,
            dev_alias,
            action,
        } = res;
        Ok((dom, src_path, dev_alias, action))
    }
    fn domain_event_io_error_reason_msg(
        &mut self,
    ) -> Result<(RemoteNonnullDomain, String, String, i32, String), Error> {
        trace!("{}", stringify!(domain_event_io_error_reason_msg));
        let res: Option<RemoteDomainEventIoErrorReasonMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventIoErrorReasonMsg {
            dom,
            src_path,
            dev_alias,
            action,
            reason,
        } = res;
        Ok((dom, src_path, dev_alias, action, reason))
    }
    fn domain_event_lifecycle_msg(&mut self) -> Result<(RemoteNonnullDomain, i32, i32), Error> {
        trace!("{}", stringify!(domain_event_lifecycle_msg));
        let res: Option<RemoteDomainEventLifecycleMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventLifecycleMsg { dom, event, detail } = res;
        Ok((dom, event, detail))
    }
    fn domain_event_memory_device_size_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, String, u64), Error> {
        trace!("{}", stringify!(domain_event_memory_device_size_change_msg));
        let res: Option<RemoteDomainEventMemoryDeviceSizeChangeMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventMemoryDeviceSizeChangeMsg {
            callback_id,
            dom,
            alias,
            size,
        } = res;
        Ok((callback_id, dom, alias, size))
    }
    fn domain_event_memory_failure_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, i32, i32, u32), Error> {
        trace!("{}", stringify!(domain_event_memory_failure_msg));
        let res: Option<RemoteDomainEventMemoryFailureMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventMemoryFailureMsg {
            callback_id,
            dom,
            recipient,
            action,
            flags,
        } = res;
        Ok((callback_id, dom, recipient, action, flags))
    }
    fn domain_event_pmsuspend_disk_msg(&mut self) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_event_pmsuspend_disk_msg));
        let res: Option<RemoteDomainEventPmsuspendDiskMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventPmsuspendDiskMsg { dom } = res;
        Ok(dom)
    }
    fn domain_event_pmsuspend_msg(&mut self) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_event_pmsuspend_msg));
        let res: Option<RemoteDomainEventPmsuspendMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventPmsuspendMsg { dom } = res;
        Ok(dom)
    }
    fn domain_event_pmwakeup_msg(&mut self) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_event_pmwakeup_msg));
        let res: Option<RemoteDomainEventPmwakeupMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventPmwakeupMsg { dom } = res;
        Ok(dom)
    }
    fn domain_event_reboot_msg(&mut self) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_event_reboot_msg));
        let res: Option<RemoteDomainEventRebootMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventRebootMsg { dom } = res;
        Ok(dom)
    }
    fn domain_event_rtc_change_msg(&mut self) -> Result<(RemoteNonnullDomain, i64), Error> {
        trace!("{}", stringify!(domain_event_rtc_change_msg));
        let res: Option<RemoteDomainEventRtcChangeMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventRtcChangeMsg { dom, offset } = res;
        Ok((dom, offset))
    }
    fn domain_event_tray_change_msg(
        &mut self,
    ) -> Result<(RemoteNonnullDomain, String, i32), Error> {
        trace!("{}", stringify!(domain_event_tray_change_msg));
        let res: Option<RemoteDomainEventTrayChangeMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventTrayChangeMsg {
            dom,
            dev_alias,
            reason,
        } = res;
        Ok((dom, dev_alias, reason))
    }
    fn domain_event_watchdog_msg(&mut self) -> Result<(RemoteNonnullDomain, i32), Error> {
        trace!("{}", stringify!(domain_event_watchdog_msg));
        let res: Option<RemoteDomainEventWatchdogMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventWatchdogMsg { dom, action } = res;
        Ok((dom, action))
    }
    fn network_event_lifecycle_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullNetwork, i32, i32), Error> {
        trace!("{}", stringify!(network_event_lifecycle_msg));
        let res: Option<RemoteNetworkEventLifecycleMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteNetworkEventLifecycleMsg {
            callback_id,
            net,
            event,
            detail,
        } = res;
        Ok((callback_id, net, event, detail))
    }
    fn node_device_event_lifecycle_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullNodeDevice, i32, i32), Error> {
        trace!("{}", stringify!(node_device_event_lifecycle_msg));
        let res: Option<RemoteNodeDeviceEventLifecycleMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteNodeDeviceEventLifecycleMsg {
            callback_id,
            dev,
            event,
            detail,
        } = res;
        Ok((callback_id, dev, event, detail))
    }
    fn node_device_event_update_msg(&mut self) -> Result<(i32, RemoteNonnullNodeDevice), Error> {
        trace!("{}", stringify!(node_device_event_update_msg));
        let res: Option<RemoteNodeDeviceEventUpdateMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteNodeDeviceEventUpdateMsg { callback_id, dev } = res;
        Ok((callback_id, dev))
    }
    fn secret_event_lifecycle_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullSecret, i32, i32), Error> {
        trace!("{}", stringify!(secret_event_lifecycle_msg));
        let res: Option<RemoteSecretEventLifecycleMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteSecretEventLifecycleMsg {
            callback_id,
            secret,
            event,
            detail,
        } = res;
        Ok((callback_id, secret, event, detail))
    }
    fn secret_event_value_changed_msg(&mut self) -> Result<(i32, RemoteNonnullSecret), Error> {
        trace!("{}", stringify!(secret_event_value_changed_msg));
        let res: Option<RemoteSecretEventValueChangedMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteSecretEventValueChangedMsg {
            callback_id,
            secret,
        } = res;
        Ok((callback_id, secret))
    }
    fn storage_pool_event_lifecycle_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullStoragePool, i32, i32), Error> {
        trace!("{}", stringify!(storage_pool_event_lifecycle_msg));
        let res: Option<RemoteStoragePoolEventLifecycleMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteStoragePoolEventLifecycleMsg {
            callback_id,
            pool,
            event,
            detail,
        } = res;
        Ok((callback_id, pool, event, detail))
    }
    fn storage_pool_event_refresh_msg(&mut self) -> Result<(i32, RemoteNonnullStoragePool), Error> {
        trace!("{}", stringify!(storage_pool_event_refresh_msg));
        let res: Option<RemoteStoragePoolEventRefreshMsg> = msg(self)?;
        let res = res.unwrap();
        let RemoteStoragePoolEventRefreshMsg { callback_id, pool } = res;
        Ok((callback_id, pool))
    }
    fn storage_vol_upload_data(&mut self, buf: &[u8]) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_upload_data));
        upload(self, RemoteProcedure::RemoteProcStorageVolUpload, buf)
    }
    fn storage_vol_upload_hole(&mut self, length: i64, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_upload_hole));
        send_hole(
            self,
            RemoteProcedure::RemoteProcStorageVolUpload,
            length,
            flags,
        )
    }
    fn storage_vol_upload_complete(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_upload_complete));
        upload_completed(self, RemoteProcedure::RemoteProcStorageVolUpload)
    }
}
fn call<S, D>(
    client: &mut (impl Libvirt + ?Sized),
    procedure: RemoteProcedure,
    args: Option<S>,
) -> Result<Option<D>, Error>
where
    S: Serialize,
    D: DeserializeOwned,
{
    client.serial_add(1);
    send(
        client,
        procedure,
        protocol::VirNetMessageType::VirNetCall,
        protocol::VirNetMessageStatus::VirNetOk,
        args.map(|a| VirNetRequest::Data(a)),
    )?;
    match recv(client)? {
        Some(VirNetResponse::Data(res)) => Ok(Some(res)),
        None => Ok(None),
        _ => unreachable!(),
    }
}
fn msg<D>(client: &mut (impl Libvirt + ?Sized)) -> Result<Option<D>, Error>
where
    D: DeserializeOwned,
{
    match recv(client)? {
        Some(VirNetResponse::Data(res)) => Ok(Some(res)),
        None => Ok(None),
        _ => unreachable!(),
    }
}
fn download(client: &mut (impl Libvirt + ?Sized)) -> Result<Option<VirNetStream>, Error> {
    let body: Option<VirNetResponse<()>> = recv(client)?;
    match body {
        Some(VirNetResponse::Stream(stream)) => Ok(Some(stream)),
        None => Ok(None),
        _ => unreachable!(),
    }
}
fn upload(
    client: &mut (impl Libvirt + ?Sized),
    procedure: RemoteProcedure,
    buf: &[u8],
) -> Result<(), Error> {
    let bytes = VirNetStream::Raw(buf.to_vec());
    let req: Option<VirNetRequest<()>> = Some(VirNetRequest::Stream(bytes));
    send(
        client,
        procedure,
        protocol::VirNetMessageType::VirNetStream,
        protocol::VirNetMessageStatus::VirNetContinue,
        req,
    )?;
    Ok(())
}
fn send_hole(
    client: &mut (impl Libvirt + ?Sized),
    procedure: RemoteProcedure,
    length: i64,
    flags: u32,
) -> Result<(), Error> {
    let hole = VirNetStream::Hole(protocol::VirNetStreamHole { length, flags });
    let args: Option<VirNetRequest<()>> = Some(VirNetRequest::Stream(hole));
    send(
        client,
        procedure,
        protocol::VirNetMessageType::VirNetStreamHole,
        protocol::VirNetMessageStatus::VirNetContinue,
        args,
    )?;
    Ok(())
}
fn upload_completed(
    client: &mut (impl Libvirt + ?Sized),
    procedure: RemoteProcedure,
) -> Result<(), Error> {
    let req: Option<VirNetRequest<()>> = None;
    send(
        client,
        procedure,
        protocol::VirNetMessageType::VirNetStream,
        protocol::VirNetMessageStatus::VirNetOk,
        req,
    )?;
    let _res: Option<VirNetResponse<()>> = recv(client)?;
    Ok(())
}
fn send<S>(
    client: &mut (impl Libvirt + ?Sized),
    procedure: RemoteProcedure,
    req_type: protocol::VirNetMessageType,
    req_status: protocol::VirNetMessageStatus,
    args: Option<VirNetRequest<S>>,
) -> Result<usize, Error>
where
    S: Serialize,
{
    let mut req_len: u32 = 4;
    let req_header = protocol::VirNetMessageHeader {
        prog: REMOTE_PROGRAM,
        vers: REMOTE_PROTOCOL_VERSION,
        proc: procedure as i32,
        r#type: req_type,
        serial: client.serial(),
        status: req_status,
    };
    let req_header_bytes = serde_xdr::to_bytes(&req_header).map_err(Error::SerializeError)?;
    req_len += req_header_bytes.len() as u32;
    let mut args_bytes = None;
    match args {
        Some(VirNetRequest::Data(data)) => {
            let body = serde_xdr::to_bytes(&data).map_err(Error::SerializeError)?;
            req_len += body.len() as u32;
            args_bytes = Some(body);
        }
        Some(VirNetRequest::Stream(VirNetStream::Raw(bytes))) => {
            req_len += bytes.len() as u32;
            args_bytes = Some(bytes);
        }
        Some(VirNetRequest::Stream(VirNetStream::Hole(hole))) => {
            let body = serde_xdr::to_bytes(&hole).map_err(Error::SerializeError)?;
            req_len += body.len() as u32;
            args_bytes = Some(body);
        }
        None => {}
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
    Ok(req_len as usize)
}
fn recv<D>(client: &mut (impl Libvirt + ?Sized)) -> Result<Option<VirNetResponse<D>>, Error>
where
    D: DeserializeOwned,
{
    let res_len = read_pkt_len(client)?;
    let res_header = read_res_header(client)?;
    let body_len = res_len - 28;
    if body_len == 0 {
        return Ok(None);
    }
    Ok(Some(read_res_body(client, &res_header, body_len)?))
}
fn read_pkt_len(client: &mut (impl Libvirt + ?Sized)) -> Result<usize, Error> {
    let mut res_len_bytes = [0; 4];
    client
        .inner()
        .read_exact(&mut res_len_bytes)
        .map_err(Error::ReceiveError)?;
    Ok(u32::from_be_bytes(res_len_bytes) as usize)
}
fn read_res_header(
    client: &mut (impl Libvirt + ?Sized),
) -> Result<protocol::VirNetMessageHeader, Error> {
    let mut res_header_bytes = [0; 24];
    client
        .inner()
        .read_exact(&mut res_header_bytes)
        .map_err(Error::ReceiveError)?;
    serde_xdr::from_bytes::<protocol::VirNetMessageHeader>(&res_header_bytes)
        .map_err(Error::DeserializeError)
}
fn read_res_body<D>(
    client: &mut (impl Libvirt + ?Sized),
    res_header: &protocol::VirNetMessageHeader,
    size: usize,
) -> Result<VirNetResponse<D>, Error>
where
    D: DeserializeOwned,
{
    let mut res_body_bytes = vec![0u8; size];
    client
        .inner()
        .read_exact(&mut res_body_bytes)
        .map_err(Error::ReceiveError)?;
    if res_header.status == protocol::VirNetMessageStatus::VirNetError {
        let res = serde_xdr::from_bytes::<protocol::VirNetMessageError>(&res_body_bytes)
            .map_err(Error::DeserializeError)?;
        Err(Error::ProtocolError(res))
    } else {
        match res_header.r#type {
            protocol::VirNetMessageType::VirNetReply
            | protocol::VirNetMessageType::VirNetMessage => {
                let data =
                    serde_xdr::from_bytes::<D>(&res_body_bytes).map_err(Error::DeserializeError)?;
                Ok(VirNetResponse::Data(data))
            }
            protocol::VirNetMessageType::VirNetStream => {
                let stream = VirNetStream::Raw(res_body_bytes);
                Ok(VirNetResponse::Stream(stream))
            }
            protocol::VirNetMessageType::VirNetStreamHole => {
                let hole = serde_xdr::from_bytes::<protocol::VirNetStreamHole>(&res_body_bytes)
                    .map_err(Error::DeserializeError)?;
                let stream = VirNetStream::Hole(hole);
                Ok(VirNetResponse::Stream(stream))
            }
            _ => unreachable!(),
        }
    }
}
