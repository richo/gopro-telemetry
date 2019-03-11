use gopro_telemetry::{self, gps_parser::Message};

use std::io::{self, Read};
use std::fs::File;

#[test]
fn it_works() -> Result<(), io::Error> {
    let mut file = File::open("testcases/dvid.msg")?;
    let mut content = vec![];
    file.read_to_end(&mut content)?;
    let msg = gopro_telemetry::gps_parser::parse(&content).expect("Couldn't parse data");
    assert_eq!(msg.len(), 1);

    match &msg[0] {
        Message::DVID { data } => {
            assert_eq!(data, &vec![1u32]);
        },
        other => panic!("Unexpected message: {:?}", other),
    }

    Ok(())
}
