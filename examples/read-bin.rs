use clap::{Arg, App};
use gopro_telemetry;

use failure::Error;

use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Error> {
    if ::std::env::var_os("RUST_LOG").is_none() {
        ::std::env::set_var("RUST_LOG", "INFO");
    }
    pretty_env_logger::init();

    let matches = App::new("read-bin")
         .arg(Arg::with_name("input")
              .help("the input file to use")
              .index(1)
              .required(true))
         .get_matches();

    let mut file = File::open(matches.value_of("input").unwrap())?;
    let mut content = vec![];
    file.read_to_end(&mut content)?;
    let record = gopro_telemetry::gps_parser::parse(&content);

    println!("{:?}", &record);

    Ok(())
}
