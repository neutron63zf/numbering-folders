
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Folders Numbering",
    about = "Prefix folders with number",
    author = "Nkowne63"
)]
pub struct CommandLineData {
    #[structopt(subcommand)]
    subcommand: Option<SubCommand>,
}

impl CommandLineData {
    pub fn get_subcommand(self) -> Option<SubCommand> {
        return self.subcommand;
    }
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    Order,
    Number(NumberData),
}

#[derive(Debug, StructOpt)]
pub struct NumberData {
    #[structopt(short = "n", long = "number")]
    number: i32,
    #[structopt(short = "f", long = "folder")]
    folder_name: String,
}
