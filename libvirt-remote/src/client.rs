use crate::binding::*;
use crate::error::Error;
use crate::protocol;
use log::trace;
use serde::{Serialize, de::DeserializeOwned};
use std::io::{Read, Write};
use std::net::TcpStream;
#[cfg(target_family = "unix")]
use std::os::unix::net::UnixStream;
pub trait ReadWrite: Read + Write {
    fn clone(&self) -> Result<Box<dyn ReadWrite>, Error>;
}
impl ReadWrite for TcpStream {
    fn clone(&self) -> Result<Box<dyn ReadWrite>, Error> {
        let s = self.try_clone().map_err(Error::SocketError)?;
        Ok(Box::new(s))
    }
}
#[cfg(target_family = "unix")]
impl ReadWrite for UnixStream {
    fn clone(&self) -> Result<Box<dyn ReadWrite>, Error> {
        let s = self.try_clone().map_err(Error::SocketError)?;
        Ok(Box::new(s))
    }
}
pub struct Client {
    inner: Box<dyn ReadWrite>,
    serial: u32,
}
pub struct VirNetStreamResponse {
    inner: Box<dyn ReadWrite>,
    header: protocol::VirNetMessageHeader,
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
    fn inner_clone(&self) -> Result<Box<dyn ReadWrite>, Error> {
        self.inner.clone()
    }
    fn serial_add(&mut self, value: u32) -> u32 {
        self.serial += value;
        self.serial
    }
}
pub trait Libvirt {
    fn inner(&mut self) -> &mut Box<dyn ReadWrite>;
    fn inner_clone(&self) -> Result<Box<dyn ReadWrite>, Error>;
    fn serial_add(&mut self, value: u32) -> u32;
    fn download(&mut self) -> Result<Option<VirNetStream>, Error> {
        download(self)
    }
    fn connect_open(&mut self, name: Option<String>, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_open));
        let req: Option<RemoteConnectOpenArgs> = Some(RemoteConnectOpenArgs { name, flags });
        let (_header, _res) =
            call::<RemoteConnectOpenArgs, ()>(self, RemoteProcedure::RemoteProcConnectOpen, req)?;
        Ok(())
    }
    fn connect_close(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_close));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(self, RemoteProcedure::RemoteProcConnectClose, req)?;
        Ok(())
    }
    fn connect_get_type(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_type));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectGetTypeRet>(
            self,
            RemoteProcedure::RemoteProcConnectGetType,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetTypeRet { r#type } = res;
        Ok(r#type)
    }
    fn connect_get_version(&mut self) -> Result<u64, Error> {
        trace!("{}", stringify!(connect_get_version));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectGetVersionRet>(
            self,
            RemoteProcedure::RemoteProcConnectGetVersion,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetVersionRet { hv_ver } = res;
        Ok(hv_ver)
    }
    fn connect_get_max_vcpus(&mut self, r#type: Option<String>) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_get_max_vcpus));
        let req: Option<RemoteConnectGetMaxVcpusArgs> =
            Some(RemoteConnectGetMaxVcpusArgs { r#type });
        let (_header, res) = call::<RemoteConnectGetMaxVcpusArgs, RemoteConnectGetMaxVcpusRet>(
            self,
            RemoteProcedure::RemoteProcConnectGetMaxVcpus,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetMaxVcpusRet { max_vcpus } = res;
        Ok(max_vcpus)
    }
    fn node_get_info(&mut self) -> Result<RemoteNodeGetInfoRet, Error> {
        trace!("{}", stringify!(node_get_info));
        let req: Option<()> = None;
        let (_header, res) =
            call::<(), RemoteNodeGetInfoRet>(self, RemoteProcedure::RemoteProcNodeGetInfo, req)?;
        Ok(res.unwrap())
    }
    fn connect_get_capabilities(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_capabilities));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectGetCapabilitiesRet>(
            self,
            RemoteProcedure::RemoteProcConnectGetCapabilities,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetCapabilitiesRet { capabilities } = res;
        Ok(capabilities)
    }
    fn domain_attach_device(&mut self, dom: RemoteNonnullDomain, xml: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_attach_device));
        let req: Option<RemoteDomainAttachDeviceArgs> =
            Some(RemoteDomainAttachDeviceArgs { dom, xml });
        let (_header, _res) = call::<RemoteDomainAttachDeviceArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainAttachDevice,
            req,
        )?;
        Ok(())
    }
    fn domain_create(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_create));
        let req: Option<RemoteDomainCreateArgs> = Some(RemoteDomainCreateArgs { dom });
        let (_header, _res) =
            call::<RemoteDomainCreateArgs, ()>(self, RemoteProcedure::RemoteProcDomainCreate, req)?;
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
        let (_header, res) = call::<RemoteDomainCreateXmlArgs, RemoteDomainCreateXmlRet>(
            self,
            RemoteProcedure::RemoteProcDomainCreateXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainCreateXmlRet { dom } = res;
        Ok(dom)
    }
    fn domain_define_xml(&mut self, xml: String) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_define_xml));
        let req: Option<RemoteDomainDefineXmlArgs> = Some(RemoteDomainDefineXmlArgs { xml });
        let (_header, res) = call::<RemoteDomainDefineXmlArgs, RemoteDomainDefineXmlRet>(
            self,
            RemoteProcedure::RemoteProcDomainDefineXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainDefineXmlRet { dom } = res;
        Ok(dom)
    }
    fn domain_destroy(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_destroy));
        let req: Option<RemoteDomainDestroyArgs> = Some(RemoteDomainDestroyArgs { dom });
        let (_header, _res) = call::<RemoteDomainDestroyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainDestroy,
            req,
        )?;
        Ok(())
    }
    fn domain_detach_device(&mut self, dom: RemoteNonnullDomain, xml: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_detach_device));
        let req: Option<RemoteDomainDetachDeviceArgs> =
            Some(RemoteDomainDetachDeviceArgs { dom, xml });
        let (_header, _res) = call::<RemoteDomainDetachDeviceArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainDetachDevice,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetXmlDescArgs, RemoteDomainGetXmlDescRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_get_autostart(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_get_autostart));
        let req: Option<RemoteDomainGetAutostartArgs> = Some(RemoteDomainGetAutostartArgs { dom });
        let (_header, res) = call::<RemoteDomainGetAutostartArgs, RemoteDomainGetAutostartRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetAutostart,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetInfoArgs, RemoteDomainGetInfoRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetInfo,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetMaxMemoryArgs, RemoteDomainGetMaxMemoryRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetMaxMemory,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetMaxMemoryRet { memory } = res;
        Ok(memory)
    }
    fn domain_get_max_vcpus(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_get_max_vcpus));
        let req: Option<RemoteDomainGetMaxVcpusArgs> = Some(RemoteDomainGetMaxVcpusArgs { dom });
        let (_header, res) = call::<RemoteDomainGetMaxVcpusArgs, RemoteDomainGetMaxVcpusRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetMaxVcpus,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetMaxVcpusRet { num } = res;
        Ok(num)
    }
    fn domain_get_os_type(&mut self, dom: RemoteNonnullDomain) -> Result<String, Error> {
        trace!("{}", stringify!(domain_get_os_type));
        let req: Option<RemoteDomainGetOsTypeArgs> = Some(RemoteDomainGetOsTypeArgs { dom });
        let (_header, res) = call::<RemoteDomainGetOsTypeArgs, RemoteDomainGetOsTypeRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetOsType,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetVcpusArgs, RemoteDomainGetVcpusRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetVcpus,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetVcpusRet { info, cpumaps } = res;
        Ok((info, cpumaps))
    }
    fn connect_list_defined_domains(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_defined_domains));
        let req: Option<RemoteConnectListDefinedDomainsArgs> =
            Some(RemoteConnectListDefinedDomainsArgs { maxnames });
        let (_header, res) =
            call::<RemoteConnectListDefinedDomainsArgs, RemoteConnectListDefinedDomainsRet>(
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
        let (_header, res) = call::<RemoteDomainLookupByIdArgs, RemoteDomainLookupByIdRet>(
            self,
            RemoteProcedure::RemoteProcDomainLookupById,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainLookupByIdRet { dom } = res;
        Ok(dom)
    }
    fn domain_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_lookup_by_name));
        let req: Option<RemoteDomainLookupByNameArgs> = Some(RemoteDomainLookupByNameArgs { name });
        let (_header, res) = call::<RemoteDomainLookupByNameArgs, RemoteDomainLookupByNameRet>(
            self,
            RemoteProcedure::RemoteProcDomainLookupByName,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainLookupByUuidArgs, RemoteDomainLookupByUuidRet>(
            self,
            RemoteProcedure::RemoteProcDomainLookupByUuid,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainLookupByUuidRet { dom } = res;
        Ok(dom)
    }
    fn connect_num_of_defined_domains(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_defined_domains));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectNumOfDefinedDomainsRet>(
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
        let (_header, _res) = call::<RemoteDomainPinVcpuArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainPinVcpu,
            req,
        )?;
        Ok(())
    }
    fn domain_reboot(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_reboot));
        let req: Option<RemoteDomainRebootArgs> = Some(RemoteDomainRebootArgs { dom, flags });
        let (_header, _res) =
            call::<RemoteDomainRebootArgs, ()>(self, RemoteProcedure::RemoteProcDomainReboot, req)?;
        Ok(())
    }
    fn domain_resume(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_resume));
        let req: Option<RemoteDomainResumeArgs> = Some(RemoteDomainResumeArgs { dom });
        let (_header, _res) =
            call::<RemoteDomainResumeArgs, ()>(self, RemoteProcedure::RemoteProcDomainResume, req)?;
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
        let (_header, _res) = call::<RemoteDomainSetAutostartArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetAutostart,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetMaxMemoryArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetMaxMemory,
            req,
        )?;
        Ok(())
    }
    fn domain_set_memory(&mut self, dom: RemoteNonnullDomain, memory: u64) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_memory));
        let req: Option<RemoteDomainSetMemoryArgs> =
            Some(RemoteDomainSetMemoryArgs { dom, memory });
        let (_header, _res) = call::<RemoteDomainSetMemoryArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetMemory,
            req,
        )?;
        Ok(())
    }
    fn domain_set_vcpus(&mut self, dom: RemoteNonnullDomain, nvcpus: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_vcpus));
        let req: Option<RemoteDomainSetVcpusArgs> = Some(RemoteDomainSetVcpusArgs { dom, nvcpus });
        let (_header, _res) = call::<RemoteDomainSetVcpusArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetVcpus,
            req,
        )?;
        Ok(())
    }
    fn domain_shutdown(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_shutdown));
        let req: Option<RemoteDomainShutdownArgs> = Some(RemoteDomainShutdownArgs { dom });
        let (_header, _res) = call::<RemoteDomainShutdownArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainShutdown,
            req,
        )?;
        Ok(())
    }
    fn domain_suspend(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_suspend));
        let req: Option<RemoteDomainSuspendArgs> = Some(RemoteDomainSuspendArgs { dom });
        let (_header, _res) = call::<RemoteDomainSuspendArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSuspend,
            req,
        )?;
        Ok(())
    }
    fn domain_undefine(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_undefine));
        let req: Option<RemoteDomainUndefineArgs> = Some(RemoteDomainUndefineArgs { dom });
        let (_header, _res) = call::<RemoteDomainUndefineArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainUndefine,
            req,
        )?;
        Ok(())
    }
    fn connect_list_defined_networks(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_defined_networks));
        let req: Option<RemoteConnectListDefinedNetworksArgs> =
            Some(RemoteConnectListDefinedNetworksArgs { maxnames });
        let (_header, res) =
            call::<RemoteConnectListDefinedNetworksArgs, RemoteConnectListDefinedNetworksRet>(
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
        let (_header, res) = call::<RemoteConnectListDomainsArgs, RemoteConnectListDomainsRet>(
            self,
            RemoteProcedure::RemoteProcConnectListDomains,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListDomainsRet { ids } = res;
        Ok(ids)
    }
    fn connect_list_networks(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_networks));
        let req: Option<RemoteConnectListNetworksArgs> =
            Some(RemoteConnectListNetworksArgs { maxnames });
        let (_header, res) = call::<RemoteConnectListNetworksArgs, RemoteConnectListNetworksRet>(
            self,
            RemoteProcedure::RemoteProcConnectListNetworks,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListNetworksRet { names } = res;
        Ok(names)
    }
    fn network_create(&mut self, net: RemoteNonnullNetwork) -> Result<(), Error> {
        trace!("{}", stringify!(network_create));
        let req: Option<RemoteNetworkCreateArgs> = Some(RemoteNetworkCreateArgs { net });
        let (_header, _res) = call::<RemoteNetworkCreateArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNetworkCreate,
            req,
        )?;
        Ok(())
    }
    fn network_create_xml(&mut self, xml: String) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_create_xml));
        let req: Option<RemoteNetworkCreateXmlArgs> = Some(RemoteNetworkCreateXmlArgs { xml });
        let (_header, res) = call::<RemoteNetworkCreateXmlArgs, RemoteNetworkCreateXmlRet>(
            self,
            RemoteProcedure::RemoteProcNetworkCreateXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkCreateXmlRet { net } = res;
        Ok(net)
    }
    fn network_define_xml(&mut self, xml: String) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_define_xml));
        let req: Option<RemoteNetworkDefineXmlArgs> = Some(RemoteNetworkDefineXmlArgs { xml });
        let (_header, res) = call::<RemoteNetworkDefineXmlArgs, RemoteNetworkDefineXmlRet>(
            self,
            RemoteProcedure::RemoteProcNetworkDefineXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkDefineXmlRet { net } = res;
        Ok(net)
    }
    fn network_destroy(&mut self, net: RemoteNonnullNetwork) -> Result<(), Error> {
        trace!("{}", stringify!(network_destroy));
        let req: Option<RemoteNetworkDestroyArgs> = Some(RemoteNetworkDestroyArgs { net });
        let (_header, _res) = call::<RemoteNetworkDestroyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNetworkDestroy,
            req,
        )?;
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
        let (_header, res) = call::<RemoteNetworkGetXmlDescArgs, RemoteNetworkGetXmlDescRet>(
            self,
            RemoteProcedure::RemoteProcNetworkGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn network_get_autostart(&mut self, net: RemoteNonnullNetwork) -> Result<i32, Error> {
        trace!("{}", stringify!(network_get_autostart));
        let req: Option<RemoteNetworkGetAutostartArgs> =
            Some(RemoteNetworkGetAutostartArgs { net });
        let (_header, res) = call::<RemoteNetworkGetAutostartArgs, RemoteNetworkGetAutostartRet>(
            self,
            RemoteProcedure::RemoteProcNetworkGetAutostart,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkGetAutostartRet { autostart } = res;
        Ok(autostart)
    }
    fn network_get_bridge_name(&mut self, net: RemoteNonnullNetwork) -> Result<String, Error> {
        trace!("{}", stringify!(network_get_bridge_name));
        let req: Option<RemoteNetworkGetBridgeNameArgs> =
            Some(RemoteNetworkGetBridgeNameArgs { net });
        let (_header, res) = call::<RemoteNetworkGetBridgeNameArgs, RemoteNetworkGetBridgeNameRet>(
            self,
            RemoteProcedure::RemoteProcNetworkGetBridgeName,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkGetBridgeNameRet { name } = res;
        Ok(name)
    }
    fn network_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_lookup_by_name));
        let req: Option<RemoteNetworkLookupByNameArgs> =
            Some(RemoteNetworkLookupByNameArgs { name });
        let (_header, res) = call::<RemoteNetworkLookupByNameArgs, RemoteNetworkLookupByNameRet>(
            self,
            RemoteProcedure::RemoteProcNetworkLookupByName,
            req,
        )?;
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
        let (_header, res) = call::<RemoteNetworkLookupByUuidArgs, RemoteNetworkLookupByUuidRet>(
            self,
            RemoteProcedure::RemoteProcNetworkLookupByUuid,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteNetworkSetAutostartArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNetworkSetAutostart,
            req,
        )?;
        Ok(())
    }
    fn network_undefine(&mut self, net: RemoteNonnullNetwork) -> Result<(), Error> {
        trace!("{}", stringify!(network_undefine));
        let req: Option<RemoteNetworkUndefineArgs> = Some(RemoteNetworkUndefineArgs { net });
        let (_header, _res) = call::<RemoteNetworkUndefineArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNetworkUndefine,
            req,
        )?;
        Ok(())
    }
    fn connect_num_of_defined_networks(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_defined_networks));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectNumOfDefinedNetworksRet>(
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
        let (_header, res) = call::<(), RemoteConnectNumOfDomainsRet>(
            self,
            RemoteProcedure::RemoteProcConnectNumOfDomains,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNumOfDomainsRet { num } = res;
        Ok(num)
    }
    fn connect_num_of_networks(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_networks));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectNumOfNetworksRet>(
            self,
            RemoteProcedure::RemoteProcConnectNumOfNetworks,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainCoreDumpArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainCoreDump,
            req,
        )?;
        Ok(())
    }
    fn domain_restore(&mut self, from: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_restore));
        let req: Option<RemoteDomainRestoreArgs> = Some(RemoteDomainRestoreArgs { from });
        let (_header, _res) = call::<RemoteDomainRestoreArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainRestore,
            req,
        )?;
        Ok(())
    }
    fn domain_save(&mut self, dom: RemoteNonnullDomain, to: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_save));
        let req: Option<RemoteDomainSaveArgs> = Some(RemoteDomainSaveArgs { dom, to });
        let (_header, _res) =
            call::<RemoteDomainSaveArgs, ()>(self, RemoteProcedure::RemoteProcDomainSave, req)?;
        Ok(())
    }
    fn domain_get_scheduler_type(
        &mut self,
        dom: RemoteNonnullDomain,
    ) -> Result<(String, i32), Error> {
        trace!("{}", stringify!(domain_get_scheduler_type));
        let req: Option<RemoteDomainGetSchedulerTypeArgs> =
            Some(RemoteDomainGetSchedulerTypeArgs { dom });
        let (_header, res) = call::<
            RemoteDomainGetSchedulerTypeArgs,
            RemoteDomainGetSchedulerTypeRet,
        >(
            self, RemoteProcedure::RemoteProcDomainGetSchedulerType, req
        )?;
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
        let (_header, res) =
            call::<RemoteDomainGetSchedulerParametersArgs, RemoteDomainGetSchedulerParametersRet>(
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
        let (_header, _res) = call::<RemoteDomainSetSchedulerParametersArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetSchedulerParameters,
            req,
        )?;
        Ok(())
    }
    fn connect_get_hostname(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_hostname));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectGetHostnameRet>(
            self,
            RemoteProcedure::RemoteProcConnectGetHostname,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetHostnameRet { hostname } = res;
        Ok(hostname)
    }
    fn connect_supports_feature(&mut self, feature: i32) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_supports_feature));
        let req: Option<RemoteConnectSupportsFeatureArgs> =
            Some(RemoteConnectSupportsFeatureArgs { feature });
        let (_header, res) = call::<
            RemoteConnectSupportsFeatureArgs,
            RemoteConnectSupportsFeatureRet,
        >(
            self, RemoteProcedure::RemoteProcConnectSupportsFeature, req
        )?;
        let res = res.unwrap();
        let RemoteConnectSupportsFeatureRet { supported } = res;
        Ok(supported)
    }
    fn domain_migrate_prepare(
        &mut self,
        uri_in: Option<String>,
        flags: u64,
        dname: Option<String>,
        bandwidth: u64,
    ) -> Result<(Vec<u8>, Option<String>), Error> {
        trace!("{}", stringify!(domain_migrate_prepare));
        let req: Option<RemoteDomainMigratePrepareArgs> = Some(RemoteDomainMigratePrepareArgs {
            uri_in,
            flags,
            dname,
            bandwidth,
        });
        let (_header, res) = call::<RemoteDomainMigratePrepareArgs, RemoteDomainMigratePrepareRet>(
            self,
            RemoteProcedure::RemoteProcDomainMigratePrepare,
            req,
        )?;
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
        bandwidth: u64,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_migrate_perform));
        let req: Option<RemoteDomainMigratePerformArgs> = Some(RemoteDomainMigratePerformArgs {
            dom,
            cookie,
            uri,
            flags,
            dname,
            bandwidth,
        });
        let (_header, _res) = call::<RemoteDomainMigratePerformArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainMigratePerform,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainMigrateFinishArgs, RemoteDomainMigrateFinishRet>(
            self,
            RemoteProcedure::RemoteProcDomainMigrateFinish,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainBlockStatsArgs, RemoteDomainBlockStatsRet>(
            self,
            RemoteProcedure::RemoteProcDomainBlockStats,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainInterfaceStatsArgs, RemoteDomainInterfaceStatsRet>(
            self,
            RemoteProcedure::RemoteProcDomainInterfaceStats,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn auth_list(&mut self) -> Result<Vec<RemoteAuthType>, Error> {
        trace!("{}", stringify!(auth_list));
        let req: Option<()> = None;
        let (_header, res) =
            call::<(), RemoteAuthListRet>(self, RemoteProcedure::RemoteProcAuthList, req)?;
        let res = res.unwrap();
        let RemoteAuthListRet { types } = res;
        Ok(types)
    }
    fn auth_sasl_init(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(auth_sasl_init));
        let req: Option<()> = None;
        let (_header, res) =
            call::<(), RemoteAuthSaslInitRet>(self, RemoteProcedure::RemoteProcAuthSaslInit, req)?;
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
        let (_header, res) = call::<RemoteAuthSaslStartArgs, RemoteAuthSaslStartRet>(
            self,
            RemoteProcedure::RemoteProcAuthSaslStart,
            req,
        )?;
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
        let (_header, res) = call::<RemoteAuthSaslStepArgs, RemoteAuthSaslStepRet>(
            self,
            RemoteProcedure::RemoteProcAuthSaslStep,
            req,
        )?;
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
        let (_header, res) =
            call::<(), RemoteAuthPolkitRet>(self, RemoteProcedure::RemoteProcAuthPolkit, req)?;
        let res = res.unwrap();
        let RemoteAuthPolkitRet { complete } = res;
        Ok(complete)
    }
    fn connect_num_of_storage_pools(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_storage_pools));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectNumOfStoragePoolsRet>(
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
        let (_header, res) =
            call::<RemoteConnectListStoragePoolsArgs, RemoteConnectListStoragePoolsRet>(
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
        let (_header, res) = call::<(), RemoteConnectNumOfDefinedStoragePoolsRet>(
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
        let (_header, res) = call::<
            RemoteConnectListDefinedStoragePoolsArgs,
            RemoteConnectListDefinedStoragePoolsRet,
        >(
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
        let (_header, res) = call::<
            RemoteConnectFindStoragePoolSourcesArgs,
            RemoteConnectFindStoragePoolSourcesRet,
        >(
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
        let (_header, res) = call::<RemoteStoragePoolCreateXmlArgs, RemoteStoragePoolCreateXmlRet>(
            self,
            RemoteProcedure::RemoteProcStoragePoolCreateXml,
            req,
        )?;
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
        let (_header, res) = call::<RemoteStoragePoolDefineXmlArgs, RemoteStoragePoolDefineXmlRet>(
            self,
            RemoteProcedure::RemoteProcStoragePoolDefineXml,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteStoragePoolCreateArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStoragePoolCreate,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteStoragePoolBuildArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStoragePoolBuild,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_destroy(&mut self, pool: RemoteNonnullStoragePool) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_destroy));
        let req: Option<RemoteStoragePoolDestroyArgs> = Some(RemoteStoragePoolDestroyArgs { pool });
        let (_header, _res) = call::<RemoteStoragePoolDestroyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStoragePoolDestroy,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteStoragePoolDeleteArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStoragePoolDelete,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_undefine(&mut self, pool: RemoteNonnullStoragePool) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_undefine));
        let req: Option<RemoteStoragePoolUndefineArgs> =
            Some(RemoteStoragePoolUndefineArgs { pool });
        let (_header, _res) = call::<RemoteStoragePoolUndefineArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStoragePoolUndefine,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteStoragePoolRefreshArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStoragePoolRefresh,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_lookup_by_name(
        &mut self,
        name: String,
    ) -> Result<RemoteNonnullStoragePool, Error> {
        trace!("{}", stringify!(storage_pool_lookup_by_name));
        let req: Option<RemoteStoragePoolLookupByNameArgs> =
            Some(RemoteStoragePoolLookupByNameArgs { name });
        let (_header, res) =
            call::<RemoteStoragePoolLookupByNameArgs, RemoteStoragePoolLookupByNameRet>(
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
        let (_header, res) =
            call::<RemoteStoragePoolLookupByUuidArgs, RemoteStoragePoolLookupByUuidRet>(
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
        let (_header, res) =
            call::<RemoteStoragePoolLookupByVolumeArgs, RemoteStoragePoolLookupByVolumeRet>(
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
        let (_header, res) = call::<RemoteStoragePoolGetInfoArgs, RemoteStoragePoolGetInfoRet>(
            self,
            RemoteProcedure::RemoteProcStoragePoolGetInfo,
            req,
        )?;
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
        let (_header, res) = call::<RemoteStoragePoolGetXmlDescArgs, RemoteStoragePoolGetXmlDescRet>(
            self,
            RemoteProcedure::RemoteProcStoragePoolGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStoragePoolGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn storage_pool_get_autostart(&mut self, pool: RemoteNonnullStoragePool) -> Result<i32, Error> {
        trace!("{}", stringify!(storage_pool_get_autostart));
        let req: Option<RemoteStoragePoolGetAutostartArgs> =
            Some(RemoteStoragePoolGetAutostartArgs { pool });
        let (_header, res) =
            call::<RemoteStoragePoolGetAutostartArgs, RemoteStoragePoolGetAutostartRet>(
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
        let (_header, _res) = call::<RemoteStoragePoolSetAutostartArgs, ()>(
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
        let (_header, res) =
            call::<RemoteStoragePoolNumOfVolumesArgs, RemoteStoragePoolNumOfVolumesRet>(
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
        let (_header, res) = call::<
            RemoteStoragePoolListVolumesArgs,
            RemoteStoragePoolListVolumesRet,
        >(
            self, RemoteProcedure::RemoteProcStoragePoolListVolumes, req
        )?;
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
        let (_header, res) = call::<RemoteStorageVolCreateXmlArgs, RemoteStorageVolCreateXmlRet>(
            self,
            RemoteProcedure::RemoteProcStorageVolCreateXml,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteStorageVolDeleteArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStorageVolDelete,
            req,
        )?;
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
        let (_header, res) = call::<
            RemoteStorageVolLookupByNameArgs,
            RemoteStorageVolLookupByNameRet,
        >(
            self, RemoteProcedure::RemoteProcStorageVolLookupByName, req
        )?;
        let res = res.unwrap();
        let RemoteStorageVolLookupByNameRet { vol } = res;
        Ok(vol)
    }
    fn storage_vol_lookup_by_key(&mut self, key: String) -> Result<RemoteNonnullStorageVol, Error> {
        trace!("{}", stringify!(storage_vol_lookup_by_key));
        let req: Option<RemoteStorageVolLookupByKeyArgs> =
            Some(RemoteStorageVolLookupByKeyArgs { key });
        let (_header, res) = call::<RemoteStorageVolLookupByKeyArgs, RemoteStorageVolLookupByKeyRet>(
            self,
            RemoteProcedure::RemoteProcStorageVolLookupByKey,
            req,
        )?;
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
        let (_header, res) = call::<
            RemoteStorageVolLookupByPathArgs,
            RemoteStorageVolLookupByPathRet,
        >(
            self, RemoteProcedure::RemoteProcStorageVolLookupByPath, req
        )?;
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
        let (_header, res) = call::<RemoteStorageVolGetInfoArgs, RemoteStorageVolGetInfoRet>(
            self,
            RemoteProcedure::RemoteProcStorageVolGetInfo,
            req,
        )?;
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
        let (_header, res) = call::<RemoteStorageVolGetXmlDescArgs, RemoteStorageVolGetXmlDescRet>(
            self,
            RemoteProcedure::RemoteProcStorageVolGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStorageVolGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn storage_vol_get_path(&mut self, vol: RemoteNonnullStorageVol) -> Result<String, Error> {
        trace!("{}", stringify!(storage_vol_get_path));
        let req: Option<RemoteStorageVolGetPathArgs> = Some(RemoteStorageVolGetPathArgs { vol });
        let (_header, res) = call::<RemoteStorageVolGetPathArgs, RemoteStorageVolGetPathRet>(
            self,
            RemoteProcedure::RemoteProcStorageVolGetPath,
            req,
        )?;
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
        let (_header, res) = call::<
            RemoteNodeGetCellsFreeMemoryArgs,
            RemoteNodeGetCellsFreeMemoryRet,
        >(
            self, RemoteProcedure::RemoteProcNodeGetCellsFreeMemory, req
        )?;
        let res = res.unwrap();
        let RemoteNodeGetCellsFreeMemoryRet { cells } = res;
        Ok(cells)
    }
    fn node_get_free_memory(&mut self) -> Result<u64, Error> {
        trace!("{}", stringify!(node_get_free_memory));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteNodeGetFreeMemoryRet>(
            self,
            RemoteProcedure::RemoteProcNodeGetFreeMemory,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainBlockPeekArgs, RemoteDomainBlockPeekRet>(
            self,
            RemoteProcedure::RemoteProcDomainBlockPeek,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainMemoryPeekArgs, RemoteDomainMemoryPeekRet>(
            self,
            RemoteProcedure::RemoteProcDomainMemoryPeek,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMemoryPeekRet { buffer } = res;
        Ok(buffer)
    }
    fn connect_domain_event_register(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_domain_event_register));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectDomainEventRegisterRet>(
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
        let (_header, res) = call::<(), RemoteConnectDomainEventDeregisterRet>(
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
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventLifecycle, req)?;
        Ok(())
    }
    fn domain_migrate_prepare2(
        &mut self,
        uri_in: Option<String>,
        flags: u64,
        dname: Option<String>,
        bandwidth: u64,
        dom_xml: String,
    ) -> Result<(Vec<u8>, Option<String>), Error> {
        trace!("{}", stringify!(domain_migrate_prepare2));
        let req: Option<RemoteDomainMigratePrepare2Args> = Some(RemoteDomainMigratePrepare2Args {
            uri_in,
            flags,
            dname,
            bandwidth,
            dom_xml,
        });
        let (_header, res) = call::<RemoteDomainMigratePrepare2Args, RemoteDomainMigratePrepare2Ret>(
            self,
            RemoteProcedure::RemoteProcDomainMigratePrepare2,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainMigrateFinish2Args, RemoteDomainMigrateFinish2Ret>(
            self,
            RemoteProcedure::RemoteProcDomainMigrateFinish2,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainMigrateFinish2Ret { ddom } = res;
        Ok(ddom)
    }
    fn connect_get_uri(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_uri));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectGetUriRet>(
            self,
            RemoteProcedure::RemoteProcConnectGetUri,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetUriRet { uri } = res;
        Ok(uri)
    }
    fn node_num_of_devices(&mut self, cap: Option<String>, flags: u32) -> Result<i32, Error> {
        trace!("{}", stringify!(node_num_of_devices));
        let req: Option<RemoteNodeNumOfDevicesArgs> =
            Some(RemoteNodeNumOfDevicesArgs { cap, flags });
        let (_header, res) = call::<RemoteNodeNumOfDevicesArgs, RemoteNodeNumOfDevicesRet>(
            self,
            RemoteProcedure::RemoteProcNodeNumOfDevices,
            req,
        )?;
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
        let (_header, res) = call::<RemoteNodeListDevicesArgs, RemoteNodeListDevicesRet>(
            self,
            RemoteProcedure::RemoteProcNodeListDevices,
            req,
        )?;
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
        let (_header, res) = call::<
            RemoteNodeDeviceLookupByNameArgs,
            RemoteNodeDeviceLookupByNameRet,
        >(
            self, RemoteProcedure::RemoteProcNodeDeviceLookupByName, req
        )?;
        let res = res.unwrap();
        let RemoteNodeDeviceLookupByNameRet { dev } = res;
        Ok(dev)
    }
    fn node_device_get_xml_desc(&mut self, name: String, flags: u32) -> Result<String, Error> {
        trace!("{}", stringify!(node_device_get_xml_desc));
        let req: Option<RemoteNodeDeviceGetXmlDescArgs> =
            Some(RemoteNodeDeviceGetXmlDescArgs { name, flags });
        let (_header, res) = call::<RemoteNodeDeviceGetXmlDescArgs, RemoteNodeDeviceGetXmlDescRet>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNodeDeviceGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn node_device_get_parent(&mut self, name: String) -> Result<Option<String>, Error> {
        trace!("{}", stringify!(node_device_get_parent));
        let req: Option<RemoteNodeDeviceGetParentArgs> =
            Some(RemoteNodeDeviceGetParentArgs { name });
        let (_header, res) = call::<RemoteNodeDeviceGetParentArgs, RemoteNodeDeviceGetParentRet>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceGetParent,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNodeDeviceGetParentRet { parent_name } = res;
        Ok(parent_name)
    }
    fn node_device_num_of_caps(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_num_of_caps));
        let req: Option<RemoteNodeDeviceNumOfCapsArgs> =
            Some(RemoteNodeDeviceNumOfCapsArgs { name });
        let (_header, res) = call::<RemoteNodeDeviceNumOfCapsArgs, RemoteNodeDeviceNumOfCapsRet>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceNumOfCaps,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNodeDeviceNumOfCapsRet { num } = res;
        Ok(num)
    }
    fn node_device_list_caps(&mut self, name: String, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(node_device_list_caps));
        let req: Option<RemoteNodeDeviceListCapsArgs> =
            Some(RemoteNodeDeviceListCapsArgs { name, maxnames });
        let (_header, res) = call::<RemoteNodeDeviceListCapsArgs, RemoteNodeDeviceListCapsRet>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceListCaps,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNodeDeviceListCapsRet { names } = res;
        Ok(names)
    }
    fn node_device_dettach(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_dettach));
        let req: Option<RemoteNodeDeviceDettachArgs> = Some(RemoteNodeDeviceDettachArgs { name });
        let (_header, _res) = call::<RemoteNodeDeviceDettachArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceDettach,
            req,
        )?;
        Ok(())
    }
    fn node_device_re_attach(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_re_attach));
        let req: Option<RemoteNodeDeviceReAttachArgs> = Some(RemoteNodeDeviceReAttachArgs { name });
        let (_header, _res) = call::<RemoteNodeDeviceReAttachArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceReAttach,
            req,
        )?;
        Ok(())
    }
    fn node_device_reset(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_reset));
        let req: Option<RemoteNodeDeviceResetArgs> = Some(RemoteNodeDeviceResetArgs { name });
        let (_header, _res) = call::<RemoteNodeDeviceResetArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceReset,
            req,
        )?;
        Ok(())
    }
    fn domain_get_security_label(
        &mut self,
        dom: RemoteNonnullDomain,
    ) -> Result<(Vec<i8>, i32), Error> {
        trace!("{}", stringify!(domain_get_security_label));
        let req: Option<RemoteDomainGetSecurityLabelArgs> =
            Some(RemoteDomainGetSecurityLabelArgs { dom });
        let (_header, res) = call::<
            RemoteDomainGetSecurityLabelArgs,
            RemoteDomainGetSecurityLabelRet,
        >(
            self, RemoteProcedure::RemoteProcDomainGetSecurityLabel, req
        )?;
        let res = res.unwrap();
        let RemoteDomainGetSecurityLabelRet { label, enforcing } = res;
        Ok((label, enforcing))
    }
    fn node_get_security_model(&mut self) -> Result<(Vec<i8>, Vec<i8>), Error> {
        trace!("{}", stringify!(node_get_security_model));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteNodeGetSecurityModelRet>(
            self,
            RemoteProcedure::RemoteProcNodeGetSecurityModel,
            req,
        )?;
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
        let (_header, res) = call::<RemoteNodeDeviceCreateXmlArgs, RemoteNodeDeviceCreateXmlRet>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceCreateXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNodeDeviceCreateXmlRet { dev } = res;
        Ok(dev)
    }
    fn node_device_destroy(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_destroy));
        let req: Option<RemoteNodeDeviceDestroyArgs> = Some(RemoteNodeDeviceDestroyArgs { name });
        let (_header, _res) = call::<RemoteNodeDeviceDestroyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceDestroy,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteStorageVolCreateXmlFromArgs, RemoteStorageVolCreateXmlFromRet>(
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
        let (_header, res) = call::<(), RemoteConnectNumOfInterfacesRet>(
            self,
            RemoteProcedure::RemoteProcConnectNumOfInterfaces,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNumOfInterfacesRet { num } = res;
        Ok(num)
    }
    fn connect_list_interfaces(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_interfaces));
        let req: Option<RemoteConnectListInterfacesArgs> =
            Some(RemoteConnectListInterfacesArgs { maxnames });
        let (_header, res) = call::<RemoteConnectListInterfacesArgs, RemoteConnectListInterfacesRet>(
            self,
            RemoteProcedure::RemoteProcConnectListInterfaces,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListInterfacesRet { names } = res;
        Ok(names)
    }
    fn interface_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullInterface, Error> {
        trace!("{}", stringify!(interface_lookup_by_name));
        let req: Option<RemoteInterfaceLookupByNameArgs> =
            Some(RemoteInterfaceLookupByNameArgs { name });
        let (_header, res) = call::<RemoteInterfaceLookupByNameArgs, RemoteInterfaceLookupByNameRet>(
            self,
            RemoteProcedure::RemoteProcInterfaceLookupByName,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteInterfaceLookupByMacStringArgs, RemoteInterfaceLookupByMacStringRet>(
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
        let (_header, res) = call::<RemoteInterfaceGetXmlDescArgs, RemoteInterfaceGetXmlDescRet>(
            self,
            RemoteProcedure::RemoteProcInterfaceGetXmlDesc,
            req,
        )?;
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
        let (_header, res) = call::<RemoteInterfaceDefineXmlArgs, RemoteInterfaceDefineXmlRet>(
            self,
            RemoteProcedure::RemoteProcInterfaceDefineXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteInterfaceDefineXmlRet { iface } = res;
        Ok(iface)
    }
    fn interface_undefine(&mut self, iface: RemoteNonnullInterface) -> Result<(), Error> {
        trace!("{}", stringify!(interface_undefine));
        let req: Option<RemoteInterfaceUndefineArgs> = Some(RemoteInterfaceUndefineArgs { iface });
        let (_header, _res) = call::<RemoteInterfaceUndefineArgs, ()>(
            self,
            RemoteProcedure::RemoteProcInterfaceUndefine,
            req,
        )?;
        Ok(())
    }
    fn interface_create(&mut self, iface: RemoteNonnullInterface, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_create));
        let req: Option<RemoteInterfaceCreateArgs> =
            Some(RemoteInterfaceCreateArgs { iface, flags });
        let (_header, _res) = call::<RemoteInterfaceCreateArgs, ()>(
            self,
            RemoteProcedure::RemoteProcInterfaceCreate,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteInterfaceDestroyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcInterfaceDestroy,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteConnectDomainXmlFromNativeArgs, RemoteConnectDomainXmlFromNativeRet>(
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
        let (_header, res) =
            call::<RemoteConnectDomainXmlToNativeArgs, RemoteConnectDomainXmlToNativeRet>(
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
        let (_header, res) = call::<(), RemoteConnectNumOfDefinedInterfacesRet>(
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
        let (_header, res) =
            call::<RemoteConnectListDefinedInterfacesArgs, RemoteConnectListDefinedInterfacesRet>(
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
        let (_header, res) = call::<(), RemoteConnectNumOfSecretsRet>(
            self,
            RemoteProcedure::RemoteProcConnectNumOfSecrets,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNumOfSecretsRet { num } = res;
        Ok(num)
    }
    fn connect_list_secrets(&mut self, maxuuids: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_secrets));
        let req: Option<RemoteConnectListSecretsArgs> =
            Some(RemoteConnectListSecretsArgs { maxuuids });
        let (_header, res) = call::<RemoteConnectListSecretsArgs, RemoteConnectListSecretsRet>(
            self,
            RemoteProcedure::RemoteProcConnectListSecrets,
            req,
        )?;
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
        let (_header, res) = call::<RemoteSecretLookupByUuidArgs, RemoteSecretLookupByUuidRet>(
            self,
            RemoteProcedure::RemoteProcSecretLookupByUuid,
            req,
        )?;
        let res = res.unwrap();
        let RemoteSecretLookupByUuidRet { secret } = res;
        Ok(secret)
    }
    fn secret_define_xml(&mut self, xml: String, flags: u32) -> Result<RemoteNonnullSecret, Error> {
        trace!("{}", stringify!(secret_define_xml));
        let req: Option<RemoteSecretDefineXmlArgs> = Some(RemoteSecretDefineXmlArgs { xml, flags });
        let (_header, res) = call::<RemoteSecretDefineXmlArgs, RemoteSecretDefineXmlRet>(
            self,
            RemoteProcedure::RemoteProcSecretDefineXml,
            req,
        )?;
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
        let (_header, res) = call::<RemoteSecretGetXmlDescArgs, RemoteSecretGetXmlDescRet>(
            self,
            RemoteProcedure::RemoteProcSecretGetXmlDesc,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteSecretSetValueArgs, ()>(
            self,
            RemoteProcedure::RemoteProcSecretSetValue,
            req,
        )?;
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
        let (_header, res) = call::<RemoteSecretGetValueArgs, RemoteSecretGetValueRet>(
            self,
            RemoteProcedure::RemoteProcSecretGetValue,
            req,
        )?;
        let res = res.unwrap();
        let RemoteSecretGetValueRet { value } = res;
        Ok(value)
    }
    fn secret_undefine(&mut self, secret: RemoteNonnullSecret) -> Result<(), Error> {
        trace!("{}", stringify!(secret_undefine));
        let req: Option<RemoteSecretUndefineArgs> = Some(RemoteSecretUndefineArgs { secret });
        let (_header, _res) = call::<RemoteSecretUndefineArgs, ()>(
            self,
            RemoteProcedure::RemoteProcSecretUndefine,
            req,
        )?;
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
        let (_header, res) = call::<RemoteSecretLookupByUsageArgs, RemoteSecretLookupByUsageRet>(
            self,
            RemoteProcedure::RemoteProcSecretLookupByUsage,
            req,
        )?;
        let res = res.unwrap();
        let RemoteSecretLookupByUsageRet { secret } = res;
        Ok(secret)
    }
    fn domain_migrate_prepare_tunnel(
        &mut self,
        flags: u64,
        dname: Option<String>,
        bandwidth: u64,
        dom_xml: String,
    ) -> Result<VirNetStreamResponse, Error> {
        trace!("{}", stringify!(domain_migrate_prepare_tunnel));
        let req: Option<RemoteDomainMigratePrepareTunnelArgs> =
            Some(RemoteDomainMigratePrepareTunnelArgs {
                flags,
                dname,
                bandwidth,
                dom_xml,
            });
        let (header, _res) = call::<RemoteDomainMigratePrepareTunnelArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainMigratePrepareTunnel,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            header,
        };
        Ok(res)
    }
    fn connect_is_secure(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_is_secure));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectIsSecureRet>(
            self,
            RemoteProcedure::RemoteProcConnectIsSecure,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectIsSecureRet { secure } = res;
        Ok(secure)
    }
    fn domain_is_active(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_is_active));
        let req: Option<RemoteDomainIsActiveArgs> = Some(RemoteDomainIsActiveArgs { dom });
        let (_header, res) = call::<RemoteDomainIsActiveArgs, RemoteDomainIsActiveRet>(
            self,
            RemoteProcedure::RemoteProcDomainIsActive,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainIsActiveRet { active } = res;
        Ok(active)
    }
    fn domain_is_persistent(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_is_persistent));
        let req: Option<RemoteDomainIsPersistentArgs> = Some(RemoteDomainIsPersistentArgs { dom });
        let (_header, res) = call::<RemoteDomainIsPersistentArgs, RemoteDomainIsPersistentRet>(
            self,
            RemoteProcedure::RemoteProcDomainIsPersistent,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn network_is_active(&mut self, net: RemoteNonnullNetwork) -> Result<i32, Error> {
        trace!("{}", stringify!(network_is_active));
        let req: Option<RemoteNetworkIsActiveArgs> = Some(RemoteNetworkIsActiveArgs { net });
        let (_header, res) = call::<RemoteNetworkIsActiveArgs, RemoteNetworkIsActiveRet>(
            self,
            RemoteProcedure::RemoteProcNetworkIsActive,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkIsActiveRet { active } = res;
        Ok(active)
    }
    fn network_is_persistent(&mut self, net: RemoteNonnullNetwork) -> Result<i32, Error> {
        trace!("{}", stringify!(network_is_persistent));
        let req: Option<RemoteNetworkIsPersistentArgs> =
            Some(RemoteNetworkIsPersistentArgs { net });
        let (_header, res) = call::<RemoteNetworkIsPersistentArgs, RemoteNetworkIsPersistentRet>(
            self,
            RemoteProcedure::RemoteProcNetworkIsPersistent,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn storage_pool_is_active(&mut self, pool: RemoteNonnullStoragePool) -> Result<i32, Error> {
        trace!("{}", stringify!(storage_pool_is_active));
        let req: Option<RemoteStoragePoolIsActiveArgs> =
            Some(RemoteStoragePoolIsActiveArgs { pool });
        let (_header, res) = call::<RemoteStoragePoolIsActiveArgs, RemoteStoragePoolIsActiveRet>(
            self,
            RemoteProcedure::RemoteProcStoragePoolIsActive,
            req,
        )?;
        let res = res.unwrap();
        let RemoteStoragePoolIsActiveRet { active } = res;
        Ok(active)
    }
    fn storage_pool_is_persistent(&mut self, pool: RemoteNonnullStoragePool) -> Result<i32, Error> {
        trace!("{}", stringify!(storage_pool_is_persistent));
        let req: Option<RemoteStoragePoolIsPersistentArgs> =
            Some(RemoteStoragePoolIsPersistentArgs { pool });
        let (_header, res) =
            call::<RemoteStoragePoolIsPersistentArgs, RemoteStoragePoolIsPersistentRet>(
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
        let (_header, res) = call::<RemoteInterfaceIsActiveArgs, RemoteInterfaceIsActiveRet>(
            self,
            RemoteProcedure::RemoteProcInterfaceIsActive,
            req,
        )?;
        let res = res.unwrap();
        let RemoteInterfaceIsActiveRet { active } = res;
        Ok(active)
    }
    fn connect_get_lib_version(&mut self) -> Result<u64, Error> {
        trace!("{}", stringify!(connect_get_lib_version));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectGetLibVersionRet>(
            self,
            RemoteProcedure::RemoteProcConnectGetLibVersion,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectGetLibVersionRet { lib_ver } = res;
        Ok(lib_ver)
    }
    fn connect_compare_cpu(&mut self, xml: String, flags: u32) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_compare_cpu));
        let req: Option<RemoteConnectCompareCpuArgs> =
            Some(RemoteConnectCompareCpuArgs { xml, flags });
        let (_header, res) = call::<RemoteConnectCompareCpuArgs, RemoteConnectCompareCpuRet>(
            self,
            RemoteProcedure::RemoteProcConnectCompareCpu,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainMemoryStatsArgs, RemoteDomainMemoryStatsRet>(
            self,
            RemoteProcedure::RemoteProcDomainMemoryStats,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainAttachDeviceFlagsArgs, ()>(
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
        let (_header, _res) = call::<RemoteDomainDetachDeviceFlagsArgs, ()>(
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
        let (_header, res) = call::<RemoteConnectBaselineCpuArgs, RemoteConnectBaselineCpuRet>(
            self,
            RemoteProcedure::RemoteProcConnectBaselineCpu,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetJobInfoArgs, RemoteDomainGetJobInfoRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetJobInfo,
            req,
        )?;
        Ok(res.unwrap())
    }
    fn domain_abort_job(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_abort_job));
        let req: Option<RemoteDomainAbortJobArgs> = Some(RemoteDomainAbortJobArgs { dom });
        let (_header, _res) = call::<RemoteDomainAbortJobArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainAbortJob,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_wipe(&mut self, vol: RemoteNonnullStorageVol, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_wipe));
        let req: Option<RemoteStorageVolWipeArgs> = Some(RemoteStorageVolWipeArgs { vol, flags });
        let (_header, _res) = call::<RemoteStorageVolWipeArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStorageVolWipe,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainMigrateSetMaxDowntimeArgs, ()>(
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
        let (_header, _res) = call::<RemoteConnectDomainEventRegisterAnyArgs, ()>(
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
        let (_header, _res) = call::<RemoteConnectDomainEventDeregisterAnyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcConnectDomainEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn domain_event_reboot(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_reboot));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventReboot, req)?;
        Ok(())
    }
    fn domain_event_rtc_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_rtc_change));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventRtcChange, req)?;
        Ok(())
    }
    fn domain_event_watchdog(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_watchdog));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventWatchdog, req)?;
        Ok(())
    }
    fn domain_event_io_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_io_error));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventIoError, req)?;
        Ok(())
    }
    fn domain_event_graphics(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_graphics));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventGraphics, req)?;
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
        let (_header, _res) = call::<RemoteDomainUpdateDeviceFlagsArgs, ()>(
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
        let (_header, res) = call::<RemoteNwfilterLookupByNameArgs, RemoteNwfilterLookupByNameRet>(
            self,
            RemoteProcedure::RemoteProcNwfilterLookupByName,
            req,
        )?;
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
        let (_header, res) = call::<RemoteNwfilterLookupByUuidArgs, RemoteNwfilterLookupByUuidRet>(
            self,
            RemoteProcedure::RemoteProcNwfilterLookupByUuid,
            req,
        )?;
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
        let (_header, res) = call::<RemoteNwfilterGetXmlDescArgs, RemoteNwfilterGetXmlDescRet>(
            self,
            RemoteProcedure::RemoteProcNwfilterGetXmlDesc,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNwfilterGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn connect_num_of_nwfilters(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_nwfilters));
        let req: Option<()> = None;
        let (_header, res) = call::<(), RemoteConnectNumOfNwfiltersRet>(
            self,
            RemoteProcedure::RemoteProcConnectNumOfNwfilters,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectNumOfNwfiltersRet { num } = res;
        Ok(num)
    }
    fn connect_list_nwfilters(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_nwfilters));
        let req: Option<RemoteConnectListNwfiltersArgs> =
            Some(RemoteConnectListNwfiltersArgs { maxnames });
        let (_header, res) = call::<RemoteConnectListNwfiltersArgs, RemoteConnectListNwfiltersRet>(
            self,
            RemoteProcedure::RemoteProcConnectListNwfilters,
            req,
        )?;
        let res = res.unwrap();
        let RemoteConnectListNwfiltersRet { names } = res;
        Ok(names)
    }
    fn nwfilter_define_xml(&mut self, xml: String) -> Result<RemoteNonnullNwfilter, Error> {
        trace!("{}", stringify!(nwfilter_define_xml));
        let req: Option<RemoteNwfilterDefineXmlArgs> = Some(RemoteNwfilterDefineXmlArgs { xml });
        let (_header, res) = call::<RemoteNwfilterDefineXmlArgs, RemoteNwfilterDefineXmlRet>(
            self,
            RemoteProcedure::RemoteProcNwfilterDefineXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNwfilterDefineXmlRet { nwfilter } = res;
        Ok(nwfilter)
    }
    fn nwfilter_undefine(&mut self, nwfilter: RemoteNonnullNwfilter) -> Result<(), Error> {
        trace!("{}", stringify!(nwfilter_undefine));
        let req: Option<RemoteNwfilterUndefineArgs> = Some(RemoteNwfilterUndefineArgs { nwfilter });
        let (_header, _res) = call::<RemoteNwfilterUndefineArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNwfilterUndefine,
            req,
        )?;
        Ok(())
    }
    fn domain_managed_save(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_managed_save));
        let req: Option<RemoteDomainManagedSaveArgs> =
            Some(RemoteDomainManagedSaveArgs { dom, flags });
        let (_header, _res) = call::<RemoteDomainManagedSaveArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainManagedSave,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteDomainHasManagedSaveImageArgs, RemoteDomainHasManagedSaveImageRet>(
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
        let (_header, _res) = call::<RemoteDomainManagedSaveRemoveArgs, ()>(
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
        let (_header, res) =
            call::<RemoteDomainSnapshotCreateXmlArgs, RemoteDomainSnapshotCreateXmlRet>(
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
        let (_header, res) =
            call::<RemoteDomainSnapshotGetXmlDescArgs, RemoteDomainSnapshotGetXmlDescRet>(
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
        let (_header, res) = call::<RemoteDomainSnapshotNumArgs, RemoteDomainSnapshotNumRet>(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotNum,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteDomainSnapshotListNamesArgs, RemoteDomainSnapshotListNamesRet>(
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
        let (_header, res) =
            call::<RemoteDomainSnapshotLookupByNameArgs, RemoteDomainSnapshotLookupByNameRet>(
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
        let (_header, res) =
            call::<RemoteDomainHasCurrentSnapshotArgs, RemoteDomainHasCurrentSnapshotRet>(
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
        let (_header, res) = call::<RemoteDomainSnapshotCurrentArgs, RemoteDomainSnapshotCurrentRet>(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotCurrent,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainRevertToSnapshotArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainRevertToSnapshot,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSnapshotDeleteArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSnapshotDelete,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetBlockInfoArgs, RemoteDomainGetBlockInfoRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetBlockInfo,
            req,
        )?;
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
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) = call::<RemoteDomainCreateWithFlagsArgs, RemoteDomainCreateWithFlagsRet>(
            self,
            RemoteProcedure::RemoteProcDomainCreateWithFlags,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetMemoryParametersArgs, ()>(
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
        let (_header, res) =
            call::<RemoteDomainGetMemoryParametersArgs, RemoteDomainGetMemoryParametersRet>(
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
        let (_header, _res) = call::<RemoteDomainSetVcpusFlagsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetVcpusFlags,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetVcpusFlagsArgs, RemoteDomainGetVcpusFlagsRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetVcpusFlags,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetVcpusFlagsRet { num } = res;
        Ok(num)
    }
    fn domain_open_console(
        &mut self,
        dom: RemoteNonnullDomain,
        dev_name: Option<String>,
        flags: u32,
    ) -> Result<VirNetStreamResponse, Error> {
        trace!("{}", stringify!(domain_open_console));
        let req: Option<RemoteDomainOpenConsoleArgs> = Some(RemoteDomainOpenConsoleArgs {
            dom,
            dev_name,
            flags,
        });
        let (header, _res) = call::<RemoteDomainOpenConsoleArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainOpenConsole,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            header,
        };
        Ok(res)
    }
    fn domain_is_updated(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_is_updated));
        let req: Option<RemoteDomainIsUpdatedArgs> = Some(RemoteDomainIsUpdatedArgs { dom });
        let (_header, res) = call::<RemoteDomainIsUpdatedArgs, RemoteDomainIsUpdatedRet>(
            self,
            RemoteProcedure::RemoteProcDomainIsUpdated,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainIsUpdatedRet { updated } = res;
        Ok(updated)
    }
    fn connect_get_sysinfo(&mut self, flags: u32) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_sysinfo));
        let req: Option<RemoteConnectGetSysinfoArgs> = Some(RemoteConnectGetSysinfoArgs { flags });
        let (_header, res) = call::<RemoteConnectGetSysinfoArgs, RemoteConnectGetSysinfoRet>(
            self,
            RemoteProcedure::RemoteProcConnectGetSysinfo,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetMemoryFlagsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetMemoryFlags,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetBlkioParametersArgs, ()>(
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
        let (_header, res) =
            call::<RemoteDomainGetBlkioParametersArgs, RemoteDomainGetBlkioParametersRet>(
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
        let (_header, _res) = call::<RemoteDomainMigrateSetMaxSpeedArgs, ()>(
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
    ) -> Result<VirNetStreamResponse, Error> {
        trace!("{}", stringify!(storage_vol_upload));
        let req: Option<RemoteStorageVolUploadArgs> = Some(RemoteStorageVolUploadArgs {
            vol,
            offset,
            length,
            flags,
        });
        let (header, _res) = call::<RemoteStorageVolUploadArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStorageVolUpload,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            header,
        };
        Ok(res)
    }
    fn storage_vol_download(
        &mut self,
        vol: RemoteNonnullStorageVol,
        offset: u64,
        length: u64,
        flags: u32,
    ) -> Result<VirNetStreamResponse, Error> {
        trace!("{}", stringify!(storage_vol_download));
        let req: Option<RemoteStorageVolDownloadArgs> = Some(RemoteStorageVolDownloadArgs {
            vol,
            offset,
            length,
            flags,
        });
        let (header, _res) = call::<RemoteStorageVolDownloadArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStorageVolDownload,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            header,
        };
        Ok(res)
    }
    fn domain_inject_nmi(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_inject_nmi));
        let req: Option<RemoteDomainInjectNmiArgs> = Some(RemoteDomainInjectNmiArgs { dom, flags });
        let (_header, _res) = call::<RemoteDomainInjectNmiArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainInjectNmi,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainScreenshotArgs, RemoteDomainScreenshotRet>(
            self,
            RemoteProcedure::RemoteProcDomainScreenshot,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetStateArgs, RemoteDomainGetStateRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetState,
            req,
        )?;
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
        bandwidth: u64,
    ) -> Result<(Vec<u8>, String), Error> {
        trace!("{}", stringify!(domain_migrate_begin3));
        let req: Option<RemoteDomainMigrateBegin3Args> = Some(RemoteDomainMigrateBegin3Args {
            dom,
            xmlin,
            flags,
            dname,
            bandwidth,
        });
        let (_header, res) = call::<RemoteDomainMigrateBegin3Args, RemoteDomainMigrateBegin3Ret>(
            self,
            RemoteProcedure::RemoteProcDomainMigrateBegin3,
            req,
        )?;
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
        bandwidth: u64,
        dom_xml: String,
    ) -> Result<(Vec<u8>, Option<String>), Error> {
        trace!("{}", stringify!(domain_migrate_prepare3));
        let req: Option<RemoteDomainMigratePrepare3Args> = Some(RemoteDomainMigratePrepare3Args {
            cookie_in,
            uri_in,
            flags,
            dname,
            bandwidth,
            dom_xml,
        });
        let (_header, res) = call::<RemoteDomainMigratePrepare3Args, RemoteDomainMigratePrepare3Ret>(
            self,
            RemoteProcedure::RemoteProcDomainMigratePrepare3,
            req,
        )?;
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
        bandwidth: u64,
        dom_xml: String,
    ) -> Result<Vec<u8>, Error> {
        trace!("{}", stringify!(domain_migrate_prepare_tunnel3));
        let req: Option<RemoteDomainMigratePrepareTunnel3Args> =
            Some(RemoteDomainMigratePrepareTunnel3Args {
                cookie_in,
                flags,
                dname,
                bandwidth,
                dom_xml,
            });
        let (_header, res) =
            call::<RemoteDomainMigratePrepareTunnel3Args, RemoteDomainMigratePrepareTunnel3Ret>(
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
        let (_header, res) = call::<RemoteDomainMigratePerform3Args, RemoteDomainMigratePerform3Ret>(
            self,
            RemoteProcedure::RemoteProcDomainMigratePerform3,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainMigrateFinish3Args, RemoteDomainMigrateFinish3Ret>(
            self,
            RemoteProcedure::RemoteProcDomainMigrateFinish3,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainMigrateConfirm3Args, ()>(
            self,
            RemoteProcedure::RemoteProcDomainMigrateConfirm3,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetSchedulerParametersFlagsArgs, ()>(
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
        let (_header, _res) = call::<RemoteInterfaceChangeBeginArgs, ()>(
            self,
            RemoteProcedure::RemoteProcInterfaceChangeBegin,
            req,
        )?;
        Ok(())
    }
    fn interface_change_commit(&mut self, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_commit));
        let req: Option<RemoteInterfaceChangeCommitArgs> =
            Some(RemoteInterfaceChangeCommitArgs { flags });
        let (_header, _res) = call::<RemoteInterfaceChangeCommitArgs, ()>(
            self,
            RemoteProcedure::RemoteProcInterfaceChangeCommit,
            req,
        )?;
        Ok(())
    }
    fn interface_change_rollback(&mut self, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_rollback));
        let req: Option<RemoteInterfaceChangeRollbackArgs> =
            Some(RemoteInterfaceChangeRollbackArgs { flags });
        let (_header, _res) = call::<RemoteInterfaceChangeRollbackArgs, ()>(
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
        let (_header, res) = call::<
            RemoteDomainGetSchedulerParametersFlagsArgs,
            RemoteDomainGetSchedulerParametersFlagsRet,
        >(
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
        let (_header, _res) = call::<(), ()>(
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
        let (_header, _res) = call::<RemoteDomainPinVcpuFlagsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainPinVcpuFlags,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSendKeyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSendKey,
            req,
        )?;
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
        let (_header, res) = call::<RemoteNodeGetCpuStatsArgs, RemoteNodeGetCpuStatsRet>(
            self,
            RemoteProcedure::RemoteProcNodeGetCpuStats,
            req,
        )?;
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
        let (_header, res) = call::<RemoteNodeGetMemoryStatsArgs, RemoteNodeGetMemoryStatsRet>(
            self,
            RemoteProcedure::RemoteProcNodeGetMemoryStats,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetControlInfoArgs, RemoteDomainGetControlInfoRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetControlInfo,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetVcpuPinInfoArgs, RemoteDomainGetVcpuPinInfoRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetVcpuPinInfo,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainGetVcpuPinInfoRet { cpumaps, num } = res;
        Ok((cpumaps, num))
    }
    fn domain_undefine_flags(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_undefine_flags));
        let req: Option<RemoteDomainUndefineFlagsArgs> =
            Some(RemoteDomainUndefineFlagsArgs { dom, flags });
        let (_header, _res) = call::<RemoteDomainUndefineFlagsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainUndefineFlags,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSaveFlagsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSaveFlags,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainRestoreFlagsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainRestoreFlags,
            req,
        )?;
        Ok(())
    }
    fn domain_destroy_flags(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_destroy_flags));
        let req: Option<RemoteDomainDestroyFlagsArgs> =
            Some(RemoteDomainDestroyFlagsArgs { dom, flags });
        let (_header, _res) = call::<RemoteDomainDestroyFlagsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainDestroyFlags,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteDomainSaveImageGetXmlDescArgs, RemoteDomainSaveImageGetXmlDescRet>(
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
        let (_header, _res) = call::<RemoteDomainSaveImageDefineXmlArgs, ()>(
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
        let (_header, _res) = call::<RemoteDomainBlockJobAbortArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainBlockJobAbort,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetBlockJobInfoArgs, RemoteDomainGetBlockJobInfoRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetBlockJobInfo,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainBlockJobSetSpeedArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainBlockJobSetSpeed,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainBlockPullArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainBlockPull,
            req,
        )?;
        Ok(())
    }
    fn domain_event_block_job(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_job));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventBlockJob, req)?;
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
        let (_header, res) =
            call::<RemoteDomainMigrateGetMaxSpeedArgs, RemoteDomainMigrateGetMaxSpeedRet>(
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
        let (_header, res) = call::<RemoteDomainBlockStatsFlagsArgs, RemoteDomainBlockStatsFlagsRet>(
            self,
            RemoteProcedure::RemoteProcDomainBlockStatsFlags,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteDomainSnapshotGetParentArgs, RemoteDomainSnapshotGetParentRet>(
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
        let (_header, _res) =
            call::<RemoteDomainResetArgs, ()>(self, RemoteProcedure::RemoteProcDomainReset, req)?;
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
        let (_header, res) =
            call::<RemoteDomainSnapshotNumChildrenArgs, RemoteDomainSnapshotNumChildrenRet>(
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
        let (_header, res) = call::<
            RemoteDomainSnapshotListChildrenNamesArgs,
            RemoteDomainSnapshotListChildrenNamesRet,
        >(
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
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventDiskChange, req)?;
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
        let (_header, _res) = call::<RemoteDomainOpenGraphicsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainOpenGraphics,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteNodeSuspendForDurationArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNodeSuspendForDuration,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainBlockResizeArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainBlockResize,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetBlockIoTuneArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetBlockIoTune,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetBlockIoTuneArgs, RemoteDomainGetBlockIoTuneRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetBlockIoTune,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetNumaParametersArgs, ()>(
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
        let (_header, res) =
            call::<RemoteDomainGetNumaParametersArgs, RemoteDomainGetNumaParametersRet>(
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
        let (_header, _res) = call::<RemoteDomainSetInterfaceParametersArgs, ()>(
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
        let (_header, res) =
            call::<RemoteDomainGetInterfaceParametersArgs, RemoteDomainGetInterfaceParametersRet>(
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
        let (_header, _res) = call::<RemoteDomainShutdownFlagsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainShutdownFlags,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteStorageVolWipePatternArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStorageVolWipePattern,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteStorageVolResizeArgs, ()>(
            self,
            RemoteProcedure::RemoteProcStorageVolResize,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainPmSuspendForDurationArgs, ()>(
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
        let (_header, res) = call::<RemoteDomainGetCpuStatsArgs, RemoteDomainGetCpuStatsRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetCpuStats,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetDiskErrorsArgs, RemoteDomainGetDiskErrorsRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetDiskErrors,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetMetadataArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetMetadata,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetMetadataArgs, RemoteDomainGetMetadataRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetMetadata,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainBlockRebaseArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainBlockRebase,
            req,
        )?;
        Ok(())
    }
    fn domain_pm_wakeup(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pm_wakeup));
        let req: Option<RemoteDomainPmWakeupArgs> = Some(RemoteDomainPmWakeupArgs { dom, flags });
        let (_header, _res) = call::<RemoteDomainPmWakeupArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainPmWakeup,
            req,
        )?;
        Ok(())
    }
    fn domain_event_tray_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_tray_change));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventTrayChange, req)?;
        Ok(())
    }
    fn domain_event_pmwakeup(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmwakeup));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventPmwakeup, req)?;
        Ok(())
    }
    fn domain_event_pmsuspend(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmsuspend));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventPmsuspend, req)?;
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
        let (_header, res) =
            call::<RemoteDomainSnapshotIsCurrentArgs, RemoteDomainSnapshotIsCurrentRet>(
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
        let (_header, res) =
            call::<RemoteDomainSnapshotHasMetadataArgs, RemoteDomainSnapshotHasMetadataRet>(
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
        let (_header, res) = call::<RemoteConnectListAllDomainsArgs, RemoteConnectListAllDomainsRet>(
            self,
            RemoteProcedure::RemoteProcConnectListAllDomains,
            req,
        )?;
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
        let (_header, res) = call::<
            RemoteDomainListAllSnapshotsArgs,
            RemoteDomainListAllSnapshotsRet,
        >(
            self, RemoteProcedure::RemoteProcDomainListAllSnapshots, req
        )?;
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
        let (_header, res) = call::<
            RemoteDomainSnapshotListAllChildrenArgs,
            RemoteDomainSnapshotListAllChildrenRet,
        >(
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
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) = call::<RemoteDomainGetHostnameArgs, RemoteDomainGetHostnameRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetHostname,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteDomainGetSecurityLabelListArgs, RemoteDomainGetSecurityLabelListRet>(
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
        let (_header, _res) = call::<RemoteDomainPinEmulatorArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainPinEmulator,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteDomainGetEmulatorPinInfoArgs, RemoteDomainGetEmulatorPinInfoRet>(
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
        let (_header, res) =
            call::<RemoteConnectListAllStoragePoolsArgs, RemoteConnectListAllStoragePoolsRet>(
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
        let (_header, res) =
            call::<RemoteStoragePoolListAllVolumesArgs, RemoteStoragePoolListAllVolumesRet>(
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
        let (_header, res) = call::<
            RemoteConnectListAllNetworksArgs,
            RemoteConnectListAllNetworksRet,
        >(
            self, RemoteProcedure::RemoteProcConnectListAllNetworks, req
        )?;
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
        let (_header, res) =
            call::<RemoteConnectListAllInterfacesArgs, RemoteConnectListAllInterfacesRet>(
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
        let (_header, res) =
            call::<RemoteConnectListAllNodeDevicesArgs, RemoteConnectListAllNodeDevicesRet>(
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
        let (_header, res) =
            call::<RemoteConnectListAllNwfiltersArgs, RemoteConnectListAllNwfiltersRet>(
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
        let (_header, res) = call::<RemoteConnectListAllSecretsArgs, RemoteConnectListAllSecretsRet>(
            self,
            RemoteProcedure::RemoteProcConnectListAllSecrets,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteNodeSetMemoryParametersArgs, ()>(
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
        let (_header, res) =
            call::<RemoteNodeGetMemoryParametersArgs, RemoteNodeGetMemoryParametersRet>(
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
        let (_header, _res) = call::<RemoteDomainBlockCommitArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainBlockCommit,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteNetworkUpdateArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNetworkUpdate,
            req,
        )?;
        Ok(())
    }
    fn domain_event_pmsuspend_disk(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmsuspend_disk));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) = call::<RemoteNodeGetCpuMapArgs, RemoteNodeGetCpuMapRet>(
            self,
            RemoteProcedure::RemoteProcNodeGetCpuMap,
            req,
        )?;
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
        let (_header, _res) =
            call::<RemoteDomainFstrimArgs, ()>(self, RemoteProcedure::RemoteProcDomainFstrim, req)?;
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
        let (_header, _res) = call::<RemoteDomainSendProcessSignalArgs, ()>(
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
    ) -> Result<VirNetStreamResponse, Error> {
        trace!("{}", stringify!(domain_open_channel));
        let req: Option<RemoteDomainOpenChannelArgs> =
            Some(RemoteDomainOpenChannelArgs { dom, name, flags });
        let (header, _res) = call::<RemoteDomainOpenChannelArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainOpenChannel,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            header,
        };
        Ok(res)
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
        let (_header, res) = call::<
            RemoteNodeDeviceLookupScsiHostByWwnArgs,
            RemoteNodeDeviceLookupScsiHostByWwnRet,
        >(
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
        let (_header, res) = call::<RemoteDomainGetJobStatsArgs, RemoteDomainGetJobStatsRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetJobStats,
            req,
        )?;
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
        let (_header, res) = call::<
            RemoteDomainMigrateGetCompressionCacheArgs,
            RemoteDomainMigrateGetCompressionCacheRet,
        >(
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
        let (_header, _res) = call::<RemoteDomainMigrateSetCompressionCacheArgs, ()>(
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
        let (_header, _res) = call::<RemoteNodeDeviceDetachFlagsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceDetachFlags,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteDomainMigrateBegin3ParamsArgs, RemoteDomainMigrateBegin3ParamsRet>(
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
        let (_header, res) =
            call::<RemoteDomainMigratePrepare3ParamsArgs, RemoteDomainMigratePrepare3ParamsRet>(
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
        let (_header, res) = call::<
            RemoteDomainMigratePrepareTunnel3ParamsArgs,
            RemoteDomainMigratePrepareTunnel3ParamsRet,
        >(
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
        let (_header, res) =
            call::<RemoteDomainMigratePerform3ParamsArgs, RemoteDomainMigratePerform3ParamsRet>(
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
        let (_header, res) =
            call::<RemoteDomainMigrateFinish3ParamsArgs, RemoteDomainMigrateFinish3ParamsRet>(
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
        let (_header, _res) = call::<RemoteDomainMigrateConfirm3ParamsArgs, ()>(
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
        let (_header, _res) = call::<RemoteDomainSetMemoryStatsPeriodArgs, ()>(
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
        let (_header, res) =
            call::<RemoteDomainCreateXmlWithFilesArgs, RemoteDomainCreateXmlWithFilesRet>(
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
        let (_header, res) = call::<RemoteDomainCreateWithFilesArgs, RemoteDomainCreateWithFilesRet>(
            self,
            RemoteProcedure::RemoteProcDomainCreateWithFiles,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainCreateWithFilesRet { dom } = res;
        Ok(dom)
    }
    fn domain_event_device_removed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_device_removed));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) =
            call::<RemoteConnectGetCpuModelNamesArgs, RemoteConnectGetCpuModelNamesRet>(
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
        let (_header, res) = call::<
            RemoteConnectNetworkEventRegisterAnyArgs,
            RemoteConnectNetworkEventRegisterAnyRet,
        >(
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
        let (_header, _res) = call::<RemoteConnectNetworkEventDeregisterAnyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcConnectNetworkEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn network_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(network_event_lifecycle));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcNetworkEventLifecycle, req)?;
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
        let (_header, res) = call::<
            RemoteConnectDomainEventCallbackRegisterAnyArgs,
            RemoteConnectDomainEventCallbackRegisterAnyRet,
        >(
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
        let (_header, _res) = call::<RemoteConnectDomainEventCallbackDeregisterAnyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcConnectDomainEventCallbackDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_lifecycle));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackLifecycle,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_reboot(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_reboot));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackReboot,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_rtc_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_rtc_change));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackRtcChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_watchdog(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_watchdog));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackWatchdog,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_io_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackIoError,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_graphics(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_graphics));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackGraphics,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_io_error_reason(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error_reason));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackIoErrorReason,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_control_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_control_error));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackControlError,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_block_job(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_block_job));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackBlockJob,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_disk_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_disk_change));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackDiskChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_tray_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_tray_change));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackTrayChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmwakeup(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmwakeup));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackPmwakeup,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmsuspend(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackPmsuspend,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_balloon_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_balloon_change));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackBalloonChange,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmsuspend_disk(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend_disk));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackPmsuspendDisk,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_device_removed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_device_removed));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, _res) = call::<RemoteDomainCoreDumpWithFormatArgs, ()>(
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
        let (_header, res) = call::<RemoteDomainFsfreezeArgs, RemoteDomainFsfreezeRet>(
            self,
            RemoteProcedure::RemoteProcDomainFsfreeze,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainFsthawArgs, RemoteDomainFsthawRet>(
            self,
            RemoteProcedure::RemoteProcDomainFsthaw,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetTimeArgs, RemoteDomainGetTimeRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetTime,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetTimeArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetTime,
            req,
        )?;
        Ok(())
    }
    fn domain_event_block_job2(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_job2));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcDomainEventBlockJob2, req)?;
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
        let (_header, res) = call::<RemoteNodeGetFreePagesArgs, RemoteNodeGetFreePagesRet>(
            self,
            RemoteProcedure::RemoteProcNodeGetFreePages,
            req,
        )?;
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
        let (_header, res) = call::<RemoteNetworkGetDhcpLeasesArgs, RemoteNetworkGetDhcpLeasesRet>(
            self,
            RemoteProcedure::RemoteProcNetworkGetDhcpLeases,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteConnectGetDomainCapabilitiesArgs, RemoteConnectGetDomainCapabilitiesRet>(
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
        let (_header, _res) = call::<RemoteDomainOpenGraphicsFdArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainOpenGraphicsFd,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteConnectGetAllDomainStatsArgs, RemoteConnectGetAllDomainStatsRet>(
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
        let (_header, _res) = call::<RemoteDomainBlockCopyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainBlockCopy,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_tunable(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_tunable));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) = call::<RemoteNodeAllocPagesArgs, RemoteNodeAllocPagesRet>(
            self,
            RemoteProcedure::RemoteProcNodeAllocPages,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNodeAllocPagesRet { ret } = res;
        Ok(ret)
    }
    fn domain_event_callback_agent_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_agent_lifecycle));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) = call::<RemoteDomainGetFsinfoArgs, RemoteDomainGetFsinfoRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetFsinfo,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainDefineXmlFlagsArgs, RemoteDomainDefineXmlFlagsRet>(
            self,
            RemoteProcedure::RemoteProcDomainDefineXmlFlags,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetIothreadInfoArgs, RemoteDomainGetIothreadInfoRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetIothreadInfo,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainPinIothreadArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainPinIothread,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteDomainInterfaceAddressesArgs, RemoteDomainInterfaceAddressesRet>(
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
        let (_header, _res) = call::<(), ()>(
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
        let (_header, _res) = call::<RemoteDomainAddIothreadArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainAddIothread,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainDelIothreadArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainDelIothread,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetUserPasswordArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetUserPassword,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainRenameArgs, RemoteDomainRenameRet>(
            self,
            RemoteProcedure::RemoteProcDomainRename,
            req,
        )?;
        let res = res.unwrap();
        let RemoteDomainRenameRet { retcode } = res;
        Ok(retcode)
    }
    fn domain_event_callback_migration_iteration(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_migration_iteration));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventCallbackMigrationIteration,
            req,
        )?;
        Ok(())
    }
    fn connect_register_close_callback(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_register_close_callback));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcConnectRegisterCloseCallback,
            req,
        )?;
        Ok(())
    }
    fn connect_unregister_close_callback(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_unregister_close_callback));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcConnectUnregisterCloseCallback,
            req,
        )?;
        Ok(())
    }
    fn connect_event_connection_closed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_event_connection_closed));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcConnectEventConnectionClosed,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_job_completed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_job_completed));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, _res) = call::<RemoteDomainMigrateStartPostCopyArgs, ()>(
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
        let (_header, res) = call::<RemoteDomainGetPerfEventsArgs, RemoteDomainGetPerfEventsRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetPerfEvents,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetPerfEventsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetPerfEvents,
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
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) = call::<
            RemoteConnectStoragePoolEventRegisterAnyArgs,
            RemoteConnectStoragePoolEventRegisterAnyRet,
        >(
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
        let (_header, _res) = call::<RemoteConnectStoragePoolEventDeregisterAnyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcConnectStoragePoolEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_event_lifecycle));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) = call::<RemoteDomainGetGuestVcpusArgs, RemoteDomainGetGuestVcpusRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetGuestVcpus,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainSetGuestVcpusArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetGuestVcpus,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_event_refresh(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_event_refresh));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) = call::<
            RemoteConnectNodeDeviceEventRegisterAnyArgs,
            RemoteConnectNodeDeviceEventRegisterAnyRet,
        >(
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
        let (_header, _res) = call::<RemoteConnectNodeDeviceEventDeregisterAnyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcConnectNodeDeviceEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn node_device_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_event_lifecycle));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceEventLifecycle,
            req,
        )?;
        Ok(())
    }
    fn node_device_event_update(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_event_update));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcNodeDeviceEventUpdate, req)?;
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
        let (_header, res) = call::<
            RemoteStorageVolGetInfoFlagsArgs,
            RemoteStorageVolGetInfoFlagsRet,
        >(
            self, RemoteProcedure::RemoteProcStorageVolGetInfoFlags, req
        )?;
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
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) = call::<
            RemoteConnectSecretEventRegisterAnyArgs,
            RemoteConnectSecretEventRegisterAnyRet,
        >(
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
        let (_header, _res) = call::<RemoteConnectSecretEventDeregisterAnyArgs, ()>(
            self,
            RemoteProcedure::RemoteProcConnectSecretEventDeregisterAny,
            req,
        )?;
        Ok(())
    }
    fn secret_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(secret_event_lifecycle));
        let req: Option<()> = None;
        let (_header, _res) =
            call::<(), ()>(self, RemoteProcedure::RemoteProcSecretEventLifecycle, req)?;
        Ok(())
    }
    fn secret_event_value_changed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(secret_event_value_changed));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, _res) = call::<RemoteDomainSetVcpuArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetVcpu,
            req,
        )?;
        Ok(())
    }
    fn domain_event_block_threshold(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_threshold));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, _res) = call::<RemoteDomainSetBlockThresholdArgs, ()>(
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
        let (_header, res) =
            call::<RemoteDomainMigrateGetMaxDowntimeArgs, RemoteDomainMigrateGetMaxDowntimeRet>(
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
        let (_header, res) =
            call::<RemoteDomainManagedSaveGetXmlDescArgs, RemoteDomainManagedSaveGetXmlDescRet>(
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
        let (_header, _res) = call::<RemoteDomainManagedSaveDefineXmlArgs, ()>(
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
        let (_header, _res) = call::<RemoteDomainSetLifecycleActionArgs, ()>(
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
        let (_header, res) = call::<
            RemoteStoragePoolLookupByTargetPathArgs,
            RemoteStoragePoolLookupByTargetPathRet,
        >(
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
        let (_header, _res) = call::<RemoteDomainDetachDeviceAliasArgs, ()>(
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
        let (_header, res) =
            call::<RemoteConnectCompareHypervisorCpuArgs, RemoteConnectCompareHypervisorCpuRet>(
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
        let (_header, res) =
            call::<RemoteConnectBaselineHypervisorCpuArgs, RemoteConnectBaselineHypervisorCpuRet>(
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
        let (_header, res) = call::<RemoteNodeGetSevInfoArgs, RemoteNodeGetSevInfoRet>(
            self,
            RemoteProcedure::RemoteProcNodeGetSevInfo,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteDomainGetLaunchSecurityInfoArgs, RemoteDomainGetLaunchSecurityInfoRet>(
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
        let (_header, res) = call::<
            RemoteNwfilterBindingLookupByPortDevArgs,
            RemoteNwfilterBindingLookupByPortDevRet,
        >(
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
        let (_header, res) =
            call::<RemoteNwfilterBindingGetXmlDescArgs, RemoteNwfilterBindingGetXmlDescRet>(
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
        let (_header, res) =
            call::<RemoteNwfilterBindingCreateXmlArgs, RemoteNwfilterBindingCreateXmlRet>(
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
        let (_header, _res) = call::<RemoteNwfilterBindingDeleteArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNwfilterBindingDelete,
            req,
        )?;
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
        let (_header, res) = call::<
            RemoteConnectListAllNwfilterBindingsArgs,
            RemoteConnectListAllNwfilterBindingsRet,
        >(
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
        let (_header, _res) = call::<RemoteDomainSetIothreadParamsArgs, ()>(
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
        let (_header, res) = call::<
            RemoteConnectGetStoragePoolCapabilitiesArgs,
            RemoteConnectGetStoragePoolCapabilitiesRet,
        >(
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
        let (_header, res) = call::<RemoteNetworkListAllPortsArgs, RemoteNetworkListAllPortsRet>(
            self,
            RemoteProcedure::RemoteProcNetworkListAllPorts,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteNetworkPortLookupByUuidArgs, RemoteNetworkPortLookupByUuidRet>(
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
        let (_header, res) = call::<RemoteNetworkPortCreateXmlArgs, RemoteNetworkPortCreateXmlRet>(
            self,
            RemoteProcedure::RemoteProcNetworkPortCreateXml,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteNetworkPortGetParametersArgs, RemoteNetworkPortGetParametersRet>(
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
        let (_header, _res) = call::<RemoteNetworkPortSetParametersArgs, ()>(
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
        let (_header, res) = call::<RemoteNetworkPortGetXmlDescArgs, RemoteNetworkPortGetXmlDescRet>(
            self,
            RemoteProcedure::RemoteProcNetworkPortGetXmlDesc,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteNetworkPortDeleteArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNetworkPortDelete,
            req,
        )?;
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
        let (_header, res) =
            call::<RemoteDomainCheckpointCreateXmlArgs, RemoteDomainCheckpointCreateXmlRet>(
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
        let (_header, res) =
            call::<RemoteDomainCheckpointGetXmlDescArgs, RemoteDomainCheckpointGetXmlDescRet>(
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
        let (_header, res) =
            call::<RemoteDomainListAllCheckpointsArgs, RemoteDomainListAllCheckpointsRet>(
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
        let (_header, res) = call::<
            RemoteDomainCheckpointListAllChildrenArgs,
            RemoteDomainCheckpointListAllChildrenRet,
        >(
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
        let (_header, res) =
            call::<RemoteDomainCheckpointLookupByNameArgs, RemoteDomainCheckpointLookupByNameRet>(
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
        let (_header, res) =
            call::<RemoteDomainCheckpointGetParentArgs, RemoteDomainCheckpointGetParentRet>(
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
        let (_header, _res) = call::<RemoteDomainCheckpointDeleteArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainCheckpointDelete,
            req,
        )?;
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
        let (_header, res) = call::<RemoteDomainGetGuestInfoArgs, RemoteDomainGetGuestInfoRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetGuestInfo,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteConnectSetIdentityArgs, ()>(
            self,
            RemoteProcedure::RemoteProcConnectSetIdentity,
            req,
        )?;
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
        let (_header, res) = call::<
            RemoteDomainAgentSetResponseTimeoutArgs,
            RemoteDomainAgentSetResponseTimeoutRet,
        >(
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
        let (_header, _res) = call::<RemoteDomainBackupBeginArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainBackupBegin,
            req,
        )?;
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
        let (_header, res) = call::<
            RemoteDomainBackupGetXmlDescArgs,
            RemoteDomainBackupGetXmlDescRet,
        >(
            self, RemoteProcedure::RemoteProcDomainBackupGetXmlDesc, req
        )?;
        let res = res.unwrap();
        let RemoteDomainBackupGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_event_memory_failure(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_memory_failure));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, res) =
            call::<RemoteDomainAuthorizedSshKeysGetArgs, RemoteDomainAuthorizedSshKeysGetRet>(
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
        let (_header, _res) = call::<RemoteDomainAuthorizedSshKeysSetArgs, ()>(
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
        let (_header, res) = call::<RemoteDomainGetMessagesArgs, RemoteDomainGetMessagesRet>(
            self,
            RemoteProcedure::RemoteProcDomainGetMessages,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainStartDirtyRateCalcArgs, ()>(
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
        let (_header, res) = call::<RemoteNodeDeviceDefineXmlArgs, RemoteNodeDeviceDefineXmlRet>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceDefineXml,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNodeDeviceDefineXmlRet { dev } = res;
        Ok(dev)
    }
    fn node_device_undefine(&mut self, name: String, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_undefine));
        let req: Option<RemoteNodeDeviceUndefineArgs> =
            Some(RemoteNodeDeviceUndefineArgs { name, flags });
        let (_header, _res) = call::<RemoteNodeDeviceUndefineArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceUndefine,
            req,
        )?;
        Ok(())
    }
    fn node_device_create(&mut self, name: String, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_create));
        let req: Option<RemoteNodeDeviceCreateArgs> =
            Some(RemoteNodeDeviceCreateArgs { name, flags });
        let (_header, _res) = call::<RemoteNodeDeviceCreateArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceCreate,
            req,
        )?;
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
        let (_header, res) = call::<
            RemoteNwfilterDefineXmlFlagsArgs,
            RemoteNwfilterDefineXmlFlagsRet,
        >(
            self, RemoteProcedure::RemoteProcNwfilterDefineXmlFlags, req
        )?;
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
        let (_header, res) = call::<RemoteNetworkDefineXmlFlagsArgs, RemoteNetworkDefineXmlFlagsRet>(
            self,
            RemoteProcedure::RemoteProcNetworkDefineXmlFlags,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkDefineXmlFlagsRet { net } = res;
        Ok(net)
    }
    fn node_device_get_autostart(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_get_autostart));
        let req: Option<RemoteNodeDeviceGetAutostartArgs> =
            Some(RemoteNodeDeviceGetAutostartArgs { name });
        let (_header, res) = call::<
            RemoteNodeDeviceGetAutostartArgs,
            RemoteNodeDeviceGetAutostartRet,
        >(
            self, RemoteProcedure::RemoteProcNodeDeviceGetAutostart, req
        )?;
        let res = res.unwrap();
        let RemoteNodeDeviceGetAutostartRet { autostart } = res;
        Ok(autostart)
    }
    fn node_device_set_autostart(&mut self, name: String, autostart: i32) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_set_autostart));
        let req: Option<RemoteNodeDeviceSetAutostartArgs> =
            Some(RemoteNodeDeviceSetAutostartArgs { name, autostart });
        let (_header, _res) = call::<RemoteNodeDeviceSetAutostartArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceSetAutostart,
            req,
        )?;
        Ok(())
    }
    fn node_device_is_persistent(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_is_persistent));
        let req: Option<RemoteNodeDeviceIsPersistentArgs> =
            Some(RemoteNodeDeviceIsPersistentArgs { name });
        let (_header, res) = call::<
            RemoteNodeDeviceIsPersistentArgs,
            RemoteNodeDeviceIsPersistentRet,
        >(
            self, RemoteProcedure::RemoteProcNodeDeviceIsPersistent, req
        )?;
        let res = res.unwrap();
        let RemoteNodeDeviceIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn node_device_is_active(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_is_active));
        let req: Option<RemoteNodeDeviceIsActiveArgs> = Some(RemoteNodeDeviceIsActiveArgs { name });
        let (_header, res) = call::<RemoteNodeDeviceIsActiveArgs, RemoteNodeDeviceIsActiveRet>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceIsActive,
            req,
        )?;
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
        let (_header, res) = call::<RemoteNetworkCreateXmlFlagsArgs, RemoteNetworkCreateXmlFlagsRet>(
            self,
            RemoteProcedure::RemoteProcNetworkCreateXmlFlags,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkCreateXmlFlagsRet { net } = res;
        Ok(net)
    }
    fn domain_event_memory_device_size_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_memory_device_size_change));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
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
        let (_header, _res) = call::<RemoteDomainSetLaunchSecurityStateArgs, ()>(
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
        let (_header, _res) = call::<RemoteDomainSaveParamsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSaveParams,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainRestoreParamsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainRestoreParams,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainAbortJobFlagsArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainAbortJobFlags,
            req,
        )?;
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
        let (_header, _res) = call::<RemoteDomainFdAssociateArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainFdAssociate,
            req,
        )?;
        Ok(())
    }
    fn network_set_metadata(
        &mut self,
        network: RemoteNonnullNetwork,
        r#type: i32,
        metadata: Option<String>,
        key: Option<String>,
        uri: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(network_set_metadata));
        let req: Option<RemoteNetworkSetMetadataArgs> = Some(RemoteNetworkSetMetadataArgs {
            network,
            r#type,
            metadata,
            key,
            uri,
            flags,
        });
        let (_header, _res) = call::<RemoteNetworkSetMetadataArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNetworkSetMetadata,
            req,
        )?;
        Ok(())
    }
    fn network_get_metadata(
        &mut self,
        network: RemoteNonnullNetwork,
        r#type: i32,
        uri: Option<String>,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(network_get_metadata));
        let req: Option<RemoteNetworkGetMetadataArgs> = Some(RemoteNetworkGetMetadataArgs {
            network,
            r#type,
            uri,
            flags,
        });
        let (_header, res) = call::<RemoteNetworkGetMetadataArgs, RemoteNetworkGetMetadataRet>(
            self,
            RemoteProcedure::RemoteProcNetworkGetMetadata,
            req,
        )?;
        let res = res.unwrap();
        let RemoteNetworkGetMetadataRet { metadata } = res;
        Ok(metadata)
    }
    fn network_event_callback_metadata_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(network_event_callback_metadata_change));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcNetworkEventCallbackMetadataChange,
            req,
        )?;
        Ok(())
    }
    fn node_device_update(
        &mut self,
        name: String,
        xml_desc: String,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_update));
        let req: Option<RemoteNodeDeviceUpdateArgs> = Some(RemoteNodeDeviceUpdateArgs {
            name,
            xml_desc,
            flags,
        });
        let (_header, _res) = call::<RemoteNodeDeviceUpdateArgs, ()>(
            self,
            RemoteProcedure::RemoteProcNodeDeviceUpdate,
            req,
        )?;
        Ok(())
    }
    fn domain_graphics_reload(
        &mut self,
        dom: RemoteNonnullDomain,
        r#type: u32,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_graphics_reload));
        let req: Option<RemoteDomainGraphicsReloadArgs> =
            Some(RemoteDomainGraphicsReloadArgs { dom, r#type, flags });
        let (_header, _res) = call::<RemoteDomainGraphicsReloadArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainGraphicsReload,
            req,
        )?;
        Ok(())
    }
    fn domain_get_autostart_once(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_get_autostart_once));
        let req: Option<RemoteDomainGetAutostartOnceArgs> =
            Some(RemoteDomainGetAutostartOnceArgs { dom });
        let (_header, res) = call::<
            RemoteDomainGetAutostartOnceArgs,
            RemoteDomainGetAutostartOnceRet,
        >(
            self, RemoteProcedure::RemoteProcDomainGetAutostartOnce, req
        )?;
        let res = res.unwrap();
        let RemoteDomainGetAutostartOnceRet { autostart } = res;
        Ok(autostart)
    }
    fn domain_set_autostart_once(
        &mut self,
        dom: RemoteNonnullDomain,
        autostart: i32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_autostart_once));
        let req: Option<RemoteDomainSetAutostartOnceArgs> =
            Some(RemoteDomainSetAutostartOnceArgs { dom, autostart });
        let (_header, _res) = call::<RemoteDomainSetAutostartOnceArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetAutostartOnce,
            req,
        )?;
        Ok(())
    }
    fn domain_set_throttle_group(
        &mut self,
        dom: RemoteNonnullDomain,
        group: String,
        params: Vec<RemoteTypedParam>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_throttle_group));
        let req: Option<RemoteDomainSetThrottleGroupArgs> =
            Some(RemoteDomainSetThrottleGroupArgs {
                dom,
                group,
                params,
                flags,
            });
        let (_header, _res) = call::<RemoteDomainSetThrottleGroupArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainSetThrottleGroup,
            req,
        )?;
        Ok(())
    }
    fn domain_del_throttle_group(
        &mut self,
        dom: RemoteNonnullDomain,
        group: Option<String>,
        flags: u32,
    ) -> Result<(), Error> {
        trace!("{}", stringify!(domain_del_throttle_group));
        let req: Option<RemoteDomainDelThrottleGroupArgs> =
            Some(RemoteDomainDelThrottleGroupArgs { dom, group, flags });
        let (_header, _res) = call::<RemoteDomainDelThrottleGroupArgs, ()>(
            self,
            RemoteProcedure::RemoteProcDomainDelThrottleGroup,
            req,
        )?;
        Ok(())
    }
    fn domain_event_nic_mac_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_nic_mac_change));
        let req: Option<()> = None;
        let (_header, _res) = call::<(), ()>(
            self,
            RemoteProcedure::RemoteProcDomainEventNicMacChange,
            req,
        )?;
        Ok(())
    }
    fn connect_event_connection_closed_msg(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_event_connection_closed_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteConnectEventConnectionClosedMsg { reason } = res;
        Ok(reason)
    }
    fn domain_event_balloon_change_msg(&mut self) -> Result<(RemoteNonnullDomain, u64), Error> {
        trace!("{}", stringify!(domain_event_balloon_change_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventBalloonChangeMsg { dom, actual } = res;
        Ok((dom, actual))
    }
    fn domain_event_block_job2_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, String, i32, i32), Error> {
        trace!("{}", stringify!(domain_event_block_job2_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_callback_agent_lifecycle_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, i32, i32), Error> {
        trace!("{}", stringify!(domain_event_callback_agent_lifecycle_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackBalloonChangeMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_block_job_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventBlockJobMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_block_job_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackBlockJobMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_control_error_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventControlErrorMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_control_error_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackControlErrorMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_device_added_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, String), Error> {
        trace!("{}", stringify!(domain_event_callback_device_added_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackDeviceRemovedMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_disk_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventDiskChangeMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_disk_change_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackDiskChangeMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_graphics_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventGraphicsMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_graphics_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackGraphicsMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_io_error_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventIoErrorMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackIoErrorMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_io_error_reason_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventIoErrorReasonMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error_reason_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackIoErrorReasonMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_job_completed_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, Vec<RemoteTypedParam>), Error> {
        trace!("{}", stringify!(domain_event_callback_job_completed_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackLifecycleMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_metadata_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, i32, Option<String>), Error> {
        trace!("{}", stringify!(domain_event_callback_metadata_change_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackRebootMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_rtc_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventRtcChangeMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_rtc_change_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackRtcChangeMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_tray_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteDomainEventTrayChangeMsg), Error> {
        trace!("{}", stringify!(domain_event_callback_tray_change_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackTrayChangeMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_callback_tunable_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, Vec<RemoteTypedParam>), Error> {
        trace!("{}", stringify!(domain_event_callback_tunable_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventCallbackWatchdogMsg { callback_id, msg } = res;
        Ok((callback_id, msg))
    }
    fn domain_event_control_error_msg(&mut self) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_event_control_error_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventControlErrorMsg { dom } = res;
        Ok(dom)
    }
    fn domain_event_device_removed_msg(&mut self) -> Result<(RemoteNonnullDomain, String), Error> {
        trace!("{}", stringify!(domain_event_device_removed_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventDeviceRemovedMsg { dom, dev_alias } = res;
        Ok((dom, dev_alias))
    }
    fn domain_event_disk_change_msg(&mut self) -> Result<RemoteDomainEventDiskChangeMsg, Error> {
        trace!("{}", stringify!(domain_event_disk_change_msg));
        let (_header, res) = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_graphics_msg(&mut self) -> Result<RemoteDomainEventGraphicsMsg, Error> {
        trace!("{}", stringify!(domain_event_graphics_msg));
        let (_header, res) = msg(self)?;
        Ok(res.unwrap())
    }
    fn domain_event_io_error_msg(
        &mut self,
    ) -> Result<(RemoteNonnullDomain, String, String, i32), Error> {
        trace!("{}", stringify!(domain_event_io_error_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventLifecycleMsg { dom, event, detail } = res;
        Ok((dom, event, detail))
    }
    fn domain_event_memory_device_size_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, String, u64), Error> {
        trace!("{}", stringify!(domain_event_memory_device_size_change_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
    fn domain_event_nic_mac_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullDomain, String, String, String), Error> {
        trace!("{}", stringify!(domain_event_nic_mac_change_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventNicMacChangeMsg {
            callback_id,
            dom,
            alias,
            old_mac,
            new_mac,
        } = res;
        Ok((callback_id, dom, alias, old_mac, new_mac))
    }
    fn domain_event_pmsuspend_disk_msg(&mut self) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_event_pmsuspend_disk_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventPmsuspendDiskMsg { dom } = res;
        Ok(dom)
    }
    fn domain_event_pmsuspend_msg(&mut self) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_event_pmsuspend_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventPmsuspendMsg { dom } = res;
        Ok(dom)
    }
    fn domain_event_pmwakeup_msg(&mut self) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_event_pmwakeup_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventPmwakeupMsg { dom } = res;
        Ok(dom)
    }
    fn domain_event_reboot_msg(&mut self) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_event_reboot_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventRebootMsg { dom } = res;
        Ok(dom)
    }
    fn domain_event_rtc_change_msg(&mut self) -> Result<(RemoteNonnullDomain, i64), Error> {
        trace!("{}", stringify!(domain_event_rtc_change_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventRtcChangeMsg { dom, offset } = res;
        Ok((dom, offset))
    }
    fn domain_event_tray_change_msg(
        &mut self,
    ) -> Result<(RemoteNonnullDomain, String, i32), Error> {
        trace!("{}", stringify!(domain_event_tray_change_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteDomainEventWatchdogMsg { dom, action } = res;
        Ok((dom, action))
    }
    fn network_event_callback_metadata_change_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullNetwork, i32, Option<String>), Error> {
        trace!("{}", stringify!(network_event_callback_metadata_change_msg));
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteNetworkEventCallbackMetadataChangeMsg {
            callback_id,
            net,
            r#type,
            nsuri,
        } = res;
        Ok((callback_id, net, r#type, nsuri))
    }
    fn network_event_lifecycle_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullNetwork, i32, i32), Error> {
        trace!("{}", stringify!(network_event_lifecycle_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteNodeDeviceEventUpdateMsg { callback_id, dev } = res;
        Ok((callback_id, dev))
    }
    fn secret_event_lifecycle_msg(
        &mut self,
    ) -> Result<(i32, RemoteNonnullSecret, i32, i32), Error> {
        trace!("{}", stringify!(secret_event_lifecycle_msg));
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
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
        let (_header, res) = msg(self)?;
        let res = res.unwrap();
        let RemoteStoragePoolEventRefreshMsg { callback_id, pool } = res;
        Ok((callback_id, pool))
    }
}
impl VirNetStreamResponse {
    pub fn new(inner: Box<dyn ReadWrite>, header: protocol::VirNetMessageHeader) -> Self {
        VirNetStreamResponse { inner, header }
    }
    pub fn storage_vol_upload_data(&mut self, buf: &[u8]) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_upload_data));
        upload(self, RemoteProcedure::RemoteProcStorageVolUpload, buf)
    }
    pub fn storage_vol_upload_hole(&mut self, length: i64, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_upload_hole));
        send_hole(
            self,
            RemoteProcedure::RemoteProcStorageVolUpload,
            length,
            flags,
        )
    }
    pub fn storage_vol_upload_complete(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_upload_complete));
        upload_completed(self, RemoteProcedure::RemoteProcStorageVolUpload)
    }
}
fn call<S, D>(
    client: &mut (impl Libvirt + ?Sized),
    procedure: RemoteProcedure,
    args: Option<S>,
) -> Result<(protocol::VirNetMessageHeader, Option<D>), Error>
where
    S: Serialize,
    D: DeserializeOwned,
{
    let serial = client.serial_add(1);
    let socket = client.inner();
    send(
        socket,
        procedure,
        protocol::VirNetMessageType::VirNetCall,
        serial,
        protocol::VirNetMessageStatus::VirNetOk,
        args.map(|a| VirNetRequest::Data(a)),
    )?;
    match recv(socket)? {
        (header, Some(VirNetResponse::Data(res))) => Ok((header, Some(res))),
        (header, None) => Ok((header, None)),
        _ => unreachable!(),
    }
}
fn msg<D>(
    client: &mut (impl Libvirt + ?Sized),
) -> Result<(protocol::VirNetMessageHeader, Option<D>), Error>
where
    D: DeserializeOwned,
{
    let socket = client.inner();
    match recv(socket)? {
        (header, Some(VirNetResponse::Data(res))) => Ok((header, Some(res))),
        (header, None) => Ok((header, None)),
        _ => unreachable!(),
    }
}
fn download(client: &mut (impl Libvirt + ?Sized)) -> Result<Option<VirNetStream>, Error> {
    let socket = client.inner();
    let (_, body) = recv::<()>(socket)?;
    match body {
        Some(VirNetResponse::Stream(stream)) => Ok(Some(stream)),
        None => Ok(None),
        _ => unreachable!(),
    }
}
fn upload(
    response: &mut VirNetStreamResponse,
    procedure: RemoteProcedure,
    buf: &[u8],
) -> Result<(), Error> {
    let bytes = VirNetStream::Raw(buf.to_vec());
    let req: Option<VirNetRequest<()>> = Some(VirNetRequest::Stream(bytes));
    send(
        &mut response.inner,
        procedure,
        protocol::VirNetMessageType::VirNetStream,
        response.header.serial,
        protocol::VirNetMessageStatus::VirNetContinue,
        req,
    )?;
    Ok(())
}
fn send_hole(
    response: &mut VirNetStreamResponse,
    procedure: RemoteProcedure,
    length: i64,
    flags: u32,
) -> Result<(), Error> {
    let hole = VirNetStream::Hole(protocol::VirNetStreamHole { length, flags });
    let args: Option<VirNetRequest<()>> = Some(VirNetRequest::Stream(hole));
    send(
        &mut response.inner,
        procedure,
        protocol::VirNetMessageType::VirNetStreamHole,
        response.header.serial,
        protocol::VirNetMessageStatus::VirNetContinue,
        args,
    )?;
    Ok(())
}
fn upload_completed(
    response: &mut VirNetStreamResponse,
    procedure: RemoteProcedure,
) -> Result<(), Error> {
    let req: Option<VirNetRequest<()>> = None;
    send(
        &mut response.inner,
        procedure,
        protocol::VirNetMessageType::VirNetStream,
        response.header.serial,
        protocol::VirNetMessageStatus::VirNetOk,
        req,
    )?;
    let (_header, _res) = recv::<()>(&mut response.inner)?;
    Ok(())
}
fn send<S>(
    socket: &mut Box<dyn ReadWrite>,
    procedure: RemoteProcedure,
    req_type: protocol::VirNetMessageType,
    req_serial: u32,
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
        serial: req_serial,
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
    socket
        .write_all(&req_len.to_be_bytes())
        .map_err(Error::SendError)?;
    socket
        .write_all(&req_header_bytes)
        .map_err(Error::SendError)?;
    if let Some(args_bytes) = &args_bytes {
        socket.write_all(args_bytes).map_err(Error::SendError)?;
    }
    Ok(req_len as usize)
}
fn recv<D>(
    socket: &mut Box<dyn ReadWrite>,
) -> Result<(protocol::VirNetMessageHeader, Option<VirNetResponse<D>>), Error>
where
    D: DeserializeOwned,
{
    let res_len = read_pkt_len(socket)?;
    let res_header = read_res_header(socket)?;
    let body_len = res_len - 28;
    if body_len == 0 {
        return Ok((res_header, None));
    }
    Ok((
        res_header.clone(),
        Some(read_res_body(socket, &res_header, body_len)?),
    ))
}
fn read_pkt_len(socket: &mut Box<dyn ReadWrite>) -> Result<usize, Error> {
    let mut res_len_bytes = [0; 4];
    socket
        .read_exact(&mut res_len_bytes)
        .map_err(Error::ReceiveError)?;
    Ok(u32::from_be_bytes(res_len_bytes) as usize)
}
fn read_res_header(
    socket: &mut Box<dyn ReadWrite>,
) -> Result<protocol::VirNetMessageHeader, Error> {
    let mut res_header_bytes = [0; 24];
    socket
        .read_exact(&mut res_header_bytes)
        .map_err(Error::ReceiveError)?;
    serde_xdr::from_bytes::<protocol::VirNetMessageHeader>(&res_header_bytes)
        .map_err(Error::DeserializeError)
}
fn read_res_body<D>(
    socket: &mut Box<dyn ReadWrite>,
    res_header: &protocol::VirNetMessageHeader,
    size: usize,
) -> Result<VirNetResponse<D>, Error>
where
    D: DeserializeOwned,
{
    let mut res_body_bytes = vec![0u8; size];
    socket
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
