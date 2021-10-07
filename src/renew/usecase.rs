use super::entities::{
    FolderName, FolderNameTrait, FolderNameVariant, FolderNumber, FolderRenameInstruction,
};

// パス文字列
struct PathString(String);
// 実行時エラー
struct ExecureRenameError(String);
// フォルダ名に数字をつける時の数値
struct NumberingNumber(usize);

// フォルダ名を実際に取得、操作するトレイト
trait FolderRenameExecutor {
    fn get_folder_names(&self, path: &PathString) -> Vec<FolderNameVariant>;
    fn execute_rename(
        &self,
        base: &PathString,
        instruction: FolderRenameInstruction,
    ) -> Result<(), ExecureRenameError>;
}

struct FolderNameChanger<FRE: FolderRenameExecutor> {
    executor: FRE,
    path: PathString,
}

impl<FRE: FolderRenameExecutor> FolderNameChanger<FRE> {
    fn new(executor: FRE, path: PathString) -> Self {
        Self { executor, path }
    }
    fn order(&self) -> Result<(), ExecureRenameError> {
        let path = &self.path;
        let folder_names = self.executor.get_folder_names(path);
        let mut numbered = folder_names
            .into_iter()
            .filter_map(|name| match name {
                FolderNameVariant::Normal(_) => None,
                FolderNameVariant::Numbered(name) => Some(name),
            })
            .collect::<Vec<_>>();
        let number_length = (numbered.len() - 1).to_string().len();
        numbered.sort();
        let instructions_iter = numbered
            .into_iter()
            .enumerate()
            .map(|(index, name)| name.numbering(FolderNumber(index, number_length)));
        for instruction in instructions_iter {
            self.executor.execute_rename(path, instruction)?;
        }
        return Ok(());
    }
    fn number(&self, name: FolderName, number: NumberingNumber) -> Result<(), ExecureRenameError> {
        let path = &self.path;
        let folder_names = self.executor.get_folder_names(path);
        let numbered = folder_names
            .into_iter()
            .filter_map(|name| match name {
                FolderNameVariant::Normal(_) => None,
                FolderNameVariant::Numbered(name) => Some(name),
            })
            .collect::<Vec<_>>();

        let max_number_length = if let Some(max) = numbered.iter().max() {
            (max.number.0 + 1).to_string().len()
        } else {
            number.0.to_string().len()
        };

        let instructions_iter = numbered.into_iter().map(|name| {
            let current_number = &name.number;
            let target_number_raw = if current_number.0 >= number.0 {
                current_number.0 + 1
            } else {
                current_number.0
            };
            let target_number = FolderNumber(target_number_raw, max_number_length);
            name.numbering(target_number)
        });
        self.executor.execute_rename(
            path,
            name.numbering(FolderNumber(number.0, max_number_length)),
        )?;
        for instruction in instructions_iter {
            self.executor.execute_rename(path, instruction)?;
        }
        return Ok(());
    }
}
