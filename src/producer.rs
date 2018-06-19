use std::fmt::Debug;
use capnp::capability::{Params, Promise};
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use wilde_capnp::publisher;

struct WildePublisher;

impl<T> publisher::Server<T> for WildePublisher where T: for<'c> ::capnp::traits::Owned<'c>  {
}
