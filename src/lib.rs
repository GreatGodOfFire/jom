pub mod constant_pool;
pub mod field;
mod utf8;

use binrw::{binrw, BinResult, VecArgs};
use constant_pool::{ConstantPool, ConstantPoolIndex, Fieldref};
use field::{FieldInfo, RawFieldInfo};

#[binrw]
#[brw(big, magic = 0xCAFEBABEu32)]
pub struct ClassFile {
    minor: u16,
    major: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    #[br(try_map = |x: u16| constant_pool.read_class(x).ok_or(binrw::Error::Custom { pos: 0, err: Box::new("Invalid constant pool index") }))]
    #[bw(try_map = |s| class_index(constant_pool, s.clone()))]
    this_class: String,
    #[br(try_map = |x: u16| read_class(&constant_pool, x))]
    #[bw(try_map = |s| class_index(constant_pool, s.clone()))]
    super_class: String,
    #[br(temp)]
    #[bw(calc(interfaces.len() as u16))]
    interfaces_count: u16,
    #[br(count = interfaces_count)]
    #[br(try_map = |x: Vec<u16>| x.into_iter().map(|x| read_class(&constant_pool, x)).collect::<BinResult<Vec<_>>>())]
    #[bw(try_map = |i| i.iter().map(|s| class_index(constant_pool, s.clone())).collect::<BinResult<Vec<u16>>>())]
    interfaces: Vec<String>,
    #[br(temp)]
    #[bw(calc(fields.len() as u16))]
    fields_count: u16,
    #[br(count = fields_count as usize)]
    #[br(try_map = |v: Vec<RawFieldInfo>| v.into_iter().map(|x| x.to_field_info(&constant_pool)).collect::<BinResult<Vec<_>>>())]
    #[bw(try_map = |v| v.into_iter().map(|x| x.to_raw(&constant_pool)).collect::<BinResult<Vec<_>>>())]
    fields: Vec<FieldInfo>,
    // #[br(temp)]
    // #[bw(calc(interfaces.len() as u16))]
    // fields_count: u16,
    // #[br(count = interfaces_count)]
    // #[br(try_map = |x: Vec<u16>| x.into_iter().map(|x| constant_pool.get(x).and_then(ConstantPoolIndex::as_fieldref)).collect::<Option<Vec<_>>>().ok_or(binrw::Error::Custom { pos: 0, err: Box::new("Invalid constant pool index") }))]
    // #[bw(try_map = write_fieldref)]
    // fields: Vec<Fieldref>,
    // #[br(temp)]
    // #[bw(calc(methods.len() as u16))]
    // methods_count: u16,
    // #[br(count = methods_count)]
    // methods: Vec<()>,
    // #[br(temp)]
    // #[bw(calc(attributes.len() as u16))]
    // attributes_count: u16,
    // #[br(count = attributes_count)]
    // attributes: Vec<()>,
}

fn read_class(constant_pool: &ConstantPool, index: u16) -> BinResult<String> {
    constant_pool.read(index).and_then(ConstantPoolIndex::as_class).ok_or(binrw::Error::Custom { pos: 0, err: Box::new("Failed reading class name from constant pool") })
}

fn class_index(constant_pool: &ConstantPool, s: String) -> BinResult<u16> {
    constant_pool.get_index(ConstantPoolIndex::Class(s)).ok_or(binrw::Error::Custom { pos: 0, err: Box::new("Class name not in constant pool") })
}

