use iced::{widget::text, Element, Subscription};
use iced_gif::gif;
use iced_video_player::VideoPlayer;
use iced_webp::webp;
use std::path::Path;
use std::time::{Duration, Instant};
use video_sort_gui::widget::Player;

struct App {
    player: Player,
    last_tick: Instant,
}
const FILE_GIF: &'static str = r#"C:\Users\tevon\Programming\rust\video-sort-gui\media\demo.gif"#;
const FILE_WEBP: &'static str = r#"C:\Users\tevon\Programming\rust\video-sort-gui\media\nya.webp"#;
const FILE_MP4: &'static str = r#"C:\Users\tevon\Programming\rust\video-sort-gui\media\test.mp4"#;
const REL_FILE_MP4: &'static str = r#"media\test.mp4"#;

impl Default for App {
    fn default() -> Self {
        let last_tick = Instant::now();
        let path = Path::new(REL_FILE_MP4);
        let path = &path.canonicalize().expect("path can't find root");
        println!("path: {path:?}");
        let player = Player::from_path(path).unwrap();
        Self { player, last_tick }
    }
}
impl App {
    pub fn update(&mut self, message: Message) {
        println!("message: {message:?}");
        match message {
            Message::Tick(instant) => {
                let elapsed = instant - self.last_tick;
                self.last_tick = instant;
                println!("elapsed: {elapsed:?}");
                if let Some(_u) = self.player.tick(elapsed) {
                    println!("video is over!!")
                }
            }
            Message::VideoEnd => {
                if let Player::Video { finished, .. } = &mut self.player {
                    *finished = true
                }
            }
        }
    }
    pub fn view(&self) -> Element<Message> {
        match &self.player {
            Player::Gif { frames, .. } => gif(&frames)
                .height(iced::Length::Fill)
                .width(iced::Length::Fill)
                .into(),
            Player::Webp { frames, .. } => webp(&frames)
                .height(iced::Length::Fill)
                .width(iced::Length::Fill)
                .into(),
            Player::Video { video, .. } => VideoPlayer::new(video)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .content_fit(iced::ContentFit::Contain)
                .on_end_of_stream(Message::VideoEnd)
                .into(),

            _ => text("not implemented").into(),
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        let subscriptions = vec![iced::time::every(Duration::from_millis(500)).map(Message::Tick)];
        iced::Subscription::batch(subscriptions)
    }
}

#[derive(Clone, Debug)]
enum Message {
    Tick(Instant),
    VideoEnd,
}

fn main() -> iced::Result {
    iced::application("video player", App::update, App::view)
        .subscription(App::subscription)
        .run()
}
