mod attribute;
pub mod constant_pool;
pub mod error;
pub mod field;
pub mod method;
mod utf8;

use std::io::Cursor;

use binrw::{binrw, BinRead};
use constant_pool::{constant_pool_parser, ConstantPool, ConstantPoolIndex, RawConstantPoolIndex, process_cp};
use error::JomResult;
use field::{FieldInfo, RawFieldInfo};
use method::{MethodInfo, RawMethodInfo};

#[binrw]
#[brw(big, magic = 0xCAFEBABEu32)]
struct RawClassFile {
    minor: u16,
    major: u16,
    #[br(temp)]
    #[bw(calc = constant_pool.len() as u16)]
    constant_pool_count: u16,
    #[br(args(constant_pool_count))]
    #[br(parse_with = constant_pool_parser)]
    #[bw(map = |_| vec![0u8])]
    constant_pool: Vec<RawConstantPoolIndex>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    #[br(temp)]
    #[bw(calc = interfaces.len() as u16)]
    interfaces_count: u16,
    #[br(count = interfaces_count)]
    interfaces: Vec<u16>,
    #[br(temp)]
    #[bw(calc = fields.len() as u16)]
    fields_count: u16,
    #[br(count = fields_count)]
    fields: Vec<RawFieldInfo>,
    #[br(temp)]
    #[bw(calc = methods.len() as u16)]
    methods_count: u16,
    #[br(count = methods_count)]
    methods: Vec<RawMethodInfo>,
}

pub struct ClassFile {
    minor: u16,
    major: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class: String,
    super_class: String,
    interfaces: Vec<String>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
}

impl ClassFile {
    pub fn read(slice: &[u8]) -> JomResult<Self> {
        let RawClassFile {
            minor,
            major,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
        } = RawClassFile::read(&mut Cursor::new(slice))?;

        let constant_pool = process_cp(constant_pool)?;

        let this_class = constant_pool.get_class(this_class)?;
        let super_class = constant_pool.get_class(super_class)?;
        let interfaces = interfaces
            .into_iter()
            .map(|x| constant_pool.get_class(x))
            .collect::<JomResult<Vec<_>>>()?;
        let fields = fields
            .into_iter()
            .map(|x| x.into_field_info(&constant_pool))
            .collect::<JomResult<Vec<_>>>()?;
        let methods = methods
            .into_iter()
            .map(|x| x.into_method_info(&constant_pool))
            .collect::<JomResult<Vec<_>>>()?;

        Ok(Self {
            minor,
            major,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
        })
    }
}

impl ClassFile {
    pub fn new(this_class: String) -> Self {
        let mut constant_pool = ConstantPool(vec![ConstantPoolIndex::Unusable]);
        constant_pool.0.push(ConstantPoolIndex::Utf8(this_class.clone()));
        constant_pool.0.push(ConstantPoolIndex::Class(this_class.clone()));

        let super_class = "java/lang/Object".to_owned();
        constant_pool.0.push(ConstantPoolIndex::Utf8(super_class.clone()));
        constant_pool.0.push(ConstantPoolIndex::Class(super_class.clone()));

        Self {
            minor: 0,
            major: 63,
            constant_pool,
            access_flags: 0,
            this_class,
            super_class,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
        }
    }

    pub fn minor(&self) -> u16 {
        self.minor
    }

    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn constant_pool(&self) -> &[ConstantPoolIndex] {
        &self.constant_pool.0
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn this_class(&self) -> &str {
        &self.this_class
    }

    pub fn super_class(&self) -> &str {
        &self.super_class
    }

    pub fn interfaces(&self) -> &[String] {
        &self.interfaces
    }

    pub fn fields(&self) -> &[FieldInfo] {
        &self.fields
    }

    pub fn methods(&self) -> &[MethodInfo] {
        &self.methods
    }
}
