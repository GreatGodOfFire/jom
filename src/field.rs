use std::io::Cursor;

use binrw::{binrw, BinRead, BinResult};

use crate::{
    constant_pool::{ConstantPool, ConstantPoolIndex},
    e,
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
    pub fn into_field_info(self, cp: &ConstantPool) -> BinResult<FieldInfo> {
        let name = e!(cp.read_utf8(self.name), i)?;
        let descriptor = e!(cp.read_utf8(self.descriptor), i)?;

        Ok(FieldInfo {
            access_flags: self.access_flags,
            name,
            descriptor,
            attributes: self
                .attributes
                .into_iter()
                .map(|x| x.into_attr(cp))
                .collect::<BinResult<Vec<_>>>()?,
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
    pub(crate) fn to_raw(&self, cp: &ConstantPool) -> BinResult<RawFieldInfo> {
        let name = e!(cp.index_of_utf8(self.name.clone()), v)?;
        let descriptor = e!(cp.index_of_utf8(self.descriptor.clone()), v)?;

        Ok(RawFieldInfo {
            access_flags: self.access_flags,
            name,
            descriptor,
            attributes: self
                .attributes
                .iter()
                .map(|x| x.to_raw(cp))
                .collect::<BinResult<Vec<_>>>()?,
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
    pub fn into_attr(self, cp: &ConstantPool) -> BinResult<FieldAttribute> {
        let name = e!(cp.read_utf8(self.name), i)?;

        match name.as_str() {
            "ConstantValue" => {
                let value_idx = <u16 as BinRead>::read_be(&mut Cursor::new(self.info))?;
                let value = e!(cp.read(value_idx), i)?;

                Ok(FieldAttribute::ConstantValue(e!(
                    ConstantValue::from_cp_index(value),
                    i
                )?))
            }
            "Synthetic" => Ok(FieldAttribute::Synthetic),
            "Deprecated" => Ok(FieldAttribute::Deprecated),
            "Signature" => {
                let value_idx = <u16 as BinRead>::read_be(&mut Cursor::new(self.info))?;
                let value = e!(cp.read_utf8(value_idx), i)?;

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
    fn from_cp_index(cp_index: &ConstantPoolIndex) -> Option<Self> {
        match cp_index {
            ConstantPoolIndex::Integer(i) => Some(Self::Integer(*i)),
            ConstantPoolIndex::Float(f) => Some(Self::Float(*f)),
            ConstantPoolIndex::Long(l) => Some(Self::Long(*l)),
            ConstantPoolIndex::Double(d) => Some(Self::Double(*d)),
            ConstantPoolIndex::String(s) => Some(Self::String(s.clone())),
            _ => None,
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
    pub(crate) fn to_raw(&self, cp: &ConstantPool) -> BinResult<RawFieldAttribute> {
        match self {
            FieldAttribute::ConstantValue(v) => Ok(RawFieldAttribute {
                name: e!(cp.index_of_utf8("ConstantValue".to_owned()), v)?,
                info: e!(cp.index_of(v.to_cp_index()), v)?.to_be_bytes().to_vec(),
            }),
            FieldAttribute::Synthetic => Ok(RawFieldAttribute {
                name: e!(cp.index_of_utf8("Synthetic".to_owned()), v)?,
                info: vec![],
            }),
            FieldAttribute::Deprecated => Ok(RawFieldAttribute {
                name: e!(cp.index_of_utf8("Deprecated".to_owned()), v)?,
                info: vec![],
            }),
            FieldAttribute::Signature(s) => Ok(RawFieldAttribute {
                name: e!(cp.index_of_utf8("ConstantValue".to_owned()), v)?,
                info: e!(cp.index_of_utf8(s.clone()), v)?.to_be_bytes().to_vec(),
            }),
            FieldAttribute::RuntimeVisibleAnnotations => todo!(),
            FieldAttribute::RuntimeInvisibleAnnotations => todo!(),
            FieldAttribute::RuntimeVisibleTypeAnnotations => todo!(),
            FieldAttribute::RuntimeInvisibleTypeAnnotations => todo!(),
            FieldAttribute::Unknown(name, info) => Ok(RawFieldAttribute {
                name: e!(cp.index_of_utf8(name.clone()), v)?,
                info: info.clone(),
            }),
        }
    }
}
