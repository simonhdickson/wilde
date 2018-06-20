use std::fmt::Debug;
use capnp::capability::{Params, Promise};
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};

use wilde_capnp::publisher::{Server, CreateParams, CreateResults};

struct WildePublisher;

impl<T> Server<T> for WildePublisher where T: for<'c> ::capnp::traits::Owned<'c>  {
    fn create(&mut self, _: CreateParams<T>, _: CreateResults<T>) -> Promise<(), ::capnp::Error> {
        Promise::ok(())
    }
}
