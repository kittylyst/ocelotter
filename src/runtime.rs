pub enum JVMValue {
    Boolean { val: bool },
    Byte { val: i8 },
    Short { val: i16 },
    Int { val: i32 },
    Long { val: i64 },
    Float { val: f32 },
    Double { val: f64 },
    Char,
    ObjRef,
}

impl JVMValue {
    fn name(&self) -> char {
        match *self {
            JVMValue::Boolean { val } => 'Z',
            JVMValue::Byte { val } => 'B',
            JVMValue::Short { val } => 'S',
            JVMValue::Int { val } => 'I',
            JVMValue::Long { val } => 'J',
            JVMValue::Float { val } => 'F',
            JVMValue::Double { val } => 'D',
            JVMValue::Char => 'C',
            JVMValue::ObjRef => 'A',
        }
    }
}

pub struct JVMObj {}

impl JVMObj {
    pub fn putField(&self, f: OCField, val: JVMValue) -> () {}
}

pub struct OCKlass {}

impl OCKlass {
    // FIXME: Shouldn't this be OCField for consistency
    pub fn setStaticField(&self, f: String, vals: JVMValue) -> () {}

    pub fn getName(&self) -> String {
        String::from("")
    }
}

pub enum OCMethod {}

pub struct OCField {}

impl OCField {
    pub fn getName(&self) -> String {
        String::from("")
    }

    pub fn getKlass(&self) -> OCKlass {
        return OCKlass {};
    }
}

pub struct EvaluationStack {}

impl EvaluationStack {
    // pub fn new()
}

pub struct LocalVariableTable {}

impl LocalVariableTable {
    pub fn iload(&self, idx: u8) -> JVMValue {
        return JVMValue::Int { val: 1 };
    }

    pub fn store(&self, idx: u8, val: JVMValue) -> () {}

    pub fn iinc(&self, idx: u8, incr: u8) -> () {}

    pub fn dload(&self, idx: u8) -> JVMValue {
        return JVMValue::Double { val: 0.001 };
    }

    pub fn aload(&self, idx: u8) -> JVMValue {
        return JVMValue::ObjRef {};
    }

    pub fn astore(&self, idx: u8, val: JVMValue) -> () {}
}

pub struct ClassRepository {}

impl ClassRepository {
    pub fn new() -> ClassRepository {
        ClassRepository {}
    }

    pub fn lookupField(&self, klass_name : String, idx : u16) -> OCField {
        return OCField {};
    }
}
