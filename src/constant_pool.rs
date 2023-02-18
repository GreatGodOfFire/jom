use binrw::{binrw, BinRead, BinResult};

use crate::{
    error::{JomError, JomResult},
    utf8::ModifiedUtf8,
};

pub struct ConstantPool(pub(crate) Vec<ConstantPoolIndex>);

#[binrw::parser(reader: r, endian: e)]
pub(crate) fn constant_pool_parser(count: (u16,)) -> BinResult<Vec<RawConstantPoolIndex>> {
    let count = count.0;
    let mut raw_cp = vec![RawConstantPoolIndex::Unusable];
    let mut i = 1;
    while i < count {
        let index = RawConstantPoolIndex::read_options(r, e, ())?;

        if let RawConstantPoolIndex::Long(_) | RawConstantPoolIndex::Double(_) = index {
            raw_cp.push(index);
            raw_cp.push(RawConstantPoolIndex::Unusable);
            i += 2;
        } else {
            raw_cp.push(index);
            i += 1;
        }
    }
    
    Ok(raw_cp)
}

impl ConstantPool {
    // As the constant pool length is the first index in the constant pool it will never be empty
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: u16) -> JomResult<ConstantPoolIndex> {
        self.0
            .get(index as usize)
            .cloned()
            .ok_or(JomError::out_of_bounds(index))
    }

    pub fn find(&self, index: ConstantPoolIndex) -> JomResult<u16> {
        for (i, idx) in self.0.iter().enumerate() {
            if idx == &index {
                return Ok(i as u16);
            }
        }

        Err(JomError::not_in_cp(format!("{index:?}")))
    }

    pub fn get_utf8(&self, index: u16) -> JomResult<String> {
        self.get(index).and_then(ConstantPoolIndex::into_utf8)
    }

    pub fn get_class(&self, index: u16) -> JomResult<String> {
        self.get(index).and_then(ConstantPoolIndex::into_class)
    }

    pub fn find_utf8(&self, s: String) -> JomResult<u16> {
        self.find(ConstantPoolIndex::Utf8(s))
    }

    pub fn find_class(&self, class: String) -> JomResult<u16> {
        self.find(ConstantPoolIndex::Class(class))
    }
}

#[binrw]
#[brw(big)]
#[derive(Clone, Debug)]
pub enum RawConstantPoolIndex {
    #[brw(magic(1u8))]
    Utf8(
        #[br(map = |s: ModifiedUtf8| s.to_string())]
        #[bw(try_map = |s| ModifiedUtf8::try_from(s.clone()))]
        String,
    ),
    #[brw(magic(3u8))]
    Integer(i32),
    #[brw(magic(4u8))]
    Float(f32),
    #[brw(magic(5u8))]
    Long(i64),
    #[brw(magic(6u8))]
    Double(f64),
    #[brw(magic(7u8))]
    Class(u16),
    #[brw(magic(8u8))]
    String(u16),
    #[brw(magic(9u8))]
    Fieldref(u16, u16),
    #[brw(magic(10u8))]
    Methodref(u16, u16),
    #[brw(magic(11u8))]
    InterfaceMethodref(u16, u16),
    #[brw(magic(12u8))]
    NameAndType(u16, u16),
    #[brw(magic(15u8))]
    MethodHandle(MethodHandleReferenceKind, u16),
    #[brw(magic(16u8))]
    MethodType(u16),
    #[brw(magic(17u8))]
    Dynamic(u16, u16),
    #[brw(magic(18u8))]
    InvokeDynamic(u16, u16),
    #[brw(magic(19u8))]
    Module(u16),
    #[brw(magic(20u8))]
    Package(u16),
    Unusable,
}

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MethodHandleReferenceKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantPoolIndex {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(String),
    String(String),
    Fieldref {
        class: String,
        name: String,
        descriptor: String,
    },
    Methodref {
        class: String,
        name: String,
        descriptor: String,
    },
    InterfaceMethodref {
        class: String,
        name: String,
        descriptor: String,
    },
    NameAndType(String, String),
    MethodHandle {
        kind: MethodHandleReferenceKind,
        class: String,
        name: String,
        descriptor: String,
    },
    MethodType(String),
    Dynamic {
        bootstrap_method_attr_index: u16,
        name: String,
        descriptor: String,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name: String,
        descriptor: String,
    },
    Module(String),
    Package(String),
    Unusable,
}

