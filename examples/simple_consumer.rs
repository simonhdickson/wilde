extern crate capnp;
#[macro_use] extern crate capnp_rpc;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use wilde_capnp::{consumer, consumer_function};

use capnp::capability::Promise;

use tokio_core::reactor;
use tokio_io::AsyncRead;

use simple_capnp::simple_message;
use futures::Future;

pub mod wilde_capnp {
    include!(concat!(env!("OUT_DIR"), "/wilde_capnp.rs"));
}

pub mod simple_capnp {
    include!(concat!(env!("OUT_DIR"), "/simple_capnp.rs"));
}

struct PrintFunction;

impl wilde_capnp::consumer_function::Server<simple_message::Owned> for PrintFunction  {
    fn call(&mut self,
            params: consumer_function::CallParams<simple_message::Owned>,
            mut results: consumer_function::CallResults<simple_message::Owned>)
        -> Promise<(), ::capnp::Error>
    {
        let params = pry!(params.get());
        let message = pry!(params.get_message());
        let data = pry!(message.get_data());
        println!("{}", pry!(data.get_text()));
        results.get().set_processed(true);
        Promise::ok(())
    }
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
    let con: consumer::Client<simple_message::Owned> = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

    let mut request = con.subscribe_request();
    request.get().set_topic("hello-world");
    request.get().set_consumer_group("simple-consumer");
    request.get().set_consumer("simple-consumer-1");
    request.get().set_consume_func(consumer_function::ToClient::new(PrintFunction).from_server::<::capnp_rpc::Server>());

    let _result = core.run(rpc_system.join(request.send().promise)).unwrap();
}
