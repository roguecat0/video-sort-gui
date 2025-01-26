use anyhow::{anyhow, Error as AnyError};
use iced::Task;
use iced_gif::gif;
use iced_video_player::{Video, VideoPlayer};
use iced_webp::webp;
use image_rs::codecs::gif::GifDecoder;
use image_rs::codecs::webp::WebPDecoder;
use image_rs::AnimationDecoder;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub enum Player {
    Idle,
    Video {
        video: Video,
        finished: bool,
    },
    Gif {
        frames: gif::Frames,
        duration: Duration,
        position: f64,
    },
    Webp {
        frames: webp::Frames,
        duration: Duration,
        position: f64,
    },
}
impl Clone for Player {
    fn clone(&self) -> Self {
        Self::Idle
    }
}
fn try_read_bytes(path: &Path) -> Result<Vec<u8>, AnyError> {
    Ok(std::fs::read(path)?)
}
impl Player {
    pub fn tick(&mut self, elapsed: Duration) -> Option<Update> {
        match self {
            Player::Video { finished, .. } => {
                if *finished {
                    *finished = false;
                    Some(Update::EndOfStream)
                } else {
                    None
                }
            }
            Player::Gif {
                duration, position, ..
            } => {
                *position += elapsed.as_secs_f64();
                if *position >= duration.as_secs_f64() {
                    *position = 0_f64;
                    Some(Update::EndOfStream)
                } else {
                    None
                }
            }
            Player::Webp {
                duration, position, ..
            } => {
                *position += elapsed.as_secs_f64();
                if *position >= duration.as_secs_f64() {
                    *position = 0_f64;
                    Some(Update::EndOfStream)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    pub fn from_path_async_naive(path: &Path) -> Task<Option<Self>> {
        let path = path.to_path_buf();
        let f = async move { Self::from_path(&path).ok() };
        Task::perform(f, std::convert::identity)
    }

    pub fn from_path(path: &Path) -> Result<Self, AnyError> {
        if let Some(extention) = path.to_path_buf().extension() {
            println!("extention: {extention:?}");
            match extention.to_str() {
                Some("gif") => {
                    let bytes = try_read_bytes(path)?;
                    let frames = gif::Frames::from_bytes(bytes.clone())?;
                    let duration = gif::Frames::from_bytes_with_length(bytes)?;
                    let position = 0_f64;
                    Ok(Self::Gif {
                        frames,
                        duration,
                        position,
                    })
                }
                Some("webp") => {
                    let bytes = try_read_bytes(path)?;
                    let frames = webp::Frames::from_bytes(bytes.clone())?;
                    let duration = webp::Frames::from_bytes_with_length(bytes)?;
                    println!("duration: {duration:?}");
                    let position = 0_f64;
                    Ok(Self::Webp {
                        frames,
                        duration,
                        position,
                    })
                }
                Some("mp4") | Some("webm") => {
                    let path = if path.is_absolute() {
                        path.to_path_buf()
                    } else {
                        let path = path.canonicalize()?;
                        println!("making relative: {path:?}");
                        path
                    };

                    let mut video = Video::new(
                        &url::Url::from_file_path(&path)
                            .map_err(|_| anyhow!("failed to parse path: {path:?}"))?,
                    )?;
                    video.set_looping(true);
                    Ok(Self::Video {
                        video,
                        finished: false,
                    })
                }
                None => anyhow::bail!("failed to parse {extention:?}"),
                ext => anyhow::bail!("ext not supported, {ext:?}"),
            }
        } else {
            Err(anyhow!("didn't get to make extention"))
        }
    }
}
pub enum Update {
    EndOfStream,
}

pub trait ImageVid {
    fn from_bytes_with_length(bytes: Vec<u8>) -> Result<Duration, AnyError>;
}

impl ImageVid for gif::Frames {
    fn from_bytes_with_length(bytes: Vec<u8>) -> Result<Duration, AnyError> {
        let decoder = GifDecoder::new(std::io::Cursor::new(bytes.clone()))?;

        let duration = decoder
            .into_frames()
            .into_iter()
            .flatten()
            .inspect(|frame| println!("delay: {:?}", frame.delay()))
            .fold(Duration::default(), |acc, frame| acc + frame.delay().into());

        Ok(duration)
    }
}
impl ImageVid for webp::Frames {
    fn from_bytes_with_length(bytes: Vec<u8>) -> Result<Duration, AnyError> {
        let decoder = WebPDecoder::new(std::io::Cursor::new(bytes.clone()))?;

        let duration = decoder
            .into_frames()
            .into_iter()
            .flatten()
            .fold(Duration::default(), |acc, frame| acc + frame.delay().into());

        Ok(duration)
    }
}
