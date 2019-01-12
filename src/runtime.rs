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
            JVMValue::Boolean { val: _ } => 'Z',
            JVMValue::Byte { val: _ } => 'B',
            JVMValue::Short { val: _ } => 'S',
            JVMValue::Int { val: _ } => 'I',
            JVMValue::Long { val: _ } => 'J',
            JVMValue::Float { val: _ } => 'F',
            JVMValue::Double { val: _ } => 'D',
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

pub struct OCMethod {}

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
    pub fn push(&self, val: JVMValue) -> () {}

    pub fn pop(&self) -> crate::runtime::JVMValue {
        JVMValue::Boolean { val: true }
    }

    pub fn aconst_null(&self) -> () {
        self.push(JVMValue::ObjRef {});
    }

    pub fn iconst(&self, v: i32) -> () {
        self.push(JVMValue::Int { val: v });
    }

    pub fn iadd(&self) -> () {}
    pub fn isub(&self) -> () {}
    pub fn imul(&self) -> () {}
    pub fn irem(&self) -> () {}
    pub fn ixor(&self) -> () {}
    pub fn idiv(&self) -> () {}
    pub fn iand(&self) -> () {}
    pub fn ineg(&self) -> () {}
    pub fn ior(&self) -> () {}

    pub fn dadd(&self) -> () {}
    pub fn dsub(&self) -> () {}
    pub fn dmul(&self) -> () {}

    pub fn dconst(&self, v: f64) -> () {
        self.push(JVMValue::Double { val: v });
    }

    pub fn i2d(&self) -> () {}
    pub fn dup(&self) -> () {}
    pub fn dupX1(&self) -> () {}
}

pub struct LocalVariableTable {}

impl LocalVariableTable {
    pub fn iload(&self, idx: u8) -> JVMValue {
        JVMValue::Int { val: 1 }
    }

    pub fn store(&self, idx: u8, val: JVMValue) -> () {}

    pub fn iinc(&self, idx: u8, incr: u8) -> () {}

    pub fn dload(&self, idx: u8) -> crate::runtime::JVMValue {
        JVMValue::Double { val: 0.001 }
    }

    pub fn aload(&self, idx: u8) -> crate::runtime::JVMValue {
        JVMValue::ObjRef{}
    }

    pub fn astore(&self, idx: u8, val: JVMValue) -> () {}
}

pub struct ClassRepository {}

impl ClassRepository {
    pub fn new() -> ClassRepository {
        ClassRepository {}
    }

    // FIXME: Indexes should be u16
    pub fn lookupField(&self, klass_name: &String, idx: u8) -> OCField {
        OCField {}
    }

    // FIXME: Indexes should be u16
    pub fn lookupMethodExact(&self, klass_name: &String, idx: u8) -> OCMethod {
        OCMethod {}
    }

    // FIXME: Indexes should be u16
    pub fn lookupMethodVirtual(&self, klass_name: &String, idx: u8) -> OCMethod {
        OCMethod {}
    }

    pub fn lookupKlass(&self, klass_name: &String, idx: u8) -> OCKlass {
        OCKlass {}
    }
}

pub struct SimpleLinkedJVMHeap {}

impl SimpleLinkedJVMHeap {
    pub fn allocateObj(&self, klass: OCKlass ) -> JVMValue {
        JVMValue::ObjRef{}
    }
}