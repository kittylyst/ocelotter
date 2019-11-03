use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex};

use regex::Regex;

#[macro_use]
extern crate lazy_static;

pub mod constant_pool;
pub mod interp_stack;
pub mod klass_parser;
pub mod native_methods;
pub mod object;
pub mod otfield;
pub mod otklass;
pub mod otmethod;
pub mod simple_heap;

use crate::simple_heap::SharedSimpleHeap;
use constant_pool::CpEntry;
use object::OtObj;
use ocelotter_util::file_to_bytes;
use otfield::OtField;
use otklass::OtKlass;
use otmethod::OtMethod;
use crate::constant_pool::ACC_NATIVE;

lazy_static! {
    pub static ref HEAP: Mutex<SharedSimpleHeap> = Mutex::new(SharedSimpleHeap::of());
}

//////////// RUNTIME JVM VALUES

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

//////////// LOCAL VARS

// Keep this here for now, move to separate file as and when it gets bigger

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

//////////// SHARED RUNTIME KLASS REPO

#[derive(Debug)]
pub struct SharedKlassRepo {
    klass_count: AtomicUsize,
    klass_lookup: RefCell<HashMap<String, usize>>,
    id_lookup: RefCell<HashMap<usize, OtKlass>>,
}

impl SharedKlassRepo {
    pub fn of() -> SharedKlassRepo {
        SharedKlassRepo {
            klass_lookup: RefCell::new(HashMap::new()),
            id_lookup: RefCell::new(HashMap::new()),
            klass_count: AtomicUsize::new(1),
        }
    }

    pub fn lookup_klass(&self, klass_name: &String) -> OtKlass {
        // let s = format!("{}", self);
        // dbg!(s);

        let m = self.klass_lookup.borrow();
        let kid = match m.get(klass_name) {
            Some(id) => id,
            None => panic!("No klass called {} found in repo", klass_name),
        };
        let mi = self.id_lookup.borrow();
        match mi.get(kid) {
            Some(value) => value.clone(),
            None => panic!("No klass with ID {} found in repo", kid),
        }
    }

    pub fn add_klass(&self, k: &OtKlass) -> () {
        k.set_id(self.klass_count.fetch_add(1, Ordering::SeqCst));
        let id = k.get_id();
        let k2: OtKlass = (*k).to_owned();

        let mut m = self.klass_lookup.borrow_mut();
        m.insert(k.get_name().clone(), id);
        let mut mi = self.id_lookup.borrow_mut();
        mi.insert(id, k2);
    }

    fn add_bootstrap_class(&self, cl_name: String) -> OtKlass {
        let fq_klass_fname = "./resources/lib/".to_owned() + &cl_name + ".class";
        let bytes = match file_to_bytes(Path::new(&fq_klass_fname)) {
            Ok(buf) => buf,
            _ => panic!("Error reading file {}", fq_klass_fname),
        };
        let mut parser = crate::klass_parser::OtKlassParser::of(bytes, cl_name.clone());
        parser.parse();
        parser.klass()
    }

    fn run_clinit_method(&mut self, k : &OtKlass, i_callback: fn(&mut SharedKlassRepo, &OtMethod, &mut InterpLocalVars) -> Option<JvmValue>) {
        let klass_name = k.get_name();
        let m_str: String = klass_name.clone() + ".<clinit>:()V";
        let clinit = match k.get_method_by_name_and_desc(&m_str) {
            Some(value) => value.clone(),
            // FIXME Make this a clean exit
            None => panic!("Error: Clinit method not found {}", klass_name),
        };
        // FIXME Parameter passing
        let mut vars = InterpLocalVars::of(5);
        i_callback(self, &clinit, &mut vars);
    }

    // FIXME This should be changed to read in an ocelot-rt.jar (a cut down full RT)
    // and add each class one by one before fixing up the native code that we have working
//  (repo: SharedKlassRepo, meth: &OtMethod, lvt: &mut InterpLocalVars) -> Option<JvmValue>

    pub fn bootstrap(&mut self, i_callback: fn(&mut SharedKlassRepo, &OtMethod, &mut InterpLocalVars) -> Option<JvmValue>) -> () {
        // Add java.lang.Object
        let k_obj = self.add_bootstrap_class("java/lang/Object".to_string());
        // let s = format!("{}", self);
        // dbg!(s);

        // Add j.l.O native methods (e.g. hashCode())
        k_obj.set_native_method(
            "java/lang/Object.hashCode:()I".to_string(),
            crate::native_methods::java_lang_Object__hashcode,
        );
        k_obj.set_native_method(
            "java/lang/Object.registerNatives:()V".to_string(),
            crate::native_methods::java_lang_Object__registerNatives,
        );
        self.add_klass(&k_obj);
        // FIXME Must reset the value set for the klass repo before clinit
        self.run_clinit_method(&k_obj, i_callback);

        // FIXME Add primitive arrays

        // Add boxed classes
        let k_jli = self.add_bootstrap_class("java/lang/Integer".to_string());
        self.add_klass(&k_jli);

        let k_jlic = self.add_bootstrap_class("java/lang/Integer$IntegerCache".to_string());
        self.add_klass(&k_jlic);

        // FIXME Other classes

        // Add java.lang.String
        let k_jls = self.add_bootstrap_class("java/lang/String".to_string());
        // FIXME String only has intern() as a native method, skip for now
        self.add_klass(&k_jls);

        // Add java.lang.StringBuilder
        let k_jlsb = self.add_bootstrap_class("java/lang/StringBuilder".to_string());
        self.add_klass(&k_jlsb);

        // FIXME Add java.lang.Class

        // FIXME Add class objects for already bootstrapped classes

        // Add java.lang.System
        let k_sys = self.add_bootstrap_class("java/lang/System".to_string());
        k_sys.set_native_method(
            "java/lang/System.currentTimeMillis:()J".to_string(),
            crate::native_methods::java_lang_System__currentTimeMillis,
        );
        self.add_klass(&k_sys);

        // TODO Dummy up enough of java.io.PrintStream to get System.out.println() to work
        // By faking up the class so that println(Ljava/lang/Object;) fwds to native code
        // k_obj = self.add_bootstrap_class("java/io/PrintStream".to_string());
        // k_obj.set_native_method(
        //     "println:(Ljava/lang/Object;)V".to_string(),
        //     crate::native_methods::java_io_PrintStream__println,
        // );
    }

