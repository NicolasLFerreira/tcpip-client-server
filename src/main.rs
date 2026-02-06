mod client;
mod protocol;
mod server;

use crate::client::client;
use crate::server::server;
use std::env;

enum LaunchType {
    Server,
    Client,
}

fn main() {
    let mut args = env::args();
    let _program = args.next();

    let launch: LaunchType;
    if let Some(a) = args.next()
        && a.as_str().eq("client")
    {
        println!("Launching as client");
        launch = LaunchType::Client;
    } else {
        println!("Launching as server");
        launch = LaunchType::Server;
    }

    match launch {
        LaunchType::Server => server(),
        LaunchType::Client => client(),
    }
}
