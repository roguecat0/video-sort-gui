use iced::{
    widget::{button, column, row, text},
    Element, Length, Size, Subscription,
};
use iced_video_player::VideoPlayer;
use std::{
    collections::HashMap,
    path::Path,
    time::{Duration, Instant},
};
use video_sort_gui::{
    file_handling::{self, build_paths},
    widget::Player,
};

fn main() -> iced::Result {
    iced::application("main", App::update, App::view)
        .window(iced::window::Settings {
            size: Size::new(1098.0, 664.0),
            ..Default::default()
        })
        .run()
    //.run_with(App::default)
}
use iced_gif::gif;
use iced_webp::webp;

struct App {
    path: String,
    actions: Vec<String>,
    areas: Vec<String>,
    selected_action: Option<usize>,
    selected_area: Option<usize>,
    data: Data,
    last_tick: Instant,
    player: Player,
}

#[derive(Clone, Debug)]
enum Message {
    ActionInput(String),
    AreaInput(String),
    Tick(Instant),
    VideoEnd,
}

impl Default for App {
    fn default() -> Self {
        let data = Data::default();
        let actions = vec!["push".into(), "pull".into(), "exit".into()];
        let areas = vec!["stairs".into(), "pc".into(), "kitchen".into()];
        build_paths(&vec![actions.clone(), areas.clone()], &mut vec![]);
        let last_tick = Instant::now();
        let path = data.next_path().unwrap();
        let player = Player::from_path(Path::new(&path)).expect("path is not good");

        Self {
            player,
            path,
            selected_action: None,
            selected_area: None,
            last_tick,
            actions,
            areas,
            data,
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ActionInput(s) => {
                self.selected_action = self
                    .actions
                    .iter()
                    .enumerate()
                    .find(|(_, ss)| &&s == ss)
                    .map(|e| e.0);
                self.after_button_press();
            }
            Message::AreaInput(s) => {
                self.selected_area = self
                    .areas
                    .iter()
                    .enumerate()
                    .find(|(_, ss)| &&s == ss)
                    .map(|e| e.0);
                self.after_button_press();
            }
            Message::Tick(instant) => {
                let elapsed = instant - self.last_tick;
                self.last_tick = instant;
            }
            Message::VideoEnd => {}
        }
    }
    fn after_button_press(&mut self) {
        if let (Some(selected_action), Some(selected_area)) = self.all_selected_str() {
            let selected_action = selected_action.to_string();
            let selected_area = selected_area.to_string();
            self.data.file_map.insert(
                self.path.clone(),
                (selected_action.clone(), selected_area.clone()),
            );

            if let Err(e) = file_handling::copy(&vec![selected_action, selected_area], &self.path) {
                println!("copy failed: {e}");
            }

            if let Some(path) = self.data.next_path() {
                self.path = path;
                self.player = Player::from_path(Path::new(&self.path)).expect("path working");
            } else {
                println!("paths are finished")
            }
            println!("file_map: {:?}", self.data.file_map);
            self.reset_selected();
        }
    }
    fn all_selected_str(&self) -> (Option<&str>, Option<&str>) {
        (
            self.selected_action.map(|a| self.actions[a].as_str()),
            self.selected_area.map(|a| self.areas[a].as_str()),
        )
    }
    fn reset_selected(&mut self) {
        self.selected_area = None;
        self.selected_action = None;
    }

    pub fn view(&self) -> Element<Message> {
        let row1 = self.actions.iter().fold(row(None), |acc, s| {
            acc.push(button(text(s)).on_press(Message::ActionInput(s.into())))
        });
        let row2 = self.areas.iter().fold(row(None), |acc, s| {
            acc.push(button(text(s)).on_press(Message::AreaInput(s.into())))
        });

        let col = column![
            view_player(&self.player),
            text(format!("current file is: {:?}", self.path,)),
            text(format!(
                "selected_action: {:?}, selected_area: {:?}",
                self.selected_action, self.selected_area
            )),
            text("actions"),
            row1,
            text("areas"),
            row2,
            text("combinations"),
        ];
        self.data
            .file_map
            .iter()
            .fold(col, |acc, (path, (action, area))| {
                acc.push(text(format!(
                    "path: {path}, action: {action}, area: {area}"
                )))
            })
            .into()
    }
    fn subscription(&self) -> Subscription<Message> {
        let subscriptions = vec![iced::time::every(Duration::from_millis(1000)).map(Message::Tick)];
        iced::Subscription::batch(subscriptions)
    }
}
fn view_player(player: &Player) -> Element<Message> {
    match player {
        Player::Idle => text("idle").width(Length::Fill).height(Length::Fill).into(),
        Player::Gif { frames, .. } => gif(&frames)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .into(),
        Player::Webp { frames, .. } => webp(&frames)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .into(),
        Player::Video { video, .. } => VideoPlayer::new(video)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .content_fit(iced::ContentFit::Contain)
            .on_end_of_stream(Message::VideoEnd)
            .into(),
    }
}
#[derive(Debug, Clone)]
struct Data {
    file_paths: Vec<String>,
    file_map: HashMap<String, (String, String)>,
}
impl Data {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_paths = file_handling::get_file_names_in_dir(path).unwrap();
        Ok(Self {
            file_paths,
            file_map: HashMap::default(),
        })
    }
    pub fn next_path(&self) -> Option<String> {
        let size = self.file_map.len();
        println!("file_paths {:?}", self.file_paths);
        self.file_paths.get(size).map(|s| s.into())
    }
}
impl Default for Data {
    fn default() -> Self {
        Self::new(file_handling::MEDIA).unwrap()
    }
}
