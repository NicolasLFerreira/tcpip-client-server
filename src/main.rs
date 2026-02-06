mod client;
mod server;

use crate::client::client;
use crate::server::server;
use std::env;

enum InstanceType {
    Server,
    Client,
}

fn main() {
    let mut args = env::args();
    let _program = args.next();

    let instance: InstanceType;
    if let Some(a) = args.next()
        && a.as_str().eq("client")
    {
        instance = InstanceType::Client;
    } else {
        instance = InstanceType::Server;
    }

    match instance {
        InstanceType::Server => server(),
        InstanceType::Client => client(),
    }
}
