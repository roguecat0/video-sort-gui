use super::file_handling;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};
#[derive(Debug)]

pub struct Data {
    file_paths: Vec<String>,
    index: usize,
    pub file_map: HashMap<String, (String, String)>,
}
type GenResult<T> = Result<T, Box<dyn std::error::Error>>;
impl Data {
    pub fn new(path: &str) -> GenResult<Self> {
        let file_paths = file_handling::get_file_names_in_dir(path)?
            .into_iter()
            .filter(|s| {
                !file_handling::is_file_in_dir(Path::new(file_handling::SORTED), &s)
                    .expect("path checking broke")
            })
            .collect::<Vec<_>>();

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
