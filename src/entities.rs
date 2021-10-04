use std::convert::TryInto;
use std::num::ParseIntError;

// フォルダの番号
#[derive(Debug, Clone)]
pub struct FolderNumber(usize);
// フォルダの文字列
#[derive(Debug, Clone)]
pub struct FolderName(String);
// 先頭の数字を除いたフォルダ名の文字列
#[derive(Debug, Clone)]
pub struct FolderNameNormalized(String);
// 数字付きフォルダの文字列
#[derive(Debug, Clone)]
pub struct NumberedFolderName {
    pub number: FolderNumber,
    pub normalized_name: FolderNameNormalized,
}
impl TryInto<NumberedFolderName> for FolderName {
    type Error = ParseIntError;

    fn try_into(self) -> Result<NumberedFolderName, Self::Error> {
        let number = self.get_first_number()?;
        let normalized = self.get_remaining_name();
        Ok(NumberedFolderName {
            number,
            normalized_name: normalized,
        })
    }
}
pub enum FolderNameVariant {
    Normal(FolderName),
    Numbered(NumberedFolderName),
}
// フォルダ名変更命令
pub struct FolderRenameInstruction {
    pub target: FolderNameVariant,
    pub new_name: NumberedFolderName,
}

// フォルダ名であると言うことを扱いたい時のトレイト
trait FolderNameTrait: Sized {
    fn get_name(&self) -> FolderName;
    fn numbering(&self, number: FolderNumber) -> FolderRenameInstruction;
    fn get_remaining_name(&self) -> FolderNameNormalized {
        let name = self.get_name().0;
        let head = name.split("_").next().unwrap();
        let head_parsed = head.parse::<usize>();
        match head_parsed {
            Ok(_) => FolderNameNormalized((&name[head.len()..]).to_string()),
            Err(_) => FolderNameNormalized(name),
        }
    }
    fn get_first_number(&self) -> Result<FolderNumber, ParseIntError> {
        let name = self.get_name().0;
        let head = name.split("_").next().unwrap();
        let head_parsed = head.parse::<usize>();
        head_parsed.map(|result| FolderNumber(result))
    }
}
impl FolderNameTrait for FolderName {
    fn get_name(&self) -> FolderName {
        FolderName(self.0.clone())
    }
    fn numbering(&self, number: FolderNumber) -> FolderRenameInstruction {
        let numbered = NumberedFolderName {
            number,
            normalized_name: self.get_remaining_name(),
        };
        let instruction = FolderRenameInstruction {
            target: FolderNameVariant::Normal(self.clone()),
            new_name: numbered,
        };
        instruction
    }
}
impl FolderNameTrait for NumberedFolderName {
    fn get_name(&self) -> FolderName {
        FolderName(format!(
            "{}_{}",
            self.number.0,
            self.normalized_name.0.clone()
        ))
    }
    fn numbering(&self, number: FolderNumber) -> FolderRenameInstruction {
        let numbered = NumberedFolderName {
            number,
            normalized_name: self.normalized_name.clone(),
        };
        let instruction = FolderRenameInstruction {
            target: FolderNameVariant::Numbered(self.clone()),
            new_name: numbered,
        };
        instruction
    }
    fn get_remaining_name(&self) -> FolderNameNormalized {
        self.normalized_name.clone()
    }
    fn get_first_number(&self) -> Result<FolderNumber, ParseIntError> {
        Ok(self.number.clone())
    }
}

// パス文字列
pub struct PathString(String);

// フォルダ名を実際に取得、操作するトレイト
pub trait FolderRenameExecutor {
    fn get_folder_names(&self, path: PathString) -> Vec<FolderNameVariant>;
    fn execute_rename(&self, instruction: FolderRenameInstruction) -> Result<(), String>;
}
