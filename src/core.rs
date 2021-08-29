use std::{
    num::ParseIntError,
    ops::{Add, Deref},
};

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

// enterprise business rules trait

// フォルダ名の先頭につく数字に使える型
// TODO 実装が進んだらちゃんとトレイト境界を加える
pub trait FolderNumber {}

// フォルダ名（string的な何かに実装したい）
// TODO 実装が進んだらちゃんとトレイト境界を加える
// 他のFolderNumberを与えたら他のNumberedFolderNameになって欲しいのでgenerics
pub trait FolderName<N: FolderNumber> {
    // Numberedがassociated typeであるのは、FolderNameとFolderNumberの組み合わせについて、
    // NumberedFolderNameは一つしか存在しないため
    type Numbered: NumberedFolderName<N>;
    // FolderNameの先頭の数字を読み取って、うまく行ったらNumberedにする
    fn try_get_numbered(&self) -> Result<Self::Numbered, ()>;
    // FolderNameをtarget_numberを用いてNumberedにする
    fn numbering(&self, target_number: N) -> Self::Numbered;
}

// 先頭に数字がついたフォルダ名
// いろんなFolderNumberについてそれぞれNumberedFolderNameがあって欲しいのでgenerics
pub trait NumberedFolderName<N: FolderNumber>: FolderName<N> {
    // FolderNameから数字を取り出す
    // この型ができる時点でうまくできるはずなのでResultではない
    fn get_number(&self) -> N;
}

// NumberedFolderNameのコレクション
// いろんなNumberedFolderNameがあって欲しいのでgenerics
pub trait NumberedFolderNameCollection<N: FolderNumber, NF: NumberedFolderName<N>> {
    // FolderNumberとNumberedFolderNameが決まった時点で与えられるInstructionの型は確定できる
    type Instruction: FoldersRenameInstruction;
    // orderコマンドに対応するinstructionを返す
    fn get_order_instruction(&self) -> Self::Instruction;
    // numberコマンドに対応するinstructionを返す
    // folder_nameはNumberedFoldernameに変換可能なFolderNameを取る
    fn get_number_instruction<F: FolderName<N, Numbered = NF>>(
        &self,
        folder_name: F,
        number: N,
    ) -> Self::Instruction;
}

// executeすれば反映されるやつ
pub trait FoldersRenameInstruction: Sized {
    // 実行する
    fn execute(&self) -> Result<(), ()>;
}

// application business rules trait

// interface adapter trait
