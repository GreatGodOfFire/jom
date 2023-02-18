use std::io::Cursor;

use binrw::{binrw, BinRead};

use crate::{
    constant_pool::{ConstantPool, ConstantPoolIndex},
    error::{JomError, JomResult},
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
    attributes: Vec<RawFieldAttribute>,
}

impl RawFieldInfo {
    pub fn into_field_info(self, cp: &ConstantPool) -> JomResult<FieldInfo> {
        let name = cp.read_utf8(self.name)?;
        let descriptor = cp.read_utf8(self.descriptor)?;

        Ok(FieldInfo {
            access_flags: self.access_flags,
            name,
            descriptor,
            attributes: self
                .attributes
                .into_iter()
                .map(|x| x.into_attr(cp))
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

impl FieldInfo {
    pub(crate) fn to_raw(&self, cp: &ConstantPool) -> JomResult<RawFieldInfo> {
        let name = cp.find_utf8(self.name.clone())?;
        let descriptor = cp.find_utf8(self.descriptor.clone())?;

        Ok(RawFieldInfo {
            access_flags: self.access_flags,
            name,
            descriptor,
            attributes: self
                .attributes
                .iter()
                .map(|x| x.to_raw(cp))
                .collect::<JomResult<Vec<_>>>()?,
        })
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
    pub fn into_attr(self, cp: &ConstantPool) -> JomResult<FieldAttribute> {
        let name = cp.read_utf8(self.name)?;

        match name.as_str() {
            "ConstantValue" => {
                let value_idx = <u16 as BinRead>::read_be(&mut Cursor::new(self.info))?;
                let value = cp.read(value_idx)?;

                Ok(FieldAttribute::ConstantValue(ConstantValue::from_cp_index(
                    value,
                )?))
            }
            "Synthetic" => Ok(FieldAttribute::Synthetic),
            "Deprecated" => Ok(FieldAttribute::Deprecated),
            "Signature" => {
                let value_idx = <u16 as BinRead>::read_be(&mut Cursor::new(self.info))?;
                let value = cp.read_utf8(value_idx)?;

                Ok(FieldAttribute::Signature(value))
            }
            _ => Ok(FieldAttribute::Unknown(name, self.info)),
        }
    }
}

pub enum FieldAttribute {
    ConstantValue(ConstantValue),
    Synthetic,
    Deprecated,
    Signature(String),
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    Unknown(String, Vec<u8>),
}

pub enum ConstantValue {
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
}

impl ConstantValue {
    fn from_cp_index(cp_index: ConstantPoolIndex) -> JomResult<Self> {
        match cp_index {
            ConstantPoolIndex::Integer(i) => Ok(Self::Integer(i)),
            ConstantPoolIndex::Float(f) => Ok(Self::Float(f)),
            ConstantPoolIndex::Long(l) => Ok(Self::Long(l)),
            ConstantPoolIndex::Double(d) => Ok(Self::Double(d)),
            ConstantPoolIndex::String(s) => Ok(Self::String(s)),
            x => Err(JomError::ConstantPoolIndexError(
                "Integer, Float, Long, Double or String",
                x.name(),
            )),
        }
    }

    fn to_cp_index(&self) -> ConstantPoolIndex {
        match self {
            ConstantValue::Integer(i) => ConstantPoolIndex::Integer(*i),
            ConstantValue::Float(f) => ConstantPoolIndex::Float(*f),
            ConstantValue::Long(l) => ConstantPoolIndex::Long(*l),
            ConstantValue::Double(d) => ConstantPoolIndex::Double(*d),
            ConstantValue::String(s) => ConstantPoolIndex::String(s.clone()),
        }
    }
}

impl FieldAttribute {
    pub(crate) fn to_raw(&self, cp: &ConstantPool) -> JomResult<RawFieldAttribute> {
        match self {
            FieldAttribute::ConstantValue(v) => Ok(RawFieldAttribute {
                name: cp.find_utf8("ConstantValue".to_owned())?,
                info: cp.find(v.to_cp_index())?.to_be_bytes().to_vec(),
            }),
            FieldAttribute::Synthetic => Ok(RawFieldAttribute {
                name: cp.find_utf8("Synthetic".to_owned())?,
                info: vec![],
            }),
            FieldAttribute::Deprecated => Ok(RawFieldAttribute {
                name: cp.find_utf8("Deprecated".to_owned())?,
                info: vec![],
            }),
            FieldAttribute::Signature(s) => Ok(RawFieldAttribute {
                name: cp.find_utf8("Signature".to_owned())?,
                info: cp.find_utf8(s.clone())?.to_be_bytes().to_vec(),
            }),
            FieldAttribute::RuntimeVisibleAnnotations => todo!(),
            FieldAttribute::RuntimeInvisibleAnnotations => todo!(),
            FieldAttribute::RuntimeVisibleTypeAnnotations => todo!(),
            FieldAttribute::RuntimeInvisibleTypeAnnotations => todo!(),
            FieldAttribute::Unknown(name, info) => Ok(RawFieldAttribute {
                name: cp.find_utf8(name.clone())?,
                info: info.clone(),
            }),
        }
    }
}
