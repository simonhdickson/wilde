extern crate capnp;
#[macro_use] extern crate capnp_rpc;

pub mod consumer;
pub mod producer;

pub mod wilde_capnp {
    include!(concat!("generated", "/wilde_capnp.rs"));
}