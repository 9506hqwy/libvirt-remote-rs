use serde::{Deserialize, Serialize};
pub const VIR_NET_MESSAGE_INITIAL: u32 = 65536u32;
pub const VIR_NET_MESSAGE_LEGACY_PAYLOAD_MAX: u32 = 262120u32;
pub const VIR_NET_MESSAGE_MAX: u32 = 33554432u32;
pub const VIR_NET_MESSAGE_HEADER_MAX: u32 = 24u32;
pub const VIR_NET_MESSAGE_PAYLOAD_MAX: u32 = 33554408u32;
pub const VIR_NET_MESSAGE_LEN_MAX: u32 = 4u32;
pub const VIR_NET_MESSAGE_STRING_MAX: u32 = 4194304u32;
pub const VIR_NET_MESSAGE_NUM_FDS_MAX: u32 = 32u32;
pub const VIR_NET_MESSAGE_HEADER_XDR_LEN: u32 = 4u32;
pub const VIR_UUID_BUFLEN: u32 = 16u32;
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum Virnetmessagetype {
    VirNetCall = 0i32,
    VirNetReply = 1i32,
    VirNetMessage = 2i32,
    VirNetStream = 3i32,
    VirNetCallWithFds = 4i32,
    VirNetReplyWithFds = 5i32,
    VirNetStreamHole = 6i32,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[repr(i32)]
pub enum Virnetmessagestatus {
    VirNetOk = 0i32,
    VirNetError = 1i32,
    VirNetContinue = 2i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Virnetmessageheader {
    pub prog: u32,
    pub vers: u32,
    pub proc: i32,
    pub r#type: Virnetmessagetype,
    pub serial: u32,
    pub status: Virnetmessagestatus,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Virnetmessagenonnulldomain {
    pub name: String,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
    pub id: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Virnetmessagenonnullnetwork {
    pub name: String,
    #[serde(with = "serde_xdr::opaque::fixed")]
    pub uuid: [u8; VIR_UUID_BUFLEN as usize],
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Virnetmessageerror {
    pub code: i32,
    pub domain: i32,
    pub message: Option<String>,
    pub level: i32,
    pub dom: Option<Virnetmessagenonnulldomain>,
    pub str1: Option<String>,
    pub str2: Option<String>,
    pub str3: Option<String>,
    pub int1: i32,
    pub int2: i32,
    pub net: Option<Virnetmessagenonnullnetwork>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Virnetstreamhole {
    pub length: i64,
    pub flags: u32,
}
