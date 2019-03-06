use std::fmt;
use nom::{
    self,
    be_u8, be_u16, be_u32,
};

macro_rules! parser {
    // TODO(richo) Can we do the lookup for the size_hint automatically?
    ($name:ident(L => $ty:ty)) => {
        inner_parser!($name, be_u32, "L", $ty);
    }
}

macro_rules! inner_parser {
    ($name:ident, $parser:ident, $hint:expr, $ty:ty) => {
        named!($name<&[u8], Message>,
               do_parse!(
                   tag!(stringify!($name))  >>
                   size_hint: tag!($hint) >>
                   size: be_u8      >>
                   num: be_u16      >>
                   data: count!($parser, num_values(size_hint[0], num)) >>
                   take!(padding(size_hint[0], size, num)) >>
                   (Message::$name {
                       data,
                   })
               )
        );
    }
}

fn num_values(size_hint: u8, num: u16) -> usize {
    if size_hint == 0 {
        0
    } else {
        num as usize
    }
}

fn padding(size_hint: u8, size: u8, num: u16) -> usize {
    if size_hint == 0x0 {
        // This describes a nested property, but should still be pulled out
        return 0;
    }

    let data_size = (size as u16 * num) as usize;
    let padding = data_size % 4;
    if padding == 0 {
        0
    } else {
        4 - padding
    }
}

parser!(DVID(L => DVID));

// pub fn parse(data: &[u8]) -> Result<Vec<Message>, nom::Err<&[u8]>> {
//     match records(data) {
//         Ok((left, data)) => {
//             assert!(left.len() == 0);
//             Ok(data
//                .into_iter()
//                .map(|x|{ debug!("{:?}", &x); x.parse()})
//                .collect()
//             )
//         },
//         Err(e) => Err(e),
//     }
// }

pub fn parse(data: &[u8]) -> Result<Vec<Message>, nom::Err<&[u8]>> {
    match DVID(data) {
        Ok((left, data)) => {
            assert!(left.len() == 0);
            Ok(vec![data])
        },
        Err(e) => Err(e),
    }
}

#[derive(Debug)]
pub enum Message {
    ACCL,
    DEVC,
    DVID { data: Vec<u32> },
    DVNM,
    EMPT,
    GPS5,
    GPSF,
    GPSP,
    GPSU,
    GYRO,
    SCAL,
    SIUN,
    STRM,
    TMPC,
    TSMP,
    UNIT,
    TICK,
    STNM,
    ISOG,
    SHUT,
}
