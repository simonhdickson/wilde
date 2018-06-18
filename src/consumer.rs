use std::fmt::Debug;
use capnp::capability::{Params, Promise};
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use wilde_capnp::consumer;

struct WildeConsumer;

impl<T> consumer::Server<T> for WildeConsumer where T: for<'c> ::capnp::traits::Owned<'c>  {
    
}
