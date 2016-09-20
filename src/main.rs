extern crate getopts;
mod application;
mod server;
use server::GameServer;

fn main() {

    let args = match application::parse_command_line(){
        Some(data) => data,
        None => return,
    };

    let server = GameServer::new(&args["hostname"], &args["port"]);

    server.start();
}