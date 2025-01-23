use iced::{
    widget::{button, text},
    Element,
};

#[derive(Clone, Default, Debug)]
struct App {
    path: String,
    actions: Vec<String>,
    areas: Vec<String>,
}
#[derive(Clone, Debug)]
enum Message {
    ActionButton(String),
    AreaInput(String),
}
impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            _ => (),
        }
    }
    pub fn view(&self) -> Element<Message> {
        button(text("hello")).into()
    }
}

fn main() -> iced::Result {
    iced::run("hello", App::update, App::view)
}
