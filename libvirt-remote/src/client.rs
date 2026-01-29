use crate::binding::*;
use crate::error::Error;
use crate::protocol;
use log::trace;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net::TcpStream;
#[cfg(target_family = "unix")]
use std::os::unix::net::UnixStream;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
pub trait ReadWrite: Read + Write + Send {
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
    serial: Arc<AtomicU32>,
    receiver: Arc<JoinHandle<()>>,
    receiver_run: Arc<AtomicBool>,
    channels: Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>,
    events: Arc<Mutex<Receiver<VirNetResponseRaw>>>,
}
pub struct VirNetStreamResponse<D>
where
    D: DeserializeOwned,
{
    inner: Box<dyn ReadWrite>,
    channels: Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>,
    receiver: Receiver<VirNetResponseRaw>,
    header: protocol::VirNetMessageHeader,
    body: Option<D>,
}
pub enum VirNetRequest<S>
where
    S: Serialize,
{
    Data(S),
    Stream(VirNetStream),
}
pub struct VirNetResponseRaw {
    header: protocol::VirNetMessageHeader,
    body: Option<Vec<u8>>,
}
pub struct VirNetResponseSet<D> {
    receiver: Option<Receiver<VirNetResponseRaw>>,
    header: protocol::VirNetMessageHeader,
    body: Option<D>,
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
        let (tx, rx) = channel();
        let receiver_run = Arc::new(AtomicBool::new(true));
        let channels = Arc::new(Mutex::new(HashMap::new()));
        let events = Arc::new(Mutex::new(rx));
        let t_receiver_run = Arc::clone(&receiver_run);
        let t_socket = socket.clone().unwrap();
        let t_channels = Arc::clone(&channels);
        let receiver = thread::spawn(|| {
            recv_thread(t_receiver_run, t_socket, t_channels, tx);
        });
        Client {
            inner: Box::new(socket),
            serial: Arc::new(AtomicU32::new(0)),
            receiver: Arc::new(receiver),
            receiver_run,
            channels,
            events,
        }
    }
}
impl Libvirt for Client {
    fn try_clone(&self) -> Result<Self, Error> {
        let inner = self.inner_clone()?;
        let serial = Arc::clone(&self.serial);
        let receiver = Arc::clone(&self.receiver);
        let receiver_run = Arc::clone(&self.receiver_run);
        let channels = Arc::clone(&self.channels);
        let events = Arc::clone(&self.events);
        Ok(Client {
            inner,
            serial,
            receiver,
            receiver_run,
            channels,
            events,
        })
    }
    fn fin(self) -> Result<(), Error> {
        if let Some(t) = Arc::into_inner(self.receiver) {
            trace!("{}", stringify!(fin));
            self.receiver_run.fetch_and(false, Ordering::SeqCst);
            t.join().map_err(|_| Error::ReceiverStopError)?;
        }
        Ok(())
    }
    fn inner(&mut self) -> &mut Box<dyn ReadWrite> {
        &mut self.inner
    }
    fn inner_clone(&self) -> Result<Box<dyn ReadWrite>, Error> {
        self.inner.clone()
    }
    fn serial_add(&mut self, value: u32) -> u32 {
        let prev = self.serial.fetch_add(value, Ordering::SeqCst);
        prev + value
    }
    fn receiver_running(&self) -> bool {
        self.receiver_run.load(Ordering::SeqCst)
    }
    fn add_channel(&mut self, serial: u32, sender: Sender<VirNetResponseRaw>) {
        let mut channels = self.channels.lock().unwrap();
        channels.insert(serial, sender);
    }
    fn remove_channel(&mut self, serial: u32) {
        let mut channels = self.channels.lock().unwrap();
        channels.remove(&serial);
    }
    fn channel_clone(&self) -> Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>> {
        Arc::clone(&self.channels)
    }
    fn get_event(&self, timeout: Duration) -> Result<VirNetResponseRaw, Error> {
        let raw = self
            .events
            .lock()
            .unwrap()
            .recv_timeout(timeout)
            .map_err(Error::ReceiveChannelError)?;
        Ok(raw)
    }
}
pub trait Libvirt: Send + Sized + 'static {
    fn try_clone(&self) -> Result<Self, Error>;
    fn fin(self) -> Result<(), Error>;
    fn inner(&mut self) -> &mut Box<dyn ReadWrite>;
    fn inner_clone(&self) -> Result<Box<dyn ReadWrite>, Error>;
    fn serial_add(&mut self, value: u32) -> u32;
    fn receiver_running(&self) -> bool;
    fn add_channel(&mut self, serial: u32, sender: Sender<VirNetResponseRaw>);
    fn remove_channel(&mut self, serial: u32);
    fn channel_clone(&self) -> Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>;
    fn get_event(&self, timeout: Duration) -> Result<VirNetResponseRaw, Error>;
    fn domain_open_namespace(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_open_namespace));
        let req: Option<LxcDomainOpenNamespaceArgs> =
            Some(LxcDomainOpenNamespaceArgs { dom, flags });
        let _res = call::<LxcDomainOpenNamespaceArgs, ()>(
            self,
            LXC_PROGRAM,
            LXC_PROTOCOL_VERSION,
            LxcProcedure::LxcProcDomainOpenNamespace as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_monitor_command(
        &mut self,
        dom: RemoteNonnullDomain,
        cmd: String,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(domain_monitor_command));
        let req: Option<QemuDomainMonitorCommandArgs> =
            Some(QemuDomainMonitorCommandArgs { dom, cmd, flags });
        let res = call::<QemuDomainMonitorCommandArgs, QemuDomainMonitorCommandRet>(
            self,
            QEMU_PROGRAM,
            QEMU_PROTOCOL_VERSION,
            QemuProcedure::QemuProcDomainMonitorCommand as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let QemuDomainMonitorCommandRet { result } = res;
        Ok(result)
    }
    fn domain_attach(&mut self, pid_value: u32, flags: u32) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_attach));
        let req: Option<QemuDomainAttachArgs> = Some(QemuDomainAttachArgs { pid_value, flags });
        let res = call::<QemuDomainAttachArgs, QemuDomainAttachRet>(
            self,
            QEMU_PROGRAM,
            QEMU_PROTOCOL_VERSION,
            QemuProcedure::QemuProcDomainAttach as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let QemuDomainAttachRet { dom } = res;
        Ok(dom)
    }
    fn domain_agent_command(
        &mut self,
        dom: RemoteNonnullDomain,
        cmd: String,
        timeout: i32,
        flags: u32,
    ) -> Result<Option<String>, Error> {
        trace!("{}", stringify!(domain_agent_command));
        let req: Option<QemuDomainAgentCommandArgs> = Some(QemuDomainAgentCommandArgs {
            dom,
            cmd,
            timeout,
            flags,
        });
        let res = call::<QemuDomainAgentCommandArgs, QemuDomainAgentCommandRet>(
            self,
            QEMU_PROGRAM,
            QEMU_PROTOCOL_VERSION,
            QemuProcedure::QemuProcDomainAgentCommand as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let QemuDomainAgentCommandRet { result } = res;
        Ok(result)
    }
    fn connect_domain_monitor_event_register(
        &mut self,
        dom: Option<RemoteNonnullDomain>,
        event: Option<String>,
        flags: u32,
    ) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_domain_monitor_event_register));
        let req: Option<QemuConnectDomainMonitorEventRegisterArgs> =
            Some(QemuConnectDomainMonitorEventRegisterArgs { dom, event, flags });
        let res = call::<
            QemuConnectDomainMonitorEventRegisterArgs,
            QemuConnectDomainMonitorEventRegisterRet,
        >(
            self,
            QEMU_PROGRAM,
            QEMU_PROTOCOL_VERSION,
            QemuProcedure::QemuProcConnectDomainMonitorEventRegister as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let QemuConnectDomainMonitorEventRegisterRet { callback_id } = res;
        Ok(callback_id)
    }
    fn connect_domain_monitor_event_deregister(&mut self, callback_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_domain_monitor_event_deregister));
        let req: Option<QemuConnectDomainMonitorEventDeregisterArgs> =
            Some(QemuConnectDomainMonitorEventDeregisterArgs { callback_id });
        let _res = call::<QemuConnectDomainMonitorEventDeregisterArgs, ()>(
            self,
            QEMU_PROGRAM,
            QEMU_PROTOCOL_VERSION,
            QemuProcedure::QemuProcConnectDomainMonitorEventDeregister as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_monitor_event(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_monitor_event));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            QEMU_PROGRAM,
            QEMU_PROTOCOL_VERSION,
            QemuProcedure::QemuProcDomainMonitorEvent as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_monitor_command_with_files(
        &mut self,
        dom: RemoteNonnullDomain,
        cmd: String,
        flags: u32,
    ) -> Result<String, Error> {
        trace!("{}", stringify!(domain_monitor_command_with_files));
        let req: Option<QemuDomainMonitorCommandWithFilesArgs> =
            Some(QemuDomainMonitorCommandWithFilesArgs { dom, cmd, flags });
        let res =
            call::<QemuDomainMonitorCommandWithFilesArgs, QemuDomainMonitorCommandWithFilesRet>(
                self,
                QEMU_PROGRAM,
                QEMU_PROTOCOL_VERSION,
                QemuProcedure::QemuProcDomainMonitorCommandWithFiles as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
        let QemuDomainMonitorCommandWithFilesRet { result } = res;
        Ok(result)
    }
    fn connect_open(&mut self, name: Option<String>, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_open));
        let req: Option<RemoteConnectOpenArgs> = Some(RemoteConnectOpenArgs { name, flags });
        let _res = call::<RemoteConnectOpenArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectOpen as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_close(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_close));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectClose as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_get_type(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_type));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectGetTypeRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetType as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectGetTypeRet { r#type } = res;
        Ok(r#type)
    }
    fn connect_get_version(&mut self) -> Result<u64, Error> {
        trace!("{}", stringify!(connect_get_version));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectGetVersionRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetVersion as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectGetVersionRet { hv_ver } = res;
        Ok(hv_ver)
    }
    fn connect_get_max_vcpus(&mut self, r#type: Option<String>) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_get_max_vcpus));
        let req: Option<RemoteConnectGetMaxVcpusArgs> =
            Some(RemoteConnectGetMaxVcpusArgs { r#type });
        let res = call::<RemoteConnectGetMaxVcpusArgs, RemoteConnectGetMaxVcpusRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetMaxVcpus as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectGetMaxVcpusRet { max_vcpus } = res;
        Ok(max_vcpus)
    }
    fn node_get_info(&mut self) -> Result<RemoteNodeGetInfoRet, Error> {
        trace!("{}", stringify!(node_get_info));
        let req: Option<()> = None;
        let res = call::<(), RemoteNodeGetInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeGetInfo as i32,
            false,
            req,
        )?;
        Ok(res.body.unwrap())
    }
    fn connect_get_capabilities(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_capabilities));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectGetCapabilitiesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetCapabilities as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectGetCapabilitiesRet { capabilities } = res;
        Ok(capabilities)
    }
    fn domain_attach_device(&mut self, dom: RemoteNonnullDomain, xml: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_attach_device));
        let req: Option<RemoteDomainAttachDeviceArgs> =
            Some(RemoteDomainAttachDeviceArgs { dom, xml });
        let _res = call::<RemoteDomainAttachDeviceArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainAttachDevice as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_create(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_create));
        let req: Option<RemoteDomainCreateArgs> = Some(RemoteDomainCreateArgs { dom });
        let _res = call::<RemoteDomainCreateArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCreate as i32,
            false,
            req,
        )?;
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
        let res = call::<RemoteDomainCreateXmlArgs, RemoteDomainCreateXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCreateXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainCreateXmlRet { dom } = res;
        Ok(dom)
    }
    fn domain_define_xml(&mut self, xml: String) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_define_xml));
        let req: Option<RemoteDomainDefineXmlArgs> = Some(RemoteDomainDefineXmlArgs { xml });
        let res = call::<RemoteDomainDefineXmlArgs, RemoteDomainDefineXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainDefineXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainDefineXmlRet { dom } = res;
        Ok(dom)
    }
    fn domain_destroy(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_destroy));
        let req: Option<RemoteDomainDestroyArgs> = Some(RemoteDomainDestroyArgs { dom });
        let _res = call::<RemoteDomainDestroyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainDestroy as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_detach_device(&mut self, dom: RemoteNonnullDomain, xml: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_detach_device));
        let req: Option<RemoteDomainDetachDeviceArgs> =
            Some(RemoteDomainDetachDeviceArgs { dom, xml });
        let _res = call::<RemoteDomainDetachDeviceArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainDetachDevice as i32,
            false,
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
        let res = call::<RemoteDomainGetXmlDescArgs, RemoteDomainGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_get_autostart(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_get_autostart));
        let req: Option<RemoteDomainGetAutostartArgs> = Some(RemoteDomainGetAutostartArgs { dom });
        let res = call::<RemoteDomainGetAutostartArgs, RemoteDomainGetAutostartRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetAutostart as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainGetAutostartRet { autostart } = res;
        Ok(autostart)
    }
    fn domain_get_info(
        &mut self,
        dom: RemoteNonnullDomain,
    ) -> Result<(u8, u64, u64, u16, u64), Error> {
        trace!("{}", stringify!(domain_get_info));
        let req: Option<RemoteDomainGetInfoArgs> = Some(RemoteDomainGetInfoArgs { dom });
        let res = call::<RemoteDomainGetInfoArgs, RemoteDomainGetInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainGetMaxMemoryArgs, RemoteDomainGetMaxMemoryRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetMaxMemory as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainGetMaxMemoryRet { memory } = res;
        Ok(memory)
    }
    fn domain_get_max_vcpus(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_get_max_vcpus));
        let req: Option<RemoteDomainGetMaxVcpusArgs> = Some(RemoteDomainGetMaxVcpusArgs { dom });
        let res = call::<RemoteDomainGetMaxVcpusArgs, RemoteDomainGetMaxVcpusRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetMaxVcpus as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainGetMaxVcpusRet { num } = res;
        Ok(num)
    }
    fn domain_get_os_type(&mut self, dom: RemoteNonnullDomain) -> Result<String, Error> {
        trace!("{}", stringify!(domain_get_os_type));
        let req: Option<RemoteDomainGetOsTypeArgs> = Some(RemoteDomainGetOsTypeArgs { dom });
        let res = call::<RemoteDomainGetOsTypeArgs, RemoteDomainGetOsTypeRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetOsType as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainGetVcpusArgs, RemoteDomainGetVcpusRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetVcpus as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainGetVcpusRet { info, cpumaps } = res;
        Ok((info, cpumaps))
    }
    fn connect_list_defined_domains(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_defined_domains));
        let req: Option<RemoteConnectListDefinedDomainsArgs> =
            Some(RemoteConnectListDefinedDomainsArgs { maxnames });
        let res = call::<RemoteConnectListDefinedDomainsArgs, RemoteConnectListDefinedDomainsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListDefinedDomains as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectListDefinedDomainsRet { names } = res;
        Ok(names)
    }
    fn domain_lookup_by_id(&mut self, id: i32) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_lookup_by_id));
        let req: Option<RemoteDomainLookupByIdArgs> = Some(RemoteDomainLookupByIdArgs { id });
        let res = call::<RemoteDomainLookupByIdArgs, RemoteDomainLookupByIdRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainLookupById as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainLookupByIdRet { dom } = res;
        Ok(dom)
    }
    fn domain_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_lookup_by_name));
        let req: Option<RemoteDomainLookupByNameArgs> = Some(RemoteDomainLookupByNameArgs { name });
        let res = call::<RemoteDomainLookupByNameArgs, RemoteDomainLookupByNameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainLookupByName as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainLookupByNameRet { dom } = res;
        Ok(dom)
    }
    fn domain_lookup_by_uuid(
        &mut self,
        uuid: [u8; VIR_UUID_BUFLEN as usize],
    ) -> Result<RemoteNonnullDomain, Error> {
        trace!("{}", stringify!(domain_lookup_by_uuid));
        let req: Option<RemoteDomainLookupByUuidArgs> = Some(RemoteDomainLookupByUuidArgs { uuid });
        let res = call::<RemoteDomainLookupByUuidArgs, RemoteDomainLookupByUuidRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainLookupByUuid as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainLookupByUuidRet { dom } = res;
        Ok(dom)
    }
    fn connect_num_of_defined_domains(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_defined_domains));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectNumOfDefinedDomainsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNumOfDefinedDomains as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainPinVcpuArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainPinVcpu as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_reboot(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_reboot));
        let req: Option<RemoteDomainRebootArgs> = Some(RemoteDomainRebootArgs { dom, flags });
        let _res = call::<RemoteDomainRebootArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainReboot as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_resume(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_resume));
        let req: Option<RemoteDomainResumeArgs> = Some(RemoteDomainResumeArgs { dom });
        let _res = call::<RemoteDomainResumeArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainResume as i32,
            false,
            req,
        )?;
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
        let _res = call::<RemoteDomainSetAutostartArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetAutostart as i32,
            false,
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
        let _res = call::<RemoteDomainSetMaxMemoryArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetMaxMemory as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_set_memory(&mut self, dom: RemoteNonnullDomain, memory: u64) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_memory));
        let req: Option<RemoteDomainSetMemoryArgs> =
            Some(RemoteDomainSetMemoryArgs { dom, memory });
        let _res = call::<RemoteDomainSetMemoryArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetMemory as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_set_vcpus(&mut self, dom: RemoteNonnullDomain, nvcpus: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_set_vcpus));
        let req: Option<RemoteDomainSetVcpusArgs> = Some(RemoteDomainSetVcpusArgs { dom, nvcpus });
        let _res = call::<RemoteDomainSetVcpusArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetVcpus as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_shutdown(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_shutdown));
        let req: Option<RemoteDomainShutdownArgs> = Some(RemoteDomainShutdownArgs { dom });
        let _res = call::<RemoteDomainShutdownArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainShutdown as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_suspend(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_suspend));
        let req: Option<RemoteDomainSuspendArgs> = Some(RemoteDomainSuspendArgs { dom });
        let _res = call::<RemoteDomainSuspendArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSuspend as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_undefine(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_undefine));
        let req: Option<RemoteDomainUndefineArgs> = Some(RemoteDomainUndefineArgs { dom });
        let _res = call::<RemoteDomainUndefineArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainUndefine as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_list_defined_networks(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_defined_networks));
        let req: Option<RemoteConnectListDefinedNetworksArgs> =
            Some(RemoteConnectListDefinedNetworksArgs { maxnames });
        let res = call::<RemoteConnectListDefinedNetworksArgs, RemoteConnectListDefinedNetworksRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListDefinedNetworks as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectListDefinedNetworksRet { names } = res;
        Ok(names)
    }
    fn connect_list_domains(&mut self, maxids: i32) -> Result<Vec<i32>, Error> {
        trace!("{}", stringify!(connect_list_domains));
        let req: Option<RemoteConnectListDomainsArgs> =
            Some(RemoteConnectListDomainsArgs { maxids });
        let res = call::<RemoteConnectListDomainsArgs, RemoteConnectListDomainsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListDomains as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectListDomainsRet { ids } = res;
        Ok(ids)
    }
    fn connect_list_networks(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_networks));
        let req: Option<RemoteConnectListNetworksArgs> =
            Some(RemoteConnectListNetworksArgs { maxnames });
        let res = call::<RemoteConnectListNetworksArgs, RemoteConnectListNetworksRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListNetworks as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectListNetworksRet { names } = res;
        Ok(names)
    }
    fn network_create(&mut self, net: RemoteNonnullNetwork) -> Result<(), Error> {
        trace!("{}", stringify!(network_create));
        let req: Option<RemoteNetworkCreateArgs> = Some(RemoteNetworkCreateArgs { net });
        let _res = call::<RemoteNetworkCreateArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkCreate as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn network_create_xml(&mut self, xml: String) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_create_xml));
        let req: Option<RemoteNetworkCreateXmlArgs> = Some(RemoteNetworkCreateXmlArgs { xml });
        let res = call::<RemoteNetworkCreateXmlArgs, RemoteNetworkCreateXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkCreateXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNetworkCreateXmlRet { net } = res;
        Ok(net)
    }
    fn network_define_xml(&mut self, xml: String) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_define_xml));
        let req: Option<RemoteNetworkDefineXmlArgs> = Some(RemoteNetworkDefineXmlArgs { xml });
        let res = call::<RemoteNetworkDefineXmlArgs, RemoteNetworkDefineXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkDefineXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNetworkDefineXmlRet { net } = res;
        Ok(net)
    }
    fn network_destroy(&mut self, net: RemoteNonnullNetwork) -> Result<(), Error> {
        trace!("{}", stringify!(network_destroy));
        let req: Option<RemoteNetworkDestroyArgs> = Some(RemoteNetworkDestroyArgs { net });
        let _res = call::<RemoteNetworkDestroyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkDestroy as i32,
            false,
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
        let res = call::<RemoteNetworkGetXmlDescArgs, RemoteNetworkGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNetworkGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn network_get_autostart(&mut self, net: RemoteNonnullNetwork) -> Result<i32, Error> {
        trace!("{}", stringify!(network_get_autostart));
        let req: Option<RemoteNetworkGetAutostartArgs> =
            Some(RemoteNetworkGetAutostartArgs { net });
        let res = call::<RemoteNetworkGetAutostartArgs, RemoteNetworkGetAutostartRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkGetAutostart as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNetworkGetAutostartRet { autostart } = res;
        Ok(autostart)
    }
    fn network_get_bridge_name(&mut self, net: RemoteNonnullNetwork) -> Result<String, Error> {
        trace!("{}", stringify!(network_get_bridge_name));
        let req: Option<RemoteNetworkGetBridgeNameArgs> =
            Some(RemoteNetworkGetBridgeNameArgs { net });
        let res = call::<RemoteNetworkGetBridgeNameArgs, RemoteNetworkGetBridgeNameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkGetBridgeName as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNetworkGetBridgeNameRet { name } = res;
        Ok(name)
    }
    fn network_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullNetwork, Error> {
        trace!("{}", stringify!(network_lookup_by_name));
        let req: Option<RemoteNetworkLookupByNameArgs> =
            Some(RemoteNetworkLookupByNameArgs { name });
        let res = call::<RemoteNetworkLookupByNameArgs, RemoteNetworkLookupByNameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkLookupByName as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNetworkLookupByUuidArgs, RemoteNetworkLookupByUuidRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkLookupByUuid as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteNetworkSetAutostartArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkSetAutostart as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn network_undefine(&mut self, net: RemoteNonnullNetwork) -> Result<(), Error> {
        trace!("{}", stringify!(network_undefine));
        let req: Option<RemoteNetworkUndefineArgs> = Some(RemoteNetworkUndefineArgs { net });
        let _res = call::<RemoteNetworkUndefineArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkUndefine as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_num_of_defined_networks(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_defined_networks));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectNumOfDefinedNetworksRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNumOfDefinedNetworks as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectNumOfDefinedNetworksRet { num } = res;
        Ok(num)
    }
    fn connect_num_of_domains(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_domains));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectNumOfDomainsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNumOfDomains as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectNumOfDomainsRet { num } = res;
        Ok(num)
    }
    fn connect_num_of_networks(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_networks));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectNumOfNetworksRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNumOfNetworks as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainCoreDumpArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCoreDump as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_restore(&mut self, from: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_restore));
        let req: Option<RemoteDomainRestoreArgs> = Some(RemoteDomainRestoreArgs { from });
        let _res = call::<RemoteDomainRestoreArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainRestore as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_save(&mut self, dom: RemoteNonnullDomain, to: String) -> Result<(), Error> {
        trace!("{}", stringify!(domain_save));
        let req: Option<RemoteDomainSaveArgs> = Some(RemoteDomainSaveArgs { dom, to });
        let _res = call::<RemoteDomainSaveArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSave as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_get_scheduler_type(
        &mut self,
        dom: RemoteNonnullDomain,
    ) -> Result<(String, i32), Error> {
        trace!("{}", stringify!(domain_get_scheduler_type));
        let req: Option<RemoteDomainGetSchedulerTypeArgs> =
            Some(RemoteDomainGetSchedulerTypeArgs { dom });
        let res = call::<RemoteDomainGetSchedulerTypeArgs, RemoteDomainGetSchedulerTypeRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetSchedulerType as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res =
            call::<RemoteDomainGetSchedulerParametersArgs, RemoteDomainGetSchedulerParametersRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcDomainGetSchedulerParameters as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetSchedulerParametersArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetSchedulerParameters as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_get_hostname(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_hostname));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectGetHostnameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetHostname as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectGetHostnameRet { hostname } = res;
        Ok(hostname)
    }
    fn connect_supports_feature(&mut self, feature: i32) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_supports_feature));
        let req: Option<RemoteConnectSupportsFeatureArgs> =
            Some(RemoteConnectSupportsFeatureArgs { feature });
        let res = call::<RemoteConnectSupportsFeatureArgs, RemoteConnectSupportsFeatureRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectSupportsFeature as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainMigratePrepareArgs, RemoteDomainMigratePrepareRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigratePrepare as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainMigratePerformArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigratePerform as i32,
            false,
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
        let res = call::<RemoteDomainMigrateFinishArgs, RemoteDomainMigrateFinishRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateFinish as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainBlockStatsArgs, RemoteDomainBlockStatsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBlockStats as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainInterfaceStatsArgs, RemoteDomainInterfaceStatsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainInterfaceStats as i32,
            false,
            req,
        )?;
        Ok(res.body.unwrap())
    }
    fn auth_list(&mut self) -> Result<Vec<RemoteAuthType>, Error> {
        trace!("{}", stringify!(auth_list));
        let req: Option<()> = None;
        let res = call::<(), RemoteAuthListRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcAuthList as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteAuthListRet { types } = res;
        Ok(types)
    }
    fn auth_sasl_init(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(auth_sasl_init));
        let req: Option<()> = None;
        let res = call::<(), RemoteAuthSaslInitRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcAuthSaslInit as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteAuthSaslStartArgs, RemoteAuthSaslStartRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcAuthSaslStart as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteAuthSaslStepArgs, RemoteAuthSaslStepRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcAuthSaslStep as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<(), RemoteAuthPolkitRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcAuthPolkit as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteAuthPolkitRet { complete } = res;
        Ok(complete)
    }
    fn connect_num_of_storage_pools(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_storage_pools));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectNumOfStoragePoolsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNumOfStoragePools as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectNumOfStoragePoolsRet { num } = res;
        Ok(num)
    }
    fn connect_list_storage_pools(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_storage_pools));
        let req: Option<RemoteConnectListStoragePoolsArgs> =
            Some(RemoteConnectListStoragePoolsArgs { maxnames });
        let res = call::<RemoteConnectListStoragePoolsArgs, RemoteConnectListStoragePoolsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListStoragePools as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectListStoragePoolsRet { names } = res;
        Ok(names)
    }
    fn connect_num_of_defined_storage_pools(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_defined_storage_pools));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectNumOfDefinedStoragePoolsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNumOfDefinedStoragePools as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectNumOfDefinedStoragePoolsRet { num } = res;
        Ok(num)
    }
    fn connect_list_defined_storage_pools(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_defined_storage_pools));
        let req: Option<RemoteConnectListDefinedStoragePoolsArgs> =
            Some(RemoteConnectListDefinedStoragePoolsArgs { maxnames });
        let res = call::<
            RemoteConnectListDefinedStoragePoolsArgs,
            RemoteConnectListDefinedStoragePoolsRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListDefinedStoragePools as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<
            RemoteConnectFindStoragePoolSourcesArgs,
            RemoteConnectFindStoragePoolSourcesRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectFindStoragePoolSources as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteStoragePoolCreateXmlArgs, RemoteStoragePoolCreateXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolCreateXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteStoragePoolDefineXmlArgs, RemoteStoragePoolDefineXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolDefineXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteStoragePoolCreateArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolCreate as i32,
            false,
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
        let _res = call::<RemoteStoragePoolBuildArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolBuild as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_destroy(&mut self, pool: RemoteNonnullStoragePool) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_destroy));
        let req: Option<RemoteStoragePoolDestroyArgs> = Some(RemoteStoragePoolDestroyArgs { pool });
        let _res = call::<RemoteStoragePoolDestroyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolDestroy as i32,
            false,
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
        let _res = call::<RemoteStoragePoolDeleteArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolDelete as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_undefine(&mut self, pool: RemoteNonnullStoragePool) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_undefine));
        let req: Option<RemoteStoragePoolUndefineArgs> =
            Some(RemoteStoragePoolUndefineArgs { pool });
        let _res = call::<RemoteStoragePoolUndefineArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolUndefine as i32,
            false,
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
        let _res = call::<RemoteStoragePoolRefreshArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolRefresh as i32,
            false,
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
        let res = call::<RemoteStoragePoolLookupByNameArgs, RemoteStoragePoolLookupByNameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolLookupByName as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteStoragePoolLookupByUuidArgs, RemoteStoragePoolLookupByUuidRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolLookupByUuid as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteStoragePoolLookupByVolumeArgs, RemoteStoragePoolLookupByVolumeRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolLookupByVolume as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteStoragePoolLookupByVolumeRet { pool } = res;
        Ok(pool)
    }
    fn storage_pool_get_info(
        &mut self,
        pool: RemoteNonnullStoragePool,
    ) -> Result<(u8, u64, u64, u64), Error> {
        trace!("{}", stringify!(storage_pool_get_info));
        let req: Option<RemoteStoragePoolGetInfoArgs> = Some(RemoteStoragePoolGetInfoArgs { pool });
        let res = call::<RemoteStoragePoolGetInfoArgs, RemoteStoragePoolGetInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolGetInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteStoragePoolGetXmlDescArgs, RemoteStoragePoolGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteStoragePoolGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn storage_pool_get_autostart(&mut self, pool: RemoteNonnullStoragePool) -> Result<i32, Error> {
        trace!("{}", stringify!(storage_pool_get_autostart));
        let req: Option<RemoteStoragePoolGetAutostartArgs> =
            Some(RemoteStoragePoolGetAutostartArgs { pool });
        let res = call::<RemoteStoragePoolGetAutostartArgs, RemoteStoragePoolGetAutostartRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolGetAutostart as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteStoragePoolSetAutostartArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolSetAutostart as i32,
            false,
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
        let res = call::<RemoteStoragePoolNumOfVolumesArgs, RemoteStoragePoolNumOfVolumesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolNumOfVolumes as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteStoragePoolListVolumesArgs, RemoteStoragePoolListVolumesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolListVolumes as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteStorageVolCreateXmlArgs, RemoteStorageVolCreateXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolCreateXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteStorageVolDeleteArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolDelete as i32,
            false,
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
        let res = call::<RemoteStorageVolLookupByNameArgs, RemoteStorageVolLookupByNameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolLookupByName as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteStorageVolLookupByNameRet { vol } = res;
        Ok(vol)
    }
    fn storage_vol_lookup_by_key(&mut self, key: String) -> Result<RemoteNonnullStorageVol, Error> {
        trace!("{}", stringify!(storage_vol_lookup_by_key));
        let req: Option<RemoteStorageVolLookupByKeyArgs> =
            Some(RemoteStorageVolLookupByKeyArgs { key });
        let res = call::<RemoteStorageVolLookupByKeyArgs, RemoteStorageVolLookupByKeyRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolLookupByKey as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteStorageVolLookupByPathArgs, RemoteStorageVolLookupByPathRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolLookupByPath as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteStorageVolLookupByPathRet { vol } = res;
        Ok(vol)
    }
    fn storage_vol_get_info(
        &mut self,
        vol: RemoteNonnullStorageVol,
    ) -> Result<(i8, u64, u64), Error> {
        trace!("{}", stringify!(storage_vol_get_info));
        let req: Option<RemoteStorageVolGetInfoArgs> = Some(RemoteStorageVolGetInfoArgs { vol });
        let res = call::<RemoteStorageVolGetInfoArgs, RemoteStorageVolGetInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolGetInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteStorageVolGetXmlDescArgs, RemoteStorageVolGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteStorageVolGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn storage_vol_get_path(&mut self, vol: RemoteNonnullStorageVol) -> Result<String, Error> {
        trace!("{}", stringify!(storage_vol_get_path));
        let req: Option<RemoteStorageVolGetPathArgs> = Some(RemoteStorageVolGetPathArgs { vol });
        let res = call::<RemoteStorageVolGetPathArgs, RemoteStorageVolGetPathRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolGetPath as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNodeGetCellsFreeMemoryArgs, RemoteNodeGetCellsFreeMemoryRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeGetCellsFreeMemory as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeGetCellsFreeMemoryRet { cells } = res;
        Ok(cells)
    }
    fn node_get_free_memory(&mut self) -> Result<u64, Error> {
        trace!("{}", stringify!(node_get_free_memory));
        let req: Option<()> = None;
        let res = call::<(), RemoteNodeGetFreeMemoryRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeGetFreeMemory as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainBlockPeekArgs, RemoteDomainBlockPeekRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBlockPeek as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainMemoryPeekArgs, RemoteDomainMemoryPeekRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMemoryPeek as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainMemoryPeekRet { buffer } = res;
        Ok(buffer)
    }
    fn connect_domain_event_register(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_domain_event_register));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectDomainEventRegisterRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectDomainEventRegister as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectDomainEventRegisterRet { cb_registered } = res;
        Ok(cb_registered)
    }
    fn connect_domain_event_deregister(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_domain_event_deregister));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectDomainEventDeregisterRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectDomainEventDeregister as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectDomainEventDeregisterRet { cb_registered } = res;
        Ok(cb_registered)
    }
    fn domain_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_lifecycle));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventLifecycle as i32,
            false,
            req,
        )?;
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
        let res = call::<RemoteDomainMigratePrepare2Args, RemoteDomainMigratePrepare2Ret>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigratePrepare2 as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainMigrateFinish2Args, RemoteDomainMigrateFinish2Ret>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateFinish2 as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainMigrateFinish2Ret { ddom } = res;
        Ok(ddom)
    }
    fn connect_get_uri(&mut self) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_uri));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectGetUriRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetUri as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectGetUriRet { uri } = res;
        Ok(uri)
    }
    fn node_num_of_devices(&mut self, cap: Option<String>, flags: u32) -> Result<i32, Error> {
        trace!("{}", stringify!(node_num_of_devices));
        let req: Option<RemoteNodeNumOfDevicesArgs> =
            Some(RemoteNodeNumOfDevicesArgs { cap, flags });
        let res = call::<RemoteNodeNumOfDevicesArgs, RemoteNodeNumOfDevicesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeNumOfDevices as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNodeListDevicesArgs, RemoteNodeListDevicesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeListDevices as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNodeDeviceLookupByNameArgs, RemoteNodeDeviceLookupByNameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceLookupByName as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeDeviceLookupByNameRet { dev } = res;
        Ok(dev)
    }
    fn node_device_get_xml_desc(&mut self, name: String, flags: u32) -> Result<String, Error> {
        trace!("{}", stringify!(node_device_get_xml_desc));
        let req: Option<RemoteNodeDeviceGetXmlDescArgs> =
            Some(RemoteNodeDeviceGetXmlDescArgs { name, flags });
        let res = call::<RemoteNodeDeviceGetXmlDescArgs, RemoteNodeDeviceGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeDeviceGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn node_device_get_parent(&mut self, name: String) -> Result<Option<String>, Error> {
        trace!("{}", stringify!(node_device_get_parent));
        let req: Option<RemoteNodeDeviceGetParentArgs> =
            Some(RemoteNodeDeviceGetParentArgs { name });
        let res = call::<RemoteNodeDeviceGetParentArgs, RemoteNodeDeviceGetParentRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceGetParent as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeDeviceGetParentRet { parent_name } = res;
        Ok(parent_name)
    }
    fn node_device_num_of_caps(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_num_of_caps));
        let req: Option<RemoteNodeDeviceNumOfCapsArgs> =
            Some(RemoteNodeDeviceNumOfCapsArgs { name });
        let res = call::<RemoteNodeDeviceNumOfCapsArgs, RemoteNodeDeviceNumOfCapsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceNumOfCaps as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeDeviceNumOfCapsRet { num } = res;
        Ok(num)
    }
    fn node_device_list_caps(&mut self, name: String, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(node_device_list_caps));
        let req: Option<RemoteNodeDeviceListCapsArgs> =
            Some(RemoteNodeDeviceListCapsArgs { name, maxnames });
        let res = call::<RemoteNodeDeviceListCapsArgs, RemoteNodeDeviceListCapsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceListCaps as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeDeviceListCapsRet { names } = res;
        Ok(names)
    }
    fn node_device_dettach(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_dettach));
        let req: Option<RemoteNodeDeviceDettachArgs> = Some(RemoteNodeDeviceDettachArgs { name });
        let _res = call::<RemoteNodeDeviceDettachArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceDettach as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn node_device_re_attach(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_re_attach));
        let req: Option<RemoteNodeDeviceReAttachArgs> = Some(RemoteNodeDeviceReAttachArgs { name });
        let _res = call::<RemoteNodeDeviceReAttachArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceReAttach as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn node_device_reset(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_reset));
        let req: Option<RemoteNodeDeviceResetArgs> = Some(RemoteNodeDeviceResetArgs { name });
        let _res = call::<RemoteNodeDeviceResetArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceReset as i32,
            false,
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
        let res = call::<RemoteDomainGetSecurityLabelArgs, RemoteDomainGetSecurityLabelRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetSecurityLabel as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainGetSecurityLabelRet { label, enforcing } = res;
        Ok((label, enforcing))
    }
    fn node_get_security_model(&mut self) -> Result<(Vec<i8>, Vec<i8>), Error> {
        trace!("{}", stringify!(node_get_security_model));
        let req: Option<()> = None;
        let res = call::<(), RemoteNodeGetSecurityModelRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeGetSecurityModel as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNodeDeviceCreateXmlArgs, RemoteNodeDeviceCreateXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceCreateXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeDeviceCreateXmlRet { dev } = res;
        Ok(dev)
    }
    fn node_device_destroy(&mut self, name: String) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_destroy));
        let req: Option<RemoteNodeDeviceDestroyArgs> = Some(RemoteNodeDeviceDestroyArgs { name });
        let _res = call::<RemoteNodeDeviceDestroyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceDestroy as i32,
            false,
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
        let res = call::<RemoteStorageVolCreateXmlFromArgs, RemoteStorageVolCreateXmlFromRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolCreateXmlFrom as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteStorageVolCreateXmlFromRet { vol } = res;
        Ok(vol)
    }
    fn connect_num_of_interfaces(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_interfaces));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectNumOfInterfacesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNumOfInterfaces as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectNumOfInterfacesRet { num } = res;
        Ok(num)
    }
    fn connect_list_interfaces(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_interfaces));
        let req: Option<RemoteConnectListInterfacesArgs> =
            Some(RemoteConnectListInterfacesArgs { maxnames });
        let res = call::<RemoteConnectListInterfacesArgs, RemoteConnectListInterfacesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListInterfaces as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectListInterfacesRet { names } = res;
        Ok(names)
    }
    fn interface_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullInterface, Error> {
        trace!("{}", stringify!(interface_lookup_by_name));
        let req: Option<RemoteInterfaceLookupByNameArgs> =
            Some(RemoteInterfaceLookupByNameArgs { name });
        let res = call::<RemoteInterfaceLookupByNameArgs, RemoteInterfaceLookupByNameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceLookupByName as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteInterfaceLookupByMacStringArgs, RemoteInterfaceLookupByMacStringRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceLookupByMacString as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteInterfaceGetXmlDescArgs, RemoteInterfaceGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteInterfaceDefineXmlArgs, RemoteInterfaceDefineXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceDefineXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteInterfaceDefineXmlRet { iface } = res;
        Ok(iface)
    }
    fn interface_undefine(&mut self, iface: RemoteNonnullInterface) -> Result<(), Error> {
        trace!("{}", stringify!(interface_undefine));
        let req: Option<RemoteInterfaceUndefineArgs> = Some(RemoteInterfaceUndefineArgs { iface });
        let _res = call::<RemoteInterfaceUndefineArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceUndefine as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn interface_create(&mut self, iface: RemoteNonnullInterface, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_create));
        let req: Option<RemoteInterfaceCreateArgs> =
            Some(RemoteInterfaceCreateArgs { iface, flags });
        let _res = call::<RemoteInterfaceCreateArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceCreate as i32,
            false,
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
        let _res = call::<RemoteInterfaceDestroyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceDestroy as i32,
            false,
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
        let res = call::<RemoteConnectDomainXmlFromNativeArgs, RemoteConnectDomainXmlFromNativeRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectDomainXmlFromNative as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteConnectDomainXmlToNativeArgs, RemoteConnectDomainXmlToNativeRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectDomainXmlToNative as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectDomainXmlToNativeRet { native_config } = res;
        Ok(native_config)
    }
    fn connect_num_of_defined_interfaces(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_defined_interfaces));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectNumOfDefinedInterfacesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNumOfDefinedInterfaces as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectNumOfDefinedInterfacesRet { num } = res;
        Ok(num)
    }
    fn connect_list_defined_interfaces(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_defined_interfaces));
        let req: Option<RemoteConnectListDefinedInterfacesArgs> =
            Some(RemoteConnectListDefinedInterfacesArgs { maxnames });
        let res =
            call::<RemoteConnectListDefinedInterfacesArgs, RemoteConnectListDefinedInterfacesRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcConnectListDefinedInterfaces as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
        let RemoteConnectListDefinedInterfacesRet { names } = res;
        Ok(names)
    }
    fn connect_num_of_secrets(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_secrets));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectNumOfSecretsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNumOfSecrets as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectNumOfSecretsRet { num } = res;
        Ok(num)
    }
    fn connect_list_secrets(&mut self, maxuuids: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_secrets));
        let req: Option<RemoteConnectListSecretsArgs> =
            Some(RemoteConnectListSecretsArgs { maxuuids });
        let res = call::<RemoteConnectListSecretsArgs, RemoteConnectListSecretsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListSecrets as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectListSecretsRet { uuids } = res;
        Ok(uuids)
    }
    fn secret_lookup_by_uuid(
        &mut self,
        uuid: [u8; VIR_UUID_BUFLEN as usize],
    ) -> Result<RemoteNonnullSecret, Error> {
        trace!("{}", stringify!(secret_lookup_by_uuid));
        let req: Option<RemoteSecretLookupByUuidArgs> = Some(RemoteSecretLookupByUuidArgs { uuid });
        let res = call::<RemoteSecretLookupByUuidArgs, RemoteSecretLookupByUuidRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcSecretLookupByUuid as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteSecretLookupByUuidRet { secret } = res;
        Ok(secret)
    }
    fn secret_define_xml(&mut self, xml: String, flags: u32) -> Result<RemoteNonnullSecret, Error> {
        trace!("{}", stringify!(secret_define_xml));
        let req: Option<RemoteSecretDefineXmlArgs> = Some(RemoteSecretDefineXmlArgs { xml, flags });
        let res = call::<RemoteSecretDefineXmlArgs, RemoteSecretDefineXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcSecretDefineXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteSecretGetXmlDescArgs, RemoteSecretGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcSecretGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteSecretSetValueArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcSecretSetValue as i32,
            false,
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
        let res = call::<RemoteSecretGetValueArgs, RemoteSecretGetValueRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcSecretGetValue as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteSecretGetValueRet { value } = res;
        Ok(value)
    }
    fn secret_undefine(&mut self, secret: RemoteNonnullSecret) -> Result<(), Error> {
        trace!("{}", stringify!(secret_undefine));
        let req: Option<RemoteSecretUndefineArgs> = Some(RemoteSecretUndefineArgs { secret });
        let _res = call::<RemoteSecretUndefineArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcSecretUndefine as i32,
            false,
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
        let res = call::<RemoteSecretLookupByUsageArgs, RemoteSecretLookupByUsageRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcSecretLookupByUsage as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteSecretLookupByUsageRet { secret } = res;
        Ok(secret)
    }
    fn domain_migrate_prepare_tunnel(
        &mut self,
        flags: u64,
        dname: Option<String>,
        bandwidth: u64,
        dom_xml: String,
    ) -> Result<VirNetStreamResponse<()>, Error> {
        trace!("{}", stringify!(domain_migrate_prepare_tunnel));
        let req: Option<RemoteDomainMigratePrepareTunnelArgs> =
            Some(RemoteDomainMigratePrepareTunnelArgs {
                flags,
                dname,
                bandwidth,
                dom_xml,
            });
        let res = call::<RemoteDomainMigratePrepareTunnelArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigratePrepareTunnel as i32,
            true,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            channels: self.channel_clone(),
            receiver: res.receiver.unwrap(),
            header: res.header,
            body: None,
        };
        Ok(res)
    }
    fn connect_is_secure(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_is_secure));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectIsSecureRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectIsSecure as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectIsSecureRet { secure } = res;
        Ok(secure)
    }
    fn domain_is_active(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_is_active));
        let req: Option<RemoteDomainIsActiveArgs> = Some(RemoteDomainIsActiveArgs { dom });
        let res = call::<RemoteDomainIsActiveArgs, RemoteDomainIsActiveRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainIsActive as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainIsActiveRet { active } = res;
        Ok(active)
    }
    fn domain_is_persistent(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_is_persistent));
        let req: Option<RemoteDomainIsPersistentArgs> = Some(RemoteDomainIsPersistentArgs { dom });
        let res = call::<RemoteDomainIsPersistentArgs, RemoteDomainIsPersistentRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainIsPersistent as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn network_is_active(&mut self, net: RemoteNonnullNetwork) -> Result<i32, Error> {
        trace!("{}", stringify!(network_is_active));
        let req: Option<RemoteNetworkIsActiveArgs> = Some(RemoteNetworkIsActiveArgs { net });
        let res = call::<RemoteNetworkIsActiveArgs, RemoteNetworkIsActiveRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkIsActive as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNetworkIsActiveRet { active } = res;
        Ok(active)
    }
    fn network_is_persistent(&mut self, net: RemoteNonnullNetwork) -> Result<i32, Error> {
        trace!("{}", stringify!(network_is_persistent));
        let req: Option<RemoteNetworkIsPersistentArgs> =
            Some(RemoteNetworkIsPersistentArgs { net });
        let res = call::<RemoteNetworkIsPersistentArgs, RemoteNetworkIsPersistentRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkIsPersistent as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNetworkIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn storage_pool_is_active(&mut self, pool: RemoteNonnullStoragePool) -> Result<i32, Error> {
        trace!("{}", stringify!(storage_pool_is_active));
        let req: Option<RemoteStoragePoolIsActiveArgs> =
            Some(RemoteStoragePoolIsActiveArgs { pool });
        let res = call::<RemoteStoragePoolIsActiveArgs, RemoteStoragePoolIsActiveRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolIsActive as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteStoragePoolIsActiveRet { active } = res;
        Ok(active)
    }
    fn storage_pool_is_persistent(&mut self, pool: RemoteNonnullStoragePool) -> Result<i32, Error> {
        trace!("{}", stringify!(storage_pool_is_persistent));
        let req: Option<RemoteStoragePoolIsPersistentArgs> =
            Some(RemoteStoragePoolIsPersistentArgs { pool });
        let res = call::<RemoteStoragePoolIsPersistentArgs, RemoteStoragePoolIsPersistentRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolIsPersistent as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteStoragePoolIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn interface_is_active(&mut self, iface: RemoteNonnullInterface) -> Result<i32, Error> {
        trace!("{}", stringify!(interface_is_active));
        let req: Option<RemoteInterfaceIsActiveArgs> = Some(RemoteInterfaceIsActiveArgs { iface });
        let res = call::<RemoteInterfaceIsActiveArgs, RemoteInterfaceIsActiveRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceIsActive as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteInterfaceIsActiveRet { active } = res;
        Ok(active)
    }
    fn connect_get_lib_version(&mut self) -> Result<u64, Error> {
        trace!("{}", stringify!(connect_get_lib_version));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectGetLibVersionRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetLibVersion as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectGetLibVersionRet { lib_ver } = res;
        Ok(lib_ver)
    }
    fn connect_compare_cpu(&mut self, xml: String, flags: u32) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_compare_cpu));
        let req: Option<RemoteConnectCompareCpuArgs> =
            Some(RemoteConnectCompareCpuArgs { xml, flags });
        let res = call::<RemoteConnectCompareCpuArgs, RemoteConnectCompareCpuRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectCompareCpu as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainMemoryStatsArgs, RemoteDomainMemoryStatsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMemoryStats as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainAttachDeviceFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainAttachDeviceFlags as i32,
            false,
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
        let _res = call::<RemoteDomainDetachDeviceFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainDetachDeviceFlags as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_baseline_cpu(&mut self, xml_cpus: Vec<String>, flags: u32) -> Result<String, Error> {
        trace!("{}", stringify!(connect_baseline_cpu));
        let req: Option<RemoteConnectBaselineCpuArgs> =
            Some(RemoteConnectBaselineCpuArgs { xml_cpus, flags });
        let res = call::<RemoteConnectBaselineCpuArgs, RemoteConnectBaselineCpuRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectBaselineCpu as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectBaselineCpuRet { cpu } = res;
        Ok(cpu)
    }
    fn domain_get_job_info(
        &mut self,
        dom: RemoteNonnullDomain,
    ) -> Result<RemoteDomainGetJobInfoRet, Error> {
        trace!("{}", stringify!(domain_get_job_info));
        let req: Option<RemoteDomainGetJobInfoArgs> = Some(RemoteDomainGetJobInfoArgs { dom });
        let res = call::<RemoteDomainGetJobInfoArgs, RemoteDomainGetJobInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetJobInfo as i32,
            false,
            req,
        )?;
        Ok(res.body.unwrap())
    }
    fn domain_abort_job(&mut self, dom: RemoteNonnullDomain) -> Result<(), Error> {
        trace!("{}", stringify!(domain_abort_job));
        let req: Option<RemoteDomainAbortJobArgs> = Some(RemoteDomainAbortJobArgs { dom });
        let _res = call::<RemoteDomainAbortJobArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainAbortJob as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn storage_vol_wipe(&mut self, vol: RemoteNonnullStorageVol, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(storage_vol_wipe));
        let req: Option<RemoteStorageVolWipeArgs> = Some(RemoteStorageVolWipeArgs { vol, flags });
        let _res = call::<RemoteStorageVolWipeArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolWipe as i32,
            false,
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
        let _res = call::<RemoteDomainMigrateSetMaxDowntimeArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateSetMaxDowntime as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_domain_event_register_any(&mut self, event_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_domain_event_register_any));
        let req: Option<RemoteConnectDomainEventRegisterAnyArgs> =
            Some(RemoteConnectDomainEventRegisterAnyArgs { event_id });
        let _res = call::<RemoteConnectDomainEventRegisterAnyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectDomainEventRegisterAny as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_domain_event_deregister_any(&mut self, event_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_domain_event_deregister_any));
        let req: Option<RemoteConnectDomainEventDeregisterAnyArgs> =
            Some(RemoteConnectDomainEventDeregisterAnyArgs { event_id });
        let _res = call::<RemoteConnectDomainEventDeregisterAnyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectDomainEventDeregisterAny as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_reboot(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_reboot));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventReboot as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_rtc_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_rtc_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventRtcChange as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_watchdog(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_watchdog));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventWatchdog as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_io_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_io_error));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventIoError as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_graphics(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_graphics));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventGraphics as i32,
            false,
            req,
        )?;
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
        let _res = call::<RemoteDomainUpdateDeviceFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainUpdateDeviceFlags as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn nwfilter_lookup_by_name(&mut self, name: String) -> Result<RemoteNonnullNwfilter, Error> {
        trace!("{}", stringify!(nwfilter_lookup_by_name));
        let req: Option<RemoteNwfilterLookupByNameArgs> =
            Some(RemoteNwfilterLookupByNameArgs { name });
        let res = call::<RemoteNwfilterLookupByNameArgs, RemoteNwfilterLookupByNameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNwfilterLookupByName as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNwfilterLookupByUuidArgs, RemoteNwfilterLookupByUuidRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNwfilterLookupByUuid as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNwfilterGetXmlDescArgs, RemoteNwfilterGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNwfilterGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNwfilterGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn connect_num_of_nwfilters(&mut self) -> Result<i32, Error> {
        trace!("{}", stringify!(connect_num_of_nwfilters));
        let req: Option<()> = None;
        let res = call::<(), RemoteConnectNumOfNwfiltersRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNumOfNwfilters as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectNumOfNwfiltersRet { num } = res;
        Ok(num)
    }
    fn connect_list_nwfilters(&mut self, maxnames: i32) -> Result<Vec<String>, Error> {
        trace!("{}", stringify!(connect_list_nwfilters));
        let req: Option<RemoteConnectListNwfiltersArgs> =
            Some(RemoteConnectListNwfiltersArgs { maxnames });
        let res = call::<RemoteConnectListNwfiltersArgs, RemoteConnectListNwfiltersRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListNwfilters as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectListNwfiltersRet { names } = res;
        Ok(names)
    }
    fn nwfilter_define_xml(&mut self, xml: String) -> Result<RemoteNonnullNwfilter, Error> {
        trace!("{}", stringify!(nwfilter_define_xml));
        let req: Option<RemoteNwfilterDefineXmlArgs> = Some(RemoteNwfilterDefineXmlArgs { xml });
        let res = call::<RemoteNwfilterDefineXmlArgs, RemoteNwfilterDefineXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNwfilterDefineXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNwfilterDefineXmlRet { nwfilter } = res;
        Ok(nwfilter)
    }
    fn nwfilter_undefine(&mut self, nwfilter: RemoteNonnullNwfilter) -> Result<(), Error> {
        trace!("{}", stringify!(nwfilter_undefine));
        let req: Option<RemoteNwfilterUndefineArgs> = Some(RemoteNwfilterUndefineArgs { nwfilter });
        let _res = call::<RemoteNwfilterUndefineArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNwfilterUndefine as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_managed_save(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_managed_save));
        let req: Option<RemoteDomainManagedSaveArgs> =
            Some(RemoteDomainManagedSaveArgs { dom, flags });
        let _res = call::<RemoteDomainManagedSaveArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainManagedSave as i32,
            false,
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
        let res = call::<RemoteDomainHasManagedSaveImageArgs, RemoteDomainHasManagedSaveImageRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainHasManagedSaveImage as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainManagedSaveRemoveArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainManagedSaveRemove as i32,
            false,
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
        let res = call::<RemoteDomainSnapshotCreateXmlArgs, RemoteDomainSnapshotCreateXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotCreateXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainSnapshotGetXmlDescArgs, RemoteDomainSnapshotGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainSnapshotGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_snapshot_num(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_snapshot_num));
        let req: Option<RemoteDomainSnapshotNumArgs> =
            Some(RemoteDomainSnapshotNumArgs { dom, flags });
        let res = call::<RemoteDomainSnapshotNumArgs, RemoteDomainSnapshotNumRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotNum as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainSnapshotListNamesArgs, RemoteDomainSnapshotListNamesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotListNames as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainSnapshotLookupByNameArgs, RemoteDomainSnapshotLookupByNameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotLookupByName as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainHasCurrentSnapshotArgs, RemoteDomainHasCurrentSnapshotRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainHasCurrentSnapshot as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainSnapshotCurrentArgs, RemoteDomainSnapshotCurrentRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotCurrent as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainRevertToSnapshotArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainRevertToSnapshot as i32,
            false,
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
        let _res = call::<RemoteDomainSnapshotDeleteArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotDelete as i32,
            false,
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
        let res = call::<RemoteDomainGetBlockInfoArgs, RemoteDomainGetBlockInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetBlockInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventIoErrorReason as i32,
            false,
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
        let res = call::<RemoteDomainCreateWithFlagsArgs, RemoteDomainCreateWithFlagsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCreateWithFlags as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetMemoryParametersArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetMemoryParameters as i32,
            false,
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
        let res = call::<RemoteDomainGetMemoryParametersArgs, RemoteDomainGetMemoryParametersRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetMemoryParameters as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetVcpusFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetVcpusFlags as i32,
            false,
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
        let res = call::<RemoteDomainGetVcpusFlagsArgs, RemoteDomainGetVcpusFlagsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetVcpusFlags as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainGetVcpusFlagsRet { num } = res;
        Ok(num)
    }
    fn domain_open_console(
        &mut self,
        dom: RemoteNonnullDomain,
        dev_name: Option<String>,
        flags: u32,
    ) -> Result<VirNetStreamResponse<()>, Error> {
        trace!("{}", stringify!(domain_open_console));
        let req: Option<RemoteDomainOpenConsoleArgs> = Some(RemoteDomainOpenConsoleArgs {
            dom,
            dev_name,
            flags,
        });
        let res = call::<RemoteDomainOpenConsoleArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainOpenConsole as i32,
            true,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            channels: self.channel_clone(),
            receiver: res.receiver.unwrap(),
            header: res.header,
            body: None,
        };
        Ok(res)
    }
    fn domain_is_updated(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_is_updated));
        let req: Option<RemoteDomainIsUpdatedArgs> = Some(RemoteDomainIsUpdatedArgs { dom });
        let res = call::<RemoteDomainIsUpdatedArgs, RemoteDomainIsUpdatedRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainIsUpdated as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainIsUpdatedRet { updated } = res;
        Ok(updated)
    }
    fn connect_get_sysinfo(&mut self, flags: u32) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_sysinfo));
        let req: Option<RemoteConnectGetSysinfoArgs> = Some(RemoteConnectGetSysinfoArgs { flags });
        let res = call::<RemoteConnectGetSysinfoArgs, RemoteConnectGetSysinfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetSysinfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetMemoryFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetMemoryFlags as i32,
            false,
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
        let _res = call::<RemoteDomainSetBlkioParametersArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetBlkioParameters as i32,
            false,
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
        let res = call::<RemoteDomainGetBlkioParametersArgs, RemoteDomainGetBlkioParametersRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetBlkioParameters as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainMigrateSetMaxSpeedArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateSetMaxSpeed as i32,
            false,
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
    ) -> Result<VirNetStreamResponse<()>, Error> {
        trace!("{}", stringify!(storage_vol_upload));
        let req: Option<RemoteStorageVolUploadArgs> = Some(RemoteStorageVolUploadArgs {
            vol,
            offset,
            length,
            flags,
        });
        let res = call::<RemoteStorageVolUploadArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolUpload as i32,
            true,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            channels: self.channel_clone(),
            receiver: res.receiver.unwrap(),
            header: res.header,
            body: None,
        };
        Ok(res)
    }
    fn storage_vol_download(
        &mut self,
        vol: RemoteNonnullStorageVol,
        offset: u64,
        length: u64,
        flags: u32,
    ) -> Result<VirNetStreamResponse<()>, Error> {
        trace!("{}", stringify!(storage_vol_download));
        let req: Option<RemoteStorageVolDownloadArgs> = Some(RemoteStorageVolDownloadArgs {
            vol,
            offset,
            length,
            flags,
        });
        let res = call::<RemoteStorageVolDownloadArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolDownload as i32,
            true,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            channels: self.channel_clone(),
            receiver: res.receiver.unwrap(),
            header: res.header,
            body: None,
        };
        Ok(res)
    }
    fn domain_inject_nmi(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_inject_nmi));
        let req: Option<RemoteDomainInjectNmiArgs> = Some(RemoteDomainInjectNmiArgs { dom, flags });
        let _res = call::<RemoteDomainInjectNmiArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainInjectNmi as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_screenshot(
        &mut self,
        dom: RemoteNonnullDomain,
        screen: u32,
        flags: u32,
    ) -> Result<VirNetStreamResponse<RemoteDomainScreenshotRet>, Error> {
        trace!("{}", stringify!(domain_screenshot));
        let req: Option<RemoteDomainScreenshotArgs> =
            Some(RemoteDomainScreenshotArgs { dom, screen, flags });
        let res = call::<RemoteDomainScreenshotArgs, RemoteDomainScreenshotRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainScreenshot as i32,
            true,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            channels: self.channel_clone(),
            receiver: res.receiver.unwrap(),
            header: res.header,
            body: res.body,
        };
        Ok(res)
    }
    fn domain_get_state(
        &mut self,
        dom: RemoteNonnullDomain,
        flags: u32,
    ) -> Result<(i32, i32), Error> {
        trace!("{}", stringify!(domain_get_state));
        let req: Option<RemoteDomainGetStateArgs> = Some(RemoteDomainGetStateArgs { dom, flags });
        let res = call::<RemoteDomainGetStateArgs, RemoteDomainGetStateRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetState as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainMigrateBegin3Args, RemoteDomainMigrateBegin3Ret>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateBegin3 as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainMigratePrepare3Args, RemoteDomainMigratePrepare3Ret>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigratePrepare3 as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
    ) -> Result<VirNetStreamResponse<RemoteDomainMigratePrepareTunnel3Ret>, Error> {
        trace!("{}", stringify!(domain_migrate_prepare_tunnel3));
        let req: Option<RemoteDomainMigratePrepareTunnel3Args> =
            Some(RemoteDomainMigratePrepareTunnel3Args {
                cookie_in,
                flags,
                dname,
                bandwidth,
                dom_xml,
            });
        let res =
            call::<RemoteDomainMigratePrepareTunnel3Args, RemoteDomainMigratePrepareTunnel3Ret>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcDomainMigratePrepareTunnel3 as i32,
                true,
                req,
            )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            channels: self.channel_clone(),
            receiver: res.receiver.unwrap(),
            header: res.header,
            body: res.body,
        };
        Ok(res)
    }
    fn domain_migrate_perform3(
        &mut self,
        args: RemoteDomainMigratePerform3Args,
    ) -> Result<Vec<u8>, Error> {
        trace!("{}", stringify!(domain_migrate_perform3));
        let req: Option<RemoteDomainMigratePerform3Args> = Some(args);
        let res = call::<RemoteDomainMigratePerform3Args, RemoteDomainMigratePerform3Ret>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigratePerform3 as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainMigrateFinish3Args, RemoteDomainMigrateFinish3Ret>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateFinish3 as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainMigrateConfirm3Args, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateConfirm3 as i32,
            false,
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
        let _res = call::<RemoteDomainSetSchedulerParametersFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetSchedulerParametersFlags as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn interface_change_begin(&mut self, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_begin));
        let req: Option<RemoteInterfaceChangeBeginArgs> =
            Some(RemoteInterfaceChangeBeginArgs { flags });
        let _res = call::<RemoteInterfaceChangeBeginArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceChangeBegin as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn interface_change_commit(&mut self, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_commit));
        let req: Option<RemoteInterfaceChangeCommitArgs> =
            Some(RemoteInterfaceChangeCommitArgs { flags });
        let _res = call::<RemoteInterfaceChangeCommitArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceChangeCommit as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn interface_change_rollback(&mut self, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(interface_change_rollback));
        let req: Option<RemoteInterfaceChangeRollbackArgs> =
            Some(RemoteInterfaceChangeRollbackArgs { flags });
        let _res = call::<RemoteInterfaceChangeRollbackArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcInterfaceChangeRollback as i32,
            false,
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
        let res = call::<
            RemoteDomainGetSchedulerParametersFlagsArgs,
            RemoteDomainGetSchedulerParametersFlagsRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetSchedulerParametersFlags as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainGetSchedulerParametersFlagsRet { params } = res;
        Ok(params)
    }
    fn domain_event_control_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_control_error));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventControlError as i32,
            false,
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
        let _res = call::<RemoteDomainPinVcpuFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainPinVcpuFlags as i32,
            false,
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
        let _res = call::<RemoteDomainSendKeyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSendKey as i32,
            false,
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
        let res = call::<RemoteNodeGetCpuStatsArgs, RemoteNodeGetCpuStatsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeGetCpuStats as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNodeGetMemoryStatsArgs, RemoteNodeGetMemoryStatsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeGetMemoryStats as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainGetControlInfoArgs, RemoteDomainGetControlInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetControlInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainGetVcpuPinInfoArgs, RemoteDomainGetVcpuPinInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetVcpuPinInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainGetVcpuPinInfoRet { cpumaps, num } = res;
        Ok((cpumaps, num))
    }
    fn domain_undefine_flags(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_undefine_flags));
        let req: Option<RemoteDomainUndefineFlagsArgs> =
            Some(RemoteDomainUndefineFlagsArgs { dom, flags });
        let _res = call::<RemoteDomainUndefineFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainUndefineFlags as i32,
            false,
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
        let _res = call::<RemoteDomainSaveFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSaveFlags as i32,
            false,
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
        let _res = call::<RemoteDomainRestoreFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainRestoreFlags as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_destroy_flags(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_destroy_flags));
        let req: Option<RemoteDomainDestroyFlagsArgs> =
            Some(RemoteDomainDestroyFlagsArgs { dom, flags });
        let _res = call::<RemoteDomainDestroyFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainDestroyFlags as i32,
            false,
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
        let res = call::<RemoteDomainSaveImageGetXmlDescArgs, RemoteDomainSaveImageGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSaveImageGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSaveImageDefineXmlArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSaveImageDefineXml as i32,
            false,
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
        let _res = call::<RemoteDomainBlockJobAbortArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBlockJobAbort as i32,
            false,
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
        let res = call::<RemoteDomainGetBlockJobInfoArgs, RemoteDomainGetBlockJobInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetBlockJobInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainBlockJobSetSpeedArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBlockJobSetSpeed as i32,
            false,
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
        let _res = call::<RemoteDomainBlockPullArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBlockPull as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_block_job(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_job));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventBlockJob as i32,
            false,
            req,
        )?;
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
        let res = call::<RemoteDomainMigrateGetMaxSpeedArgs, RemoteDomainMigrateGetMaxSpeedRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateGetMaxSpeed as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainBlockStatsFlagsArgs, RemoteDomainBlockStatsFlagsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBlockStatsFlags as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainSnapshotGetParentArgs, RemoteDomainSnapshotGetParentRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotGetParent as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainSnapshotGetParentRet { snap } = res;
        Ok(snap)
    }
    fn domain_reset(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_reset));
        let req: Option<RemoteDomainResetArgs> = Some(RemoteDomainResetArgs { dom, flags });
        let _res = call::<RemoteDomainResetArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainReset as i32,
            false,
            req,
        )?;
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
        let res = call::<RemoteDomainSnapshotNumChildrenArgs, RemoteDomainSnapshotNumChildrenRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotNumChildren as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<
            RemoteDomainSnapshotListChildrenNamesArgs,
            RemoteDomainSnapshotListChildrenNamesRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotListChildrenNames as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainSnapshotListChildrenNamesRet { names } = res;
        Ok(names)
    }
    fn domain_event_disk_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_disk_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventDiskChange as i32,
            false,
            req,
        )?;
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
        let _res = call::<RemoteDomainOpenGraphicsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainOpenGraphics as i32,
            false,
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
        let _res = call::<RemoteNodeSuspendForDurationArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeSuspendForDuration as i32,
            false,
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
        let _res = call::<RemoteDomainBlockResizeArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBlockResize as i32,
            false,
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
        let _res = call::<RemoteDomainSetBlockIoTuneArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetBlockIoTune as i32,
            false,
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
        let res = call::<RemoteDomainGetBlockIoTuneArgs, RemoteDomainGetBlockIoTuneRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetBlockIoTune as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetNumaParametersArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetNumaParameters as i32,
            false,
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
        let res = call::<RemoteDomainGetNumaParametersArgs, RemoteDomainGetNumaParametersRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetNumaParameters as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetInterfaceParametersArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetInterfaceParameters as i32,
            false,
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
        let res =
            call::<RemoteDomainGetInterfaceParametersArgs, RemoteDomainGetInterfaceParametersRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcDomainGetInterfaceParameters as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
        let RemoteDomainGetInterfaceParametersRet { params, nparams } = res;
        Ok((params, nparams))
    }
    fn domain_shutdown_flags(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_shutdown_flags));
        let req: Option<RemoteDomainShutdownFlagsArgs> =
            Some(RemoteDomainShutdownFlagsArgs { dom, flags });
        let _res = call::<RemoteDomainShutdownFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainShutdownFlags as i32,
            false,
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
        let _res = call::<RemoteStorageVolWipePatternArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolWipePattern as i32,
            false,
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
        let _res = call::<RemoteStorageVolResizeArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolResize as i32,
            false,
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
        let _res = call::<RemoteDomainPmSuspendForDurationArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainPmSuspendForDuration as i32,
            false,
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
        let res = call::<RemoteDomainGetCpuStatsArgs, RemoteDomainGetCpuStatsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetCpuStats as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainGetDiskErrorsArgs, RemoteDomainGetDiskErrorsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetDiskErrors as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetMetadataArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetMetadata as i32,
            false,
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
        let res = call::<RemoteDomainGetMetadataArgs, RemoteDomainGetMetadataRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetMetadata as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainBlockRebaseArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBlockRebase as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_pm_wakeup(&mut self, dom: RemoteNonnullDomain, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(domain_pm_wakeup));
        let req: Option<RemoteDomainPmWakeupArgs> = Some(RemoteDomainPmWakeupArgs { dom, flags });
        let _res = call::<RemoteDomainPmWakeupArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainPmWakeup as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_tray_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_tray_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventTrayChange as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_pmwakeup(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmwakeup));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventPmwakeup as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_pmsuspend(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmsuspend));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventPmsuspend as i32,
            false,
            req,
        )?;
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
        let res = call::<RemoteDomainSnapshotIsCurrentArgs, RemoteDomainSnapshotIsCurrentRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotIsCurrent as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainSnapshotHasMetadataArgs, RemoteDomainSnapshotHasMetadataRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotHasMetadata as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteConnectListAllDomainsArgs, RemoteConnectListAllDomainsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListAllDomains as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainListAllSnapshotsArgs, RemoteDomainListAllSnapshotsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainListAllSnapshots as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<
            RemoteDomainSnapshotListAllChildrenArgs,
            RemoteDomainSnapshotListAllChildrenRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSnapshotListAllChildren as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainSnapshotListAllChildrenRet { snapshots, ret } = res;
        Ok((snapshots, ret))
    }
    fn domain_event_balloon_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_balloon_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventBalloonChange as i32,
            false,
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
        let res = call::<RemoteDomainGetHostnameArgs, RemoteDomainGetHostnameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetHostname as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainGetSecurityLabelListArgs, RemoteDomainGetSecurityLabelListRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetSecurityLabelList as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainPinEmulatorArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainPinEmulator as i32,
            false,
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
        let res = call::<RemoteDomainGetEmulatorPinInfoArgs, RemoteDomainGetEmulatorPinInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetEmulatorPinInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteConnectListAllStoragePoolsArgs, RemoteConnectListAllStoragePoolsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListAllStoragePools as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteStoragePoolListAllVolumesArgs, RemoteStoragePoolListAllVolumesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolListAllVolumes as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteConnectListAllNetworksArgs, RemoteConnectListAllNetworksRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListAllNetworks as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteConnectListAllInterfacesArgs, RemoteConnectListAllInterfacesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListAllInterfaces as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteConnectListAllNodeDevicesArgs, RemoteConnectListAllNodeDevicesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListAllNodeDevices as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteConnectListAllNwfiltersArgs, RemoteConnectListAllNwfiltersRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListAllNwfilters as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteConnectListAllSecretsArgs, RemoteConnectListAllSecretsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListAllSecrets as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteNodeSetMemoryParametersArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeSetMemoryParameters as i32,
            false,
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
        let res = call::<RemoteNodeGetMemoryParametersArgs, RemoteNodeGetMemoryParametersRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeGetMemoryParameters as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainBlockCommitArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBlockCommit as i32,
            false,
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
        let _res = call::<RemoteNetworkUpdateArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkUpdate as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_pmsuspend_disk(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_pmsuspend_disk));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventPmsuspendDisk as i32,
            false,
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
        let res = call::<RemoteNodeGetCpuMapArgs, RemoteNodeGetCpuMapRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeGetCpuMap as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainFstrimArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainFstrim as i32,
            false,
            req,
        )?;
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
        let _res = call::<RemoteDomainSendProcessSignalArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSendProcessSignal as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_open_channel(
        &mut self,
        dom: RemoteNonnullDomain,
        name: Option<String>,
        flags: u32,
    ) -> Result<VirNetStreamResponse<()>, Error> {
        trace!("{}", stringify!(domain_open_channel));
        let req: Option<RemoteDomainOpenChannelArgs> =
            Some(RemoteDomainOpenChannelArgs { dom, name, flags });
        let res = call::<RemoteDomainOpenChannelArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainOpenChannel as i32,
            true,
            req,
        )?;
        let res = VirNetStreamResponse {
            inner: self.inner_clone()?,
            channels: self.channel_clone(),
            receiver: res.receiver.unwrap(),
            header: res.header,
            body: None,
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
        let res = call::<
            RemoteNodeDeviceLookupScsiHostByWwnArgs,
            RemoteNodeDeviceLookupScsiHostByWwnRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceLookupScsiHostByWwn as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainGetJobStatsArgs, RemoteDomainGetJobStatsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetJobStats as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<
            RemoteDomainMigrateGetCompressionCacheArgs,
            RemoteDomainMigrateGetCompressionCacheRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateGetCompressionCache as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainMigrateSetCompressionCacheArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateSetCompressionCache as i32,
            false,
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
        let _res = call::<RemoteNodeDeviceDetachFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceDetachFlags as i32,
            false,
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
        let res = call::<RemoteDomainMigrateBegin3ParamsArgs, RemoteDomainMigrateBegin3ParamsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateBegin3Params as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res =
            call::<RemoteDomainMigratePrepare3ParamsArgs, RemoteDomainMigratePrepare3ParamsRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcDomainMigratePrepare3Params as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
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
        let res = call::<
            RemoteDomainMigratePrepareTunnel3ParamsArgs,
            RemoteDomainMigratePrepareTunnel3ParamsRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigratePrepareTunnel3Params as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res =
            call::<RemoteDomainMigratePerform3ParamsArgs, RemoteDomainMigratePerform3ParamsRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcDomainMigratePerform3Params as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainMigrateFinish3ParamsArgs, RemoteDomainMigrateFinish3ParamsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateFinish3Params as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainMigrateConfirm3ParamsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateConfirm3Params as i32,
            false,
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
        let _res = call::<RemoteDomainSetMemoryStatsPeriodArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetMemoryStatsPeriod as i32,
            false,
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
        let res = call::<RemoteDomainCreateXmlWithFilesArgs, RemoteDomainCreateXmlWithFilesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCreateXmlWithFiles as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainCreateWithFilesArgs, RemoteDomainCreateWithFilesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCreateWithFiles as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainCreateWithFilesRet { dom } = res;
        Ok(dom)
    }
    fn domain_event_device_removed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_device_removed));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventDeviceRemoved as i32,
            false,
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
        let res = call::<RemoteConnectGetCpuModelNamesArgs, RemoteConnectGetCpuModelNamesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetCpuModelNames as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<
            RemoteConnectNetworkEventRegisterAnyArgs,
            RemoteConnectNetworkEventRegisterAnyRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNetworkEventRegisterAny as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectNetworkEventRegisterAnyRet { callback_id } = res;
        Ok(callback_id)
    }
    fn connect_network_event_deregister_any(&mut self, callback_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_network_event_deregister_any));
        let req: Option<RemoteConnectNetworkEventDeregisterAnyArgs> =
            Some(RemoteConnectNetworkEventDeregisterAnyArgs { callback_id });
        let _res = call::<RemoteConnectNetworkEventDeregisterAnyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNetworkEventDeregisterAny as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn network_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(network_event_lifecycle));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkEventLifecycle as i32,
            false,
            req,
        )?;
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
        let res = call::<
            RemoteConnectDomainEventCallbackRegisterAnyArgs,
            RemoteConnectDomainEventCallbackRegisterAnyRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectDomainEventCallbackRegisterAny as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteConnectDomainEventCallbackDeregisterAnyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectDomainEventCallbackDeregisterAny as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_lifecycle));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackLifecycle as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_reboot(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_reboot));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackReboot as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_rtc_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_rtc_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackRtcChange as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_watchdog(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_watchdog));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackWatchdog as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_io_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackIoError as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_graphics(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_graphics));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackGraphics as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_io_error_reason(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_io_error_reason));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackIoErrorReason as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_control_error(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_control_error));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackControlError as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_block_job(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_block_job));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackBlockJob as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_disk_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_disk_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackDiskChange as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_tray_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_tray_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackTrayChange as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmwakeup(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmwakeup));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackPmwakeup as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmsuspend(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackPmsuspend as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_balloon_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_balloon_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackBalloonChange as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_pmsuspend_disk(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_pmsuspend_disk));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackPmsuspendDisk as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_device_removed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_device_removed));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackDeviceRemoved as i32,
            false,
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
        let _res = call::<RemoteDomainCoreDumpWithFormatArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCoreDumpWithFormat as i32,
            false,
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
        let res = call::<RemoteDomainFsfreezeArgs, RemoteDomainFsfreezeRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainFsfreeze as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainFsthawArgs, RemoteDomainFsthawRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainFsthaw as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainGetTimeArgs, RemoteDomainGetTimeRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetTime as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetTimeArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetTime as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_block_job2(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_job2));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventBlockJob2 as i32,
            false,
            req,
        )?;
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
        let res = call::<RemoteNodeGetFreePagesArgs, RemoteNodeGetFreePagesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeGetFreePages as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNetworkGetDhcpLeasesArgs, RemoteNetworkGetDhcpLeasesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkGetDhcpLeases as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res =
            call::<RemoteConnectGetDomainCapabilitiesArgs, RemoteConnectGetDomainCapabilitiesRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcConnectGetDomainCapabilities as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainOpenGraphicsFdArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainOpenGraphicsFd as i32,
            false,
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
        let res = call::<RemoteConnectGetAllDomainStatsArgs, RemoteConnectGetAllDomainStatsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetAllDomainStats as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainBlockCopyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBlockCopy as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_tunable(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_tunable));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackTunable as i32,
            false,
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
        let res = call::<RemoteNodeAllocPagesArgs, RemoteNodeAllocPagesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeAllocPages as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeAllocPagesRet { ret } = res;
        Ok(ret)
    }
    fn domain_event_callback_agent_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_agent_lifecycle));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackAgentLifecycle as i32,
            false,
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
        let res = call::<RemoteDomainGetFsinfoArgs, RemoteDomainGetFsinfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetFsinfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainDefineXmlFlagsArgs, RemoteDomainDefineXmlFlagsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainDefineXmlFlags as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainGetIothreadInfoArgs, RemoteDomainGetIothreadInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetIothreadInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainPinIothreadArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainPinIothread as i32,
            false,
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
        let res = call::<RemoteDomainInterfaceAddressesArgs, RemoteDomainInterfaceAddressesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainInterfaceAddresses as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainInterfaceAddressesRet { ifaces } = res;
        Ok(ifaces)
    }
    fn domain_event_callback_device_added(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_device_added));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackDeviceAdded as i32,
            false,
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
        let _res = call::<RemoteDomainAddIothreadArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainAddIothread as i32,
            false,
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
        let _res = call::<RemoteDomainDelIothreadArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainDelIothread as i32,
            false,
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
        let _res = call::<RemoteDomainSetUserPasswordArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetUserPassword as i32,
            false,
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
        let res = call::<RemoteDomainRenameArgs, RemoteDomainRenameRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainRename as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainRenameRet { retcode } = res;
        Ok(retcode)
    }
    fn domain_event_callback_migration_iteration(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_migration_iteration));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackMigrationIteration as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_register_close_callback(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_register_close_callback));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectRegisterCloseCallback as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_unregister_close_callback(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_unregister_close_callback));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectUnregisterCloseCallback as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_event_connection_closed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(connect_event_connection_closed));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectEventConnectionClosed as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_callback_job_completed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_callback_job_completed));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackJobCompleted as i32,
            false,
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
        let _res = call::<RemoteDomainMigrateStartPostCopyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainMigrateStartPostCopy as i32,
            false,
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
        let res = call::<RemoteDomainGetPerfEventsArgs, RemoteDomainGetPerfEventsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetPerfEvents as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetPerfEventsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetPerfEvents as i32,
            false,
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
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackDeviceRemovalFailed as i32,
            false,
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
        let res = call::<
            RemoteConnectStoragePoolEventRegisterAnyArgs,
            RemoteConnectStoragePoolEventRegisterAnyRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectStoragePoolEventRegisterAny as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectStoragePoolEventRegisterAnyRet { callback_id } = res;
        Ok(callback_id)
    }
    fn connect_storage_pool_event_deregister_any(&mut self, callback_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_storage_pool_event_deregister_any));
        let req: Option<RemoteConnectStoragePoolEventDeregisterAnyArgs> =
            Some(RemoteConnectStoragePoolEventDeregisterAnyArgs { callback_id });
        let _res = call::<RemoteConnectStoragePoolEventDeregisterAnyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectStoragePoolEventDeregisterAny as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_event_lifecycle));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolEventLifecycle as i32,
            false,
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
        let res = call::<RemoteDomainGetGuestVcpusArgs, RemoteDomainGetGuestVcpusRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetGuestVcpus as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetGuestVcpusArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetGuestVcpus as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn storage_pool_event_refresh(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(storage_pool_event_refresh));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolEventRefresh as i32,
            false,
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
        let res = call::<
            RemoteConnectNodeDeviceEventRegisterAnyArgs,
            RemoteConnectNodeDeviceEventRegisterAnyRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNodeDeviceEventRegisterAny as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectNodeDeviceEventRegisterAnyRet { callback_id } = res;
        Ok(callback_id)
    }
    fn connect_node_device_event_deregister_any(&mut self, callback_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_node_device_event_deregister_any));
        let req: Option<RemoteConnectNodeDeviceEventDeregisterAnyArgs> =
            Some(RemoteConnectNodeDeviceEventDeregisterAnyArgs { callback_id });
        let _res = call::<RemoteConnectNodeDeviceEventDeregisterAnyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectNodeDeviceEventDeregisterAny as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn node_device_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_event_lifecycle));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceEventLifecycle as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn node_device_event_update(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_event_update));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceEventUpdate as i32,
            false,
            req,
        )?;
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
        let res = call::<RemoteStorageVolGetInfoFlagsArgs, RemoteStorageVolGetInfoFlagsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStorageVolGetInfoFlags as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventCallbackMetadataChange as i32,
            false,
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
        let res = call::<
            RemoteConnectSecretEventRegisterAnyArgs,
            RemoteConnectSecretEventRegisterAnyRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectSecretEventRegisterAny as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteConnectSecretEventRegisterAnyRet { callback_id } = res;
        Ok(callback_id)
    }
    fn connect_secret_event_deregister_any(&mut self, callback_id: i32) -> Result<(), Error> {
        trace!("{}", stringify!(connect_secret_event_deregister_any));
        let req: Option<RemoteConnectSecretEventDeregisterAnyArgs> =
            Some(RemoteConnectSecretEventDeregisterAnyArgs { callback_id });
        let _res = call::<RemoteConnectSecretEventDeregisterAnyArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectSecretEventDeregisterAny as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn secret_event_lifecycle(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(secret_event_lifecycle));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcSecretEventLifecycle as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn secret_event_value_changed(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(secret_event_value_changed));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcSecretEventValueChanged as i32,
            false,
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
        let _res = call::<RemoteDomainSetVcpuArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetVcpu as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_block_threshold(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_block_threshold));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventBlockThreshold as i32,
            false,
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
        let _res = call::<RemoteDomainSetBlockThresholdArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetBlockThreshold as i32,
            false,
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
        let res =
            call::<RemoteDomainMigrateGetMaxDowntimeArgs, RemoteDomainMigrateGetMaxDowntimeRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcDomainMigrateGetMaxDowntime as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
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
        let res =
            call::<RemoteDomainManagedSaveGetXmlDescArgs, RemoteDomainManagedSaveGetXmlDescRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcDomainManagedSaveGetXmlDesc as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainManagedSaveDefineXmlArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainManagedSaveDefineXml as i32,
            false,
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
        let _res = call::<RemoteDomainSetLifecycleActionArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetLifecycleAction as i32,
            false,
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
        let res = call::<
            RemoteStoragePoolLookupByTargetPathArgs,
            RemoteStoragePoolLookupByTargetPathRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcStoragePoolLookupByTargetPath as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainDetachDeviceAliasArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainDetachDeviceAlias as i32,
            false,
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
        let res =
            call::<RemoteConnectCompareHypervisorCpuArgs, RemoteConnectCompareHypervisorCpuRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcConnectCompareHypervisorCpu as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
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
        let res =
            call::<RemoteConnectBaselineHypervisorCpuArgs, RemoteConnectBaselineHypervisorCpuRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcConnectBaselineHypervisorCpu as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNodeGetSevInfoArgs, RemoteNodeGetSevInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeGetSevInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res =
            call::<RemoteDomainGetLaunchSecurityInfoArgs, RemoteDomainGetLaunchSecurityInfoRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcDomainGetLaunchSecurityInfo as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
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
        let res = call::<
            RemoteNwfilterBindingLookupByPortDevArgs,
            RemoteNwfilterBindingLookupByPortDevRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNwfilterBindingLookupByPortDev as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNwfilterBindingGetXmlDescArgs, RemoteNwfilterBindingGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNwfilterBindingGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNwfilterBindingCreateXmlArgs, RemoteNwfilterBindingCreateXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNwfilterBindingCreateXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteNwfilterBindingDeleteArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNwfilterBindingDelete as i32,
            false,
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
        let res = call::<
            RemoteConnectListAllNwfilterBindingsArgs,
            RemoteConnectListAllNwfilterBindingsRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectListAllNwfilterBindings as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetIothreadParamsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetIothreadParams as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn connect_get_storage_pool_capabilities(&mut self, flags: u32) -> Result<String, Error> {
        trace!("{}", stringify!(connect_get_storage_pool_capabilities));
        let req: Option<RemoteConnectGetStoragePoolCapabilitiesArgs> =
            Some(RemoteConnectGetStoragePoolCapabilitiesArgs { flags });
        let res = call::<
            RemoteConnectGetStoragePoolCapabilitiesArgs,
            RemoteConnectGetStoragePoolCapabilitiesRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectGetStoragePoolCapabilities as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNetworkListAllPortsArgs, RemoteNetworkListAllPortsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkListAllPorts as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNetworkPortLookupByUuidArgs, RemoteNetworkPortLookupByUuidRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkPortLookupByUuid as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNetworkPortCreateXmlArgs, RemoteNetworkPortCreateXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkPortCreateXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNetworkPortGetParametersArgs, RemoteNetworkPortGetParametersRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkPortGetParameters as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteNetworkPortSetParametersArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkPortSetParameters as i32,
            false,
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
        let res = call::<RemoteNetworkPortGetXmlDescArgs, RemoteNetworkPortGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkPortGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteNetworkPortDeleteArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkPortDelete as i32,
            false,
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
        let res = call::<RemoteDomainCheckpointCreateXmlArgs, RemoteDomainCheckpointCreateXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCheckpointCreateXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainCheckpointGetXmlDescArgs, RemoteDomainCheckpointGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCheckpointGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainListAllCheckpointsArgs, RemoteDomainListAllCheckpointsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainListAllCheckpoints as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<
            RemoteDomainCheckpointListAllChildrenArgs,
            RemoteDomainCheckpointListAllChildrenRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCheckpointListAllChildren as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res =
            call::<RemoteDomainCheckpointLookupByNameArgs, RemoteDomainCheckpointLookupByNameRet>(
                self,
                REMOTE_PROGRAM,
                REMOTE_PROTOCOL_VERSION,
                RemoteProcedure::RemoteProcDomainCheckpointLookupByName as i32,
                false,
                req,
            )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteDomainCheckpointGetParentArgs, RemoteDomainCheckpointGetParentRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCheckpointGetParent as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainCheckpointDeleteArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainCheckpointDelete as i32,
            false,
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
        let res = call::<RemoteDomainGetGuestInfoArgs, RemoteDomainGetGuestInfoRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetGuestInfo as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteConnectSetIdentityArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcConnectSetIdentity as i32,
            false,
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
        let res = call::<
            RemoteDomainAgentSetResponseTimeoutArgs,
            RemoteDomainAgentSetResponseTimeoutRet,
        >(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainAgentSetResponseTimeout as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainBackupBeginArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBackupBegin as i32,
            false,
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
        let res = call::<RemoteDomainBackupGetXmlDescArgs, RemoteDomainBackupGetXmlDescRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainBackupGetXmlDesc as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteDomainBackupGetXmlDescRet { xml } = res;
        Ok(xml)
    }
    fn domain_event_memory_failure(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_memory_failure));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventMemoryFailure as i32,
            false,
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
        let res = call::<RemoteDomainAuthorizedSshKeysGetArgs, RemoteDomainAuthorizedSshKeysGetRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainAuthorizedSshKeysGet as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainAuthorizedSshKeysSetArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainAuthorizedSshKeysSet as i32,
            false,
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
        let res = call::<RemoteDomainGetMessagesArgs, RemoteDomainGetMessagesRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetMessages as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainStartDirtyRateCalcArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainStartDirtyRateCalc as i32,
            false,
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
        let res = call::<RemoteNodeDeviceDefineXmlArgs, RemoteNodeDeviceDefineXmlRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceDefineXml as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeDeviceDefineXmlRet { dev } = res;
        Ok(dev)
    }
    fn node_device_undefine(&mut self, name: String, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_undefine));
        let req: Option<RemoteNodeDeviceUndefineArgs> =
            Some(RemoteNodeDeviceUndefineArgs { name, flags });
        let _res = call::<RemoteNodeDeviceUndefineArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceUndefine as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn node_device_create(&mut self, name: String, flags: u32) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_create));
        let req: Option<RemoteNodeDeviceCreateArgs> =
            Some(RemoteNodeDeviceCreateArgs { name, flags });
        let _res = call::<RemoteNodeDeviceCreateArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceCreate as i32,
            false,
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
        let res = call::<RemoteNwfilterDefineXmlFlagsArgs, RemoteNwfilterDefineXmlFlagsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNwfilterDefineXmlFlags as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNetworkDefineXmlFlagsArgs, RemoteNetworkDefineXmlFlagsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkDefineXmlFlags as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNetworkDefineXmlFlagsRet { net } = res;
        Ok(net)
    }
    fn node_device_get_autostart(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_get_autostart));
        let req: Option<RemoteNodeDeviceGetAutostartArgs> =
            Some(RemoteNodeDeviceGetAutostartArgs { name });
        let res = call::<RemoteNodeDeviceGetAutostartArgs, RemoteNodeDeviceGetAutostartRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceGetAutostart as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeDeviceGetAutostartRet { autostart } = res;
        Ok(autostart)
    }
    fn node_device_set_autostart(&mut self, name: String, autostart: i32) -> Result<(), Error> {
        trace!("{}", stringify!(node_device_set_autostart));
        let req: Option<RemoteNodeDeviceSetAutostartArgs> =
            Some(RemoteNodeDeviceSetAutostartArgs { name, autostart });
        let _res = call::<RemoteNodeDeviceSetAutostartArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceSetAutostart as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn node_device_is_persistent(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_is_persistent));
        let req: Option<RemoteNodeDeviceIsPersistentArgs> =
            Some(RemoteNodeDeviceIsPersistentArgs { name });
        let res = call::<RemoteNodeDeviceIsPersistentArgs, RemoteNodeDeviceIsPersistentRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceIsPersistent as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNodeDeviceIsPersistentRet { persistent } = res;
        Ok(persistent)
    }
    fn node_device_is_active(&mut self, name: String) -> Result<i32, Error> {
        trace!("{}", stringify!(node_device_is_active));
        let req: Option<RemoteNodeDeviceIsActiveArgs> = Some(RemoteNodeDeviceIsActiveArgs { name });
        let res = call::<RemoteNodeDeviceIsActiveArgs, RemoteNodeDeviceIsActiveRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceIsActive as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let res = call::<RemoteNetworkCreateXmlFlagsArgs, RemoteNetworkCreateXmlFlagsRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkCreateXmlFlags as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNetworkCreateXmlFlagsRet { net } = res;
        Ok(net)
    }
    fn domain_event_memory_device_size_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_memory_device_size_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventMemoryDeviceSizeChange as i32,
            false,
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
        let _res = call::<RemoteDomainSetLaunchSecurityStateArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetLaunchSecurityState as i32,
            false,
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
        let _res = call::<RemoteDomainSaveParamsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSaveParams as i32,
            false,
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
        let _res = call::<RemoteDomainRestoreParamsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainRestoreParams as i32,
            false,
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
        let _res = call::<RemoteDomainAbortJobFlagsArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainAbortJobFlags as i32,
            false,
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
        let _res = call::<RemoteDomainFdAssociateArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainFdAssociate as i32,
            false,
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
        let _res = call::<RemoteNetworkSetMetadataArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkSetMetadata as i32,
            false,
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
        let res = call::<RemoteNetworkGetMetadataArgs, RemoteNetworkGetMetadataRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkGetMetadata as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
        let RemoteNetworkGetMetadataRet { metadata } = res;
        Ok(metadata)
    }
    fn network_event_callback_metadata_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(network_event_callback_metadata_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNetworkEventCallbackMetadataChange as i32,
            false,
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
        let _res = call::<RemoteNodeDeviceUpdateArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcNodeDeviceUpdate as i32,
            false,
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
        let _res = call::<RemoteDomainGraphicsReloadArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGraphicsReload as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_get_autostart_once(&mut self, dom: RemoteNonnullDomain) -> Result<i32, Error> {
        trace!("{}", stringify!(domain_get_autostart_once));
        let req: Option<RemoteDomainGetAutostartOnceArgs> =
            Some(RemoteDomainGetAutostartOnceArgs { dom });
        let res = call::<RemoteDomainGetAutostartOnceArgs, RemoteDomainGetAutostartOnceRet>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainGetAutostartOnce as i32,
            false,
            req,
        )?;
        let res = res.body.unwrap();
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
        let _res = call::<RemoteDomainSetAutostartOnceArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetAutostartOnce as i32,
            false,
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
        let _res = call::<RemoteDomainSetThrottleGroupArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainSetThrottleGroup as i32,
            false,
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
        let _res = call::<RemoteDomainDelThrottleGroupArgs, ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainDelThrottleGroup as i32,
            false,
            req,
        )?;
        Ok(())
    }
    fn domain_event_nic_mac_change(&mut self) -> Result<(), Error> {
        trace!("{}", stringify!(domain_event_nic_mac_change));
        let req: Option<()> = None;
        let _res = call::<(), ()>(
            self,
            REMOTE_PROGRAM,
            REMOTE_PROTOCOL_VERSION,
            RemoteProcedure::RemoteProcDomainEventNicMacChange as i32,
            false,
            req,
        )?;
        Ok(())
    }
}
impl<D> VirNetStreamResponse<D>
where
    D: DeserializeOwned,
{
    pub fn new(
        inner: Box<dyn ReadWrite>,
        channels: Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>,
        receiver: Receiver<VirNetResponseRaw>,
        header: protocol::VirNetMessageHeader,
        body: Option<D>,
    ) -> Self {
        VirNetStreamResponse {
            inner,
            channels,
            receiver,
            header,
            body,
        }
    }
    pub fn fin(&self) {
        let mut channels = self.channels.lock().unwrap();
        channels.remove(&self.header.serial);
    }
    pub fn data(&self) -> Option<&D> {
        self.body.as_ref()
    }
    pub fn download(&mut self) -> Result<Option<VirNetStream>, Error> {
        download(self)
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
impl TryFrom<VirNetResponseRaw> for QemuDomainMonitorEventMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteConnectEventConnectionClosedMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventBalloonChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventBlockJob2Msg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventBlockJobMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventBlockThresholdMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackAgentLifecycleMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackBalloonChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackBlockJobMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackControlErrorMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackDeviceAddedMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackDeviceRemovalFailedMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackDeviceRemovedMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackDiskChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackGraphicsMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackIoErrorMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackIoErrorReasonMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackJobCompletedMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackLifecycleMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackMetadataChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackMigrationIterationMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackPmsuspendDiskMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackPmsuspendMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackPmwakeupMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackRebootMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackRtcChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackTrayChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackTunableMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventCallbackWatchdogMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventControlErrorMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventDeviceRemovedMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventDiskChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventGraphicsMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventIoErrorMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventIoErrorReasonMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventLifecycleMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventMemoryDeviceSizeChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventMemoryFailureMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventNicMacChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventPmsuspendDiskMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventPmsuspendMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventPmwakeupMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventRebootMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventRtcChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventTrayChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteDomainEventWatchdogMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteNetworkEventCallbackMetadataChangeMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteNetworkEventLifecycleMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteNodeDeviceEventLifecycleMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteNodeDeviceEventUpdateMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteSecretEventLifecycleMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteSecretEventValueChangedMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteStoragePoolEventLifecycleMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<VirNetResponseRaw> for RemoteStoragePoolEventRefreshMsg {
    type Error = Error;
    fn try_from(value: VirNetResponseRaw) -> Result<Self, Self::Error> {
        let header = value.header;
        let body = value.body.unwrap();
        match deserialize_body(&header, body) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok(body),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }
}
fn call<S, D>(
    client: &mut impl Libvirt,
    program: u32,
    version: u32,
    procedure: i32,
    stream: bool,
    args: Option<S>,
) -> Result<VirNetResponseSet<D>, Error>
where
    S: Serialize,
    D: DeserializeOwned,
{
    let serial = client.serial_add(1);
    if !client.receiver_running() {
        return Err(Error::ReceiverNotStartedError);
    }
    let (tx, rx) = channel();
    client.add_channel(serial, tx);
    let socket = client.inner();
    if let Err(e) = send(
        socket,
        program,
        version,
        procedure,
        protocol::VirNetMessageType::VirNetCall,
        serial,
        protocol::VirNetMessageStatus::VirNetOk,
        args.map(|a| VirNetRequest::Data(a)),
    ) {
        client.remove_channel(serial);
        return Err(e);
    }
    let ret = read_data::<D>(stream, client.channel_clone(), &rx, serial);
    ret.map(|(header, body)| VirNetResponseSet {
        receiver: Some(rx),
        header,
        body,
    })
}
fn download<D>(response: &mut VirNetStreamResponse<D>) -> Result<Option<VirNetStream>, Error>
where
    D: DeserializeOwned,
{
    let serial = response.header.serial;
    let res = response
        .receiver
        .recv_timeout(Duration::from_secs(180))
        .map_err(Error::ReceiveChannelError)?;
    if let Some(res_body_bytes) = res.body {
        match deserialize_body::<()>(&res.header, res_body_bytes) {
            Ok(res_body) => match res_body {
                VirNetResponse::Stream(stream) => Ok(Some(stream)),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    } else {
        response.channels.lock().unwrap().remove(&serial);
        Ok(None)
    }
}
fn upload<D>(
    response: &mut VirNetStreamResponse<D>,
    procedure: RemoteProcedure,
    buf: &[u8],
) -> Result<(), Error>
where
    D: DeserializeOwned,
{
    let bytes = VirNetStream::Raw(buf.to_vec());
    let req: Option<VirNetRequest<()>> = Some(VirNetRequest::Stream(bytes));
    send(
        &mut response.inner,
        response.header.prog,
        response.header.vers,
        procedure as i32,
        protocol::VirNetMessageType::VirNetStream,
        response.header.serial,
        protocol::VirNetMessageStatus::VirNetContinue,
        req,
    )?;
    Ok(())
}
fn send_hole<D>(
    response: &mut VirNetStreamResponse<D>,
    procedure: RemoteProcedure,
    length: i64,
    flags: u32,
) -> Result<(), Error>
where
    D: DeserializeOwned,
{
    let hole = VirNetStream::Hole(protocol::VirNetStreamHole { length, flags });
    let args: Option<VirNetRequest<()>> = Some(VirNetRequest::Stream(hole));
    send(
        &mut response.inner,
        response.header.prog,
        response.header.vers,
        procedure as i32,
        protocol::VirNetMessageType::VirNetStreamHole,
        response.header.serial,
        protocol::VirNetMessageStatus::VirNetContinue,
        args,
    )?;
    Ok(())
}
fn upload_completed<D>(
    response: &mut VirNetStreamResponse<D>,
    procedure: RemoteProcedure,
) -> Result<(), Error>
where
    D: DeserializeOwned,
{
    let req: Option<VirNetRequest<()>> = None;
    send(
        &mut response.inner,
        response.header.prog,
        response.header.vers,
        procedure as i32,
        protocol::VirNetMessageType::VirNetStream,
        response.header.serial,
        protocol::VirNetMessageStatus::VirNetOk,
        req,
    )?;
    let (_header, _res) = read_data::<()>(
        false,
        Arc::clone(&response.channels),
        &response.receiver,
        response.header.serial,
    )?;
    Ok(())
}
#[allow(clippy::too_many_arguments)]
fn send<S>(
    socket: &mut Box<dyn ReadWrite>,
    program: u32,
    version: u32,
    procedure: i32,
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
        prog: program,
        vers: version,
        proc: procedure,
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
    let mut bytes = vec![];
    bytes.extend(req_len.to_be_bytes());
    bytes.extend(req_header_bytes);
    if let Some(args_bytes) = &args_bytes {
        bytes.extend(args_bytes);
    }
    socket.write_all(&bytes).map_err(Error::SendError)?;
    Ok(bytes.len())
}
fn recv_thread(
    receiver_run: Arc<AtomicBool>,
    socket: Box<dyn ReadWrite>,
    channels: Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>,
    events: Sender<VirNetResponseRaw>,
) {
    trace!("receiver started.");
    let mut socket = socket;
    while receiver_run.load(Ordering::SeqCst) {
        match recv_raw(&mut socket) {
            Ok((header, body_bytes)) => {
                let serial = header.serial;
                let raw = VirNetResponseRaw {
                    header,
                    body: body_bytes,
                };
                if let Some(tx) = channels.lock().unwrap().get(&serial) {
                    if let Err(e) = tx.send(raw) {
                        trace!("receiver failed to send {}.", e);
                    }
                } else if raw.header.r#type == protocol::VirNetMessageType::VirNetMessage {
                    if let Err(e) = events.send(raw) {
                        trace!("receiver failed to send {}.", e);
                    }
                } else {
                    trace!("receiver not found for serial No.{}.", serial);
                }
            }
            Err(Error::ReceiveError(e)) => {
                trace!("receiver error {}.", e);
                if e.kind() == ErrorKind::UnexpectedEof {
                    receiver_run.fetch_and(false, Ordering::SeqCst);
                }
            }
            Err(e) => {
                trace!("receiver error {}.", e);
            }
        }
    }
    trace!("receiver stopped.");
}
fn recv_raw(
    socket: &mut Box<dyn ReadWrite>,
) -> Result<(protocol::VirNetMessageHeader, Option<Vec<u8>>), Error> {
    let res_len = read_pkt_len(socket)?;
    let res_header = read_res_header(socket)?;
    let body_len = res_len - 28;
    if body_len == 0 {
        return Ok((res_header, None));
    }
    Ok((res_header, Some(read_res_body(socket, body_len)?)))
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
fn read_res_body(socket: &mut Box<dyn ReadWrite>, size: usize) -> Result<Vec<u8>, Error> {
    let mut res_body_bytes = vec![0u8; size];
    socket
        .read_exact(&mut res_body_bytes)
        .map_err(Error::ReceiveError)?;
    Ok(res_body_bytes)
}
fn read_data<D>(
    stream: bool,
    channels: Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>,
    rx: &Receiver<VirNetResponseRaw>,
    serial: u32,
) -> Result<(protocol::VirNetMessageHeader, Option<D>), Error>
where
    D: DeserializeOwned,
{
    let res = rx
        .recv_timeout(Duration::from_secs(180))
        .map_err(Error::ReceiveChannelError)?;
    let ret = if let Some(res_body_bytes) = res.body {
        match deserialize_body(&res.header, res_body_bytes) {
            Ok(res_body) => match res_body {
                VirNetResponse::Data(body) => Ok((res.header, Some(body))),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    } else {
        Ok((res.header, None))
    };
    if !stream {
        channels.lock().unwrap().remove(&serial);
    }
    ret
}
fn deserialize_body<D>(
    res_header: &protocol::VirNetMessageHeader,
    res_body_bytes: Vec<u8>,
) -> Result<VirNetResponse<D>, Error>
where
    D: DeserializeOwned,
{
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
