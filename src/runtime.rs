use std::fmt;

#[derive(Copy, Clone)]
pub enum JVMValue {
    Boolean { val: bool },
    Byte { val: i8 },
    Short { val: i16 },
    Int { val: i32 },
    Long { val: i64 },
    Float { val: f32 },
    Double { val: f64 },
    Char { val: char },
    ObjRef { val: JVMObj },
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
            JVMValue::Char { val: _ } => 'C',
            JVMValue::ObjRef { val: _ } => 'A',
        }
    }
}

impl fmt::Display for JVMValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JVMValue::Boolean { val: v } => write!(f, "{}", v),
            JVMValue::Byte { val: v } => write!(f, "{}", v),
            JVMValue::Short { val: v } => write!(f, "{}", v),
            JVMValue::Int { val: v } => write!(f, "{}", v),
            JVMValue::Long { val: v } => write!(f, "{}", v),
            JVMValue::Float { val: v } => write!(f, "{}", v),
            JVMValue::Double { val: v } => write!(f, "{}", v),
            JVMValue::Char { val: v } => write!(f, "{}", v),
            JVMValue::ObjRef { val: v } => write!(f, "{}", v),
        }
    }
}

#[derive(Copy, Clone)]
pub struct JVMObj {
    mark: u64,
    klassid: u32, // FIXME: This should become a pointer at some point
}

impl JVMObj {
    pub fn put_field(&self, _f: ot_field, _val: JVMValue) -> () {}

    pub fn get_null() -> JVMObj {
        JVMObj {
            mark: 0u64,
            klassid: 0u32,
        }
    }

    pub fn is_null(&self) -> bool {
        if self.mark == 0u64 && self.klassid == 0u32 {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for JVMObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MarK: {} ; Klass: {}", self.mark, self.klassid)
    }
}

pub struct ot_klass {
    name: String,
    super_name: String,
}

impl ot_klass {
    pub fn of(klass_name: String, super_klass: String) -> ot_klass {
        ot_klass {
            name: klass_name,
            super_name: super_klass,
        }
    }

    // FIXME: Shouldn't this be ot_field for consistency
    pub fn set_static_field(&self, _f: String, _vals: JVMValue) -> () {}

    pub fn get_name(&mut self) -> String {
        self.name.to_owned()
    }

    pub fn get_super_name(&mut self) -> String {
        self.super_name.to_owned()
    }
}

pub struct ot_method {}

pub struct ot_field {}

impl ot_field {
    pub fn get_name(&self) -> String {
        String::from("")
    }

    pub fn get_klass(&self) -> ot_klass {
        return ot_klass {
            name: "DUMMY_CLASS".to_string(),
            super_name: "DUMMY_SUPER".to_string(),
        };
    }
}

pub struct interp_eval_stack {
    stack: Vec<JVMValue>,
}

impl interp_eval_stack {
    pub fn new() -> interp_eval_stack {
        interp_eval_stack { stack: Vec::new() }
    }

    pub fn push(&mut self, val: JVMValue) -> () {
        let s = &mut self.stack;
        s.push(val);
    }

    pub fn pop(&mut self) -> JVMValue {
        let s = &mut self.stack;
        match s.pop() {
            Some(value) => value,
            None => panic!("pop() on empty stack"),
        }
    }

    pub fn aconst_null(&mut self) -> () {
        self.push(JVMValue::ObjRef {
            val: JVMObj::get_null(),
        });
    }

    pub fn iconst(&mut self, v: i32) -> () {
        self.push(JVMValue::Int { val: v });
    }

    pub fn iadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JVMValue::Int { val: i1 + i2 });
    }

    pub fn isub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JVMValue::Int { val: i1 - i2 });
    }
    pub fn imul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JVMValue::Int { val: i1 * i2 });
    }

    pub fn irem(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JVMValue::Int { val: i2 % i1 });
    }
    pub fn ixor(&self) -> () {}
    pub fn idiv(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JVMValue::Int { val: i2 / i1 });
    }
    pub fn iand(&self) -> () {}
    pub fn ineg(&mut self) -> () {
        let i1 = match self.pop() {
            JVMValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JVMValue::Int { val: -i1 });
    }
    pub fn ior(&self) -> () {}

    pub fn dadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JVMValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JVMValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JVMValue::Double { val: i1 + i2 });
    }
    pub fn dsub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JVMValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JVMValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JVMValue::Double { val: i1 - i2 });
    }
    pub fn dmul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JVMValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JVMValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JVMValue::Double { val: i1 * i2 });
    }

    pub fn dconst(&mut self, v: f64) -> () {
        self.push(JVMValue::Double { val: v });
    }

    pub fn i2d(&self) -> () {}
    pub fn dup(&mut self) -> () {
        let i1 = self.pop();
        self.push(i1.to_owned());
        self.push(i1.to_owned());
    }
    pub fn dupX1(&mut self) -> () {
        let i1 = self.pop();
        let i2 = self.pop();
        self.push(i1);
        self.push(i2);
        self.push(i1);
    }
}

pub struct LocalVariableTable {}

impl LocalVariableTable {
    pub fn iload(&self, _idx: u8) -> JVMValue {
        // FIXME Type checks...
        JVMValue::Int { val: 1 }
    }

    pub fn store(&self, _idx: u8, _val: JVMValue) -> () {
        // FIXME Load from LVT
    }

    pub fn iinc(&self, _idx: u8, _incr: u8) -> () {}

    pub fn dload(&self, _idx: u8) -> crate::runtime::JVMValue {
        JVMValue::Double { val: 0.001 }
    }

    pub fn aload(&self, _idx: u8) -> crate::runtime::JVMValue {
        // FIXME Load from LVT
        JVMValue::ObjRef {
            val: JVMObj::get_null(),
        }
    }

    pub fn astore(&self, _idx: u8, _val: JVMValue) -> () {}
}

pub struct ClassRepository {}

impl ClassRepository {
    pub fn new() -> ClassRepository {
        ClassRepository {}
    }

    // FIXME: Indexes should be u16
    pub fn lookup_field(&self, _klass_name: &String, _idx: u16) -> ot_field {
        ot_field {}
    }

    // FIXME: Indexes should be u16
    pub fn lookup_method_exact(&self, _klass_name: &String, _idx: u16) -> ot_method {
        ot_method {}
    }

    // FIXME: Indexes should be u16
    pub fn lookup_method_virtual(&self, _klass_name: &String, _idx: u16) -> ot_method {
        ot_method {}
    }

    pub fn lookup_klass(&self, _klass_name: &String, _idx: u16) -> ot_klass {
        ot_klass {
            name: "DUMMY_CLASS".to_string(),
            super_name: "DUMMY_SUPER".to_string(),
        }
    }
}

pub struct SimpleLinkedJVMHeap {}

impl SimpleLinkedJVMHeap {
    pub fn allocate_obj(&self, _klass: ot_klass) -> JVMObj {
        // FIXME
        JVMObj::get_null()
    }
}
