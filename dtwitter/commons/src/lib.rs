pub mod errors;
pub mod p2p;
pub mod peer_commands;
pub mod prelude;

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    pretty_env_logger::init();
}
