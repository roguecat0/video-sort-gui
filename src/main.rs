use iced::{
    widget::{button, column, container, row, text, Button, Row, Text},
    window,
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
use video_sort_gui::data::Data;
use video_sort_gui::{
    file_handling::{self, build_paths},
    widget::Player,
};

fn main() -> iced::Result {
    iced::application("main", App::update, App::view)
        .window(iced::window::Settings {
            size: Size::new(598.0, 664.0),
            ..Default::default()
        })
        .centered()
        .subscription(App::subscription)
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
    KeyboardDigit(u32),
    WindowId(Option<window::Id>),
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
        let path = data.next_path().expect("no unsorted videos left");
        let next_path = data.next_path();
        let f = Player::from_path_async_naive(&path).map(Message::Loaded);
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
            Task::batch(vec![f, iced::window::get_latest().map(Message::WindowId)]),
        )
    }
    fn subscription(&self) -> Subscription<Message> {
        use iced::keyboard;

        let keyboard_sub = keyboard::on_key_press(|key, _| {
            if let keyboard::Key::Character(key) = key {
                //println!("key: {key:?}");
                key.as_str()
                    .chars()
                    .next()
                    .and_then(|c| c.to_digit(10))
                    .map(Message::KeyboardDigit)
            } else {
                None
            }
        });

        let subscriptions = vec![
            iced::time::every(Duration::from_millis(1000)).map(Message::Tick),
            keyboard_sub,
        ];
        iced::Subscription::batch(subscriptions)
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
                self.total_after_press()
            }
            Message::AreaInput(s) => {
                self.selected_area = self
                    .areas
                    .iter()
                    .enumerate()
                    .find(|(_, ss)| &&s == ss)
                    .map(|e| e.0);
                self.total_after_press()
            }
            Message::Tick(instant) => {
                let elapsed = instant - self.last_tick;
                self.last_tick = instant;
                Task::none()
            }
            Message::VideoEnd => Task::none(),
            Message::Loaded(player) => self.loading_handler(player),
            Message::KeyboardDigit(d) => match d {
                1..=9 => {
                    self.keyboard_map_button(d - 1);
                    self.total_after_press()
                }
                _ => Task::none(),
            },
            Message::WindowId(Some(id)) => window::maximize(id, true),
            Message::WindowId(None) => {
                println!("win id not found :/");
                Task::none()
            }
        }
    }
    fn total_after_press(&mut self) -> Task<Message> {
        if let (true, Some(path)) = (self.after_button_press(), self.next_path.clone()) {
            self.path = path.clone();
            self.next_path = self.data.next_path();
            if let Player::Idle = self.play_buff {
            } else {
                std::mem::swap(&mut self.player, &mut self.play_buff);
            }
            if let Some(p) = &self.next_path {
                Player::from_path_async_naive(&p).map(Message::Loaded)
            } else {
                Task::none()
            }
        } else {
            Task::none()
        }
    }
    fn keyboard_map_button(&mut self, d: u32) {
        if None == self.selected_action {
            self.selected_action = self.actions.get(d as usize).map(|_| d as usize);
        } else {
            self.selected_area = self.areas.get(d as usize).map(|_| d as usize);
        }
    }

    fn loading_handler(&mut self, player: Option<Player>) -> Task<Message> {
        let p = match (player, &self.player) {
            (None, _) => {
                let file_path = if let Player::Idle = self.player {
                    &self.path
                } else {
                    &self
                        .next_path
                        .clone()
                        .expect("there to be a path that failed to load")
                };
                print!("failed to load file: {file_path:?}");
                Task::none()
            }
            (Some(vid), Player::Idle) => {
                self.player = vid;
                if let Some(path) = &self.next_path {
                    Player::from_path_async_naive(path).map(Message::Loaded)
                } else {
                    Task::none()
                }
            }
            (Some(vid), _) => {
                self.play_buff = vid;
                Task::none()
            }
        };
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

            println!(
                "data: {:?}, index: {}",
                self.data.file_paths, self.data.index
            );
            if let Err(e) = file_handling::copy(&vec![selected_action, selected_area], &self.path) {
                println!("copy failed: {e}");
            }
            self.reset_selected();
            if self.data.file_paths.len() + 1 <= self.data.index {
                println!("paths are finished");
                panic!("no more paths no more fun");
            }
            true
        } else {
            false
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
    fn create_button(&self, s: &str, action: bool, i: usize) -> Button<Message> {
        let message = if action {
            Message::ActionInput(s.into())
        } else {
            Message::AreaInput(s.into())
        };
        let b = button(container(text(format!("{s}: ({i})")).size(16)).center(Length::Fill))
            .on_press(message)
            .width(200);
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
        let sub_head_clr = Color::parse("#8a9091").unwrap();
        let action_text: Text = text("Actions: ").size(20).center();
        let area_text: Text = text("Areas: ").size(20).center();
        let row1: Row<Message> = self
            .actions
            .iter()
            .enumerate()
            .fold(
                row![action_text],
                |acc: Row<Message>, (i, s): (usize, &String)| {
                    acc.push(self.create_button(s, true, i))
                },
            )
            .align_y(Center)
            .padding(10)
            .spacing(10);
        let row2 = self
            .areas
            .iter()
            .enumerate()
            .fold(row![area_text], |acc, (i, s)| {
                acc.push(self.create_button(s, false, i))
            })
            .align_y(Center)
            .padding(10)
            .spacing(10);
        let video = container(view_player(&self.player))
            .width(Length::Fill)
            .height(Length::FillPortion(3));

        let col: Element<Message> = column![
            text(format!("current file is: {:?}", self.path,)).color(sub_head_clr),
            row1,
            row2,
        ]
        .align_x(Center)
        .spacing(12)
        .into();
        let col = container(col).center(Length::Fill);
        Element::from(column![video, col].spacing(10))
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
