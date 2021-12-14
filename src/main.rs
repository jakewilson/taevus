extern crate clap;

use clap::{App, Arg, ArgMatches};

use tempus_cli::utils;

fn main() {
    let matches = parse_args();

    // this arg is required, so it's safe to unwrap
    let project = matches.value_of("project").unwrap();

    let date_range = match matches.value_of("date-range") {
        Some(range) => match utils::parse_date_range(&range) {
            Ok(date_range) => Some(date_range),
            Err(err) => panic!("{}", err),
        },
        None => None,
    };

    if matches.is_present("hours") {
        tempus_cli::print_total_log_time(&project, &date_range);
    } else if matches.is_present("start") {
        tempus_cli::print_session_start(&project);
    } else if matches.is_present("times") {
        tempus_cli::print_times(&project, &date_range);
    } else if matches.is_present("delete") {
        tempus_cli::delete_session(&project);
    } else {
        tempus_cli::do_session(&project);
    }
}

fn parse_args() -> ArgMatches<'static> {
    App::new("Tempus")
        .version("1.1.0")
        .author("Jake Wilson")
        .about("Easy time tracking")
        .arg(
            Arg::with_name("project")
                .short("p")
                .long("project")
                .value_name("project")
                .takes_value(true)
                .required(true)
                .help("project name"),
        )
        .arg(
            Arg::with_name("hours")
                .long("hours")
                .help("Calculates hours worked for a project"),
        )
        .arg(
            Arg::with_name("start")
                .short("s")
                .long("start")
                .help("Prints current session start time"),
        )
        .arg(
            Arg::with_name("date-range")
                .short("d")
                .long("date-range")
                .value_name("date-range")
                .takes_value(true)
                .help("Inclusive date range filter"),
        )
        .arg(
            Arg::with_name("times")
                .long("times")
                .help("Prints session times for a project"),
        )
        .arg(
            Arg::with_name("delete")
                .short("x")
                .long("delete")
                .help("Ends the session without recording it"),
        )
        .get_matches()
}
