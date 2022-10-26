//! Interface for users to interact with this application
//! Each command will query the server (via the client), and return the result

use crate::client;

pub fn run_cli() {
    println!("Running cli!");
    client::get_server_status();
}
