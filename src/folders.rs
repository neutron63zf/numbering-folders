use super::core::{FolderNumbered, RenameFolderNumber};
use super::entities::{FolderName, FolderNameString, FolderNumberInt};
use std::fs;
use std::path;
use std::str::FromStr;

pub fn sample() {
    let mut paths = fs::read_dir(".").unwrap();
    let path = paths.next().unwrap().unwrap();
    let f = FolderNameString(path.file_name().into_string().unwrap());
    let tn = FolderNumberInt(1);
    let _nfns = f.numbering(tn);
}

fn get_folders(path_string: &String) -> Vec<path::PathBuf> {
    let paths = fs::read_dir(path_string).unwrap();

    return paths
        .flat_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_dir() {
                return vec![path];
            }
            return vec![];
        })
        .collect::<Vec<_>>();
}

pub fn folder_numbers(base_path_string: String) -> Vec<FolderNumbered> {
    return get_folders(&base_path_string)
        .iter()
        .map(|folder| {
            let path_string = folder
                .as_path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let number = path_string
                .split("_")
                .next()
                .unwrap()
                .parse::<i32>()
                .map(|pr| Some(pr))
                .unwrap_or(None);
            return FolderNumbered {
                base_path_string: base_path_string.clone(),
                path_string,
                number,
            };
        })
        .collect();
}

impl RenameFolderNumber {
    pub fn exec_rename(self) {
        let base_dir = path::PathBuf::from_str(self.base_path_string.as_str()).unwrap();
        let mut split = (&self.path_string).split("_");
        let first = split.next().unwrap();
        let target_name = format!(
            "{}{}_{}",
            "0".repeat(self.fill as usize),
            self.target_number,
            // firstが数字だった場合は残りを、そうでない場合はpath_stringをそのまま使う
            first
                .parse::<i32>()
                .map(|_| split.collect::<Vec<&str>>().join("_"))
                .unwrap_or(self.path_string.clone())
        );
        let from_path = base_dir.join(&self.path_string);
        let to_path = base_dir.join(target_name);
        fs::rename(&from_path, &to_path).unwrap();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    const TARGET_ENV: &str = "TARGET";
    #[test]
    fn folder_list() {
        let target = env::var(TARGET_ENV).unwrap_or("./".to_string());
        let folders = get_folders(&target);
        folders.iter().for_each(|folder| {
            let name = folder.as_path().file_name().unwrap();
            println!("{:?}", name);
        });
    }
    #[test]
    #[ignore = "cannot run twice without preparation"]
    fn folder_rename_test() {
        let target = env::var(TARGET_ENV).unwrap_or("./".to_string());
        let instruction = RenameFolderNumber {
            base_path_string: target.clone(),
            path_string: env::var("FROM").unwrap_or("temp".to_string()),
            target_number: 25,
            fill: 1,
        };
        instruction.exec_rename();
    }
}
