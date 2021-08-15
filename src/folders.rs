use std::convert::TryInto;
use std::fs;
use std::path;
use std::str::FromStr;

#[derive(Debug)]
pub struct FolderNumbered {
    path_string: String,
    number: Option<i32>,
}

#[derive(Debug)]
pub struct RenameFolderNumber {
    path_string: String,
    target_number: i32,
    fill: i32,
}

fn get_folders(path_string: String) -> Vec<path::PathBuf> {
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

fn folder_as_numbered(folder: &path::PathBuf) -> FolderNumbered {
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
        path_string,
        number,
    };
}

pub fn folder_numbers(path_string: String) -> Vec<FolderNumbered> {
    return get_folders(path_string)
        .iter()
        .map(|p| folder_as_numbered(p))
        .collect();
}

pub fn folder_rename(base_dir_path: String, instruction: RenameFolderNumber) {
    let base_dir = path::PathBuf::from_str(base_dir_path.as_str()).unwrap();
    let mut split = (&instruction.path_string).split("_");
    let first = split.next().unwrap();
    let target_name = format!(
        "{}{}_{}",
        "0".repeat(instruction.fill.try_into().unwrap()),
        instruction.target_number,
        // firstが数字だった場合は残りを、そうでない場合はpath_stringをそのまま使う
        first
            .parse::<i32>()
            .map(|_| split.collect::<String>())
            .unwrap_or(instruction.path_string.clone())
    );
    let from_path = base_dir.join(&instruction.path_string);
    let to_path = base_dir.join(target_name);
    fs::rename(from_path, to_path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    const TARGET_ENV: &str = "TARGET";
    #[test]
    fn folder_list() {
        let target = env::var(TARGET_ENV).unwrap_or("./".to_string());
        let folders = get_folders(target);
        folders.iter().for_each(|folder| {
            let name = folder.as_path().file_name().unwrap();
            println!("{:?}", name);
        });
    }
    #[test]
    fn folder_numbers() {
        let target = env::var(TARGET_ENV).unwrap_or("./".to_string());
        let folders = get_folders(target);
        let folders = folders
            .iter()
            .map(|folder| folder_as_numbered(folder))
            .collect::<Vec<_>>();
        folders.iter().for_each(|folder| println!("{:?}", folder));
    }
    #[test]
    #[ignore = "cannot run twice without preparation"]
    fn folder_rename_test() {
        let target = env::var(TARGET_ENV).unwrap_or("./".to_string());
        let instruction = RenameFolderNumber {
            path_string: env::var("FROM").unwrap_or("temp".to_string()),
            target_number: 25,
            fill: 1,
        };
        folder_rename(target, instruction);
    }
}
