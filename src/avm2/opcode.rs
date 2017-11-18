enum_from_primitive! {
#[allow(dead_code)]
#[derive(Debug,PartialEq,Clone,Copy)]
pub enum OpCode {
    Add = 0xA0,
    AddI = 0xC5,
    AsType = 0x86,
    AsTypeLate = 0x87,
    BitAnd = 0xA8,
    BitNot = 0x97,
    BitOr = 0xa9,
    BitXor = 0xaa,
    Call = 0x41,
    CallMethod = 0x43,
    CallProperty = 0x46,
    CallPropLex = 0x4c,
    CallPropVoid = 0x4f,
    CallStatic = 0x44,
    CallSuper = 0x45,
    CallSuperVoid = 0x4e,
    CheckFilter = 0x78,
    Coerce = 0x80,
    CoerceA = 0x82,
    CoerceS = 0x85,
    Construct = 0x42,
    ConstructProp = 0x4a,
    ConstructSuper = 0x49,
    ConvertB = 0x76,
    ConvertI = 0x73,
    ConvertD = 0x75,
    ConvertO = 0x77,
    ConvertU = 0x74,
    ConvertS = 0x70,
    Debug = 0xef,
    DebugFile = 0xf1,
    DebugLine = 0xf0,
    DecLocal = 0x94,
    DecLocalI = 0xc3,
    Decrement = 0x93,
    DecrementI = 0xc1,
    DeleteProperty = 0x6a,
    Divide = 0xa3,
    Dup = 0x2a,
    Dxns = 0x06,
    DxnsLate = 0x07,
    Equals = 0xab,
    EscXAttr = 0x72,
    EscXElem = 0x71,
    FindProperty = 0x5e,
    FindPropStrict = 0x5d,
    GetDescendants = 0x59,
    GetGlobalScope = 0x64,
    GetGlobalSlot = 0x6e,
    GetLex = 0x60,
    GetLocal = 0x62,
    GetLocal0 = 0xd0,
    GetLocal1 = 0xd1,
    GetLocal2 = 0xd2,
    GetLocal3 = 0xd3,
    GetProperty = 0x66,
    GetScopeObject = 0x65,
    GetSlot = 0x6c,
    GetSuper = 0x04,
    GreaterEquals = 0xb0, // Listed incorrectly in AVM2 specs.
    GreaterThan = 0xaf,
    HasNext = 0x1f,
    HasNext2 = 0x32,
    IfEq = 0x13,
    IfFalse = 0x12,
    IfGe = 0x18,
    IfGt = 0x17,
    IfLe = 0x16,
    IfLt = 0x15,
    IfNge = 0x0f,
    IfNgt = 0x0e,
    IfNle = 0x0d,
    IfNlt = 0x0c,
    IfNe = 0x14,
    IfStrictEq = 0x19,
    IfStrictNe = 0x1a,
    IfTrue = 0x11,
    In = 0xb4,
    IncLocal = 0x92,
    IncLocalI = 0xc2,
    Increment = 0x91,
    IncrementI = 0xc0,
    InitProperty = 0x68,
    InstanceOf = 0xb1,
    IsType = 0xb2,
    IsTypeLate = 0xb3,
    Jump = 0x10,
    Kill = 0x08,
    Label = 0x09,
    LessEquals = 0xae,
    LessThan = 0xad,
    LookupSwitch = 0x1b,
    LShift = 0xa5,
    Modulo = 0xa4,
    Multiply = 0xa2,
    MultiplyI = 0xc7,
    Negate = 0x90,
    NegateI = 0xc4,
    NewActivation = 0x57,
    NewArray = 0x56,
    NewCatch = 0x5a,
    NewClass = 0x58,
    NewFunction = 0x40,
    NewObject = 0x55,
    NextName = 0x1e,
    NextValue = 0x23,
    Nop = 0x02,
    Not = 0x96,
    Pop = 0x29,
    PopScope = 0x1d,
    PushByte = 0x24,
    PushDouble = 0x2f,
    PushFalse = 0x27,
    PushInt = 0x2d,
    PushNamespace = 0x31,
    PushNaN = 0x28,
    PushNull = 0x20,
    PushScope = 0x30,
    PushShort = 0x25,
    PushString = 0x2c,
    PushTrue = 0x26,
    PushUint = 0x2e,
    PushUndefined = 0x21,
    PushWith = 0x1c,
    ReturnValue = 0x48,
    ReturnVoid = 0x47,
    RShift = 0xa6,
    SetLocal = 0x63,
    SetLocal0 = 0xd4,
    SetLocal1 = 0xd5,
    SetLocal2 = 0xd6,
    SetLocal3 = 0xd7,
    SetGlobalSlot = 0x6f,
    SetProperty = 0x61,
    SetSlot = 0x6d,
    SetSuper = 0x05,
    StrictEquals = 0xac,
    Subtract = 0xa1,
    SubtractI = 0xc6,
    Swap = 0x2b,
    Throw = 0x03,
    TypeOf = 0x95,
    URShift = 0xa7,
}
}