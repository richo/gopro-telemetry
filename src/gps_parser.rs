use nom::{
    self,
    be_u8, be_u16, be_u32,
    be_i8, be_i16, be_i32,
    be_f32, be_f64,
};

macro_rules! parser {
    // TODO(richo) Can we do the lookup for the size_hint automatically?
    ($name:ident(L => $ty:ty)) => { inner_parser!($name, be_u32, "L", $ty); };
    ($name:ident(l => $ty:ty)) => { inner_parser!($name, be_i32, "l", $ty); };
    ($name:ident(S => $ty:ty)) => { inner_parser!($name, be_u16, "S", $ty); };
    ($name:ident(s => $ty:ty)) => { inner_parser!($name, be_i16, "s", $ty); };
    // TODO(richo) this doesn't really work gracefully, since this should be turned into an actual
    // String type probably?
    ($name:ident(c => $ty:ty)) => { inner_parser!($name, be_u8, "c", $ty); };
    // TODO(richo) figure out a callback syntax to deal with parsing dates out?
    // ($name:ident(c => $ty:ty)) => { inner_parser!($name, be_u8, "c", $ty); };
    ($name:ident(f => $ty:ty)) => { inner_parser!($name, be_f32, "f", $ty); };
}

macro_rules! inner_parser {
    ($name:ident, $parser:ident, $hint:expr, $ty:ty) => {
        named!($name<&[u8], Message>,
               do_parse!(
                   _name: tag!(stringify!($name)) >>
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
named!(DEVC<&[u8], Message>,
       do_parse!(
           tag!("DEVC")  >>
           size_hint: tag!("\x00") >>
           size: be_u8      >>
           num: be_u16      >>
           body: take!(size as u16 * num) >>
           take!(padding(size_hint[0], size, num)) >>
           (Message::DEVC {
               children: outer_parser(body)?.1
           })
       )
);
// parser!(DEVC(\u{0}', size:
// parser!(STRM(\u{0}', size:

parser!(ACCL(s => ACCL));
parser!(DVID(L => DVID));
parser!(DVNM(c => DVNM));
parser!(GPS5(l => GPS5));
parser!(GPSF(L => GPSF));
parser!(GPSP(S => GPSP));
// parser!(GPSU(U => GPSU));
parser!(GYRO(s => GYRO));
parser!(ISOG(f => ISOG));
parser!(SCALl(l => SCALl));
parser!(SCALs(s => SCALs));
parser!(SHUT(f => SHUT));
parser!(SIUN(c => SIUN));
parser!(STNM(c => STNM));
parser!(TICK(L => TICK));
parser!(TMPC(f => TMPC));
parser!(TSMP(L => TSMP));
parser!(UNIT(c => UNIT));


named!(outer_parser<&[u8], Vec<Message> >,
       many1!(complete!(
       switch!(peek!(take!(4)),
               b"DEVC" => call!(DEVC) |
               b"ACCL" => call!(ACCL) |
               b"ACCL" => call!(ACCL) |
               b"DVID" => call!(DVID) |
               b"DVNM" => call!(DVNM) |
               b"GPS5" => call!(GPS5) |
               b"GPSF" => call!(GPSF) |
               b"GPSP" => call!(GPSP) |
               b"GYRO" => call!(GYRO) |
               b"ISOG" => call!(ISOG) |
               b"SCAL" => alt!(SCALl | SCALs) |
               b"SHUT" => call!(SHUT) |
               b"SIUN" => call!(SIUN) |
               b"STNM" => call!(STNM) |
               b"TICK" => call!(TICK) |
               b"TMPC" => call!(TMPC) |
               b"TSMP" => call!(TSMP) |
               b"UNIT" => call!(UNIT)
       )))
);


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
    match outer_parser(data) {
        Ok((left, data)) => {
            assert!(left.len() == 0);
            Ok(data)
        },
        Err(e) => Err(e),
    }
}

pub fn parse_dvid(data: &[u8]) -> Result<Message, nom::Err<&[u8]>> {
    match DVID(data) {
        Ok((left, data)) => {
            assert!(left.len() == 0);
            Ok(data)
        },
        Err(e) => Err(e),
    }
}

#[derive(Debug)]
pub enum Message {
    ACCL { data: Vec<i16> },
    DEVC { children: Vec<Message> },
    DVID { data: Vec<u32> },
    DVNM { data: Vec<u8> },
    EMPT,
    GPS5 { data: Vec<i32> },
    GPSF { data: Vec<u32>},
    GPSP { data: Vec<u16> },
    GPSU,
    GYRO { data: Vec<i16> },
    SCALl { data: Vec<i32> },
    SCALs { data: Vec<i16> },
    SIUN { data: Vec<u8> },
    STRM,
    TMPC { data: Vec<f32> },
    TSMP { data: Vec<u32> },
    UNIT { data: Vec<u8> },
    TICK { data: Vec<u32>},
    STNM { data: Vec<u8> },
    ISOG { data: Vec<f32> },
    SHUT { data: Vec<f32> },
}
