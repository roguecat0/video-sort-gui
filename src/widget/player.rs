use anyhow::Result;
use iced_gif::gif;
use iced_video_player::{Video, VideoPlayer};
use iced_webp::webp;
use std::time::Duration;

pub enum Player {
    Vid { video: Video, position: f64 },
    Gif { frames: Option<gif::Frames> },
    Webp { frames: Option<webp::Frames> },
}

//pub trait ImageVid {
//    fn from_bytes_with_length(bytes: Vec<u8>) -> Result<(Self, Duration)>;
//}
//
//impl ImageVid for gif::Frames {
//    fn from_bytes_with_length(bytes: Vec<u8>) -> Result<(Self, Duration)> {
//        let decoder = Decoder::new(io::Cursor::new(bytes))?;
//
//        let total_bytes = decoder.total_bytes();
//        let mut duration = Duration::default();
//
//        let frames = decoder
//            .into_frames()
//            .into_iter()
//            .map(|result| {
//                result
//                    .inspect(|frame| duration += frame.delay())
//                    .map(|frame| frame.into())
//            })
//            .collect::<Result<Vec<_>, _>>()?;
//
//        let first = frames.first().cloned().unwrap();
//
//        Ok((
//            Frames {
//                total_bytes,
//                first,
//                frames,
//            },
//            duration,
//        ))
//    }
//}
