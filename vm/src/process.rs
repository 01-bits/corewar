use common::constants::REG_NUMBER;

#[derive(Clone)]
pub struct Process {
    pub id: usize,
    pub pc: usize,
    pub registers: [i32; REG_NUMBER as usize],
    pub carry: bool,
    pub wait_cycles: u32,
    pub current_op: Option<u8>,
    pub last_live_cycle: u64,
}
