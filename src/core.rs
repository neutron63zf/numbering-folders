#[derive(Debug)]
pub struct FolderNumbered {
    pub base_path_string: String,
    pub path_string: String,
    pub number: Option<i32>,
}

#[derive(Debug)]
pub struct RenameFolderNumber {
    pub base_path_string: String,
    pub path_string: String,
    pub target_number: i32,
    pub fill: i32,
}
