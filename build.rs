extern crate capnpc;

use std::env;

fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/wilde.capnp")
        .run()
        .expect("schema compiler command");
    
    capnpc::CompilerCommand::new()
        .src_prefix("examples/schema")
        .file("examples/schema/simple.capnp")
        .run()
        .expect("schema compiler command");
}
