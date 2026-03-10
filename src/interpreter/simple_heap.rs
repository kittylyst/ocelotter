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

    pub fn allocate_long_arr(&mut self, size: i32) -> usize {
        let obj_id = self.obj_count.fetch_add(1, Ordering::SeqCst);
        let out = OtObj::long_arr_of(size, obj_id);
        self.alloc.push(out);
        obj_id
    }

    pub fn allocate_float_arr(&mut self, size: i32) -> usize {
        let obj_id = self.obj_count.fetch_add(1, Ordering::SeqCst);
        let out = OtObj::float_arr_of(size, obj_id);
        self.alloc.push(out);
        obj_id
    }

    pub fn allocate_double_arr(&mut self, size: i32) -> usize {
        let obj_id = self.obj_count.fetch_add(1, Ordering::SeqCst);
        let out = OtObj::double_arr_of(size, obj_id);
        self.alloc.push(out);
        obj_id
    }

    pub fn allocate_byte_arr(&mut self, size: i32) -> usize {
        let obj_id = self.obj_count.fetch_add(1, Ordering::SeqCst);
        let out = OtObj::byte_arr_of(size, obj_id);
        self.alloc.push(out);
        obj_id
    }

    pub fn allocate_short_arr(&mut self, size: i32) -> usize {
        let obj_id = self.obj_count.fetch_add(1, Ordering::SeqCst);
        let out = OtObj::short_arr_of(size, obj_id);
        self.alloc.push(out);
        obj_id
    }

    pub fn allocate_char_arr(&mut self, size: i32) -> usize {
        let obj_id = self.obj_count.fetch_add(1, Ordering::SeqCst);
        let out = OtObj::char_arr_of(size, obj_id);
        self.alloc.push(out);
        obj_id
    }

    pub fn allocate_ref_arr(&mut self, size: i32) -> usize {
        let obj_id = self.obj_count.fetch_add(1, Ordering::SeqCst);
        let out = OtObj::ref_arr_of(size, obj_id);
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
    pub fn put_field(&self, id: usize, f: OtField, v: JvmValue) {
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

    pub fn iastore(&mut self, id: usize, pos: i32, v: i32) {
        let idx = pos as usize;
        let obj = match self.alloc.get_mut(id) {
            Some(val) => val,
            None => panic!("Error: object {} not found", id),
        };
        match obj {
            OtObj::VmArrInt { elements, .. } => elements[idx] = v,
            _ => panic!("Non-int[] seen in heap during IASTORE at {}", id),
        }
    }

    pub fn iaload(&self, id: usize, pos: i32) -> i32 {
        let idx = pos as usize;
        match self.alloc.get(id) {
            Some(OtObj::VmArrInt { elements, .. }) => elements[idx],
            Some(_) => panic!("Non-int[] seen in heap during IALOAD at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn lastore(&mut self, id: usize, pos: i32, v: i64) {
        let idx = pos as usize;
        match self.alloc.get_mut(id) {
            Some(OtObj::VmArrLong { elements, .. }) => elements[idx] = v,
            Some(_) => panic!("Non-long[] seen in heap during LASTORE at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn laload(&self, id: usize, pos: i32) -> i64 {
        let idx = pos as usize;
        match self.alloc.get(id) {
            Some(OtObj::VmArrLong { elements, .. }) => elements[idx],
            Some(_) => panic!("Non-long[] seen in heap during LALOAD at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn fastore(&mut self, id: usize, pos: i32, v: f32) {
        let idx = pos as usize;
        match self.alloc.get_mut(id) {
            Some(OtObj::VmArrFloat { elements, .. }) => elements[idx] = v,
            Some(_) => panic!("Non-float[] seen in heap during FASTORE at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn faload(&self, id: usize, pos: i32) -> f32 {
        let idx = pos as usize;
        match self.alloc.get(id) {
            Some(OtObj::VmArrFloat { elements, .. }) => elements[idx],
            Some(_) => panic!("Non-float[] seen in heap during FALOAD at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn dastore(&mut self, id: usize, pos: i32, v: f64) {
        let idx = pos as usize;
        match self.alloc.get_mut(id) {
            Some(OtObj::VmArrDouble { elements, .. }) => elements[idx] = v,
            Some(_) => panic!("Non-double[] seen in heap during DASTORE at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn daload(&self, id: usize, pos: i32) -> f64 {
        let idx = pos as usize;
        match self.alloc.get(id) {
            Some(OtObj::VmArrDouble { elements, .. }) => elements[idx],
            Some(_) => panic!("Non-double[] seen in heap during DALOAD at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn bastore(&mut self, id: usize, pos: i32, v: i8) {
        let idx = pos as usize;
        match self.alloc.get_mut(id) {
            Some(OtObj::VmArrByte { elements, .. }) => elements[idx] = v,
            Some(_) => panic!("Non-byte[] seen in heap during BASTORE at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn baload(&self, id: usize, pos: i32) -> i8 {
        let idx = pos as usize;
        match self.alloc.get(id) {
            Some(OtObj::VmArrByte { elements, .. }) => elements[idx],
            Some(_) => panic!("Non-byte[] seen in heap during BALOAD at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn sastore(&mut self, id: usize, pos: i32, v: i16) {
        let idx = pos as usize;
        match self.alloc.get_mut(id) {
            Some(OtObj::VmArrShort { elements, .. }) => elements[idx] = v,
            Some(_) => panic!("Non-short[] seen in heap during SASTORE at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn saload(&self, id: usize, pos: i32) -> i16 {
        let idx = pos as usize;
        match self.alloc.get(id) {
            Some(OtObj::VmArrShort { elements, .. }) => elements[idx],
            Some(_) => panic!("Non-short[] seen in heap during SALOAD at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn castore(&mut self, id: usize, pos: i32, v: u16) {
        let idx = pos as usize;
        match self.alloc.get_mut(id) {
            Some(OtObj::VmArrChar { elements, .. }) => elements[idx] = v,
            Some(_) => panic!("Non-char[] seen in heap during CASTORE at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn caload(&self, id: usize, pos: i32) -> u16 {
        let idx = pos as usize;
        match self.alloc.get(id) {
            Some(OtObj::VmArrChar { elements, .. }) => elements[idx],
            Some(_) => panic!("Non-char[] seen in heap during CALOAD at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn aastore(&mut self, id: usize, pos: i32, v: usize) {
        let idx = pos as usize;
        match self.alloc.get_mut(id) {
            Some(OtObj::VmArrRef { elements, .. }) => elements[idx] = v,
            Some(_) => panic!("Non-ref[] seen in heap during AASTORE at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn aaload(&self, id: usize, pos: i32) -> usize {
        let idx = pos as usize;
        match self.alloc.get(id) {
            Some(OtObj::VmArrRef { elements, .. }) => elements[idx],
            Some(_) => panic!("Non-ref[] seen in heap during AALOAD at {}", id),
            None => panic!("Error: object {} not found", id),
        }
    }

    pub fn arraylength(&self, id: usize) -> i32 {
        match self.alloc.get(id) {
            Some(obj) => obj.length(),
            None => panic!("Error: object {} not found", id),
        }
    }
}
