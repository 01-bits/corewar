use crate::error::fatal;
use common::constants::MAX_PLAYERS;
use std::env;
use std::path::PathBuf;
use std::process;

pub fn parse_args() -> (Option<u64>, Vec<PathBuf>) {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "corewar".to_string());
    let raw: Vec<String> = args.collect();
    if raw.is_empty() {
        usage(&program);
        process::exit(0);
    }

    let mut dump_cycle = None;
    let mut champions = Vec::new();
    let mut index = 0;
    while index < raw.len() {
        match raw[index].as_str() {
            "-d" => {
                if dump_cycle.is_some() {
                    fatal("error: duplicate -d flag");
                }
                let Some(value) = raw.get(index + 1) else {
                     fatal("error: missing value for -d");
                };
                dump_cycle = Some(
                    value
                        .parse::<u64>()
                        .unwrap_or_else(|_| fatal("error: NB_CYCLES must be a non-negative integer")),
                );
                index += 2;
            }
            other if other.ends_with(".cor") => {
                champions.push(PathBuf::from(other));
                index += 1;
            }
            other => fatal(format!("error: unexpected argument '{other}'")),
        }
    }

    if champions.is_empty() {
        usage(&program);
        process::exit(0);
    }
    if champions.len() > MAX_PLAYERS {
        fatal(format!("error: too many players (max {MAX_PLAYERS})"));
    }
    (dump_cycle, champions)
}

fn usage(program: &str) {
    println!("Usage: {program} [-d NB_CYCLES] champion1.cor [champion2.cor ... champion4.cor]");
}
