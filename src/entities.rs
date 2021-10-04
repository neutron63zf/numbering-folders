use std::convert::TryInto;
use std::num::ParseIntError;

// フォルダの番号
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FolderNumber(usize, usize);
// フォルダの文字列
#[derive(Debug, Clone)]
struct FolderName(String);
// 先頭の数字を除いたフォルダ名の文字列
#[derive(Debug, Clone, PartialEq, Eq)]
struct FolderNameNormalized(String);
// 数字付きフォルダの文字列
#[derive(Debug, Clone, PartialEq, Eq)]
struct NumberedFolderName {
    number: FolderNumber,
    normalized_name: FolderNameNormalized,
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
impl PartialOrd for NumberedFolderName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.number.cmp(&other.number))
    }
}
impl Ord for NumberedFolderName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
enum FolderNameVariant {
    Normal(FolderName),
    Numbered(NumberedFolderName),
}
// フォルダ名変更命令
struct FolderRenameInstruction {
    target: FolderNameVariant,
    new_name: NumberedFolderName,
}

// フォルダ名であると言うことを扱いたい時のトレイト
trait FolderNameTrait {
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
        head_parsed.map(|result| FolderNumber(result, head.len()))
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
        let pat_zero = self.number.1 - self.number.0.to_string().len();
        let zero_str = "0".repeat(pat_zero);
        FolderName(format!(
            "{}{}_{}",
            zero_str,
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
impl FolderNameVariant {
    fn numbering(&self, number: FolderNumber) -> FolderRenameInstruction {
        match self {
            FolderNameVariant::Normal(name) => name.numbering(number),
            FolderNameVariant::Numbered(name) => name.numbering(number),
        }
    }
}

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
