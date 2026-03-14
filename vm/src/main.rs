use vm::cli::parse_args;
//use vm::loader::load_players;
//use vm::vm::Vm;

fn main() {
    let (dump_cycle, champion_paths) = parse_args();
       println!("{:?}, {:?}",dump_cycle,champion_paths);
    // let players = loa_dplayers(&champion_paths);
    // let mut vm = Vm::new(players, dump_cycle);
    // vm.run();
}
