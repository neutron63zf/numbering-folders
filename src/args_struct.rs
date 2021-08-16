use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Folders Numbering",
    about = "Prefix folders with number",
    author = "Nkowne63"
)]
pub struct CommandLineData {
    #[structopt(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    Order,
    Number(NumberData),
}

#[derive(Debug, StructOpt)]
pub struct NumberData {
    #[structopt(short = "n", long = "number")]
    pub number: i32,
    #[structopt(short = "f", long = "folder")]
    pub folder_name: String,
}
