use std::io::Cursor;

use binrw::{binrw, BinRead};

use crate::{
    attribute::{CodeAttribute, RawAttribute},
    constant_pool::ConstantPool,
    error::JomResult,
};

#[binrw]
pub(super) struct RawCode {
    max_stack: u16,
    max_locals: u16,
    #[br(temp)]
    #[bw(calc = code.len() as u32)]
    code_count: u32,
    #[br(count = code_count)]
    code: Vec<u8>,
    #[br(temp)]
    #[bw(calc = exception_table.len() as u16)]
    exception_table_count: u16,
    #[br(count = exception_table_count)]
    exception_table: Vec<RawException>,
    #[br(temp)]
    #[bw(calc = attributes.len() as u16)]
    attributes_count: u16,
    #[br(count = attributes_count)]
    attributes: Vec<RawAttribute>,
}

#[binrw]
pub(super) struct RawException {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl RawException {
    fn into_exception(self, constant_pool: &ConstantPool) -> JomResult<Exception> {
        let RawException {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        } = self;

        let catch_type = constant_pool.read_class(catch_type)?;

        Ok(Exception {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }
}

pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<Exception>,
    pub attributes: Vec<CodeAttribute>,
}

impl Code {
    pub(crate) fn read(info: &[u8], constant_pool: &ConstantPool) -> JomResult<Self> {
        let RawCode {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        } = RawCode::read_be(&mut Cursor::new(info))?;

        let exception_table = exception_table
            .into_iter()
            .map(|x| x.into_exception(constant_pool))
            .collect::<JomResult<Vec<_>>>()?;
        let attributes = attributes
            .into_iter()
            .map(|x| x.into_code_attr(constant_pool))
            .collect::<JomResult<Vec<_>>>()?;

        Ok(Self {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }
}

pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: String,
}
