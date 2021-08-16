use fnum::args_struct;
use fnum::number;
use fnum::order;

#[paw::main]
fn main(opt: args_struct::CommandLineData) {
    match opt.subcommand {
        Some(args_struct::SubCommand::Order) => order::order_command(),
        Some(args_struct::SubCommand::Number(number_data)) => {
            number::number_command(number_data.folder_name, number_data.number)
        }
        None => println!("No subcommand given"),
    }
}
