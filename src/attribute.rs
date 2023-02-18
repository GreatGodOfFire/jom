use std::io::Cursor;

use binrw::{binrw, BinRead, VecArgs};

use crate::{constant_pool::ConstantPool, error::JomResult, method::code::Code};

#[binrw]
pub(crate) struct RawAttribute {
    name: u16,
    #[br(temp)]
    #[bw(calc = info.len() as u32)]
    #[br(dbg)]
    info_length: u32,
    #[br(count = info_length)]
    info: Vec<u8>,
}

impl RawAttribute {
    pub fn into_method_attr(self, cp: &ConstantPool) -> JomResult<MethodAttribute> {
        let name = cp.read_utf8(self.name)?;

        match name.as_str() {
            "Code" => Ok(MethodAttribute::Code(Code::read(&self.info, cp)?)),
            "Synthetic" => Ok(MethodAttribute::Synthetic),
            "Deprecated" => Ok(MethodAttribute::Deprecated),
            "Signature" => {
                let value_idx = <u16 as BinRead>::read_be(&mut Cursor::new(self.info))?;
                let value = cp.read_utf8(value_idx)?;

                Ok(MethodAttribute::Signature(value))
            }
            _ => Ok(MethodAttribute::Unknown(name, self.info)),
        }
    }

    pub fn into_code_attr(self, cp: &ConstantPool) -> JomResult<CodeAttribute> {
        let name = cp.read_utf8(self.name)?;

        match name.as_str() {
            "LineNumberTable" => {
                let mut cursor = Cursor::new(self.info);

                let len = <u16 as BinRead>::read_be(&mut cursor)? as usize;
                let table = <Vec<LineNumberTableIndex> as BinRead>::read_be_args(
                    &mut cursor,
                    VecArgs {
                        count: len,
                        inner: (),
                    },
                )?;

                Ok(CodeAttribute::LineNumberTable(table))
            }
            "LocalVariableTable" => {
                let mut cursor = Cursor::new(self.info);

                let len = <u16 as BinRead>::read_be(&mut cursor)? as usize;
                let table = <Vec<RawLocalVariableTableIndex> as BinRead>::read_be_args(
                    &mut cursor,
                    VecArgs {
                        count: len,
                        inner: (),
                    },
                )?;
                let table = table
                    .into_iter()
                    .map(|x| x.into_table_index(cp))
                    .collect::<JomResult<Vec<_>>>()?;

                Ok(CodeAttribute::LocalVariableTable(table))
            }
            "LocalVariableTypeTable" => {
                let mut cursor = Cursor::new(self.info);

                let len = <u16 as BinRead>::read_be(&mut cursor)? as usize;
                let table = <Vec<RawLocalVariableTypeTableIndex> as BinRead>::read_be_args(
                    &mut cursor,
                    VecArgs {
                        count: len,
                        inner: (),
                    },
                )?;
                let table = table
                    .into_iter()
                    .map(|x| x.into_table_index(cp))
                    .collect::<JomResult<Vec<_>>>()?;

                Ok(CodeAttribute::LocalVariableTypeTable(table))
            }
            "StackMapTable" => todo!(),
            "RuntimeVisibleTypeAnnotations" => todo!(),
            "RuntimeInvisibleTypeAnnotations" => todo!(),
            _ => Ok(CodeAttribute::Unknown(name, self.info)),
        }
    }
}

pub enum MethodAttribute {
    Code(Code),
    Exceptions,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    MethodParameters,
    Synthetic,
    Deprecated,
    Signature(String),
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    Unknown(String, Vec<u8>),
}

impl MethodAttribute {
    pub(crate) fn to_raw(&self, cp: &ConstantPool) -> JomResult<RawAttribute> {
        match self {
            MethodAttribute::Code { .. } => todo!(),
            MethodAttribute::Exceptions => todo!(),
            MethodAttribute::RuntimeVisibleParameterAnnotations => todo!(),
            MethodAttribute::RuntimeInvisibleParameterAnnotations => todo!(),
            MethodAttribute::AnnotationDefault => todo!(),
            MethodAttribute::MethodParameters => todo!(),
            MethodAttribute::Synthetic => Ok(RawAttribute {
                name: cp.find_utf8("Synthetic".to_owned())?,
                info: vec![],
            }),
            MethodAttribute::Deprecated => Ok(RawAttribute {
                name: cp.find_utf8("Deprecated".to_owned())?,
                info: vec![],
            }),
            MethodAttribute::Signature(s) => Ok(RawAttribute {
                name: cp.find_utf8("Signature".to_owned())?,
                info: cp.find_utf8(s.clone())?.to_be_bytes().to_vec(),
            }),
            MethodAttribute::RuntimeVisibleAnnotations => todo!(),
            MethodAttribute::RuntimeInvisibleAnnotations => todo!(),
            MethodAttribute::RuntimeVisibleTypeAnnotations => todo!(),
            MethodAttribute::RuntimeInvisibleTypeAnnotations => todo!(),
            MethodAttribute::Unknown(name, info) => Ok(RawAttribute {
                name: cp.find_utf8(name.clone())?,
                info: info.clone(),
            }),
        }
    }
}

pub enum CodeAttribute {
    LineNumberTable(Vec<LineNumberTableIndex>),
    LocalVariableTable(Vec<LocalVariableTableIndex>),
    LocalVariableTypeTable(Vec<LocalVariableTypeTableIndex>),
    StackMapTable,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    Unknown(String, Vec<u8>),
}

#[binrw]
pub struct LineNumberTableIndex {
    start_pc: u16,
    line_number: u16,
}

#[binrw]
struct RawLocalVariableTableIndex {
    start_pc: u16,
    length: u16,
    name: u16,
    descriptor: u16,
    index: u16,
}

impl RawLocalVariableTableIndex {
    fn into_table_index(self, constant_pool: &ConstantPool) -> JomResult<LocalVariableTableIndex> {
        let RawLocalVariableTableIndex {
            start_pc,
            length,
            name,
            descriptor,
            index,
        } = self;

        let name = constant_pool.read_utf8(name)?;
        let descriptor = constant_pool.read_utf8(descriptor)?;

        Ok(LocalVariableTableIndex {
            start_pc,
            length,
            name,
            descriptor,
            index,
        })
    }
}

pub struct LocalVariableTableIndex {
    pub start_pc: u16,
    pub length: u16,
    pub name: String,
    pub descriptor: String,
    pub index: u16,
}

#[binrw]
struct RawLocalVariableTypeTableIndex {
    start_pc: u16,
    length: u16,
    name: u16,
    descriptor: u16,
    index: u16,
}

impl RawLocalVariableTypeTableIndex {
    fn into_table_index(
        self,
        constant_pool: &ConstantPool,
    ) -> JomResult<LocalVariableTypeTableIndex> {
        let RawLocalVariableTypeTableIndex {
            start_pc,
            length,
            name,
            descriptor,
            index,
        } = self;

        let name = constant_pool.read_utf8(name)?;
        let descriptor = constant_pool.read_utf8(descriptor)?;

        Ok(LocalVariableTypeTableIndex {
            start_pc,
            length,
            name,
            descriptor,
            index,
        })
    }
}

pub struct LocalVariableTypeTableIndex {
    pub start_pc: u16,
    pub length: u16,
    pub name: String,
    pub descriptor: String,
    pub index: u16,
}
