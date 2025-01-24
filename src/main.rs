use file_handling::build_paths;
use iced::{
    widget::{button, column, row, text},
    Element,
};
use std::collections::HashMap;

fn main() -> iced::Result {
    iced::run("hello", App::update, App::view)
}

#[derive(Clone, Debug)]
struct App {
    path: String,
    actions: Vec<String>,
    areas: Vec<String>,
    selected_action: Option<usize>,
    selected_area: Option<usize>,
    data: Data,
}

#[derive(Clone, Debug)]
enum Message {
    ActionInput(String),
    AreaInput(String),
}

impl Default for App {
    fn default() -> Self {
        let data = Data::default();
        let actions = vec!["push".into(), "pull".into(), "exit".into()];
        let areas = vec!["stairs".into(), "pc".into(), "kitchen".into()];
        build_paths(&vec![actions.clone(), areas.clone()], &mut vec![]);

        Self {
            path: data.next_path().unwrap(),
            actions,
            areas,
            selected_action: None,
            selected_area: None,
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
mod file_handling {
    use std::ffi::OsStr;
    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};

    const ROOT: &'static str = "src";
    const SORTED: &'static str = "sorted";
    pub const MEDIA: &'static str = "media";

    pub fn build_paths(recursive_dirs: &Vec<Vec<String>>, indexes: &mut Vec<usize>) {
        if indexes.len() == recursive_dirs.len() {
            let path =
                recursive_dirs
                    .iter()
                    .enumerate()
                    .fold(PathBuf::from(SORTED), |acc, (i, dir)| {
                        let mut acc = acc;
                        let s: String = dir[indexes[i]].clone();
                        acc.push(s);
                        acc
                    });
            let err = fs::DirBuilder::new().recursive(true).create(&path);
        } else {
            for i in 0..recursive_dirs[indexes.len()].len() {
                indexes.push(i);
                build_paths(recursive_dirs, indexes);
                indexes.pop();
            }
        }
    }
    pub fn copy(
        picked_dirs: &Vec<String>,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = PathBuf::from(file_path);
        let mut new_path = PathBuf::from(SORTED);
        new_path.push(PathBuf::from_iter(picked_dirs.iter()));
        new_path.push(path_to_filename(&file_path));
        println!("old: {file_path:?}, new: {new_path:?}");
        fs::copy(file_path, new_path)?;
        Ok(())
    }
    fn path_to_filename(old_path: &Path) -> String {
        old_path
            .iter()
            .skip(1)
            .flat_map(|s| s.to_str().map(|ss| ss.to_string()))
            .reduce(|acc, s| acc + "_" + &s)
            .unwrap()
    }

    pub fn get_file_names_in_dir(path: &str) -> io::Result<Vec<String>> {
        let dir = Path::new(path);
        let files = visit_dirs(dir, vec![])?;
        Ok(files)
    }

    // one possible implementation of walking a directory only visiting files
    fn visit_dirs(dir: &Path, mut file_names: Vec<String>) -> io::Result<Vec<String>> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    file_names = visit_dirs(&path, file_names)?;
                } else {
                    file_names.push(path.to_str().unwrap().into());
                }
            }
        }
        Ok(file_names)
    }
}
