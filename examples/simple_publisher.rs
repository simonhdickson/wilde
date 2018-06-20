extern crate capnp;
extern crate capnp_rpc;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use wilde_capnp::{publisher, message};
use simple_capnp::simple_message;

use capnp::message::Builder;

use tokio_core::reactor;
use tokio_io::AsyncRead;

use futures::Future;

pub mod wilde_capnp {
    include!(concat!(env!("OUT_DIR"), "/wilde_capnp.rs"));
}

pub mod simple_capnp {
    include!(concat!(env!("OUT_DIR"), "/simple_capnp.rs"));
}

pub fn main() {
    use std::net::ToSocketAddrs;
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} client HOST:PORT", args[0]);
        return;
    }

    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();

    let addr = args[2].to_socket_addrs().unwrap().next().expect("could not parse address");
    let stream = core.run(::tokio_core::net::TcpStream::connect(&addr, &handle)).unwrap();
    stream.set_nodelay(true).unwrap();
    let (reader, writer) = stream.split();

    let rpc_network = Box::new(twoparty::VatNetwork::new(reader, writer, rpc_twoparty_capnp::Side::Client, Default::default()));

    let mut rpc_system = RpcSystem::new(rpc_network, None);
    let con: publisher::Client<simple_message::Owned> = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

    let mut request = con.create_request();
    request.get().set_topic("hello-world");

    let publisher_func = request.send().pipeline.get_publish_func();
    
    loop {
        let mut builder = Builder::new_default();
        let mut message_builder = builder.init_root::<message::Builder<simple_message::Owned>>();
        message_builder.set_id("");
        message_builder.reborrow().init_data().set_text("");

        let mut request = publisher_func.call_request();
        request.get().set_message(message_builder.as_reader()).unwrap();

        let result = request.send().promise.wait();

        assert!(result.is_ok(), "failed to publish");

        println!("published event {}", "");
    }
}
