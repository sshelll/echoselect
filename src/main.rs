use clap::Parser;
use dialoguer::Select;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    default: Option<String>,

    #[clap(name = "INPUT")]
    input: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let selection = Select::new()
        .default(0)
        .items(&args.input)
        .interact_opt()
        .unwrap();
    match selection {
        Some(selection) => println!("{}", args.input[selection]),
        None if args.default.is_some() => println!("{}", args.default.unwrap()),
        _ => (),
    }
}
