use iced::{
    widget::{button, column, container, row, text},
    Element, Length, Size, Subscription, Task,
};
use iced_video_player::VideoPlayer;
use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
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
        //.run()
        .run_with(App::with_taks)
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

#[derive(Clone)]
enum Message {
    ActionInput(String),
    AreaInput(String),
    Tick(Instant),
    VideoEnd,
    Loaded(Option<Player>),
}
impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("is message enume")
    }
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
    pub fn with_taks() -> (Self, Task<Message>) {
        let data = Data::default();
        let actions = vec!["push".into(), "pull".into(), "exit".into()];
        let areas = vec!["stairs".into(), "pc".into(), "kitchen".into()];
        build_paths(&vec![actions.clone(), areas.clone()], &mut vec![]);
        let last_tick = Instant::now();
        let path = data.next_path().unwrap();
        let path2 = path.clone();
        //let player = Player::from_path(pPath).expect("path is not good");

        (
            Self {
                player: Player::Idle,
                path,
                selected_action: None,
                selected_area: None,
                last_tick,
                actions,
                areas,
                data,
            },
            Player::from_path_async_naive(Path::new(&path2)).map(Message::Loaded),
        )
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ActionInput(s) => {
                self.selected_action = self
                    .actions
                    .iter()
                    .enumerate()
                    .find(|(_, ss)| &&s == ss)
                    .map(|e| e.0);
                if let Some(path) = self.after_button_press() {
                    Player::from_path_async_naive(&path).map(Message::Loaded)
                } else {
                    Task::none()
                }
            }
            Message::AreaInput(s) => {
                self.selected_area = self
                    .areas
                    .iter()
                    .enumerate()
                    .find(|(_, ss)| &&s == ss)
                    .map(|e| e.0);
                if let Some(path) = self.after_button_press() {
                    Player::from_path_async_naive(&path).map(Message::Loaded)
                } else {
                    Task::none()
                }
            }
            Message::Tick(instant) => {
                let elapsed = instant - self.last_tick;
                self.last_tick = instant;
                Task::none()
            }
            Message::VideoEnd => Task::none(),
            Message::Loaded(player) => {
                match player {
                    Some(player) => self.player = player,
                    None => println!("loading failed"),
                }
                Task::none()
            }
        }
    }
    fn after_button_press(&mut self) -> Option<PathBuf> {
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
            self.reset_selected();
            if let Some(path) = self.data.next_path() {
                println!("has path!!! ");
                self.path = path;
                Some(PathBuf::from(&self.path))
            } else {
                println!("paths are finished");
                None
            }
        } else {
            None
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
            container(view_player(&self.player)).width(400).height(400),
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
    let vid = match player {
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
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .content_fit(iced::ContentFit::Contain)
            .on_end_of_stream(Message::VideoEnd)
            .into(),
    };
    vid
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
