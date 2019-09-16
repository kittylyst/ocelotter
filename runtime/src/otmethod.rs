use std::fmt;
use std::cell::Cell;

use crate::constant_pool::CpAttr;
use crate::constant_pool::ACC_NATIVE;
use crate::InterpLocalVars;
use crate::JvmValue;

#[derive(Clone)]
pub struct OtMethod {
    klass_name: String,
    flags: u16,
    name: String,
    name_desc: String,
    name_idx: u16,
    desc_idx: u16,
    code: Vec<u8>,
    native_code: Cell<Option<fn(&InterpLocalVars) -> Option<JvmValue>>>,
    attrs: Vec<CpAttr>,
}

impl OtMethod {
    pub fn of(
        klass_name: String,
        name: String,
        desc: String,
        flags: u16,
        name_idx: u16,
        desc_idx: u16,
    ) -> OtMethod {
        let name_and_desc = name.clone() + ":" + &desc.clone();
        OtMethod {
            klass_name: klass_name.to_string(),
            flags: flags,
            name: name.clone(),
            name_desc: name_and_desc,
            attrs: Vec::new(),
            code: Vec::new(),
            native_code: Cell::new(None),
            // FIXME
            name_idx: desc_idx,
            desc_idx: desc_idx,
        }
    }

    pub fn set_attr(&self, _index: u16, _attr: CpAttr) -> () {}

    pub fn set_code(&mut self, code: Vec<u8>) -> () {
        self.code = code;
    }

    pub fn get_code(&self) -> Vec<u8> {
        self.code.clone()
    }

    pub fn get_klass_name(&self) -> String {
        self.klass_name.clone()
    }

    pub fn get_desc(&self) -> String {
        self.name_desc.clone()
    }

    pub fn get_fq_name_desc(&self) -> String {
        self.klass_name.clone() + "." + &self.name_desc.clone()
    }

    pub fn get_flags(&self) -> u16 {
        self.flags
    }

    pub fn is_native(&self) -> bool {
        self.flags & ACC_NATIVE == ACC_NATIVE
    }

    pub fn set_native_code(&self, n_code: fn(&InterpLocalVars) -> Option<JvmValue>) {
        self.native_code.set(Some(n_code));
    }

    pub fn get_native_code(&self) -> Option<fn(&InterpLocalVars) -> Option<JvmValue>> {
        self.native_code.get()
    }

    // HACK Replace with proper local var size by parsing class attributes properly
    pub fn get_local_var_size(&self) -> u8 {
        255
    }
}

impl fmt::Debug for OtMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.klass_name, self.name_desc)
    }
}

impl fmt::Display for OtMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.klass_name, self.name_desc)
    }
}
