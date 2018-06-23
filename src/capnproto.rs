use std::fmt::Debug;
use capnp::capability::{Params, Promise};
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};

use wilde_capnp::{publisher, publisher_function};
use wilde_capnp::consumer;

struct WildePublish;

impl<T> publisher_function::Server<T> for WildePublish where T: for<'c> ::capnp::traits::Owned<'c>   {
    fn call(&mut self, params: publisher_function::CallParams<T>, mut results: publisher_function::CallResults<T>) -> Promise<(), ::capnp::Error> {

        let params = pry!(params.get());
        let message = pry!(params.get_message());
        let data = pry!(message.get_data());

        //println!("{}", pry!(data.get_text()));

        //results.get().set_processed(true);
        Promise::ok(())
    }
}

struct WildePublisher;

impl<T: 'static> publisher::Server<T> for WildePublisher where T: for<'c> ::capnp::traits::Owned<'c>  {
    fn create(&mut self, params: publisher::CreateParams<T>, mut results: publisher::CreateResults<T>) -> Promise<(), ::capnp::Error> {

        results.get().set_publish_func(publisher_function::ToClient::new(WildePublish).from_server::<::capnp_rpc::Server>());

        Promise::ok(())
    }
}

struct WildeConsumer;

impl<T> consumer::Server<T> for WildeConsumer where T: for<'c> ::capnp::traits::Owned<'c>  {
    fn subscribe(&mut self, params: consumer::SubscribeParams<T>, results: consumer::SubscribeResults<T>) -> Promise<(), ::capnp::Error> {

        Promise::ok(())
    }
}
