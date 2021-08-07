#![allow(dead_code)]

/// Make assertions about an MCU>
mod assertions;
/// Useful bitwise operations.
mod bits;
/// Enumeration for errors thrown by an MCU.
mod error;
/// Execution and decoding logic.
mod exec;
/// Generic utility functions.
mod util;

/// Collection of RISC-V constants.
pub mod constants;
/// Decoding macros.
pub mod decode;

/// Encoding macros.
pub mod encode;

/// Reference implementation of an MCU.
pub mod mcu;

///
pub mod assembler;

/// Traits to be implementation by other implementations of
/// an MCU.
pub mod traits;

pub use assertions::Assertions;
pub use error::{AssemblerError, RiscvError};
pub use exec::exec_one;
