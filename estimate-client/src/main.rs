//! Exposes the client-cli as an executable

mod cli;
pub (crate) mod client;

use cli::run_cli;

// Expose cli as the main executable
pub fn main() {
    run_cli();
}
