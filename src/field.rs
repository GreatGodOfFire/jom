use binrw::binrw;

use crate::{
    constant_pool::ConstantPool,
    error::JomResult, attribute::{RawAttribute, FieldAttribute},
};

#[binrw]
pub(crate) struct RawFieldInfo {
    access_flags: u16,
    name: u16,
    descriptor: u16,
    #[br(temp)]
    #[bw(calc = attributes.len() as u16)]
    attributes_count: u16,
    #[br(count = attributes_count)]
    attributes: Vec<RawAttribute>,
}

impl RawFieldInfo {
    pub fn into_field_info(self, cp: &ConstantPool) -> JomResult<FieldInfo> {
        let name = cp.get_utf8(self.name)?;
        let descriptor = cp.get_utf8(self.descriptor)?;

        Ok(FieldInfo {
            access_flags: self.access_flags,
            name,
            descriptor,
            attributes: self
                .attributes
                .into_iter()
                .map(|x| x.into_field_attr(cp))
                .collect::<JomResult<Vec<_>>>()?,
        })
    }
}

pub struct FieldInfo {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<FieldAttribute>,
}
