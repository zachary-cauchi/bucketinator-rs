use bucketinator_rs::{app::App, cli, config::BucketinatorConfiguration};

fn main() {
    let conf = match BucketinatorConfiguration::get_config() {
        Ok(conf) => conf,
        Err(e) => panic!(
            "Could not load config file, cannot continue execution: {}",
            e
        ),
    };

    let mut app = App::new(conf);

    cli::enter_cli(&mut app);
}
