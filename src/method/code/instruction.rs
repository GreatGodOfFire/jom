use binrw::binrw;

#[binrw]
#[brw(big)]
#[repr(u8)]
pub enum Instruction {
    #[brw(magic = 0x00u8)]
    Nop,
    #[brw(magic = 0x01u8)]
    AConstNull = 1,
    #[brw(magic = 0x02u8)]
    IConstM1,
    #[brw(magic = 0x03u8)]
    IConst0,
    #[brw(magic = 0x04u8)]
    IConst1,
    #[brw(magic = 0x05u8)]
    IConst2,
    #[brw(magic = 0x06u8)]
    IConst3,
    #[brw(magic = 0x07u8)]
    IConst4,
    #[brw(magic = 0x08u8)]
    IConst5,
    #[brw(magic = 0x09u8)]
    LConst0,
    #[brw(magic = 0x0au8)]
    LConst1,
    #[brw(magic = 0x0bu8)]
    FConst0,
    #[brw(magic = 0x0cu8)]
    FConst1,
    #[brw(magic = 0x0du8)]
    FConst2,
    #[brw(magic = 0x0eu8)]
    DConst0,
    #[brw(magic = 0x0fu8)]
    DConst1,
    #[brw(magic = 0x10u8)]
    BiPush(u8),
    #[brw(magic = 0x11u8)]
    Sipush(u16),
    #[brw(magic = 0x12u8)]
    Ldc(u8),
    #[brw(magic = 0x13u8)]
    LdcW(u16),
    #[brw(magic = 0x14u8)]
    Ldc2W(u16),
    #[brw(magic = 0x15u8)]
    ILoad(u8),
    #[brw(magic = 0x16u8)]
    LLoad(u8),
    #[brw(magic = 0x17u8)]
    FLoad(u8),
    #[brw(magic = 0x18u8)]
    Dload(u8),
    #[brw(magic = 0x19u8)]
    ALoad(u8),
    #[brw(magic = 0x1au8)]
    ILoad0,
    #[brw(magic = 0x1bu8)]
    ILoad1,
    #[brw(magic = 0x1cu8)]
    ILoad2,
    #[brw(magic = 0x1du8)]
    ILoad3,
    #[brw(magic = 0x1eu8)]
    LLoad0,
    #[brw(magic = 0x1fu8)]
    LLoad1,
    #[brw(magic = 0x20u8)]
    LLoad2,
    #[brw(magic = 0x21u8)]
    LLoad3,
    #[brw(magic = 0x22u8)]
    FLoad0,
    #[brw(magic = 0x23u8)]
    FLoad1,
    #[brw(magic = 0x24u8)]
    FLoad2,
    #[brw(magic = 0x25u8)]
    FLoad3,
    #[brw(magic = 0x26u8)]
    DLoad0,
    #[brw(magic = 0x27u8)]
    DLoad1,
    #[brw(magic = 0x28u8)]
    DLoad2,
    #[brw(magic = 0x29u8)]
    DLoad3,
    #[brw(magic = 0x2au8)]
    ALoad0,
    #[brw(magic = 0x2bu8)]
    ALoad1,
    #[brw(magic = 0x2cu8)]
    ALoad2,
    #[brw(magic = 0x2du8)]
    ALoad3,
    #[brw(magic = 0x2eu8)]
    IALoad,
    #[brw(magic = 0x2fu8)]
    LALoad,
    #[brw(magic = 0x30u8)]
    FALoad,
    #[brw(magic = 0x31u8)]
    DALoad,
    #[brw(magic = 0x32u8)]
    AALoad,
    #[brw(magic = 0x33u8)]
    BALoad,
    #[brw(magic = 0x34u8)]
    CALoad,
    #[brw(magic = 0x35u8)]
    SALoad,
    #[brw(magic = 0x36u8)]
    IStore(u8),
    #[brw(magic = 0x37u8)]
    LStore(u8),
    #[brw(magic = 0x38u8)]
    FStore(u8),
    #[brw(magic = 0x39u8)]
    DStore(u8),
    #[brw(magic = 0x3au8)]
    AStore(u8),
    #[brw(magic = 0x3bu8)]
    IStore0,
    #[brw(magic = 0x3cu8)]
    IStore1,
    #[brw(magic = 0x3du8)]
    IStore2,
    #[brw(magic = 0x3eu8)]
    IStore3,
    #[brw(magic = 0x3fu8)]
    LStore0,
    #[brw(magic = 0x40u8)]
    LStore1,
    #[brw(magic = 0x41u8)]
    LStore2,
    #[brw(magic = 0x42u8)]
    LStore3,
    #[brw(magic = 0x43u8)]
    FStore0,
    #[brw(magic = 0x44u8)]
    FStore1,
    #[brw(magic = 0x45u8)]
    FStore2,
    #[brw(magic = 0x46u8)]
    FStore3,
    #[brw(magic = 0x47u8)]
    DStore0,
    #[brw(magic = 0x48u8)]
    DStore1,
    #[brw(magic = 0x49u8)]
    DStore2,
    #[brw(magic = 0x4au8)]
    DStore3,
    #[brw(magic = 0x4bu8)]
    AStore0,
    #[brw(magic = 0x4cu8)]
    AStore1,
    #[brw(magic = 0x4du8)]
    AStore2,
    #[brw(magic = 0x4eu8)]
    AStore3,
    #[brw(magic = 0x4fu8)]
    IAStore,
    #[brw(magic = 0x50u8)]
    LAStore,
    #[brw(magic = 0x51u8)]
    FAStore,
    #[brw(magic = 0x52u8)]
    DAStore,
    #[brw(magic = 0x53u8)]
    AAStore,
    #[brw(magic = 0x54u8)]
    BAStore,
    #[brw(magic = 0x55u8)]
    CAStore,
    #[brw(magic = 0x56u8)]
    SAStore,
    #[brw(magic = 0x57u8)]
    Pop,
    #[brw(magic = 0x58u8)]
    Pop2,
    #[brw(magic = 0x59u8)]
    Dup,
    #[brw(magic = 0x5au8)]
    DupX1,
    #[brw(magic = 0x5bu8)]
    DupX2,
    #[brw(magic = 0x5cu8)]
    Dup2,
    #[brw(magic = 0x5du8)]
    Dup2X1,
    #[brw(magic = 0x5eu8)]
    Dup2X2,
    #[brw(magic = 0x5fu8)]
    Swap,
    #[brw(magic = 0x60u8)]
    IAdd,
    #[brw(magic = 0x61u8)]
    LAdd,
    #[brw(magic = 0x62u8)]
    FAdd,
    #[brw(magic = 0x63u8)]
    DAdd,
    #[brw(magic = 0x64u8)]
    ISub,
    #[brw(magic = 0x65u8)]
    LSub,
    #[brw(magic = 0x66u8)]
    FSub,
    #[brw(magic = 0x67u8)]
    DSub,
    #[brw(magic = 0x68u8)]
    IMul,
    #[brw(magic = 0x69u8)]
    LMul,
    #[brw(magic = 0x6au8)]
    FMul,
    #[brw(magic = 0x6bu8)]
    DMul,
    #[brw(magic = 0x6cu8)]
    IDiv,
    #[brw(magic = 0x6du8)]
    LDiv,
    #[brw(magic = 0x6eu8)]
    FDiv,
    #[brw(magic = 0x6fu8)]
    DDiv,
    #[brw(magic = 0x70u8)]
    IRem,
    #[brw(magic = 0x71u8)]
    LRem,
    #[brw(magic = 0x72u8)]
    FRem,
    #[brw(magic = 0x73u8)]
    DRem,
    #[brw(magic = 0x74u8)]
    INeg,
    #[brw(magic = 0x75u8)]
    LNeg,
    #[brw(magic = 0x76u8)]
    FNeg,
    #[brw(magic = 0x77u8)]
    DNeg,
    #[brw(magic = 0x78u8)]
    IShl,
    #[brw(magic = 0x79u8)]
    LShl,
    #[brw(magic = 0x7au8)]
    IShr,
    #[brw(magic = 0x7bu8)]
    LShr,
    #[brw(magic = 0x7cu8)]
    IUShr,
    #[brw(magic = 0x7du8)]
    LUShr,
    #[brw(magic = 0x7eu8)]
    IAnd,
    #[brw(magic = 0x7fu8)]
    LAnd,
    #[brw(magic = 0x80u8)]
    IOr,
    #[brw(magic = 0x81u8)]
    LOr,
    #[brw(magic = 0x82u8)]
    IXor,
    #[brw(magic = 0x83u8)]
    LXor,
    #[brw(magic = 0x84u8)]
    IInc(u8, u8),
    #[brw(magic = 0x85u8)]
    I2L,
    #[brw(magic = 0x86u8)]
    I2F,
    #[brw(magic = 0x87u8)]
    I2D,
    #[brw(magic = 0x88u8)]
    L2I,
    #[brw(magic = 0x89u8)]
    L2F,
    #[brw(magic = 0x8au8)]
    L2D,
    #[brw(magic = 0x8bu8)]
    F2I,
    #[brw(magic = 0x8cu8)]
    F2L,
    #[brw(magic = 0x8du8)]
    F2D,
    #[brw(magic = 0x8eu8)]
    D2I,
    #[brw(magic = 0x8fu8)]
    D2L,
    #[brw(magic = 0x90u8)]
    D2F,
    #[brw(magic = 0x91u8)]
    I2B,
    #[brw(magic = 0x92u8)]
    I2C,
    #[brw(magic = 0x93u8)]
    I2S,
    #[brw(magic = 0x94u8)]
    LCmp,
    #[brw(magic = 0x95u8)]
    FCmpL,
    #[brw(magic = 0x96u8)]
    FCmpG,
    #[brw(magic = 0x97u8)]
    DCmpL,
    #[brw(magic = 0x98u8)]
    DCmpG,
    #[brw(magic = 0x99u8)]
    IfEq(u16),
    #[brw(magic = 0x9au8)]
    IfNe(u16),
    #[brw(magic = 0x9bu8)]
    IfLt(u16),
    #[brw(magic = 0x9cu8)]
    IfGe(u16),
    #[brw(magic = 0x9du8)]
    IfGt(u16),
    #[brw(magic = 0x9eu8)]
    IfLe(u16),
    #[brw(magic = 0x9fu8)]
    IfICmpEq(u16),
    #[brw(magic = 0xa0u8)]
    IfICmpNe(u16),
    #[brw(magic = 0xa1u8)]
    IfICmpLt(u16),
    #[brw(magic = 0xa2u8)]
    IfICmpGe(u16),
    #[brw(magic = 0xa3u8)]
    IfICmpGt(u16),
    #[brw(magic = 0xa4u8)]
    IfICmpLe(u16),
    #[brw(magic = 0xa5u8)]
    IfACmpEq(u16),
    #[brw(magic = 0xa6u8)]
    IfACmpNe(u16),
    #[brw(magic = 0xa7u8)]
    GoTo(u16),
    #[brw(magic = 0xa8u8)]
    Jsr(u16),
    #[brw(magic = 0xa9u8)]
    Ret(u8),
    // TODO: TableSwitch
    // #[brw(magic = 0xaau8)]
    // TableSwitch {
    //     #[br(align_before = 4)]
    //     default: u32,
    //     low: u32,
    //     high: u32,
    // }
    //     
    #[brw(magic = 0xabu8)]
    LookupSwitch {
        #[br(align_before = 4)]
        default: u32,
        npairs: u32,
        #[br(count = npairs)]
        pairs: Vec<(i32, i32)>,
    },
    #[brw(magic = 0xacu8)]
    IReturn,
    #[brw(magic = 0xadu8)]
    LReturn,
    #[brw(magic = 0xaeu8)]
    FReturn,
    #[brw(magic = 0xafu8)]
    DReturn,
    #[brw(magic = 0xb0u8)]
    AReturn,
    #[brw(magic = 0xb1u8)]
    Return,
    #[brw(magic = 0xb2u8)]
    GetStatic(u16),
    #[brw(magic = 0xb3u8)]
    PutStatic(u16),
    #[brw(magic = 0xb4u8)]
    GetField(u16),
    #[brw(magic = 0xb5u8)]
    PutField(u16),
    #[brw(magic = 0xb6u8)]
    InvokeVirtual(u16),
    #[brw(magic = 0xb7u8)]
    InvokeSpecial(u16),
    #[brw(magic = 0xb8u8)]
    InvokeStatic(u16),
    // TODO: Find better way to express 0
    #[brw(magic = 0xb9u8)]
    InvokeInterface(u16, u8, #[br(temp)] #[bw(calc = ())] #[brw(magic = 0u8)] ()),
    #[brw(magic = 0xbau8)]
    InvokeDynamic(u16, #[br(temp)] #[bw(calc = ())] #[brw(magic = 0u16)] ()),
    #[brw(magic = 0xbbu8)]
    New(u16),
    #[brw(magic = 0xbcu8)]
    NewArray(AType),
    #[brw(magic = 0xbdu8)]
    ANewArray(u16),
    #[brw(magic = 0xbeu8)]
    ArrayLength,
    #[brw(magic = 0xbfu8)]
    AThrow,
    #[brw(magic = 0xc0u8)]
    CheckCast(u16),
    #[brw(magic = 0xc1u8)]
    InstanceOf(u16),
    #[brw(magic = 0xc2u8)]
    MonitorEnter,
    #[brw(magic = 0xc3u8)]
    MonitorExit,
    // TOOD: W I D E
    // Wide
    #[brw(magic = 0xc5u8)]
    MultiANewArray(u16, u8),
    #[brw(magic = 0xc6u8)]
    IfNull(u16),
    #[brw(magic = 0xc7u8)]
    IfNonNull(u16),
    #[brw(magic = 0xc8u8)]
    GotoW(u32),
    #[brw(magic = 0xc9u8)]
    JsrW(u32),
}

#[binrw]
#[brw(repr = u8)]
pub enum AType {
    Boolean = 4,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
}
