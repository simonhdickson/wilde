use std::fmt::Debug;
use capnp::capability::{Params, Promise};
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};

use wilde_capnp::consumer::{Server, SubscribeParams, SubscribeResults};

struct WildeConsumer;

impl<T> Server<T> for WildeConsumer where T: for<'c> ::capnp::traits::Owned<'c>  {
    fn subscribe(&mut self, _: SubscribeParams<T>, _: SubscribeResults<T>) -> Promise<(), ::capnp::Error> {
        Promise::ok(())
    }
}
