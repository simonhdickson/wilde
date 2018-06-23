#[macro_use] extern crate capnp_rpc;
extern crate capnp;
extern crate clap;
extern crate failure;

use clap::{Arg, App};

pub mod capnproto;
pub mod queue;

pub mod wilde_capnp {
    include!(concat!(env!("OUT_DIR"), "/wilde_capnp.rs"));
}

fn main () {
    
}
