use binrw::{binread, binrw, BinRead};

use crate::utf8::ModifiedUtf8;

pub(crate) mod macros {
    #[macro_export]
    macro_rules! e {
        ($e:expr, i) => {
            e!($e, "Invalid constant pool index")
        };
        ($e:expr, v) => {
            e!($e, "Value not in constant pool")
        };
        ($e:expr) => {
            e!($e, "Invalid constant pool")
        };
        ($e:expr, $l:literal) => {
            $e.ok_or(binrw::Error::Custom { pos: 0, err: Box::new($l)})
        };
    }
}

#[binrw]
#[brw(big)]
pub struct ConstantPool {
    #[br(temp)]
    #[bw(calc(constant_pool.len() as u16 + 1))]
    constant_pool_count: u16,
    #[br(parse_with = |r, e, _: ()| constant_pool_parser(r, e, constant_pool_count))]
    #[bw(map = write_cp)]
    constant_pool: Vec<ConstantPoolIndex>,
}

#[binrw::parser(reader: r, endian: e)]
fn constant_pool_parser(count: u16) -> binrw::BinResult<Vec<ConstantPoolIndex>> {
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

    process_cp(raw_cp)
}

impl ConstantPool {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.constant_pool.len()
    }

    pub fn read(&self, index: u16) -> Option<&ConstantPoolIndex> {
        self.constant_pool.get(index as usize)
    }

    pub(crate) fn get_index(&self, index: ConstantPoolIndex) -> Option<u16> {
        for (i, idx) in self.constant_pool.iter().enumerate() {
            if idx == &index {
                return Some(i as u16);
            }
        }

        None
    }

    pub fn read_utf8(&self, index: u16) -> Option<String> {
        self.read(index).and_then(ConstantPoolIndex::as_string)
    }

    pub fn read_class(&self, index: u16) -> Option<String> {
        self.read(index).and_then(ConstantPoolIndex::as_class)
    }

    pub(crate) fn class_index(&self, class: String) -> Option<u16> {
        self.get_index(ConstantPoolIndex::Class(class))
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
    Int(i32),
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
    Int(i32),
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

fn process_cp(raw_cp: Vec<RawConstantPoolIndex>) -> binrw::BinResult<Vec<ConstantPoolIndex>> {
    let mut cp = vec![None; raw_cp.len()];

    for i in 0..raw_cp.len() {
        resolve_index(i, &raw_cp, &mut cp);
    }

    cp.into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or(binrw::Error::Custom {
            pos: 0,
            err: Box::new("Invalid constant pool"),
        })
}

fn resolve_index<'a>(
    i: usize,
    raw_cp: &[RawConstantPoolIndex],
    cp: &'a mut Vec<Option<ConstantPoolIndex>>,
) -> Option<&'a ConstantPoolIndex> {
    if cp[i].is_none() {
        let raw = raw_cp[i].clone();
        cp[i] = match raw {
            RawConstantPoolIndex::Utf8(s) => Some(ConstantPoolIndex::Utf8(s)),
            RawConstantPoolIndex::Int(i) => Some(ConstantPoolIndex::Int(i)),
            RawConstantPoolIndex::Float(f) => Some(ConstantPoolIndex::Float(f)),
            RawConstantPoolIndex::Long(l) => Some(ConstantPoolIndex::Long(l)),
            RawConstantPoolIndex::Double(d) => Some(ConstantPoolIndex::Double(d)),
            RawConstantPoolIndex::Class(i) => Some(ConstantPoolIndex::Class(
                resolve_index(i as usize, raw_cp, cp)?
                    .as_utf8()?
            )),
            RawConstantPoolIndex::String(i) => Some(ConstantPoolIndex::String(
                resolve_index(i as usize, raw_cp, cp)?
                    .as_utf8()?
            )),
            RawConstantPoolIndex::Fieldref(c, nty) => {
                let class = resolve_index(c as usize, raw_cp, cp)?.as_class()?;
                let (name, descriptor) =
                    resolve_index(nty as usize, raw_cp, cp)?.as_name_and_type()?;
                Some(ConstantPoolIndex::Fieldref {
                    class,
                    name,
                    descriptor,
                })
            }
            RawConstantPoolIndex::Methodref(c, nty) => {
                let class = resolve_index(c as usize, raw_cp, cp)?.as_class()?;
                let (name, descriptor) =
                    resolve_index(nty as usize, raw_cp, cp)?.as_name_and_type()?;
                Some(ConstantPoolIndex::Methodref {
                    class,
                    name,
                    descriptor,
                })
            }
            RawConstantPoolIndex::InterfaceMethodref(c, nty) => {
                let class = resolve_index(c as usize, raw_cp, cp)?.as_class()?;
                let (name, descriptor) =
                    resolve_index(nty as usize, raw_cp, cp)?.as_name_and_type()?;
                Some(ConstantPoolIndex::InterfaceMethodref {
                    class,
                    name,
                    descriptor,
                })
            }
            RawConstantPoolIndex::NameAndType(n, d) => {
                let name = resolve_index(n as usize, raw_cp, cp)?.as_utf8()?;
                let descriptor = resolve_index(d as usize, raw_cp, cp)?.as_utf8()?;
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
                        } = index.as_fieldref()?;
                        Some(ConstantPoolIndex::MethodHandle { kind, class, name, descriptor })
                    }
                    MethodHandleReferenceKind::InvokeVirtual
                    | MethodHandleReferenceKind::NewInvokeSpecial => {
                        let Methodref {
                            class,
                            name,
                            descriptor,
                        } = index.as_methodref()?;
                        Some(ConstantPoolIndex::MethodHandle { kind, class, name, descriptor })
                    }
                    MethodHandleReferenceKind::InvokeStatic
                    | MethodHandleReferenceKind::InvokeSpecial => {
                        let (class, name, descriptor) = index
                            .as_methodref()
                            .map(
                                |Methodref {
                                     class,
                                     name,
                                     descriptor,
                                 }| (class, name, descriptor),
                            )
                            .or(index.as_interface_methodref().map(
                                |InterfaceMethodref {
                                     class,
                                     name,
                                     descriptor,
                                 }| (class, name, descriptor),
                            ))?;
                        Some(ConstantPoolIndex::MethodHandle { kind, class, name, descriptor })
                    }
                    MethodHandleReferenceKind::InvokeInterface => {
                        let InterfaceMethodref {
                            class,
                            name,
                            descriptor,
                        } = index.as_interface_methodref()?;
                        Some(ConstantPoolIndex::MethodHandle { kind, class, name, descriptor })
                    }
                }
            }
            RawConstantPoolIndex::MethodType(i) => Some(ConstantPoolIndex::MethodType(
                resolve_index(i as usize, raw_cp, cp)?
                    .as_utf8()?
            )),
            RawConstantPoolIndex::Dynamic(index, name_and_type) => {
                let (name, descriptor) = resolve_index(name_and_type as usize, raw_cp, cp)?.as_name_and_type()?;

                Some(ConstantPoolIndex::Dynamic { bootstrap_method_attr_index: index, name, descriptor })
            }
            RawConstantPoolIndex::InvokeDynamic(index, name_and_type) => {
                let (name, descriptor) = resolve_index(name_and_type as usize, raw_cp, cp)?.as_name_and_type()?;

                Some(ConstantPoolIndex::InvokeDynamic { bootstrap_method_attr_index: index, name, descriptor })
            }
            RawConstantPoolIndex::Module(i) => Some(ConstantPoolIndex::Module(
                resolve_index(i as usize, raw_cp, cp)?
                    .as_utf8()?
            )),
            RawConstantPoolIndex::Package(i) => Some(ConstantPoolIndex::Package(
                resolve_index(i as usize, raw_cp, cp)?
                    .as_utf8()?
            )),
            RawConstantPoolIndex::Unusable => Some(ConstantPoolIndex::Unusable),
        }
    }

    return cp[i].as_ref();
}

