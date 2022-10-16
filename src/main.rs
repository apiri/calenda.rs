use calendars::entry::Entry;
use clap::Parser;
use std::ops::Sub;
use time::macros::datetime;

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
            let mut entries = Vec::<Entry>::new();

            entries.push(Entry::new(
                datetime!(2022 - 10 - 14 13:00),
                datetime!(2022 - 10 - 14 15:00),
            ));
            entries.push(Entry::new(
                datetime!(2022 - 10 - 16 0:00),
                datetime!(2022 - 10 - 18 13:00),
            ));

            let mut previous: Option<&Entry> = None;

            for entry in entries.iter() {
                println!(
                    "{:?} with duration {} minutes",
                    entry,
                    entry.duration_mins()
                );
                if previous.is_some() {
                    println!("Duration since previous is {:?} minutes", {
                        entry.start().sub(*previous.unwrap().end()).whole_minutes()
                    })
                }
                previous = Some(entry)
            }
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
