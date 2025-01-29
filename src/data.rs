use super::file_handling;
use rusqlite;
use std::{collections::HashMap, fmt::Debug, path::PathBuf};
#[derive(Debug)]
pub struct Data {
    file_paths: Vec<String>,
    index: usize,
    pub file_map: HashMap<String, (String, String)>,
    pub conn: rusqlite::Connection,
}
impl Data {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_paths = file_handling::get_file_names_in_dir(path).unwrap();
        let conn = rusqlite::Connection::open_in_memory()?;
        Ok(Self {
            conn,
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
