use clap::{App, Arg, SubCommand};
use rand;
use std::error::Error;

/// If we can't parse out the user input, exit with a nice error
fn extract_to_usize_or_exit(input: Option<&str>) -> usize {
    let input = match input {
        Some(token) => token,
        None => {
            eprintln!("No value provided for positional args");
            std::process::exit(1);
        }
    };
    match input.parse() {
        Ok(number) => number,
        Err(error) => {
            eprintln!("Could not parse input \"{}\" to a number", input);
            std::process::exit(1);
        }
    }
}

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
                    Arg::with_name("end")
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
            // pull out variables
            let act = extract_to_usize_or_exit(arg_matches.value_of("act"));
            let scene = extract_to_usize_or_exit(arg_matches.value_of("scene"));
            let start = extract_to_usize_or_exit(arg_matches.value_of("start"));
            let end = extract_to_usize_or_exit(arg_matches.value_of("end"));
            // basic bounds checking
            if end < start {
                eprintln!("Line selection is invalid");
                std::process::exit(1);
            }
            let selection = lear::text(act, scene, start..=end).unwrap_or_else(|err| {
                match err {
                    lear::LearError::IoError(e) => {
                        eprintln!("Internal deserialization error {:?}", e);
                    }
                    lear::LearError::InvalidAct(act) => {
                        eprintln!("Act {} is not present", act);
                    }
                    lear::LearError::InvalidScene { act, scene } => {
                        eprintln!("Act {}, scene {} is not present", act, scene)
                    }
                    lear::LearError::InvalidLines { act, scene, lines } => {
                        eprintln!(
                            "Lines {}-{} are not present in act {}, scene {}",
                            lines.start(),
                            lines.end(),
                            act,
                            scene
                        );
                    }
                };
                std::process::exit(1);
            });
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
