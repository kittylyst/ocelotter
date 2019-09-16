use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::RefCell;
use std::path::Path;

use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref HEAP: Mutex<SharedSimpleHeap> = Mutex::new(SharedSimpleHeap::of());
}

lazy_static! {
    pub static ref REPO: Mutex<SharedKlassRepo> = Mutex::new(SharedKlassRepo::of());
}


pub mod constant_pool;
pub mod klass_parser;
pub mod native_methods;
pub mod object;
pub mod otfield;
pub mod otklass;
pub mod otmethod;

// use constant_pool::CpAttr;
use constant_pool::CpEntry;
use object::OtObj;
use ocelotter_util::file_to_bytes;
use otfield::OtField;
use otklass::OtKlass;
use otmethod::OtMethod;

use crate::constant_pool::ACC_NATIVE;



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

#[derive(Debug)]
pub struct SharedKlassRepo {
    klass_count: AtomicUsize,
    klass_lookup: HashMap<String, usize>,
    id_lookup: HashMap<usize, OtKlass>,
}

impl SharedKlassRepo {
    pub fn of() -> SharedKlassRepo {
        SharedKlassRepo {
            klass_lookup: HashMap::new(),
            id_lookup: HashMap::new(),
            klass_count: AtomicUsize::new(1),
        }
    }

    fn add_bootstrap_class(&mut self, cl_name: String) -> &mut OtKlass {
        let fq_klass_fname = "./resources/lib/".to_owned() + &cl_name + ".class";
        let bytes = match file_to_bytes(Path::new(&fq_klass_fname)) {
            Ok(buf) => buf,
            _ => panic!("Error reading file {}", fq_klass_fname),
        };
        let mut parser = crate::klass_parser::OtKlassParser::of(bytes, cl_name.clone());
        parser.parse();
        let mut k = parser.klass();
        self.add_klass(&mut k);
        self.lookup_mutable_klass(&cl_name)
    }

    pub fn bootstrap(&mut self) -> () {
        // Add java.lang.Object
        let mut k_obj = self.add_bootstrap_class("java/lang/Object".to_string());

        // Add j.l.O native methods (e.g. hashCode())
        k_obj.set_native_method(
            "java/lang/Object.hashCode:()I".to_string(),
            crate::native_methods::java_lang_Object__hashcode,
        );

        // FIXME Add primitive arrays

        // Add java.lang.String
        self.add_bootstrap_class("java/lang/String".to_string());
        // FIXME String only has intern() as a native method, skip for now

        // Add java.lang.StringBuilder
        self.add_bootstrap_class("java/lang/StringBuilder".to_string());

        // FIXME Add java.lang.Class

        // FIXME Add class objects for already bootstrapped classes

        // Add java.lang.System
        k_obj = self.add_bootstrap_class("java/lang/System".to_string());
        k_obj.set_native_method(
            "java/lang/System.currentTimeMillis:()J".to_string(),
            crate::native_methods::java_lang_System__currentTimeMillis,
        );

        // TODO Dummy up enough of java.io.PrintStream to get System.out.println() to work
        // By faking up the class so that println(Ljava/lang/Object;) fwds to native code
        // k_obj = self.add_bootstrap_class("java/io/PrintStream".to_string());
        // k_obj.set_native_method(
        //     "println:(Ljava/lang/Object;)V".to_string(),
        //     crate::native_methods::java_io_PrintStream__println,
        // );

        ()
    }

    // FIXME SHOULD THIS BE DONE BY INDEX OR DESC???
    pub fn lookup_field(&self, klass_name: &String, _idx: u16) -> OtField {
        let kid = match self.klass_lookup.get(klass_name) {
            Some(id) => id,
            None => panic!("No klass called {} found in repo", klass_name),
        };
        // let opt_f : Option<&OtField> = match self.id_lookup.get(kid) {
        //     Some(k) => k.get_field_by_name_and_desc(fq_name_desc.clone()),
        //     None => panic!("No klass with ID {} found in repo", kid),
        // };
        // match opt_meth {
        //     Some(k) => k.clone(),
        //     None => panic!("No method {} found on klass {} ", fq_name_desc.clone(), kid),
        // }
        // FIXME DUMMY
        OtField::of(
            "DUMMY_KLASS".to_string(),
            "DUMMY_FIELD".to_string(),
            "I".to_string(),
            0,
            1,
            2,
        )
    }

    // FIXME Lookup offset properly
    pub fn get_field_offset(&self, kid: usize, f: OtField) -> usize {
        0
    }

    pub fn put_static(&self, klass_name: String, f: OtField, v: JvmValue) -> () {
        // FIXME Handle storage properly
    }

