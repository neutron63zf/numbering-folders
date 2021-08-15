use fnum::args_struct::{CommandLineData, SubCommand};

#[paw::main]
fn main(opt: CommandLineData) {
    match opt.get_subcommand() {
        Some(SubCommand::Order) => println!("Start ordering..."),
        Some(SubCommand::Number(number_data)) => println!("number data: {:?}", number_data),
        None => println!("No subcommand given"),
    }
}
