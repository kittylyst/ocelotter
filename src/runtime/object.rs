use std::fmt;
use std::ptr;

use crate::runtime::JvmValue;
use crate::runtime::OtField;
use crate::runtime::OtKlass;

#[derive(Clone, Copy)]
pub enum OtObj {
    vm_obj { mark: u64, klass: *const OtKlass },
    vm_arr { mark: u64, klass: *const OtKlass },
}

impl OtObj {
    pub fn of(klass: &OtKlass) -> OtObj {
        OtObj::vm_obj {
            mark: 0u64,
            klass: klass,
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
            OtObj::vm_arr { mark: m, klass: _ } => m,
        }
    }

    pub fn get_klass(&self) -> *const OtKlass {
        match *self {
            OtObj::vm_obj { mark: _, klass: k } => k,
            OtObj::vm_arr { mark: _, klass: k } => k,
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