    pub fn lookup_method_exact(&self, klass_name: &String, fq_name_desc: String) -> OtMethod {
        let kid = match self.klass_lookup.get(klass_name) {
            Some(id) => id,
            None => panic!("No klass called {} found in repo", klass_name),
        };
        let opt_meth = match self.id_lookup.get(kid) {
            Some(k) => k.get_method_by_name_and_desc(&fq_name_desc),
            None => panic!("No klass with ID {} found in repo", kid),
        };
        match opt_meth {
            Some(k) => k.clone(),
            None => panic!("No method {} found on klass {} ", fq_name_desc.clone(), kid),
        }
    }

    // m_idx is IDX in CP of current class
    pub fn lookup_method_virtual(&self, klass_name: &String, m_idx: u16) -> OtMethod {
        // Get klass
        let kid = match self.klass_lookup.get(klass_name) {
            Some(id) => id,
            None => panic!("No klass called {} found in repo", klass_name),
        };
        match self.id_lookup.get(kid) {
            Some(k) => k.get_method_by_offset_virtual(m_idx),
            None => panic!("No klass with ID {} found in repo", kid),
        }
    }

    pub fn lookup_klass(&self, klass_name: &String) -> &OtKlass {
        let kid = match self.klass_lookup.get(klass_name) {
            Some(id) => id,
            None => panic!("No klass called {} found in repo", klass_name),
        };
        match self.id_lookup.get(kid) {
            Some(value) => value,
            None => panic!("No klass with ID {} found in repo", kid),
        }
    }

    pub fn lookup_mutable_klass(&mut self, klass_name: &String) -> &mut OtKlass {
        for (id, k) in &mut self.id_lookup {
            if *k.get_name() == *klass_name {
                return k;
            }
        }
        panic!("Klass not found")
    }

    pub fn add_klass(&mut self, k: &OtKlass) -> () {
        k.set_id(self.klass_count.fetch_add(1, Ordering::SeqCst));
        let id = k.get_id();
        let k2: OtKlass = (*k).to_owned();

        self.klass_lookup.insert(k.get_name().clone(), id);
        self.id_lookup.insert(id, k2);
    }
}

pub struct SharedSimpleHeap {
    obj_count: AtomicUsize,
    // Free list
    // Alloc table
    alloc: Vec<OtObj>,
}

impl SharedSimpleHeap {
    pub fn of() -> SharedSimpleHeap {
        let mut out = SharedSimpleHeap {
            obj_count: AtomicUsize::new(1),
            alloc: Vec::new(),
        };
        let null_obj = OtObj::get_null();
        out.alloc.push(null_obj);
        out
    }

    pub fn allocate_obj(&mut self, klass: &OtKlass) -> usize {
        let klass_id = klass.get_id();
        let obj_id: usize = self.obj_count.fetch_add(1, Ordering::SeqCst);
        let out = OtObj::obj_of(klass_id, obj_id, klass.make_default());
        self.alloc.push(out);
        obj_id
    }

    pub fn allocate_int_arr(&mut self, size: i32) -> usize {
        let obj_id = self.obj_count.fetch_add(1, Ordering::SeqCst);
        let out = OtObj::int_arr_of(size, obj_id);
        self.alloc.push(out);
        obj_id
    }

    pub fn get_obj(&self, id: usize) -> &OtObj {
        match self.alloc.get(id) {
            Some(val) => val,
            None => panic!("Error: object {} not found", id),
        }
    }

    // FIXME Handle storage properly
    pub fn put_field(&self, id: usize, f: OtField, v: JvmValue) -> () {
        // Get object from heap
        // match self.alloc.get(id) {
        //     Some(val) => val.put_field(f, v),
        //     None => panic!("Error: object {} not found", id),
        // };
    }

    pub fn get_field(&self, id: usize, f: OtField) -> JvmValue {
        // Get object from heap
        let obj = match self.alloc.get(id) {
            Some(val) => val,
            None => panic!("Error: object {} not found", id),
        };
        obj.get_value(f)
    }

    pub fn iastore(&mut self, id: usize, pos: i32, v: i32) -> () {
        let p = pos as usize;
        let obj = match self.alloc.get(id) {
            Some(val) => val,
            None => panic!("Error: object {} not found", id),
        };
        let t = match obj {
            OtObj::vm_arr_int {
                id: i,
                mark: m,
                klassid: kid,
                length: _,
                elements: elts,
            } => (i, m, kid, elts),
            _ => panic!("Non-int[] seen in heap during IASTORE at {}", id),
        };
        let mut elts = t.3.clone();
        elts[pos as usize] = v;
        let obj = OtObj::vm_arr_int {
            id: *t.0,
            mark: *t.1,
            klassid: *t.2,
            length: elts.len() as i32,
            elements: elts,
        };
        self.alloc[id] = obj;
    }
}

#[cfg(test)]
mod tests;
