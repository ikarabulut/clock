use chrono::{Local, DateTime};
use clap::{Arg, Command};
use clap::builder::PossibleValuesParser;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        unimplemented!()
    }
}

fn main() {
    let command = Command::new("clock")
        .about("Gets and sets the time")
        .arg(
            Arg::new("action")
                .long("action")
                .value_parser(PossibleValuesParser::new(["get", "set"]))
                .default_value("get")
        )
        .arg(
            Arg::new("std")
                .short('s')
                .long("standard")
                .value_parser(PossibleValuesParser::new(["rfc2822", "rfc3339", "timestamp"]))
                .default_value("rfc3339")
        )
        .arg(
            Arg::new("datetime")
                .help("When <action> is 'set', apply <datetime>. \
                Otherwise, ignore.")
        );

    let args = command.get_matches();

    let action = args.get_one::<String>("action").unwrap();
    let std = args.get_one::<String>("std").unwrap();

    if action == "set" {
        unimplemented!()
    }

    let now = Clock::get();
    match std.as_str() {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
