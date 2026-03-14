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

/// Represents the type of Corewar operation as an enum.
///
/// Each variant corresponds to a specific Corewar instruction, with discriminant
/// values matching their opcode values (1-16).
use crate::types::ParamKind;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpType {
    Live = 1,
    Ld,
    St,
    Add,
    Sub,
    And,
    Or,
    Xor,
    Zjmp,
    Ldi,
    Sti,
    Fork,
    Lld,
    Lldi,
    Lfork,
    Nop,
}

impl OpType {
    /// Returns the string representation of the operation type.
    pub fn as_str(&self) -> &'static str {
        match self {
            OpType::Live => "live",
            OpType::Ld => "ld",
            OpType::St => "st",
            OpType::Add => "add",
            OpType::Sub => "sub",
            OpType::And => "and",
            OpType::Or => "or",
            OpType::Xor => "xor",
            OpType::Zjmp => "zjmp",
            OpType::Ldi => "ldi",
            OpType::Sti => "sti",
            OpType::Fork => "fork",
            OpType::Lld => "lld",
            OpType::Lldi => "lldi",
            OpType::Lfork => "lfork",
            OpType::Nop => "nop",
        }
    }

    /// Returns the opcode byte value for this operation type.
    pub fn opcode(&self) -> u8 {
        *self as u8
    }

    pub fn allowed_param_kinds(&self) -> &'static [&'static [ParamKind]] {
        const R: &[ParamKind] = &[ParamKind::Register];
        const D: &[ParamKind] = &[ParamKind::Direct];
        const ID: &[ParamKind] = &[ParamKind::Indirect, ParamKind::Direct];
        const RID: &[ParamKind] = &[ParamKind::Register, ParamKind::Indirect, ParamKind::Direct];
        const RD: &[ParamKind] = &[ParamKind::Register, ParamKind::Direct];
        const RI: &[ParamKind] = &[ParamKind::Register, ParamKind::Indirect];

        match self {
            OpType::Live => &[D],
            OpType::Ld => &[ID, R],
            OpType::St => &[R, RI],
            OpType::Add | OpType::Sub => &[R, R, R],
            OpType::And | OpType::Or | OpType::Xor => &[RID, RID, R],
            OpType::Zjmp => &[D],
            OpType::Ldi => &[RID, RD, R],
            OpType::Sti => &[R, RID, RD],
            OpType::Fork | OpType::Lfork => &[D],
            OpType::Lld => &[ID, R],
            OpType::Lldi => &[RID, RD, R],
            OpType::Nop => &[R],
        }
    }
}

/// Represents a single Corewar operation (opcode) with all its properties.
///
/// Each operation in Corewar has specific characteristics that determine how
/// it behaves in the virtual machine, including its execution cost, parameter
/// requirements, and addressing modes.
pub struct Op {
    /// The type of operation (enum variant)
    pub op_type: OpType,
    /// Number of parameters this operation takes (1-3)
    pub nb_params: usize,
    /// Number of execution cycles required to complete this operation
    pub cycles: u32,
    /// Whether this operation uses a parameter coding byte to describe parameter types
    pub has_pcode: bool,
    /// Whether this operation supports indexed addressing mode
    pub has_idx: bool,
}

impl Op {
    pub fn param_size(&self, kind: ParamKind) -> usize {
        match kind {
            ParamKind::Register => 1,
            ParamKind::Indirect => 2,
            ParamKind::Direct => {
                if self.has_idx {
                    2
                } else {
                    4
                }
            }
        }
    }
}

/// A constant array containing all 16 standard Corewar operations.
///
/// This table defines the complete instruction set for the Corewar virtual machine.
/// Each operation is indexed by its opcode value (0x01 = index 0, 0x02 = index 1, etc.).
pub const OP_TABLE: [Op; 16] = [
    Op {
        op_type: OpType::Live,
        nb_params: 1,
        cycles: 10,
        has_pcode: false,
        has_idx: false,
    },
    Op {
        op_type: OpType::Ld,
        nb_params: 2,
        cycles: 5,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        op_type: OpType::St,
        nb_params: 2,
        cycles: 5,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        op_type: OpType::Add,
        nb_params: 3,
        cycles: 10,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        op_type: OpType::Sub,
        nb_params: 3,
        cycles: 10,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        op_type: OpType::And,
        nb_params: 3,
        cycles: 6,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        op_type: OpType::Or,
        nb_params: 3,
        cycles: 6,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        op_type: OpType::Xor,
        nb_params: 3,
        cycles: 6,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        op_type: OpType::Zjmp,
        nb_params: 1,
        cycles: 20,
        has_pcode: false,
        has_idx: true,
    },
    Op {
        op_type: OpType::Ldi,
        nb_params: 3,
        cycles: 25,
        has_pcode: true,
        has_idx: true,
    },
    Op {
        op_type: OpType::Sti,
        nb_params: 3,
        cycles: 25,
        has_pcode: true,
        has_idx: true,
    },
    Op {
        op_type: OpType::Fork,
        nb_params: 1,
        cycles: 800,
        has_pcode: false,
        has_idx: true,
    },
    Op {
        op_type: OpType::Lld,
        nb_params: 2,
        cycles: 10,
        has_pcode: true,
        has_idx: false,
    },
    Op {
        op_type: OpType::Lldi,
        nb_params: 3,
        cycles: 50,
        has_pcode: true,
        has_idx: true,
    },
    Op {
        op_type: OpType::Lfork,
        nb_params: 1,
        cycles: 1000,
        has_pcode: false,
        has_idx: true,
    },
    Op {
        op_type: OpType::Nop,
        nb_params: 1,
        cycles: 2,
        has_pcode: true,
        has_idx: false,
    },
];

pub fn op_by_opcode(opcode: u8) -> Option<&'static Op> {
    if (1..=OP_TABLE.len() as u8).contains(&opcode) {
        Some(&OP_TABLE[(opcode - 1) as usize])
    } else {
        None
    }
}

