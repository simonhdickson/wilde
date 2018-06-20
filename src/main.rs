extern crate capnp;
#[macro_use] extern crate capnp_rpc;
extern crate clap;

use clap::{Arg, App};

pub mod consumer;
pub mod publisher;

pub mod wilde_capnp {
    include!(concat!(env!("OUT_DIR"), "/wilde_capnp.rs"));
}

fn main () {
    let matches =
        App::new("Wilde")
            .version("0.1.0")
            .author("Simon Dickson <simon@simonhdickson.com>")
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file"))
            .get_matches();
}
