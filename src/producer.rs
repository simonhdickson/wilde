use std::fmt::Debug;
use capnp::capability::{Params, Promise};
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use wilde_capnp::producer;

struct WildeProducer;

impl<T> producer::Server<T> for WildeProducer where T: for<'c> ::capnp::traits::Owned<'c>  {
    fn publish(&mut self, params: producer::PublishParams<T>, _: producer::PublishResults<T>) -> Promise<(), ::capnp::Error> {
        let greeting: &str = "Hello ";
        let data = pry!(pry!(params.get()).get_data());
        let response = format!("{}!", greeting);
        Promise::ok(())
     }
}
