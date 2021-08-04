#![allow(dead_code)]

mod bits;
mod error;
mod exec;
mod traits;

pub mod decode;
pub mod mcu;

pub use error::RiscvError;
pub use exec::exec_one;
pub use traits::{Memory, RegisterFile};

pub static REG_NAMES: &[&str] = &[
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];
