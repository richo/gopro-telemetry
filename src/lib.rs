#[macro_use]
extern crate log;

#[macro_use]
extern crate nom;

use std::io::Read;

use mp4parse::{self, MediaContext};
use failure::{format_err, Error};

pub mod gps_parser;

// TODO(richo) rename this once we figure out what it even is
pub struct VideoData {
    ctx: MediaContext,
}

impl VideoData {
    pub fn gps_metadata(&self) {
        for track in &self.ctx.tracks {
            if track.id != 3 {
                continue
            }
            info!("{:?}", &track);
        }
    }
}


pub fn test_read<T: Read>(mut reader: &mut T) -> Result<VideoData, Error> {
    let mut ctx = MediaContext::new();
    mp4parse::read_mp4(&mut reader, &mut ctx)
        .map_err(|err| format_err!("Error reading mp4: {:?}", err))?;
    Ok(VideoData { ctx })
}
