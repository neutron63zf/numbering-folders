use std::fmt::{Debug, Display};
use std::ops::Add;
use std::str::FromStr;

// trait

// フォルダの番号
pub trait FolderNumber {}

// フォルダ名（string的な何かに実装したい）
// 他のFolderNumberを与えたら他のNumberedFolderNameになって欲しいのでgenerics
pub trait FolderName<FN: FolderNumber> {
    // Numberedがassociated typeであるのは、FolderNameとFolderNumberの組み合わせについて、
    // NumberedFolderNameは一つしか存在しないため
    type Numbered: NumberedFolderName<FN>;
    // FolderNameの先頭の数字を読み取って、うまく行ったらNumberedにする
    fn try_get_numbered(&self) -> Result<Self::Numbered, ()>;
    // FolderNameをtarget_numberを用いてNumberedにする
    fn numbering(&self, target_number: FN) -> Self::Numbered;
}

// 先頭に数字がついたフォルダ名
// いろんなFolderNumberについてそれぞれNumberedFolderNameがあって欲しいのでgenerics
pub trait NumberedFolderName<FN: FolderNumber>: FolderName<FN> {
    // FolderNameから数字を取り出す
    // この型ができる時点でうまくできるはずなのでResultではない
    fn get_number(&self) -> FN;
}

// NumberedFolderNameのコレクション
// いろんなNumberedFolderNameがあって欲しいのでgenerics
pub trait NumberedFolderNameCollection<FN: FolderNumber, NF: NumberedFolderName<FN>> {
    // FolderNumberとNumberedFolderNameが決まった時点で与えられるInstructionの型は確定できる
    type Instruction: FoldersRenameInstruction;
    // orderコマンドに対応するinstructionを返す
    fn get_order_instruction(&self) -> Self::Instruction;
    // numberコマンドに対応するinstructionを返す
    // folder_nameはNumberedFoldernameに変換可能なFolderNameを取る
    // これがIntoではないのは、numberがないとFolderNameをNumberedFolderNameに変換できないから
    fn get_number_instruction<F: FolderName<FN, Numbered = NF>>(
        &self,
        folder_name: F,
        number: FN,
    ) -> Self::Instruction;
}

// repositoryはこれを元に実行する
pub trait FoldersRenameInstruction {}

// struct and impl

fn get_first_number<S, N>(str: S) -> Result<N, N::Err>
where
    S: Into<String>,
    N: FromStr,
{
    str.into().split("_").next().unwrap().parse()
}

fn numbering<S, N>(str: S, target_number: N) -> S
where
    S: Into<String> + std::iter::FromIterator<String> + Clone,
    N: FromStr + std::ops::Add<S, Output = S>,
{
    let number = get_first_number::<S, N>(str.clone());
    let remaining = if let Ok(_) = number {
        let str = str.into();
        let mut split = str.split("_").map(|s| s.to_owned());
        split.next();
        split.collect::<S>()
    } else {
        str
    };
    target_number + remaining
}

pub struct FolderNameString<S>(pub S)
where
    S: Into<String> + std::iter::FromIterator<String> + Clone;

impl<FN, S> FolderName<FN> for FolderNameString<S>
where
    FN: FolderNumber + FromStr + std::ops::Add<S, Output = S>,
    FN::Err: Debug,
    S: Into<String> + std::iter::FromIterator<String> + Clone,
{
    type Numbered = NumberedFolderNameString<S>;
    fn try_get_numbered(&self) -> Result<Self::Numbered, ()> {
        let number = get_first_number::<S, FN>(self.0.clone());
        if let Ok(_) = number {
            Ok(NumberedFolderNameString(self.0.clone()))
        } else {
            Err(())
        }
    }
    fn numbering(&self, target_number: FN) -> Self::Numbered {
        NumberedFolderNameString(numbering(self.0.clone(), target_number))
    }
}
pub struct NumberedFolderNameString<S>(pub S)
where
    S: Into<String> + std::iter::FromIterator<String> + Clone;

impl<FN, S> FolderName<FN> for NumberedFolderNameString<S>
where
    FN: FolderNumber + FromStr + std::ops::Add<S, Output = S>,
    FN::Err: Debug,
    S: Into<String> + std::iter::FromIterator<String> + Clone,
{
    type Numbered = NumberedFolderNameString<S>;
    fn try_get_numbered(&self) -> Result<Self::Numbered, ()> {
        Ok(NumberedFolderNameString(self.0.clone()))
    }
    fn numbering(&self, target_number: FN) -> Self::Numbered {
        NumberedFolderNameString(numbering(self.0.clone(), target_number))
    }
}

impl<FN, S> NumberedFolderName<FN> for NumberedFolderNameString<S>
where
    FN: FolderNumber + FromStr + std::ops::Add<S, Output = S>,
    FN::Err: Debug,
    S: Into<String> + std::iter::FromIterator<String> + Clone,
{
    fn get_number(&self) -> FN {
        get_first_number(self.0.clone()).unwrap()
    }
}

pub struct FolderNumberInt<N>(pub N);

impl<N> FolderNumber for FolderNumberInt<N> {}
impl<N> FromStr for FolderNumberInt<N>
where
    N: FromStr,
{
    type Err = N::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<N>() {
            Ok(u) => Ok(FolderNumberInt(u)),
            Err(err) => Err(err),
        }
    }
}
impl<N, S> Add<S> for FolderNumberInt<N>
where
    N: Display,
    S: Display + From<String>,
{
    type Output = S;
    fn add(self, other: S) -> Self::Output {
        format!("{}{}", self.0, other).into()
    }
}
