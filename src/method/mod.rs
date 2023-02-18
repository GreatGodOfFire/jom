pub mod code;

use binrw::binrw;

use crate::{
    attribute::{MethodAttribute, RawAttribute},
    constant_pool::ConstantPool,
    error::JomResult,
};

#[binrw]
pub(crate) struct RawMethodInfo {
    access_flags: u16,
    name: u16,
    descriptor: u16,
    #[br(temp)]
    #[bw(calc = attributes.len() as u16)]
    attributes_count: u16,
    #[br(count = attributes_count)]
    attributes: Vec<RawAttribute>,
}

impl RawMethodInfo {
    pub fn into_method_info(self, cp: &ConstantPool) -> JomResult<MethodInfo> {
        let name = cp.get_utf8(self.name)?;
        let descriptor = cp.get_utf8(self.descriptor)?;

        Ok(MethodInfo {
            access_flags: self.access_flags,
            name,
            descriptor,
            attributes: self
                .attributes
                .into_iter()
                .map(|x| x.into_method_attr(cp))
                .collect::<JomResult<Vec<_>>>()?,
        })
    }
}

pub struct MethodInfo {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<MethodAttribute>,
}