#[allow(clippy::ptr_arg)]
fn write_cp(cp: &Vec<ConstantPoolIndex>) -> Vec<RawConstantPoolIndex> {
    todo!()
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
    pub fn as_utf8(&self) -> Option<String> {
        if let Self::Utf8(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        if let Self::Int(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        if let Self::Float(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    pub fn as_long(&self) -> Option<i64> {
        if let Self::Long(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        if let Self::Double(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    pub fn as_class(&self) -> Option<String> {
        if let Self::Class(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let Self::String(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn as_fieldref(&self) -> Option<Fieldref> {
        if let Self::Fieldref {
            class,
            name,
            descriptor,
        } = self.clone()
        {
            Some(Fieldref {
                class,
                name,
                descriptor,
            })
        } else {
            None
        }
    }

    pub fn as_methodref(&self) -> Option<Methodref> {
        if let Self::Methodref {
            class,
            name,
            descriptor,
        } = self.clone()
        {
            Some(Methodref {
                class,
                name,
                descriptor,
            })
        } else {
            None
        }
    }

    pub fn as_interface_methodref(&self) -> Option<InterfaceMethodref> {
        if let Self::InterfaceMethodref {
            class,
            name,
            descriptor,
        } = self.clone()
        {
            Some(InterfaceMethodref {
                class,
                name,
                descriptor,
            })
        } else {
            None
        }
    }

    pub fn as_name_and_type(&self) -> Option<(String, String)> {
        if let Self::NameAndType(x, y) = self.clone() {
            Some((x, y))
        } else {
            None
        }
    }

    pub fn as_method_handle(&self) -> Option<MethodHandle> {
        if let Self::MethodHandle {
            kind,
            class,
            name,
            descriptor,
        } = self.clone()
        {
            Some(MethodHandle {
                kind,
                class,
                name,
                descriptor,
            })
        } else {
            None
        }
    }

    pub fn as_dynamic(&self) -> Option<Dynamic> {
        if let Self::Dynamic {
            bootstrap_method_attr_index,
            name,
            descriptor,
        } = self.clone()
        {
            Some(Dynamic {
                bootstrap_method_attr_index,
                name,
                descriptor,
            })
        } else {
            None
        }
    }

    pub fn as_invoke_dynamic(&self) -> Option<InvokeDynamic> {
        if let Self::InvokeDynamic {
            bootstrap_method_attr_index,
            name,
            descriptor,
        } = self.clone()
        {
            Some(InvokeDynamic {
                bootstrap_method_attr_index,
                name,
                descriptor,
            })
        } else {
            None
        }
    }

    pub fn as_module(&self) -> Option<String> {
        if let Self::Module(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn as_package(&self) -> Option<String> {
        if let Self::Package(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }
}
