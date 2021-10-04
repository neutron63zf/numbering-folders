use super::core;
use super::folders;

pub fn order_command() {
    // let folders = folders::folder_numbers("./".to_string());
    // let mut folders = folders
    //     .iter()
    //     .filter(|folder| folder.number.is_some())
    //     .collect::<Vec<_>>();
    // // someしかないのでunwrapできる
    // folders.sort_by(|a, b| a.number.unwrap().cmp(&b.number.unwrap()));
    // let count = folders.len();
    // let count_length = count.to_string().len() as i32;
    // (0..count).for_each(move |idx| {
    //     let folders = &folders;
    //     let target = folders.get(idx).unwrap();
    //     let instruction = core::RenameFolderNumber {
    //         base_path_string: target.base_path_string.clone(),
    //         path_string: target.path_string.clone(),
    //         target_number: idx as i32,
    //         fill: count_length - idx.to_string().len() as i32,
    //     };
    //     instruction.exec_rename();
    // });
}
