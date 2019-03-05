use clap::{Arg, App};
use gopro_telemetry;

use failure::Error;

use std::fs::File;

fn main() -> Result<(), Error> {
    if ::std::env::var_os("RUST_LOG").is_none() {
        ::std::env::set_var("RUST_LOG", "INFO");
    }
    pretty_env_logger::init();

    let matches = App::new("MyApp")
         .arg(Arg::with_name("input")
              .help("the input file to use")
              .index(1)
              .required(true))
         .get_matches();

    let mut file = File::open(matches.value_of("input").unwrap())?;
    let ctx = gopro_telemetry::test_read(&mut file)?;
    let gps_data = ctx.gps_metadata();

    Ok(())
}
