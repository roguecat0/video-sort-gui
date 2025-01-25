use std::path::PathBuf;

use iced::widget::{container, row};
use iced::{window, Element, Length, Size, Task};
use iced_webp::widget::webp;

fn main() {
    iced::application(App::title, App::update, App::view)
        .window(window::Settings {
            size: Size::new(498.0, 164.0),
            ..Default::default()
        })
        .run_with(App::new)
        .unwrap()
}

#[derive(Debug)]
enum Message {
    Loaded(Result<webp::Frames, webp::Error>),
}

#[derive(Default)]
struct App {
    frames: Option<webp::Frames>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let path = PathBuf::from("media/hello.webp");
        println!("path: {path:?}");

        (
            App::default(),
            webp::Frames::load_from_path(path).map(Message::Loaded),
        )
    }

    fn title(&self) -> String {
        "Iced webp".into()
    }

    fn update(&mut self, message: Message) {
        let Message::Loaded(frames) = message;

        self.frames = frames.ok();
    }

    fn view(&self) -> Element<Message> {
        if let Some(frames) = self.frames.as_ref() {
            container(webp(frames))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            row![].into()
        }
    }
}
