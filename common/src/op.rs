//! [ 01-bits ]
//!   
//!   ███╗   ██╗      ██████╗ ██╗████████╗███████╗
//!   ████╗  ██║      ██╔══██╗██║╚══██╔══╝██╔════╝
//!   ██╔██╗ ██║█████╗██████╔╝██║   ██║   ███████╗
//!   ██║╚██╗██║╚════╝██╔══██╗██║   ██║   ╚════██║
//!   ██║ ╚████║      ██████╔╝██║   ██║   ███████║
//!   ╚═╝  ╚═══╝      ╚═════╝ ╚═╝   ╚═╝   ╚══════╝
//! 
//! 
//! This module defines the operations (opcodes) used in the Corewar virtual machine.
//!
//! Corewar is a programming game where programs (called "warriors") compete in a virtual
//! memory arena. Each operation represents a machine instruction that can be executed
//! by the virtual machine.
//!
//! The `Op` struct represents a single operation with its properties:
//! - `name`: The mnemonic name of the operation
//! - `opcode`: The unique byte code identifying the operation
//! - `nb_params`: Number of parameters the operation takes
//! - `cycles`: Number of execution cycles required
//! - `has_pcode`: Whether the operation uses parameter coding byte
//! - `has_idx`: Whether the operation supports indexed addressing
//!
//! The `OP_TABLE` constant array contains all 16 standard Corewar operations.

pub struct Op {
    pub name: &'static str,
    pub opcode: u8,
    pub nb_params: usize,
    pub cycles: u32,
    pub has_pcode: bool,
    pub has_idx: bool,
}

pub const OP_TABLE: [Op; 16] = [
    Op {
        name: "live",
        opcode: 0x01,
        nb_params: 1,
        cycles: 10,
        has_pcode: false,
        has_idx: false,
    },
    Op {
        name: "ld",
        opcode: 0x02,
        nb_params: 2,
        cycles: 5,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        name: "st",
        opcode: 0x03,
        nb_params: 2,
        cycles: 5,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        name: "add",
        opcode: 0x04,
        nb_params: 3,
        cycles: 10,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        name: "sub",
        opcode: 0x05,
        nb_params: 3,
        cycles: 10,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        name: "and",
        opcode: 0x06,
        nb_params: 3,
        cycles: 6,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        name: "or",
        opcode: 0x07,
        nb_params: 3,
        cycles: 6,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        name: "xor",
        opcode: 0x08,
        nb_params: 3,
        cycles: 6,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        name: "zjmp",
        opcode: 0x09,
        nb_params: 1,
        cycles: 20,
        has_pcode: false,
        has_idx: true,
    },
    Op {
        name: "ldi",
        opcode: 0x0a,
        nb_params: 3,
        cycles: 25,
        has_pcode: true,
        has_idx: true,
    },
    Op {
        name: "sti",
        opcode: 0x0b,
        nb_params: 3,
        cycles: 25,
        has_pcode: true,
        has_idx: true,
    },
    Op {
        name: "fork",
        opcode: 0x0c,
        nb_params: 1,
        cycles: 800,
        has_pcode: false,
        has_idx: true,
    },
    Op {
        name: "lld",
        opcode: 0x0d,
        nb_params: 2,
        cycles: 10,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        name: "lldi",
        opcode: 0x0e,
        nb_params: 3,
        cycles: 50,
        has_pcode: true,
        has_idx: true,
    },
    Op {
        name: "lfork",
        opcode: 0x0f,
        nb_params: 1,
        cycles: 1000,
        has_pcode: false,
        has_idx: true,
    },
    Op {
        name: "nop",
        opcode: 0x10,
        nb_params: 1,
        cycles: 2,
        has_pcode: true,
        has_idx: false,
    },
];
