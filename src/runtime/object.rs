use std::ptr;
use std::fmt;

use crate::runtime::JvmValue;
use crate::runtime::OtField;
use crate::runtime::OtKlass;

#[derive(Copy, Clone)]
pub struct OtObj {
    mark: u64,
    klass: *const OtKlass,
}

impl OtObj {
    pub fn of(klass: &OtKlass) -> OtObj {
        OtObj {
            mark: 0u64,
            klass: klass,
        }
    }

    pub fn put_field(&self, _f: OtField, _val: JvmValue) -> () {}

    pub fn get_null() -> OtObj {
        OtObj {
            mark: 0u64,
            klass: ptr::null(),
        }
    }

    pub fn is_null(&self) -> bool {
        if self.mark == 0u64 && self.klass == ptr::null() {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for OtObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe { write!(f, "MarK: {} ; Klass: {}", self.mark, *self.klass) }
    }
}
