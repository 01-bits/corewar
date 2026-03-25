// use crate::error::fatal!;
use crate::fatal;
use common::constants::MAX_PLAYERS;
use std::env;
use std::path::PathBuf;
use std::process;

pub fn parse_args() -> (Option<u64>, Vec<PathBuf>) {
    let mut args = env::args().peekable();
    let program = args.next().unwrap_or_else(|| "corewar".to_string());
    if args.peek().is_none() {
        usage(&program);
        process::exit(0);
    }

    let mut dump_cycle = None;
    let mut champions = Vec::new();
    while let Some(arg) = args.next() {
        if arg == "-d" {
            if dump_cycle.is_none()
                && let Some(value) = args.next()
            {
                dump_cycle = Some(value.parse::<u64>().unwrap_or_else(|_| {
                    fatal!("error: NB_CYCLES must be a non-negative integer");
                }));
            } else {
                fatal!("error: duplicate -d flag OR missing argments");
            }
        } else if arg.ends_with(".cor") {
            champions.push(PathBuf::from(arg));
        } else if arg == "--help" || arg == "-h" {
            usage(&program);
            process::exit(0);
        } else {
            fatal!("error: unexpected argument '{arg}'");
        }
    }

    if champions.is_empty() {
        usage(&program);
        process::exit(0);
    }
    if champions.len() > MAX_PLAYERS {
        fatal!("error: too many players (max {MAX_PLAYERS})");
    }
    (dump_cycle, champions)
}

fn usage(program: &str) {
    println!("Usage: {program} [-d NB_CYCLES] champion1.cor [champion2.cor ... champion4.cor]");
}
