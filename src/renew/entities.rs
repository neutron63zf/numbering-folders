use std::convert::TryInto;
use std::num::ParseIntError;

// フォルダの番号
// 通常の数字と、数字を文字列にしたときの長さ
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FolderNumber(usize, usize);
// フォルダの文字列
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderName(String);
// 先頭の数字を除いたフォルダ名の文字列
#[derive(Debug, Clone, PartialEq, Eq)]
struct FolderNameNormalized(String);
// 数字付きフォルダの文字列
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberedFolderName {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FolderNameVariant {
    Normal(FolderName),
    Numbered(NumberedFolderName),
}
// フォルダ名変更命令
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderRenameInstruction {
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
            Ok(_) => FolderNameNormalized((&name[(head.len() + 1)..]).to_string()),
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

// フォルダ名に数字をつける時の数値
pub struct NumberingNumber(usize);

// フォルダ名の列に対する操作を出力する
pub struct FolderList(Vec<FolderNameVariant>);

impl FolderList {
    fn filter_numbered(&self) -> Vec<NumberedFolderName> {
        self.0
            .iter()
            .filter_map(|x| match x {
                FolderNameVariant::Numbered(x) => Some(x.clone()),
                _ => None,
            })
            .collect()
    }
    pub fn order(&self) -> Vec<FolderRenameInstruction> {
        let mut numbered = self.filter_numbered();
        let number_length = (numbered.len() - 1).to_string().len();
        numbered.sort();
        let instructions = numbered
            .into_iter()
            .enumerate()
            .map(|(i, x)| x.numbering(FolderNumber(i, number_length)))
            .collect();
        instructions
    }
    pub fn number(
        &self,
        name: FolderNameVariant,
        number: NumberingNumber,
    ) -> Vec<FolderRenameInstruction> {
        let numbered = self.filter_numbered();
        let max_number_length = if let Some(max) = numbered.iter().max() {
            (max.number.0 + 1).to_string().len()
        } else {
            number.0.to_string().len()
        };
        let mut instructions = numbered
            .into_iter()
            .map(|name| {
                let current_number = &name.number;
                let target_number = if current_number.0 >= number.0 {
                    current_number.0 + 1
                } else {
                    current_number.0
                };
                (FolderNameVariant::Numbered(name), target_number)
            })
            .collect::<Vec<_>>();
        instructions.push((name, number.0));
        // target_numberでソートし直す
        instructions.sort_by(|(_, a), (_, b)| a.cmp(b));
        // enumerateを使って間をつめる
        let instructions = instructions
            .iter()
            .enumerate()
            .map(|(i, (name, _))| {
                let target_number = FolderNumber(i, max_number_length);
                name.numbering(target_number)
            })
            .collect();
        instructions
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn folder_name_get_remaining_ok() {
        let name = FolderName("1_hoge".to_string());
        let result = name.get_remaining_name();
        assert_eq!(result.0, "hoge");
    }
    #[test]
    fn folder_name_get_remaining_err() {
        let name = FolderName("hoge".to_string());
        let result = name.get_remaining_name();
        assert_eq!(result.0, "hoge");
    }
    #[test]
    fn folder_name_get_first_num() {
        let name = FolderName("28_hoge".to_string());
        let result = name.get_first_number().unwrap();
        assert_eq!(result, FolderNumber(28, 2));
    }
    #[test]
    fn folder_name_try_into_success() {
        let folder_name = FolderName("39_hoge".to_string());
        let numbered_folder_name: Result<NumberedFolderName, _> = folder_name.try_into();
        assert!(numbered_folder_name.is_ok());
        assert_eq!(
            numbered_folder_name,
            Ok(NumberedFolderName {
                number: FolderNumber(39, 2),
                normalized_name: FolderNameNormalized("hoge".to_string())
            })
        );
    }
    #[test]
    fn folder_name_try_into_failure() {
        let folder_name = FolderName("hoge".to_string());
        let numbered_folder_name: Result<NumberedFolderName, _> = folder_name.try_into();
        assert!(numbered_folder_name.is_err());
    }
    #[test]
    fn numbered_folder_name_cmp() {
        let numbered_folder_name_1 = NumberedFolderName {
            number: FolderNumber(39, 2),
            normalized_name: FolderNameNormalized("hoge".to_string()),
        };
        let numbered_folder_name_2 = NumberedFolderName {
            number: FolderNumber(40, 2),
            normalized_name: FolderNameNormalized("hoge".to_string()),
        };
        assert!(numbered_folder_name_1 < numbered_folder_name_2);
        assert_eq!(
            numbered_folder_name_1.cmp(&numbered_folder_name_2),
            std::cmp::Ordering::Less
        );
    }
    #[test]
    fn numbering_folder_name() {
        let folder_name = FolderName("hoge".to_string());
        let instruction = folder_name.numbering(FolderNumber(39, 2));
        let instruction_expect = FolderRenameInstruction {
            target: FolderNameVariant::Normal(folder_name.clone()),
            new_name: NumberedFolderName {
                number: FolderNumber(39, 2),
                normalized_name: FolderNameNormalized("hoge".to_string()),
            },
        };
        assert_eq!(instruction, instruction_expect);
    }
    #[test]
    fn numbering_folder_get_name() {
        let numbered_folder_name = NumberedFolderName {
            number: FolderNumber(39, 2),
            normalized_name: FolderNameNormalized("hoge".to_string()),
        };
        assert_eq!(
            numbered_folder_name.get_name(),
            FolderName("39_hoge".to_string())
        );
    }
    #[test]
    fn numbering_folder_numbering() {
        let numbered_folder_name = NumberedFolderName {
            number: FolderNumber(39, 2),
            normalized_name: FolderNameNormalized("hoge".to_string()),
        };
        let instruction = numbered_folder_name.numbering(FolderNumber(40, 2));
        let instruction_expect = FolderRenameInstruction {
            target: FolderNameVariant::Numbered(numbered_folder_name),
            new_name: NumberedFolderName {
                number: FolderNumber(40, 2),
                normalized_name: FolderNameNormalized("hoge".to_string()),
            },
        };
        assert_eq!(instruction, instruction_expect);
    }
    #[test]
    fn numbering_folder_get_remaining() {
        let numbered_folder_name = NumberedFolderName {
            number: FolderNumber(39, 2),
            normalized_name: FolderNameNormalized("hoge".to_string()),
        };
        assert_eq!(
            numbered_folder_name.get_remaining_name(),
            FolderNameNormalized("hoge".to_string())
        );
    }
    #[test]
    fn numbering_folder_get_first_number() {
        let numbered_folder_name = NumberedFolderName {
            number: FolderNumber(39, 2),
            normalized_name: FolderNameNormalized("hoge".to_string()),
        };
        assert_eq!(
            numbered_folder_name.get_first_number(),
            Ok(FolderNumber(39, 2))
        );
    }
}
