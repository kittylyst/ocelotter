#![deny(unreachable_patterns)]

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::interpreter::object::OtObj;
use crate::interpreter::values::JvmValue;
use crate::klass::otfield::OtField;
use crate::klass::otklass::OtKlass;

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
        let out = OtObj::obj_of(klass_id, obj_id, klass.make_default_values());
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
        match self.alloc.get(id) {
            Some(val) => val.put_field(f.get_offset() as usize, v),
            None => panic!("Error: object {} not found", id),
        };
    }

    pub fn get_field(&self, id: usize, offset: u16) -> JvmValue {
        // Get object from heap
        let obj = match self.alloc.get(id) {
            Some(val) => val,
            None => panic!("Error: object {} not found", id),
        };
        obj.get_field_value(offset as usize)
    }

    pub fn iastore(&mut self, id: usize, pos: i32, v: i32) -> () {
        let p = pos as usize;
        let obj = match self.alloc.get(id) {
            Some(val) => val,
            None => panic!("Error: object {} not found", id),
        };
        let t = match obj {
            OtObj::VmArrInt {
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
        let obj = OtObj::VmArrInt {
            id: *t.0,
            mark: *t.1,
            klassid: *t.2,
            length: elts.len() as i32,
            elements: elts,
        };
        self.alloc[id] = obj;
    }
}
