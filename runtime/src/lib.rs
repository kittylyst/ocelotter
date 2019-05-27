use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use std::path::Path;

use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref CONTEXT: Mutex<VmContext> = Mutex::new(VmContext::of());
}

use ocelotter_util::file_to_bytes;

//////////// RUNTIME VALUES

#[derive(Clone, Debug)]
pub enum JvmValue {
    Boolean { val: bool },
    Byte { val: i8 },
    Short { val: i16 },
    Int { val: i32 },
    Long { val: i64 },
    Float { val: f32 },
    Double { val: f64 },
    Char { val: char },
    ObjRef { val: usize }, // Access objects by id
}

impl JvmValue {
    fn name(&self) -> char {
        match *self {
            JvmValue::Boolean { val: _ } => 'Z',
            JvmValue::Byte { val: _ } => 'B',
            JvmValue::Short { val: _ } => 'S',
            JvmValue::Int { val: _ } => 'I',
            JvmValue::Long { val: _ } => 'J',
            JvmValue::Float { val: _ } => 'F',
            JvmValue::Double { val: _ } => 'D',
            JvmValue::Char { val: _ } => 'C',
            JvmValue::ObjRef { val: _ } => 'A',
        }
    }
}

impl fmt::Display for JvmValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JvmValue::Boolean { val: v } => write!(f, "{}", v),
            JvmValue::Byte { val: v } => write!(f, "{}", v),
            JvmValue::Short { val: v } => write!(f, "{}", v),
            JvmValue::Int { val: v } => write!(f, "{}", v),
            JvmValue::Long { val: v } => write!(f, "{}", v),
            JvmValue::Float { val: v } => write!(f, "{}", v),
            JvmValue::Double { val: v } => write!(f, "{}", v),
            JvmValue::Char { val: v } => write!(f, "{}", v),
            JvmValue::ObjRef { val: v } => write!(f, "{}", v.clone()),
        }
    }
}

impl Default for JvmValue {
    fn default() -> JvmValue {
        JvmValue::Int { val: 0i32 }
    }
}

//////////// RUNTIME STACKS AND LOCAL VARS

pub struct InterpEvalStack {
    stack: Vec<JvmValue>,
}

impl InterpEvalStack {
    pub fn of() -> InterpEvalStack {
        InterpEvalStack { stack: Vec::new() }
    }

    pub fn push(&mut self, val: JvmValue) -> () {
        let s = &mut self.stack;
        s.push(val);
    }

    pub fn pop(&mut self) -> JvmValue {
        let s = &mut self.stack;
        match s.pop() {
            Some(value) => value,
            None => panic!("pop() on empty stack"),
        }
    }

    pub fn aconst_null(&mut self) -> () {
        self.push(JvmValue::ObjRef {
            val: 0, // OtObj::get_null(),
        });
    }

    pub fn iconst(&mut self, v: i32) -> () {
        self.push(JvmValue::Int { val: v });
    }

    pub fn iadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i1 + i2 });
    }

    pub fn isub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i1 - i2 });
    }
    pub fn imul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i1 * i2 });
    }

    pub fn irem(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i2 % i1 });
    }
    pub fn ixor(&self) -> () {}
    pub fn idiv(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i2 / i1 });
    }
    pub fn iand(&self) -> () {}
    pub fn ineg(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Int { val: -i1 });
    }
    pub fn ior(&self) -> () {}

    pub fn dadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Double { val: i1 + i2 });
    }
    pub fn dsub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Double { val: i1 - i2 });
    }
    pub fn dmul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Double { val: i1 * i2 });
    }

    pub fn dconst(&mut self, v: f64) -> () {
        self.push(JvmValue::Double { val: v });
    }

    pub fn i2d(&self) -> () {}
    pub fn dup(&mut self) -> () {
        let i1 = self.pop();
        self.push(i1.to_owned());
        self.push(i1.to_owned());
    }
    pub fn dupX1(&mut self) -> () {
        let i1 = self.pop();
        let i1c = i1.clone();
        let i2 = self.pop();
        self.push(i1);
        self.push(i2);
        self.push(i1c);
    }
}

pub struct InterpLocalVars {
    lvt: Vec<JvmValue>,
}

impl InterpLocalVars {
    pub fn of(var_count: u8) -> InterpLocalVars {
        let mut out = InterpLocalVars { lvt: Vec::new() };
        for i in 0..var_count {
            out.lvt.push(JvmValue::default());
        }

        out
    }

    pub fn load(&self, idx: u8) -> JvmValue {
        self.lvt[idx as usize].clone()
    }

    pub fn store(&mut self, idx: u8, val: JvmValue) -> () {
        self.lvt[idx as usize] = val
    }

    pub fn iinc(&mut self, idx: u8, incr: u8) -> () {
        match self.lvt[idx as usize] {
            JvmValue::Int { val: v } => {
                self.lvt[idx as usize] = JvmValue::Int { val: v + 1 };
            }
            _ => panic!("Non-integer value encountered in IINC of local var {}", idx),
        }
    }
}

//////////// SHARED RUNTIME STRUCTURES

pub struct VmContext {
    heap: SharedSimpleHeap,
    repo: SharedKlassRepo,
}

impl VmContext {
    pub fn of() -> VmContext {
        VmContext {
            heap: SharedSimpleHeap::of(),
            repo: SharedKlassRepo::of(),
        }
    }

    pub fn get_repo(&mut self) -> &mut SharedKlassRepo {
        &mut self.repo
    }

    pub fn get_heap(&mut self) -> &mut SharedSimpleHeap {
        &mut self.heap
    }
}

#[derive(Debug)]
pub struct SharedKlassRepo {
    klass_count: AtomicUsize,
    klass_lookup: HashMap<String, usize>,
}

impl SharedKlassRepo {
    pub fn of() -> SharedKlassRepo {
        SharedKlassRepo {
            klass_lookup: HashMap::new(),
            klass_count: AtomicUsize::new(1),
        }
    }

}

pub struct SharedSimpleHeap {
    obj_count: AtomicUsize,
}

impl SharedSimpleHeap {
    pub fn of() -> SharedSimpleHeap {
        let mut out = SharedSimpleHeap {
            obj_count: AtomicUsize::new(1),
        };
        out
    }

}

#[cfg(test)]
mod tests;
