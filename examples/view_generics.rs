use iced::{
    widget::{button, container, text, Container, Text},
    Color, Element,
};
use iced::{Renderer, Theme};
fn main() -> iced::Result {
    iced::run("Video Player", (), App::view)
}

struct App {
    counter: u32,
}

impl Default for App {
    fn default() -> Self {
        App { counter: 0 }
    }
}

impl App {
    fn view(&self) -> iced::Element<()> {
        let color = Color::from_rgb(1.0, 1.0, 1.0);
        let count: u32 = self.counter;
        let t: Element<(), Theme, Renderer> = text!("hello: {}", count).into();
        let t = t.explain(color.clone());
        let cont: Container<'_, ()> = container(t);
        let cont: Element<()> = cont.into();
        cont.explain(color)
    }
}
