//! Exposes the client-cli as an executable

mod cli;
pub(crate) mod client;

// Expose cli as the main executable
pub fn main() {
    cli::run_cli()
}
