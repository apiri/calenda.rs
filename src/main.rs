use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    List,
    Add,
    Remove,
}

fn main() -> Result<(), ()> {
    let args = Arguments::parse();
    match args.action {
        Action::List => {
            println!("Listing entries")
        }
        Action::Add => {
            todo!()
        }
        Action::Remove => {
            todo!()
        }
    }
    Ok(())
}
