use nom::{
    be_u8, be_u16,
};

// The Protocol
// Data starts with a label that describes the data following it. Values are all big endian, and floats are IEEE 754. Everything is packed to 4 bytes where applicable, padded with zeroes so it's 32-bit aligned.

// Labels - human readable types of proceeding data
// Type - single ascii character describing data
// Size - how big is the data type
// Count - how many values are we going to get
// Length = size * count
struct Label {
    kind: [u8; 4],
    ty: u8,
    size: u32,
}

// 00000000: 4445 5643 0001 10ac 4456 4944 4c04 0001  DEVC....DVIDL...
// 00000010: 0000 0001 4456 4e4d 6301 0006 4361 6d65  ....DVNMc...Came
// 00000020: 7261 0000 5449 434b 4c04 0001 0000 103a  ra..TICKL......:
// 00000030: 5354 524d 0001 0530 5453 4d50 4c04 0001  STRM...0TSMPL...
// 00000040: 0000 00c8 5449 434b 4c04 0001 0000 103a  ....TICKL......:

#[derive(Debug)]
pub struct Record<'a> {
    kind: String,
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
           label: dbg_dmp!(take!(4))  >>
           size_hint: be_u8 >>
           size: be_u8      >>
           num: be_u16      >>
           data: take!(calculate_data_length(size_hint, size, num)) >>
           take!(padding(calculate_data_length(size_hint, size, num))) >>
           (Record {
               kind: String::from_utf8(label.to_vec()).unwrap(),
               size_hint,
               size,
               num,
               data,
           })
       )
);

named!(records<&[u8], Vec<Record> >,
       many1!(record));

pub fn parse(data: &[u8]) -> Vec<Record> {
    records(data).unwrap().1
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
