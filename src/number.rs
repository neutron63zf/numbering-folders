use crate::core;
use crate::folders;

pub fn number_command(folder_name: String, number: i32) {
    let folders = folders::folder_numbers("./".to_string());
    let mut gte_folders = folders
        .iter()
        .filter(|folder| folder.number.unwrap_or(0) >= number);
    let before_count_length = (folders
        .iter()
        .map(|folder| folder.number.unwrap_or(0))
        .max()
        .unwrap())
    .to_string()
    .len() as i32;
    let count_length = (folders
        .iter()
        .map(|folder| folder.number.unwrap_or(0))
        .max()
        .unwrap()
        + 1)
    .to_string()
    .len() as i32;
    let is_target_num_exists = gte_folders
        .find(|folder| folder.number.unwrap_or(0) == number)
        .is_some();
    let mut instructions = Vec::new();
    instructions.push(core::RenameFolderNumber {
        base_path_string: "./".to_string(),
        path_string: folder_name,
        target_number: number,
        fill: count_length - number.to_string().len() as i32,
    });
    if is_target_num_exists {
        gte_folders.for_each(|folder| {
            let target_number = folder.number.unwrap_or(0) + 1;
            instructions.push(core::RenameFolderNumber {
                base_path_string: folder.base_path_string.clone(),
                path_string: folder.path_string.clone(),
                target_number,
                fill: count_length - target_number.to_string().len() as i32,
            });
        })
    }
    if before_count_length < count_length {
        // prefixのゼロの数が変わるので前のも変更しないといけない
        folders
            .iter()
            .filter(|folder| folder.number.is_some() && folder.number.unwrap_or(i32::MAX) < number)
            .for_each(|folder| {
                instructions.push(core::RenameFolderNumber {
                    base_path_string: folder.base_path_string.clone(),
                    path_string: folder.path_string.clone(),
                    target_number: folder.number.unwrap(),
                    fill: count_length - folder.number.unwrap().to_string().len() as i32,
                });
            });
    };
    instructions
        .into_iter()
        .for_each(|instruction| instruction.exec_rename());
}
