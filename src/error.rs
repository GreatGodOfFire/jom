use binrw::Error as BinError;
use thiserror::Error;

pub(crate) type JomResult<T> = Result<T, JomError>;

#[derive(Error, Debug)]
pub enum JomError {
    #[error("{0}")]
    BinError(#[from] BinError),
    #[error("constant pool error: expected {0}, found {0}")]
    ConstantPoolIndexError(&'static str, &'static str),
    #[error("value {0} not in constant pool")]
    ValueNotInConstantPool(String),
    #[error("constant pool index {0} is out of bounds")]
    OutOfBounds(u16),
}

impl JomError {
    pub(crate) fn new_cp_index(expected: &'static str, found: &'static str) -> Self {
        Self::ConstantPoolIndexError(expected, found)
    }

    pub(crate) fn not_in_cp(value: String) -> Self {
        Self::ValueNotInConstantPool(value)
    }

    pub(crate) fn out_of_bounds(index: u16) -> Self {
        Self::OutOfBounds(index)
    }
}
