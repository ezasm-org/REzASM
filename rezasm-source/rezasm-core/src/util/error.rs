use std::num::{ParseFloatError, ParseIntError};
use std::process;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EzasmError {
    #[error("{0}")]
    ParserError(ParserError),

    #[error("{0}")]
    SimulatorError(SimulatorError),

    #[error("internal error: {0}")]
    InternalError(InternalError),

    #[error("invalid given memory size `{0}`")]
    InvalidMemorySizeError(usize),

    #[error("invalid word size `{0}`")]
    InvalidWordSizeError(usize),

    #[error("could not open file `{0}`")]
    CouldNotOpenFileError(String),

    #[error("path `{0}` is not a file")]
    PathIsNotFileError(String),

    #[error("file `{0}` does not exist")]
    FileDoesNotExistError(String),

    #[error("action timed out")]
    TimeoutError(),
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("internal error: {0}")]
    InternalError(InternalError),

    #[error("invalid given instruction `{0}`")]
    InvalidInstructionError(String),

    #[error("instruction `{0}` cannot accept argument `{1}` at index {2}")]
    InvalidArgumentsError(String, String, usize),

    #[error("instruction `{0}` does not accept {1} arguments")]
    InvalidArgumentsCountError(String, usize),

    #[error("invalid register number `{0}`")]
    InvalidRegisterNumberError(usize),

    #[error("invalid register name `{0}`")]
    InvalidRegisterNameError(String),

    #[error("invalid character immediate `{0}`")]
    CharacterImmediateError(String),

    #[error("invalid dereference `{0}`")]
    DereferenceError(String),

    #[error("invalid label definition `{0}`")]
    LabelDefinitionError(String),

    #[error("invalid numeric immediate `{0}`")]
    NumericalImmediateError(String),

    #[error("invalid string immediate `{0}`")]
    StringImmediateError(String),

    #[error("unrecognized register `{0}`")]
    UnknownRegisterError(String),

    #[error("unrecognized token `{0}`")]
    UnknownTokenError(String),
}

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("improper usage of try_into")]
    MismatchedTryIntoError,

    #[error("improper usage of get_input_output_target")]
    GetInputOutputTargetError,
}

#[derive(Error, Debug)]
pub enum SimulatorError {
    #[error("{0}")]
    ParserError(ParserError),

    #[error("internal error: {0}")]
    InternalError(InternalError),

    #[error("attempted read to address `{0}` which is negative")]
    ReadNegativeAddressError(i64),

    #[error("attempted read to address `{0}` outside of memory")]
    ReadOutOfBoundsError(usize),

    #[error("attempted write to address `{0}` which is negative")]
    WriteNegativeAddressError(i64),

    #[error("attempted write to address `{0}` outside of memory")]
    WriteOutOfBoundsError(usize),

    #[error("attempted write to address `{0}` in read-only memory")]
    WriteToReadOnlyError(usize),

    #[error("invalid heap pointer `{0}`")]
    InvalidHeapPointerError(usize),

    #[error("invalid program counter `{0}`")]
    InvalidProgramCounterError(i64),

    #[error("invalid line number `{0}`")]
    InvalidLineNumber(i64),

    #[error("invalid file identifier `{0}`")]
    InvalidFileIdentifier(i64),

    #[error("string immediate `{0}` does not exist")]
    StringImmediateDoesNotExistError(String),

    #[error("string immediate `{0}` could not be allocated because there is not enough memory in the string region")]
    StringRegionOutOfMemoryError(String),

    #[error("label `{0}` does not exist")]
    NonExistentLabelError(String),

    #[error("label `{0}` is already in use")]
    LabelInUseError(String),

    #[error("attempted to divide by zero")]
    DivideByZeroError,
}

impl From<InternalError> for EzasmError {
    fn from(error: InternalError) -> Self {
        EzasmError::InternalError(error)
    }
}

impl From<ParserError> for EzasmError {
    fn from(error: ParserError) -> Self {
        EzasmError::ParserError(error)
    }
}

impl From<SimulatorError> for EzasmError {
    fn from(error: SimulatorError) -> Self {
        EzasmError::SimulatorError(error)
    }
}

impl From<ParserError> for SimulatorError {
    fn from(error: ParserError) -> Self {
        SimulatorError::ParserError(error)
    }
}

impl From<InternalError> for ParserError {
    fn from(error: InternalError) -> Self {
        ParserError::InternalError(error)
    }
}

impl From<InternalError> for SimulatorError {
    fn from(error: InternalError) -> Self {
        SimulatorError::InternalError(error)
    }
}

impl From<ParseFloatError> for ParserError {
    fn from(error: ParseFloatError) -> Self {
        ParserError::NumericalImmediateError(error.to_string())
    }
}

impl From<ParseIntError> for ParserError {
    fn from(error: ParseIntError) -> Self {
        ParserError::NumericalImmediateError(error.to_string())
    }
}

pub fn handle_error(error: EzasmError) -> ! {
    println!("{}", error);
    process::exit(1);
}
