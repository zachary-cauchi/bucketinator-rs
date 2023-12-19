use std::process::exit;

use log::{debug, error};
use rustodos_rs::{app::App, cli, config::RustodosConfiguration};

fn main() {
    env_logger::init();

    debug!("Loading config.");

    // First load the config.
    let conf = match RustodosConfiguration::get_config() {
        Ok(conf) => conf,
        Err(e) => {
            error!(
                "Could not load config file, cannot continue execution: {}",
                e
            );
            exit(1);
        }
    };

    debug!("Config loading. Creating app.");

    // Then create the app but DO NOT YET initialise it.
    // Reason being that the cli can exit prematurely (invalid args, printing help text, version, etc).
    let mut app = App::new(conf);

    debug!("App created. Loading entrypoint.");

    // Main entrypoint.
    cli::enter_cli(&mut app);
}
