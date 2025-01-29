use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const ROOT: &'static str = "src";
pub const SORTED: &'static str = "sorted";
pub const MEDIA: &'static str = "media";

pub fn build_paths(recursive_dirs: &Vec<Vec<String>>, indexes: &mut Vec<usize>) {
    if indexes.len() == recursive_dirs.len() {
        //let path =
        //    recursive_dirs
        //        .iter()
        //        .enumerate()
        //        .fold(PathBuf::from(SORTED), |acc, (i, dir)| {
        //            let mut acc = acc;
        //            let s: String = dir[indexes[i]].clone();
        //            acc.push(s);
        //            acc
        //        });
        let picked = picked_dirs_to_folder(
            &recursive_dirs
                .iter()
                .enumerate()
                .map(|(i, v)| v[indexes[i]].as_str())
                .collect::<Vec<&str>>(),
        );
        let mut dir = PathBuf::from(SORTED);
        dir.push(picked);
        let err = fs::DirBuilder::new().recursive(true).create(&dir);
    } else {
        for i in 0..recursive_dirs[indexes.len()].len() {
            indexes.push(i);
            build_paths(recursive_dirs, indexes);
            indexes.pop();
        }
    }
}
fn picked_dirs_to_folder(picked_dirs: &[&str]) -> String {
    picked_dirs
        .to_owned()
        .into_iter()
        .fold(String::new(), |acc, s| {
            if acc == "" {
                acc + s
            } else {
                acc + "_" + s
            }
        })
}
pub fn copy(picked_dirs: &Vec<String>, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = file_path.to_owned();
    let mut new_path = PathBuf::from(SORTED);
    new_path.push(picked_dirs_to_folder(
        &picked_dirs
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>(),
    ));
    new_path.push(path_to_filename(&file_path));
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
pub fn is_file_in_dir(dir: &Path, filename: &str) -> io::Result<bool> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let p = path.is_file().then(|| path.file_name()).flatten();
            match p.map(|f| f.to_str()) {
                Some(Some(file)) if file == &path_to_filename(Path::new(filename)) => {
                    return Ok(true);
                }
                Some(Some(file)) => {}
                Some(None) => println!("path: {path:?} is file but doesn't parst OSstr to &str"),
                None => match is_file_in_dir(&path, filename) {
                    Ok(true) => return Ok(true),
                    _ => {}
                },
            }
        }
    }
    Ok(false)
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
