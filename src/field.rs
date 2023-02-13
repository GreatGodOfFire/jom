use binrw::{binrw, BinResult};

use crate::{constant_pool::ConstantPool, e};

#[binrw]
pub(crate) struct RawFieldInfo {
    access_flags: u16,
    name: u16,
    descriptor: u16,
    #[br(temp)]
    #[bw(calc = attributes.len() as u16)]
    attributes_count: u16,
    #[br(count = attributes_count)]
    attributes: Vec<RawFieldAttribute>
}

impl RawFieldInfo {
    pub fn to_field_info(self, cp: &ConstantPool) -> BinResult<FieldInfo> {
        let name = e!(cp.read_utf8(self.name), i)?;
        let descriptor = e!(cp.read_utf8(self.descriptor), i)?;

        Ok(FieldInfo {
            access_flags: self.access_flags,
            name,
            descriptor,
            attributes: self.attributes.into_iter().map(|x| x.to_attr(cp)).collect::<BinResult<Vec<_>>>()?,
        })
    }
}

pub struct FieldInfo {
    access_flags: u16,
    name: String,
    descriptor: String,
    attributes: Vec<FieldAttribute>,
}

impl FieldInfo {
    pub(crate) fn to_raw(&self, cp: &ConstantPool) -> BinResult<RawFieldInfo> {
        todo!()
    }
}

#[binrw]
pub(crate) struct RawFieldAttribute {
    name: u16,
    #[br(temp)]
    #[bw(calc = info.len() as u32)]
    info_length: u32,
    #[br(count = info_length)]
    info: Vec<u8>,
}

impl RawFieldAttribute {
    pub fn to_attr(self, cp: &ConstantPool) -> BinResult<FieldAttribute> {
        todo!()
    }
}

pub enum FieldAttribute {
}

impl FieldAttribute {
    pub(crate) fn to_raw(&self, cp: &ConstantPool) -> BinResult<RawFieldAttribute> {
        todo!()
    }
}
