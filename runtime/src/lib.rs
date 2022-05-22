#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use std::fmt;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

pub mod constant_pool;
pub mod interp_stack;
pub mod klass_parser;
pub mod klass_repo;
pub mod native_methods;
pub mod object;
pub mod otfield;
pub mod otklass;
pub mod otmethod;
pub mod simple_heap;

use crate::simple_heap::SharedSimpleHeap;
use object::OtObj;
use otfield::OtField;
use otklass::OtKlass;
use otmethod::OtMethod;
use klass_repo::SharedKlassRepo;

lazy_static! {
    pub static ref HEAP: Mutex<SharedSimpleHeap> = Mutex::new(SharedSimpleHeap::of());
}

//////////// RUNTIME JVM VALUES

#[derive(Clone, Debug, Copy)]
pub enum JvmValue {
    Boolean(bool),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Char(char),
    ObjRef(usize), // Access objects by id
}

macro_rules! value_as {
    ($name:ident : $ctor:ident($ty:ty)) => {
        pub fn $name(self) -> Option<$ty> {
            match self {
                Self::$ctor(v) => Some(v),
                other => None,
            }
        }
    }
}


impl JvmValue {
    pub fn name(&self) -> char {
        match *self {
            JvmValue::Boolean(_) => 'Z',
            JvmValue::Byte(_) => 'B',
            JvmValue::Short(_) => 'S',
            JvmValue::Int(_) => 'I',
            JvmValue::Long(_) => 'J',
            JvmValue::Float(_) => 'F',
            JvmValue::Double(_) => 'D',
            JvmValue::Char(_) => 'C',
            JvmValue::ObjRef(_) => 'A',
        }
    }

    pub fn default_value(letter: char) -> JvmValue {
        match letter {
            'Z' => JvmValue::Boolean(false),
            'B' => JvmValue::Byte(0),
            'S' => JvmValue::Short(0),
            'I' => JvmValue::Int(0),
            'J' => JvmValue::Long(0),
            'F' => JvmValue::Float(0.0),
            'D' => JvmValue::Double(0.0),
            'C' => JvmValue::Char('\0'),
            'A' => JvmValue::ObjRef(0),
            _   => panic!("Illegal type {} seen when trying to parse", letter)
        }
    }

    value_as!(as_bool:   Boolean(bool));
    value_as!(as_byte:   Byte(i8));
    value_as!(as_short:  Short(i16));
    value_as!(as_int:    Int(i32));
    value_as!(as_long:   Long(i64));
    value_as!(as_float:  Float(f32));
    value_as!(as_double: Double(f64));
    value_as!(as_char:   Char(char));
    value_as!(as_objref: ObjRef(usize));
}

impl fmt::Display for JvmValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JvmValue::Boolean(v) => write!(f, "{}", v),
            JvmValue::Byte(v)    => write!(f, "{}", v),
            JvmValue::Short(v)   => write!(f, "{}", v),
            JvmValue::Int(v)     => write!(f, "{}", v),
            JvmValue::Long(v)    => write!(f, "{}", v),
            JvmValue::Float(v)   => write!(f, "{}", v),
            JvmValue::Double(v)  => write!(f, "{}", v),
            JvmValue::Char(v)    => write!(f, "{}", v),
            JvmValue::ObjRef(v)  => write!(f, "{}", v),
        }
    }
}

impl Default for JvmValue {
    fn default() -> JvmValue { JvmValue::Int(0) }
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
        let val = self.lvt[idx as usize].as_int()
            .unwrap_or_else(|| panic!("Non-integer value encountered in IINC of local var {}", idx));
        self.lvt[idx as usize] = JvmValue::Int(val + 1);
    }
}

#[cfg(test)]
mod tests;
