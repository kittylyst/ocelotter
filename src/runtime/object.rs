use std::fmt;

use crate::runtime::JvmValue;
use crate::runtime::OtField;

#[derive(Clone, Debug)]
pub enum OtObj {
    vm_obj {
        mark: u64,
        klassid: usize,
    },
    vm_arr_int {
        mark: u64,
        klassid: usize,
        length: i32,
        elements: Vec<i32>,
    },
    vm_arr_long {
        mark: u64,
        klassid: usize,
        length: i32,
        elements: Vec<i64>,
    },
}

impl OtObj {
    pub fn of(klass: usize) -> OtObj {
        OtObj::vm_obj {
            mark: 0u64,
            klassid: klass,
        }
    }

    pub fn int_arr_of(size: i32) -> OtObj {
        let sz = size as usize;
        let mut elts = Vec::with_capacity(sz);
        elts.resize(sz, 0);
        OtObj::vm_arr_int {
            mark: 0u64,
            klassid: 1, // FIXME Need Object in the mix soon...
            length: size,
            elements: elts,
        }
    }

    pub fn put_field(&self, _f: OtField, _val: JvmValue) -> () {}

    pub fn get_null() -> OtObj {
        OtObj::vm_obj {
            mark: 0u64,
            klassid: 0, // klassid of 0 implies null
        }
    }

    pub fn is_null(&self) -> bool {
        if self.get_mark() == 0u64 && self.get_klassid() == 0 {
            true
        } else {
            false
        }
    }

    pub fn get_mark(&self) -> u64 {
        match *self {
            OtObj::vm_obj {
                mark: m,
                klassid: _,
            } => m,
            OtObj::vm_arr_int {
                mark: m,
                klassid: _,
                length: _,
                elements: _,
            } => m,
            OtObj::vm_arr_long {
                mark: m,
                klassid: _,
                length: _,
                elements: _,
            } => m,
        }
    }

    pub fn get_klassid(&self) -> usize {
        match *self {
            OtObj::vm_obj {
                mark: _,
                klassid: k,
            } => k,
            OtObj::vm_arr_int {
                mark: _,
                klassid: k,
                length: _,
                elements: _,
            } => k,
            OtObj::vm_arr_long {
                mark: _,
                klassid: k,
                length: _,
                elements: _,
            } => k,
        }
    }

    pub fn length(&self) -> i32 {
        match *self {
            OtObj::vm_obj {
                mark: _,
                klassid: _,
            } => panic!("Attempted to take the length of a normal object!"),
            OtObj::vm_arr_int {
                mark: _,
                klassid: _,
                length: l,
                elements: _,
            } => l,
            OtObj::vm_arr_long {
                mark: _,
                klassid: _,
                length: l,
                elements: _,
            } => l,
        }
    }
}

impl fmt::Display for OtObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MarK: {} ; Klass: {}",
            self.get_mark(),
            self.get_klassid()
        )
    }
}
