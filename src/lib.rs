pub mod constant_pool;
pub mod field;
mod utf8;

use binrw::{binrw, BinResult};
use constant_pool::ConstantPool;
use field::{FieldInfo, RawFieldInfo};

#[binrw]
#[brw(big, magic = 0xCAFEBABEu32)]
pub struct ClassFile {
    minor: u16,
    major: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    #[br(try_map = |x: u16| e!(constant_pool.read_class(x), i))]
    #[bw(try_map = |s| e!(constant_pool.index_of_class(s.clone()), "Class name not in constant pool"))]
    this_class: String,
    #[br(try_map = |x: u16| e!(constant_pool.read_class(x), i))]
    #[bw(try_map = |s| e!(constant_pool.index_of_class(s.clone()), "Class name not in constant pool"))]
    super_class: String,
    #[br(temp)]
    #[bw(calc(interfaces.len() as u16))]
    interfaces_count: u16,
    #[br(count = interfaces_count)]
    #[br(try_map = |x: Vec<u16>| x.into_iter().map(|x| e!(constant_pool.read_class(x), i)).collect::<BinResult<Vec<_>>>())]
    #[bw(try_map = |i| i.iter().map(|s| e!(constant_pool.index_of_class(s.clone()), "Class name not in constant pool")).collect::<BinResult<Vec<u16>>>())]
    interfaces: Vec<String>,
    #[br(temp)]
    #[bw(calc(fields.len() as u16))]
    fields_count: u16,
    #[br(count = fields_count)]
    #[br(try_map = |v: Vec<RawFieldInfo>| v.into_iter().map(|x| x.into_field_info(&constant_pool)).collect::<BinResult<Vec<_>>>())]
    #[bw(try_map = |v| v.iter().map(|x| x.to_raw(constant_pool)).collect::<BinResult<Vec<_>>>())]
    fields: Vec<FieldInfo>,
}
