extern crate capnpc;

use std::env;

fn main() {
    env::set_var("OUT_DIR", "src/generated");

    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/wilde.capnp")
        .run()
        .expect("schema compiler command");
}
