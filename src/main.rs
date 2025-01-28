use iced::{
    widget::{button, column, container, row, text, Button, Row},
    Alignment::Center,
    Color, Element, Length, Size, Subscription, Task,
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
        .centered()
        //.run()
        .run_with(App::with_taks)
}
use iced_gif::gif;
use iced_webp::webp;

struct App {
    path: PathBuf,
    next_path: Option<PathBuf>,
    actions: Vec<String>,
    areas: Vec<String>,
    selected_action: Option<usize>,
    selected_area: Option<usize>,
    data: Data,
    last_tick: Instant,
    player: Player,
    play_buff: Player,
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
        let mut data = Data::default();
        let actions = vec!["push".into(), "pull".into(), "exit".into()];
        let areas = vec!["stairs".into(), "pc".into(), "kitchen".into()];
        build_paths(&vec![actions.clone(), areas.clone()], &mut vec![]);
        let last_tick = Instant::now();
        let path = data.next_path().unwrap();
        let player = Player::from_path(&path).expect("path is not good");

        Self {
            player,
            play_buff: Player::Idle,
            path,
            next_path: data.next_path(),
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
        let mut data = Data::default();
        let actions = vec!["push".into(), "pull".into(), "exit".into()];
        let areas = vec!["stairs".into(), "pc".into(), "kitchen".into()];
        build_paths(&vec![actions.clone(), areas.clone()], &mut vec![]);
        let last_tick = Instant::now();
        let path = data.next_path().unwrap();
        let next_path = data.next_path();
        println!("Path: {path:?}, next_path: {next_path:?}");
        (
            Self {
                path: path.clone(),
                next_path,
                player: Player::Idle,
                play_buff: Player::Idle,
                selected_action: None,
                selected_area: None,
                last_tick,
                actions,
                areas,
                data,
            },
            Player::from_path_async_naive(&path).map(Message::Loaded),
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
                if let (true, Some(path)) = (self.after_button_press(), &self.next_path.clone()) {
                    self.path = path.clone();
                    self.next_path = self.data.next_path();
                    if let Player::Idle = self.play_buff {
                    } else {
                        std::mem::swap(&mut self.player, &mut self.play_buff);
                    }
                    println!("sending button load request: {:?}", path);
                    if let Some(p) = &self.next_path {
                        Player::from_path_async_naive(&p).map(Message::Loaded)
                    } else {
                        Task::none()
                    }
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
                if let (true, Some(path)) = (self.after_button_press(), self.next_path.clone()) {
                    println!("both are pressed");
                    self.path = path.clone();
                    self.next_path = self.data.next_path();
                    if let Player::Idle = self.play_buff {
                    } else {
                        std::mem::swap(&mut self.player, &mut self.play_buff);
                    }
                    println!("sending button load request: {:?}", path);
                    if let Some(p) = &self.next_path {
                        Player::from_path_async_naive(&p).map(Message::Loaded)
                    } else {
                        Task::none()
                    }
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
            Message::Loaded(player) => self.loading_handler(player),
        }
    }
    fn loading_handler(&mut self, player: Option<Player>) -> Task<Message> {
        println!("done loading!!!");
        let p = match (player, &self.player) {
            (None, _) => {
                println!("loading failed");
                Task::none()
            }
            (Some(vid), Player::Idle) => {
                println!(
                    "first loading! path: {:?}, next_path: {:?}",
                    self.path, self.next_path
                );
                self.player = vid;
                if let Some(path) = &self.next_path {
                    println!("sending next load request, from first; path: {:?}", path);
                    Player::from_path_async_naive(path).map(Message::Loaded)
                } else {
                    Task::none()
                }
            }
            (Some(vid), _) => {
                println!(
                    "after first path: {:?}, next_path: {:?}",
                    self.path, self.next_path
                );
                self.play_buff = vid;
                Task::none()
            }
        };
        println!(
            "after loads vids are: player: {:?}, buffer: {:?}",
            self.player, self.play_buff
        );
        p
    }
    fn after_button_press(&mut self) -> bool {
        if let (Some(selected_action), Some(selected_area)) = self.all_selected_str() {
            let selected_action = selected_action.to_string();
            let selected_area = selected_area.to_string();
            self.data.file_map.insert(
                self.path.to_str().unwrap().to_string(),
                (selected_action.clone(), selected_area.clone()),
            );

            if let Err(e) = file_handling::copy(&vec![selected_action, selected_area], &self.path) {
                println!("copy failed: {e}");
            }
            self.reset_selected();
            true
        } else {
            false
        }
    }
    fn handle_next_path(&mut self) {
        if let Some(path) = &self.next_path {
            println!("has path!!! ");
            self.path = path.to_owned();
            self.next_path = self.data.next_path();
        } else {
            println!("paths are finished");
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
    fn create_button(&self, s: &str, action: bool) -> Button<Message> {
        let message = if action {
            Message::ActionInput(s.into())
        } else {
            Message::AreaInput(s.into())
        };
        let b = button(text(s.to_string())).on_press(message);
        match self.all_selected_str() {
            (Some(sel), _) if sel == s && action => {
                b.style(|theme, status| iced::widget::button::secondary(theme, status))
            }
            (_, Some(sel)) if sel == s && !action => {
                b.style(|theme, status| iced::widget::button::secondary(theme, status))
            }
            _ => b,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let clr = Color::from_rgb(1.0, 1.0, 1.0);
        let row1: Row<Message> = self
            .actions
            .iter()
            .fold(row(None), |acc: Row<Message>, s: &String| {
                acc.push(self.create_button(s, true))
            })
            .align_y(Center)
            .padding(10)
            .spacing(10);
        let row2 = self
            .areas
            .iter()
            .fold(row(None), |acc, s| acc.push(self.create_button(s, false)))
            .align_y(Center)
            .padding(10)
            .spacing(10);
        let video = container(view_player(&self.player).explain(Color::from_rgb(1.0, 1.0, 1.0)))
            .width(Length::Fill)
            .height(Length::FillPortion(2));

        let col: Element<Message> = column![
            text(format!("current file is: {:?}", self.path,)),
            text("actions"),
            row1,
            text("areas"),
            row2,
        ]
        .width(Length::Fill)
        .height(Length::FillPortion(1))
        .into();
        Element::from(column![video, col]).explain(clr)
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
    index: usize,
    file_map: HashMap<String, (String, String)>,
}
impl Data {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_paths = file_handling::get_file_names_in_dir(path).unwrap();
        Ok(Self {
            file_paths,
            index: 0,
            file_map: HashMap::default(),
        })
    }
    pub fn next_path(&mut self) -> Option<PathBuf> {
        //let size = self.file_map.len();
        //println!("file_paths {:?}", self.file_paths);
        self.index += 1;
        self.file_paths
            .get(self.index - 1)
            .map(|s| PathBuf::from(s))
    }
}
impl Default for Data {
    fn default() -> Self {
        Self::new(file_handling::MEDIA).unwrap()
    }
}
