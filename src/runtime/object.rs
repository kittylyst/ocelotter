use std::fmt;
use std::ptr;

use crate::runtime::JvmValue;
use crate::runtime::OtField;
use crate::runtime::OtKlass;

#[derive(Clone, Debug)]
pub enum OtObj {
    vm_obj {
        mark: u64,
        klass: *const OtKlass,
    },
    vm_arr_int {
        mark: u64,
        klass: *const OtKlass,
        length: i32,
        elements: Vec<i32>,
    },
    vm_arr_long {
        mark: u64,
        klass: *const OtKlass,
        length: i32,
        elements: Vec<i64>,
    },
}

impl OtObj {
    pub fn of(klass: &OtKlass) -> OtObj {
        OtObj::vm_obj {
            mark: 0u64,
            klass: klass,
        }
    }

    pub fn int_arr_of(size: i32) -> OtObj {
        let sz = size as usize;
        let mut elts = Vec::with_capacity(sz);
        elts.resize(sz, 0);
        OtObj::vm_arr_int {
            mark: 0u64,
            klass: ptr::null(), // FIXME Need Object in the mix soon...
            length: size,
            elements: elts,
        }
    }

    pub fn put_field(&self, _f: OtField, _val: JvmValue) -> () {}

    pub fn get_null() -> OtObj {
        OtObj::vm_obj {
            mark: 0u64,
            klass: ptr::null(),
        }
    }

    pub fn is_null(&self) -> bool {
        if self.get_mark() == 0u64 && self.get_klass() == ptr::null() {
            true
        } else {
            false
        }
    }

    pub fn get_mark(&self) -> u64 {
        match *self {
            OtObj::vm_obj { mark: m, klass: _ } => m,
            OtObj::vm_arr_int {
                mark: m,
                klass: _,
                length: _,
                elements: _,
            } => m,
            OtObj::vm_arr_long {
                mark: m,
                klass: _,
                length: _,
                elements: _,
            } => m,
        }
    }

    pub fn get_klass(&self) -> *const OtKlass {
        match *self {
            OtObj::vm_obj { mark: _, klass: k } => k,
            OtObj::vm_arr_int {
                mark: _,
                klass: k,
                length: _,
                elements: _,
            } => k,
            OtObj::vm_arr_long {
                mark: _,
                klass: k,
                length: _,
                elements: _,
            } => k,
        }
    }

    pub fn length(&self) -> i32 {
        match *self {
            OtObj::vm_obj { mark: _, klass: _ } => {
                panic!("Attempted to take the length of a normal object!")
            }
            OtObj::vm_arr_int {
                mark: _,
                klass: _,
                length: l,
                elements: _,
            } => l,
            OtObj::vm_arr_long {
                mark: _,
                klass: _,
                length: l,
                elements: _,
            } => l,
        }
    }
}

impl fmt::Display for OtObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(
                f,
                "MarK: {} ; Klass: {}",
                self.get_mark(),
                *self.get_klass()
            )
        }
    }
}
