use gopro_telemetry::{self, gps_parser::Message};

use std::io::{self, Read};
use std::fs::File;

#[test]
fn read_dvid() -> Result<(), io::Error> {
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

#[test]
fn read_devc() -> Result<(), io::Error> {
    let mut file = File::open("testcases/devc.msg")?;
    let mut content = vec![];
    file.read_to_end(&mut content)?;
    let msg = gopro_telemetry::gps_parser::parse(&content).expect("Couldn't parse data");
    assert_eq!(msg.len(), 1);

    match &msg[0] {
        Message::DEVC { children } => {
            assert_eq!(children, &vec![
                       Message::DVID { data: vec![1] },
                       Message::DVNM { data: vec![67, 97, 109, 101, 114, 97] },
                       Message::TICK { data: vec![4154] }
            ]);
        },
        other => panic!("Unexpected message: {:?}", other),
    }

    Ok(())
}
