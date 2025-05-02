#!/bin/bash

set -eu

VERSION="v11.3.0"
TOOL_VERSION="0.4.0"
PROTO_URL="https://raw.githubusercontent.com/libvirt/libvirt/${VERSION}/src/rpc/virnetprotocol.x"
REMOTE_URL="https://raw.githubusercontent.com/libvirt/libvirt/${VERSION}/src/remote/remote_protocol.x"
RPCGEN_BIN="https://github.com/9506hqwy/xdr-rs/releases/download/${TOOL_VERSION}/rpcgen-${TOOL_VERSION}-x86_64-unknown-linux-gnu.tar.gz"

WORKDIR=`mktemp -d`
trap 'rm -rf ${WORKDIR}' EXIT

# Download rpcgen binary.
curl -sSL -o ${WORKDIR}/rpcgen.tar.gz ${RPCGEN_BIN}
tar -C ${WORKDIR} -zxf ${WORKDIR}/rpcgen.tar.gz
chmod 755 ${WORKDIR}/rpcgen

# Generate binding.
curl -sSL -o ${WORKDIR}/virnetprotocol.x ${PROTO_URL}
cat - << EOF >> ${WORKDIR}/virnetprotocol.x
const VIR_UUID_BUFLEN = 16;
EOF
${WORKDIR}/rpcgen ${WORKDIR}/virnetprotocol.x > libvirt-remote/src/protocol.rs
rustfmt libvirt-remote/src/protocol.rs

curl -sSL -o ${WORKDIR}/remote_protocol.x ${REMOTE_URL}
cat - << EOF >> ${WORKDIR}/remote_protocol.x
const VIR_SECURITY_MODEL_BUFLEN = 256;
const VIR_SECURITY_LABEL_BUFLEN = 4096;
const VIR_SECURITY_DOI_BUFLEN = 256;
const VIR_UUID_BUFLEN = 16;
const VIR_TYPED_PARAM_INT = 1;
const VIR_TYPED_PARAM_UINT = 2;
const VIR_TYPED_PARAM_LLONG = 3;
const VIR_TYPED_PARAM_ULLONG = 4;
const VIR_TYPED_PARAM_DOUBLE = 5;
const VIR_TYPED_PARAM_BOOLEAN = 6;
const VIR_TYPED_PARAM_STRING = 7;
EOF
${WORKDIR}/rpcgen ${WORKDIR}/remote_protocol.x > libvirt-remote/src/binding.rs
rustfmt libvirt-remote/src/binding.rs

# Generate client.
cargo run --example libvirt-remote-gen libvirt-remote/src/binding.rs > libvirt-remote/src/client.rs
rustfmt libvirt-remote/src/client.rs
