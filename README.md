# Libvirt remote for Rust

This crate provides a pure Rust interface for interacting with libvirt.

This library uses [libvirt RPC infrastructure](https://libvirt.org/kbase/internals/rpc.html)
to communicate libvirt server.
The packet encoding and decoding and stub code generation uses [xdr-rs](https://github.com/9506hqwy/xdr-rs) crate.

## Examples

see [virsh](./virsh) directory.

## References

- [Reference Manual for libvirt](https://libvirt.org/html/index.html)
