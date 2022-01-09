use super::entities::{FolderList, FolderNameVariant, FolderRenameInstruction, NumberingNumber};

// パス文字列
struct PathString(String);
// 実行時エラー
struct ExecureRenameError(String);

// フォルダ名を実際に取得、操作するトレイト
trait FolderRenameExecutor {
    fn get_folder_names(&self, path: &PathString) -> FolderList;
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
        let folder_list = self.executor.get_folder_names(path);
        for instruction in folder_list.order() {
            self.executor.execute_rename(path, instruction)?;
        }
        Ok(())
    }
    fn number(
        &self,
        name: FolderNameVariant,
        number: NumberingNumber,
    ) -> Result<(), ExecureRenameError> {
        let path = &self.path;
        let folder_list = self.executor.get_folder_names(path);
        for instruction in folder_list.number(name, number) {
            self.executor.execute_rename(path, instruction)?;
        }
        Ok(())
    }
}
