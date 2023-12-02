use bucketinator_rs::{app::App, cli, config::BucketinatorConfiguration};

fn main() {
    // First load the config.

    let conf = match BucketinatorConfiguration::get_config() {
        Ok(conf) => conf,
        Err(e) => panic!(
            "Could not load config file, cannot continue execution: {}",
            e
        ),
    };

    // Then create the app but DO NOT YET initialise it.
    // Reason being that the cli can exit prematurely (invalid args, printing help text, version, etc).
    let mut app = App::new(conf);

    // Main entrypoint.
    cli::enter_cli(&mut app);
}