    pub fn klass_name_from_fq(klass_name: &String) -> String {
        lazy_static! {
            static ref KLASS_NAME: Regex =
                Regex::new("((?:([a-zA-Z_$][a-zA-Z\\d_$]*(?:/[a-zA-Z_$][a-zA-Z\\d_$]*)*)/)?([a-zA-Z_$][a-zA-Z\\d_$]*))\\.").unwrap();
        }
        let caps = KLASS_NAME.captures(klass_name).unwrap();
        // Capture the package name and the class name via the use of a nexted group
        caps.get(1).map_or("".to_string(), |m| m.as_str().to_string())
    }

    pub fn klass_name_from_dotted_fq(klass_name: &String) -> String {
        lazy_static! {
            static ref KLASS_NAME_DOTTED: Regex = 
                Regex::new("(?:([a-zA-Z_$][a-zA-Z\\d_$]*(?:\\.[a-zA-Z_$][a-zA-Z\\d_$]*)*)\\.)?([a-zA-Z_$][a-zA-Z\\d_$]*)").unwrap();
        }
        let caps = KLASS_NAME_DOTTED.captures(klass_name).unwrap();
        // In dotted syntax the field / method name comes after the final dot, hence no nested group
        caps.get(1).map_or("".to_string(), |m| m.as_str().to_string())
    }

    pub fn lookup_static_field(&self, klass_name: &String, idx: u16) -> OtField {
        let current_klass = self.lookup_klass(klass_name);

        // Lookup the Fully-Qualified field name from the CP index
        let fq_name_desc = current_klass.cp_as_string(idx);
        let target_klass_name = &SharedKlassRepo::klass_name_from_fq(&fq_name_desc);
        let target_klass = self.lookup_klass(&target_klass_name);

        let opt_f = target_klass.get_static_field_by_name_and_desc(&fq_name_desc);

        match opt_f {
            Some(f) => f.clone(),
            None => panic!(
                "No static field {} found on klass {} ",
                fq_name_desc.clone(),
                target_klass_name
            ),
        }
    }

    pub fn lookup_instance_field(&self, klass_name: &String, idx: u16) -> OtField {
        let current_klass = self.lookup_klass(klass_name);

        // Lookup the Fully-Qualified field name from the CP index
        let fq_name_desc = current_klass.cp_as_string(idx);
        let target_klass_name = &SharedKlassRepo::klass_name_from_fq(&fq_name_desc);
        let target_klass = self.lookup_klass(&target_klass_name);

        let opt_f = target_klass.get_instance_field_by_name_and_desc(&fq_name_desc);

        match opt_f {
            Some(f) => f.clone(),
            None => panic!(
                "No instance field {} found on klass {} ",
                fq_name_desc.clone(),
                target_klass_name
            ),
        }
    }

    // FIXME Lookup offset properly
    pub fn get_field_offset(&self, kid: usize, f: OtField) -> usize {
        0
    }

    pub fn put_static(&self, klass_name: String, f: OtField, v: JvmValue) -> () {
        // FIXME Handle storage properly
    }

    pub fn lookup_method_exact(&self, klass_name: &String, fq_name_desc: String) -> OtMethod {
        let m = self.klass_lookup.borrow();
        let kid = match m.get(klass_name) {
            Some(id) => id,
            None => panic!("No klass called {} found in repo", klass_name),
        };
        let mi = self.id_lookup.borrow();
        let opt_meth = match mi.get(kid) {
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
        let m = self.klass_lookup.borrow();
        let kid = match m.get(klass_name) {
            Some(id) => id,
            None => panic!("No klass called {} found in repo", klass_name),
        };
        let mi = self.id_lookup.borrow();
        match mi.get(kid) {
            Some(k) => k.get_method_by_offset_virtual(m_idx),
            None => panic!("No klass with ID {} found in repo", kid),
        }
    }

}

//     klass_lookup: HashMap<String, usize>,
//    id_lookup: HashMap<usize, OtKlass>,
impl fmt::Display for SharedKlassRepo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} with klasses {:?}",
            self.klass_count, self.id_lookup
        )
    }
}

impl Clone for SharedKlassRepo {
    fn clone(&self) -> SharedKlassRepo {
        SharedKlassRepo {
            klass_lookup: self.klass_lookup.clone(),
            id_lookup: self.id_lookup.clone(),
            klass_count: AtomicUsize::new(self.klass_count.fetch_add(0, Ordering::SeqCst)),
        }
    }
}


/////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests;
