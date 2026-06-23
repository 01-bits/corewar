// use crate::error::fatal!;
use crate::fatal;
use common::constants::MAX_PLAYERS;
use std::env::args;
use std::path::PathBuf;
use std::process;

pub fn parse_args() -> (Option<u64>, Vec<PathBuf>) {
    let mut args = args();
    let program = args.next().unwrap();
    let mut nb_cycles = None;
    let mut players: Vec<PathBuf> = vec![];
    while let Some(arg) = args.next() {
        if arg == "-d" {
            if let Some(nb_string) = args.next()
                && let Ok(nb) = nb_string.parse::<u64>()
            {
                nb_cycles = Some(nb);
            }
        } else if arg.ends_with(".cor") {
            players.push(PathBuf::from(arg));
        } else {
            fatal!("Error: this file dosn't end with .cor != {}", arg);
        }
    }

    if players.len() == 0 || players.len() > MAX_PLAYERS {
        usage(&program);
    }

    (nb_cycles, players)
}

fn usage(program: &str) {
    fatal!("Usage: {program} [-d NB_CYCLES] champion1.cor [champion2.cor ... champion4.cor]");
}
