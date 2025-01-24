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
