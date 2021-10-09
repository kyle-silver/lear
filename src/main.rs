use clap::{App, Arg, SubCommand};
use rand;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Lear")
        .version("0.2.0")
        .author("Kyle Silver")
        .about("Prints a random passage from King Lear")
        .subcommand(
            SubCommand::with_name("quote")
                .about("Print a specific selection from the text")
                .arg(
                    Arg::with_name("act")
                        .help("The act of the play")
                        .required(true),
                )
                .arg(
                    Arg::with_name("scene")
                        .help("The scene number within the act")
                        .required(true),
                )
                .arg(
                    Arg::with_name("start")
                        .help("The first line of the excerpt")
                        .required(true),
                )
                .arg(
                    Arg::with_name("stop")
                        .help("The last line of the excerpt")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("contents").about("Show a table of contents for the play"),
        )
        .get_matches();
    match matches.subcommand() {
        ("contents", Some(_)) => {
            lear::display_contents();
        }
        ("quote", Some(arg_matches)) => {
            let selection = lear::text(5, 4, 2..=1000).unwrap();
            lear::display(&selection);
        }
        _ => {
            let mut rng = rand::thread_rng();
            let blocks = lear::blocks_to_show(&mut rng)?;
            lear::display(&blocks);
        }
    }
    Ok(())
}