pub(crate) fn process_cp(raw_cp: Vec<RawConstantPoolIndex>) -> JomResult<ConstantPool> {
    let mut cp = vec![None; raw_cp.len()];

    for i in 0..raw_cp.len() {
        resolve_index(i, &raw_cp, &mut cp)?;
    }

    Ok(ConstantPool(cp.into_iter().collect::<Option<Vec<_>>>().unwrap()))
}

fn resolve_index<'a>(
    i: usize,
    raw_cp: &[RawConstantPoolIndex],
    cp: &'a mut Vec<Option<ConstantPoolIndex>>,
) -> JomResult<&'a ConstantPoolIndex> {
    if cp[i].is_none() {
        let raw = raw_cp[i].clone();
        cp[i] = match raw {
            RawConstantPoolIndex::Utf8(s) => Some(ConstantPoolIndex::Utf8(s)),
            RawConstantPoolIndex::Integer(i) => Some(ConstantPoolIndex::Integer(i)),
            RawConstantPoolIndex::Float(f) => Some(ConstantPoolIndex::Float(f)),
            RawConstantPoolIndex::Long(l) => Some(ConstantPoolIndex::Long(l)),
            RawConstantPoolIndex::Double(d) => Some(ConstantPoolIndex::Double(d)),
            RawConstantPoolIndex::Class(i) => Some(ConstantPoolIndex::Class(
                resolve_index(i as usize, raw_cp, cp)?.clone().into_utf8()?,
            )),
            RawConstantPoolIndex::String(i) => Some(ConstantPoolIndex::String(
                resolve_index(i as usize, raw_cp, cp)?.clone().into_utf8()?,
            )),
            RawConstantPoolIndex::Fieldref(c, nty) => {
                let class = resolve_index(c as usize, raw_cp, cp)?
                    .clone()
                    .into_class()?;
                let (name, descriptor) = resolve_index(nty as usize, raw_cp, cp)?
                    .clone()
                    .into_name_and_type()?;
                Some(ConstantPoolIndex::Fieldref {
                    class,
                    name,
                    descriptor,
                })
            }
            RawConstantPoolIndex::Methodref(c, nty) => {
                let class = resolve_index(c as usize, raw_cp, cp)?
                    .clone()
                    .into_class()?;
                let (name, descriptor) = resolve_index(nty as usize, raw_cp, cp)?
                    .clone()
                    .into_name_and_type()?;
                Some(ConstantPoolIndex::Methodref {
                    class,
                    name,
                    descriptor,
                })
            }
            RawConstantPoolIndex::InterfaceMethodref(c, nty) => {
                let class = resolve_index(c as usize, raw_cp, cp)?
                    .clone()
                    .into_class()?;
                let (name, descriptor) = resolve_index(nty as usize, raw_cp, cp)?
                    .clone()
                    .into_name_and_type()?;
                Some(ConstantPoolIndex::InterfaceMethodref {
                    class,
                    name,
                    descriptor,
                })
            }
            RawConstantPoolIndex::NameAndType(n, d) => {
                let name = resolve_index(n as usize, raw_cp, cp)?.clone().into_utf8()?;
                let descriptor = resolve_index(d as usize, raw_cp, cp)?.clone().into_utf8()?;
                Some(ConstantPoolIndex::NameAndType(name, descriptor))
            }
            RawConstantPoolIndex::MethodHandle(kind, index) => {
                let index = resolve_index(index as usize, raw_cp, cp)?;

                match kind {
                    MethodHandleReferenceKind::GetField
                    | MethodHandleReferenceKind::GetStatic
                    | MethodHandleReferenceKind::PutField
                    | MethodHandleReferenceKind::PutStatic => {
                        let Fieldref {
                            class,
                            name,
                            descriptor,
                        } = index.clone().into_fieldref()?;
                        Some(ConstantPoolIndex::MethodHandle {
                            kind,
                            class,
                            name,
                            descriptor,
                        })
                    }
                    MethodHandleReferenceKind::InvokeVirtual
                    | MethodHandleReferenceKind::NewInvokeSpecial => {
                        let Methodref {
                            class,
                            name,
                            descriptor,
                        } = index.clone().into_methodref()?;
                        Some(ConstantPoolIndex::MethodHandle {
                            kind,
                            class,
                            name,
                            descriptor,
                        })
                    }
                    MethodHandleReferenceKind::InvokeStatic
                    | MethodHandleReferenceKind::InvokeSpecial => {
                        let (class, name, descriptor) = index
                            .clone()
                            .into_methodref()
                            .map(
                                |Methodref {
                                     class,
                                     name,
                                     descriptor,
                                 }| (class, name, descriptor),
                            )
                            .or(index.clone().into_interface_methodref().map(
                                |InterfaceMethodref {
                                     class,
                                     name,
                                     descriptor,
                                 }| (class, name, descriptor),
                            ))?;
                        Some(ConstantPoolIndex::MethodHandle {
                            kind,
                            class,
                            name,
                            descriptor,
                        })
                    }
                    MethodHandleReferenceKind::InvokeInterface => {
                        let InterfaceMethodref {
                            class,
                            name,
                            descriptor,
                        } = index.clone().into_interface_methodref()?;
                        Some(ConstantPoolIndex::MethodHandle {
                            kind,
                            class,
                            name,
                            descriptor,
                        })
                    }
                }
            }
            RawConstantPoolIndex::MethodType(i) => Some(ConstantPoolIndex::MethodType(
                resolve_index(i as usize, raw_cp, cp)?.clone().into_utf8()?,
            )),
            RawConstantPoolIndex::Dynamic(index, name_and_type) => {
                let (name, descriptor) = resolve_index(name_and_type as usize, raw_cp, cp)?
                    .clone()
                    .into_name_and_type()?;

                Some(ConstantPoolIndex::Dynamic {
                    bootstrap_method_attr_index: index,
                    name,
                    descriptor,
                })
            }
            RawConstantPoolIndex::InvokeDynamic(index, name_and_type) => {
                let (name, descriptor) = resolve_index(name_and_type as usize, raw_cp, cp)?
                    .clone()
                    .into_name_and_type()?;

                Some(ConstantPoolIndex::InvokeDynamic {
                    bootstrap_method_attr_index: index,
                    name,
                    descriptor,
                })
            }
            RawConstantPoolIndex::Module(i) => Some(ConstantPoolIndex::Module(
                resolve_index(i as usize, raw_cp, cp)?.clone().into_utf8()?,
            )),
            RawConstantPoolIndex::Package(i) => Some(ConstantPoolIndex::Package(
                resolve_index(i as usize, raw_cp, cp)?.clone().into_utf8()?,
            )),
            RawConstantPoolIndex::Unusable => Some(ConstantPoolIndex::Unusable),
        }
    }

    return Ok(cp[i].as_ref().unwrap());
}

