use std::cell::Cell;
use std::fmt;

use crate::JvmValue;
use crate::OtField;

// If we need this, we'd better impl it manually
// #[derive(Debug)]
pub enum OtObj {
    VmObj {
        id: usize,
        mark: u64,
        klassid: usize,
        fields: Vec<Cell<JvmValue>>,
    },
    VmArrInt {
        id: usize,
        mark: u64,
        klassid: usize,
        length: i32,
        elements: Vec<i32>,
    },
    VmArrLong {
        id: usize,
        mark: u64,
        klassid: usize,
        length: i32,
        elements: Vec<i64>,
    },
}

impl OtObj {
    pub fn obj_of(klass_id: usize, obj_id: usize, initial: Vec<JvmValue>) -> OtObj {
        OtObj::VmObj {
            id: obj_id,
            mark: 0u64,
            klassid: klass_id,
            fields: initial.into_iter().map(|s| Cell::new(s)).collect(),
        }
    }

    pub fn int_arr_of(size: i32, obj_id: usize) -> OtObj {
        let sz = size as usize;
        let mut elts = Vec::with_capacity(sz);
        elts.resize(sz, 0);
        OtObj::VmArrInt {
            id: obj_id,
            mark: 0u64,
            klassid: 2, // FIXME Need Object in the mix soon...
            length: size,
            elements: elts,
        }
    }

    pub fn put_field(&self, offset: usize, val: JvmValue) -> () {
        let (kid, fields) = match self {
            OtObj::VmObj {
                id: _,
                mark: _,
                klassid: id,
                fields: fs,
            } => (id, fs),
            _ => panic!("Not an object"),
        };
        // Get klass
        dbg!("Made it to object get_field_offset");
        // Lookup offset in klass
        // let offset = REPO.lock().get_field_offset(*kid, f);
        match self {
            OtObj::VmObj {
                id: _,
                mark: _,
                klassid: _,
                fields: fs,
            } => {
                fs[offset].set(val);
            }
            _ => panic!("Not an object"),
        };
    }

    pub fn get_field_value(&self, offset: usize) -> JvmValue {
        let (kid, fields) = match self {
            OtObj::VmObj {
                id: _,
                mark: _,
                klassid: id,
                fields: fs,
            } => (id, fs),
            _ => panic!("Not an object"),
        };
        // Get klass
        dbg!("Made it to object get_field_offset");
        match fields.get(offset) {
            Some(v) => {
                // let v = cell.get();
                (*v).get()
            }
            None => panic!("Fields should hold a value"),
        }
    }

    pub fn get_null() -> OtObj {
        OtObj::VmObj {
            id: 0,
            mark: 0u64,
            klassid: 0, // klassid of 0 implies null
            fields: Vec::new(),
        }
    }

    pub fn is_null(&self) -> bool {
        if self.get_mark() == 0u64 && self.get_klassid() == 0 {
            true
        } else {
            false
        }
    }

    pub fn get_id(&self) -> usize {
        match *self {
            OtObj::VmObj {
                id: i,
                mark: _,
                klassid: _,
                fields: _,
            } => i,
            OtObj::VmArrInt {
                id: i,
                mark: _,
                klassid: _,
                length: _,
                elements: _,
            } => i,
            OtObj::VmArrLong {
                id: i,
                mark: _,
                klassid: _,
                length: _,
                elements: _,
            } => i,
        }
    }

    pub fn get_mark(&self) -> u64 {
        match *self {
            OtObj::VmObj {
                id: _,
                mark: m,
                klassid: _,
                fields: _,
            } => m,
            OtObj::VmArrInt {
                id: _,
                mark: m,
                klassid: _,
                length: _,
                elements: _,
            } => m,
            OtObj::VmArrLong {
                id: _,
                mark: m,
                klassid: _,
                length: _,
                elements: _,
            } => m,
        }
    }

    pub fn get_klassid(&self) -> usize {
        match *self {
            OtObj::VmObj {
                id: _,
                mark: _,
                klassid: k,
                fields: _,
            } => k,
            OtObj::VmArrInt {
                id: _,
                mark: _,
                klassid: k,
                length: _,
                elements: _,
            } => k,
            OtObj::VmArrLong {
                id: _,
                mark: _,
                klassid: k,
                length: _,
                elements: _,
            } => k,
        }
    }

    pub fn length(&self) -> i32 {
        match *self {
            OtObj::VmObj {
                id: _,
                mark: _,
                klassid: _,
                fields: _,
            } => panic!("Attempted to take the length of a normal object!"),
            OtObj::VmArrInt {
                id: _,
                mark: _,
                klassid: _,
                length: l,
                elements: _,
            } => l,
            OtObj::VmArrLong {
                id: _,
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
