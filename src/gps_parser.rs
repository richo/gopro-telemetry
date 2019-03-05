use nom::{
    self,
    be_u8, be_u16,
};
#[derive(Debug)]
pub struct Record<'a> {
    kind: &'a [u8],
    size_hint: u8,
    size: u8,
    num: u16,
    data: &'a [u8],
}

fn calculate_data_length(size_hint: u8, size: u8, num: u16) -> usize {
    debug!("size_hint: {:x}({}), size: {:x}, num: {:x}", &size_hint, size_hint as char, &size, &num);
    if size_hint == 0x0 {
        // This describes a nested property, but should still be pulled out
        return 0;
    }

    debug!("Taking {}", (size as u16 * num) as usize);
    (size as u16 * num) as usize
}

fn padding(len: usize) -> usize {
    let ret = {
        let padding = len % 4;
        if padding == 0 {
            0
        } else {
            4 - padding
        }
    };
    debug!("Padding {}", &ret);
    ret
}

named!(record<&[u8], Record>,
       do_parse!(
           // TODO(richo) Do we want to reject invalid messages here?
           label: take!(4)  >>
           size_hint: be_u8 >>
           size: be_u8      >>
           num: be_u16      >>
           data: take!(calculate_data_length(size_hint, size, num)) >>
           take!(padding(calculate_data_length(size_hint, size, num))) >>
           (Record {
               kind: label,
               size_hint,
               size,
               num,
               data,
           })
       )
);

named!(records<&[u8], Vec<Record> >,
       many1!(complete!(record)));

pub fn parse(data: &[u8]) -> Result<Vec<Message>, nom::Err<&[u8]>> {
    match records(data) {
        Ok((left, data)) => {
            assert!(left.len() == 0);
            Ok(data
               .into_iter()
               .map(|x| x.parse())
               .collect()
            )
        },
        Err(e) => Err(e),
    }
}

// #[allow(non_camel_case_names)]
#[derive(Debug)]
pub enum Message {
    ACCL,
    DEVC,
    DVID,
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

impl<'a> Record<'a> {
    fn parse(self) -> Message {
        match &self.kind[..] {
            b"ACCL" => {
                Message::ACCL
            },
            b"DEVC" => {
                Message::DEVC
            },
            b"DVID" => {
                Message::DVID
            },
            b"DVNM" => {
                Message::DVNM
            },
            b"EMPT" => {
                Message::EMPT
            },
            b"GPS5" => {
                Message::GPS5
            },
            b"GPSF" => {
                Message::GPSF
            },
            b"GPSP" => {
                Message::GPSP
            },
            b"GPSU" => {
                Message::GPSU
            },
            b"GYRO" => {
                Message::GYRO
            },
            b"SCAL" => {
                Message::SCAL
            },
            b"SIUN" => {
                Message::SIUN
            },
            b"STRM" => {
                Message::STRM
            },
            b"TMPC" => {
                Message::TMPC
            },
            b"TSMP" => {
                Message::TSMP
            },
            b"UNIT" => {
                Message::UNIT
            },
            b"TICK" => {
                Message::TICK
            },
            b"STNM" => {
                Message::STNM
            },
            b"ISOG" => {
                Message::ISOG
            },
            b"SHUT" => {
                Message::SHUT
            },
            other => {
                panic!("unknown block: {}", String::from_utf8(other.to_vec()).unwrap());
            }
        }
    }
}
// ACCL - accelerometer reading x/y/z
// DEVC - device
// DVID - device ID, possibly hard-coded to 0x1
// DVNM - devicde name, string "Camera"
// EMPT - empty packet
// GPS5 - GPS data (lat, lon, alt, speed, 3d speed)
// GPSF - GPS fix (none, 2d, 3d)
// GPSP - GPS positional accuracy in cm
// GPSU - GPS acquired timestamp; potentially different than "camera time"
// GYRO - gryroscope reading x/y/z
// SCAL - scale factor, a multiplier for subsequent data
// SIUN - SI units; strings (m/s², rad/s)
// STRM - ¯\_(ツ)_/¯
// TMPC - temperature
// TSMP - total number of samples
// UNIT - alternative units; strings (deg, m, m/s)
