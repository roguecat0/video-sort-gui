use iced::{widget::text, Element, Subscription};
use iced_gif::gif;
use std::path::Path;
use std::time::{Duration, Instant};
use video_sort_gui::widget::Player;

struct App {
    player: Player,
    last_tick: Instant,
}
impl Default for App {
    fn default() -> Self {
        let last_tick = Instant::now();
        let player = Player::from_path(Path::new(
            r#"C:\Users\tevon\Programming\rust\video-sort-gui\media\demo.gif"#,
        ))
        .unwrap();

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
                    println!("video is over")
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
            _ => text("not implemented").into(),
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        let subscriptions = vec![iced::time::every(Duration::from_millis(1000)).map(Message::Tick)];
        iced::Subscription::batch(subscriptions)
    }
}

#[derive(Clone, Debug)]
enum Message {
    Tick(Instant),
}

fn main() -> iced::Result {
    iced::application("video player", App::update, App::view)
        .subscription(App::subscription)
        .run()
}