pub struct Fieldref {
    pub class: String,
    pub name: String,
    pub descriptor: String,
}
pub struct Methodref {
    pub class: String,
    pub name: String,
    pub descriptor: String,
}
pub struct InterfaceMethodref {
    pub class: String,
    pub name: String,
    pub descriptor: String,
}
pub struct MethodHandle {
    pub kind: MethodHandleReferenceKind,
    pub class: String,
    pub name: String,
    pub descriptor: String,
}
pub struct Dynamic {
    pub bootstrap_method_attr_index: u16,
    pub name: String,
    pub descriptor: String,
}
pub struct InvokeDynamic {
    pub bootstrap_method_attr_index: u16,
    pub name: String,
    pub descriptor: String,
}

impl ConstantPoolIndex {
    pub fn name(&self) -> &'static str {
        match self {
            ConstantPoolIndex::Utf8(_) => "Utf8",
            ConstantPoolIndex::Integer(_) => "Integer",
            ConstantPoolIndex::Float(_) => "Float",
            ConstantPoolIndex::Long(_) => "Long",
            ConstantPoolIndex::Double(_) => "Double",
            ConstantPoolIndex::Class(_) => "Class",
            ConstantPoolIndex::String(_) => "String",
            ConstantPoolIndex::Fieldref { .. } => "Fieldref",
            ConstantPoolIndex::Methodref { .. } => "Methodref",
            ConstantPoolIndex::InterfaceMethodref { .. } => "InterfaceMethodref",
            ConstantPoolIndex::NameAndType(_, _) => "NameAndType",
            ConstantPoolIndex::MethodHandle { .. } => "MethodHandle",
            ConstantPoolIndex::MethodType(_) => "MethodType",
            ConstantPoolIndex::Dynamic { .. } => "Dynamic",
            ConstantPoolIndex::InvokeDynamic { .. } => "InvokeDynamic",
            ConstantPoolIndex::Module(_) => "Module",
            ConstantPoolIndex::Package(_) => "Package",
            ConstantPoolIndex::Unusable => "Unusable",
        }
    }

    pub fn into_utf8(self) -> JomResult<String> {
        match self {
            Self::Utf8(x) => Ok(x),
            x => Err(JomError::new_cp_index("Utf8", x.name())),
        }
    }

    pub fn into_integer(self) -> JomResult<i32> {
        match self {
            Self::Integer(x) => Ok(x),
            x => Err(JomError::new_cp_index("Integer", x.name())),
        }
    }

    pub fn into_float(self) -> JomResult<f32> {
        match self {
            Self::Float(x) => Ok(x),
            x => Err(JomError::new_cp_index("Float", x.name())),
        }
    }

    pub fn into_long(self) -> JomResult<i64> {
        match self {
            Self::Long(x) => Ok(x),
            x => Err(JomError::new_cp_index("Long", x.name())),
        }
    }

    pub fn into_double(self) -> JomResult<f64> {
        match self {
            Self::Double(x) => Ok(x),
            x => Err(JomError::new_cp_index("Double", x.name())),
        }
    }

    pub fn into_class(self) -> JomResult<String> {
        match self {
            Self::Class(x) => Ok(x),
            x => Err(JomError::new_cp_index("Class", x.name())),
        }
    }

    pub fn into_string(self) -> JomResult<String> {
        match self {
            Self::String(x) => Ok(x),
            x => Err(JomError::new_cp_index("String", x.name())),
        }
    }

    pub fn into_fieldref(self) -> JomResult<Fieldref> {
        match self {
            Self::Fieldref {
                class,
                name,
                descriptor,
            } => Ok(Fieldref {
                class,
                name,
                descriptor,
            }),
            x => Err(JomError::new_cp_index("Fieldref", x.name())),
        }
    }

    pub fn into_methodref(self) -> JomResult<Methodref> {
        match self {
            Self::Methodref {
                class,
                name,
                descriptor,
            } => Ok(Methodref {
                class,
                name,
                descriptor,
            }),
            x => Err(JomError::new_cp_index("Methodref", x.name())),
        }
    }

    pub fn into_interface_methodref(self) -> JomResult<InterfaceMethodref> {
        match self {
            Self::InterfaceMethodref {
                class,
                name,
                descriptor,
            } => Ok(InterfaceMethodref {
                class,
                name,
                descriptor,
            }),
            x => Err(JomError::new_cp_index("InterfaceMethodref", x.name())),
        }
    }

    pub fn into_name_and_type(self) -> JomResult<(String, String)> {
        match self {
            Self::NameAndType(x, y) => Ok((x, y)),
            x => Err(JomError::new_cp_index("NameAndType", x.name())),
        }
    }

    pub fn into_method_handle(self) -> JomResult<MethodHandle> {
        match self {
            Self::MethodHandle {
                kind,
                class,
                name,
                descriptor,
            } => Ok(MethodHandle {
                kind,
                class,
                name,
                descriptor,
            }),
            x => Err(JomError::new_cp_index("MethodHandle", x.name())),
        }
    }

    pub fn into_dynamic(self) -> JomResult<Dynamic> {
        match self {
            Self::Dynamic {
                bootstrap_method_attr_index,
                name,
                descriptor,
            } => Ok(Dynamic {
                bootstrap_method_attr_index,
                name,
                descriptor,
            }),
            x => Err(JomError::new_cp_index("Dynamic", x.name())),
        }
    }

    pub fn into_invoke_dynamic(self) -> JomResult<InvokeDynamic> {
        match self {
            Self::InvokeDynamic {
                bootstrap_method_attr_index,
                name,
                descriptor,
            } => Ok(InvokeDynamic {
                bootstrap_method_attr_index,
                name,
                descriptor,
            }),
            x => Err(JomError::new_cp_index("InvokeDynamic", x.name())),
        }
    }

    pub fn into_module(self) -> JomResult<String> {
        match self {
            Self::Module(x) => Ok(x),
            x => Err(JomError::new_cp_index("Module(x)", x.name())),
        }
    }

    pub fn into_package(self) -> JomResult<String> {
        match self {
            Self::Package(x) => Ok(x),
            x => Err(JomError::new_cp_index("Package(x)", x.name())),
        }
    }
}
