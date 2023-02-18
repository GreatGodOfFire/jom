use std::io::Cursor;

use binrw::{binrw, BinRead, VecArgs};

use crate::{constant_pool::{ConstantPool, ConstantPoolIndex}, error::{JomResult, JomError}, method::code::Code};

#[binrw]
pub(crate) struct RawAttribute {
    name: u16,
    #[br(temp)]
    #[bw(calc = info.len() as u32)]
    info_length: u32,
    #[br(count = info_length)]
    info: Vec<u8>,
}

impl RawAttribute {
    pub fn into_field_attr(self, cp: &ConstantPool) -> JomResult<FieldAttribute> {
        let name = cp.get_utf8(self.name)?;

        match name.as_str() {
            "ConstantValue" => {
                let value_idx = <u16 as BinRead>::read_be(&mut Cursor::new(self.info))?;
                let value = cp.get(value_idx)?;

                Ok(FieldAttribute::ConstantValue(ConstantValue::from_cp_index(
                    value,
                )?))
            }
            "Synthetic" => Ok(FieldAttribute::Synthetic),
            "Deprecated" => Ok(FieldAttribute::Deprecated),
            "Signature" => {
                let value_idx = <u16 as BinRead>::read_be(&mut Cursor::new(self.info))?;
                let value = cp.get_utf8(value_idx)?;

                Ok(FieldAttribute::Signature(value))
            }
            _ => Ok(FieldAttribute::Unknown(name, self.info)),
        }
    }


    pub fn into_method_attr(self, cp: &ConstantPool) -> JomResult<MethodAttribute> {
        let name = cp.get_utf8(self.name)?;

        match name.as_str() {
            "Code" => Ok(MethodAttribute::Code(Code::read(&self.info, cp)?)),
            "Synthetic" => Ok(MethodAttribute::Synthetic),
            "Deprecated" => Ok(MethodAttribute::Deprecated),
            "Signature" => {
                let value_idx = <u16 as BinRead>::read_be(&mut Cursor::new(self.info))?;
                let value = cp.get_utf8(value_idx)?;

                Ok(MethodAttribute::Signature(value))
            }
            _ => Ok(MethodAttribute::Unknown(name, self.info)),
        }
    }

    pub fn into_code_attr(self, cp: &ConstantPool) -> JomResult<CodeAttribute> {
        let name = cp.get_utf8(self.name)?;

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
    pub(crate) fn from_cp_index(cp_index: ConstantPoolIndex) -> JomResult<Self> {
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

        let name = constant_pool.get_utf8(name)?;
        let descriptor = constant_pool.get_utf8(descriptor)?;

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

        let name = constant_pool.get_utf8(name)?;
        let descriptor = constant_pool.get_utf8(descriptor)?;

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
