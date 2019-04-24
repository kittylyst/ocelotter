use std::collections::HashMap;
use std::fmt;

use crate::constant_pool::CpEntry;
use crate::otfield::OtField;
use crate::otmethod::OtMethod;
use crate::InterpLocalVars;
use crate::JvmValue;

//////////// RUNTIME KLASS AND RELATED HANDLING

#[derive(Clone, Debug)]
pub struct OtKlass {
    id: usize,
    name: String,
    super_name: String,
    flags: u16,
    cp_entries: Vec<CpEntry>,
    methods: Vec<OtMethod>,
    fields: Vec<OtField>,
    m_name_desc_lookup: HashMap<String, usize>,
    f_name_desc_lookup: HashMap<String, usize>,
}

impl OtKlass {
    pub fn of(
        klass_name: String,
        super_klass: String,
        flags: u16,
        cp_entries: &Vec<CpEntry>,
        methods: &Vec<OtMethod>,
        fields: &Vec<OtField>,
    ) -> OtKlass {
        let mut m_lookup = HashMap::new();
        let mut i = 0;
        while i < methods.len() {
            let meth = match methods.get(i).clone() {
                Some(val) => val.clone(),
                None => panic!("Error: method {} not found on {}", i, klass_name),
            };
            m_lookup.insert(meth.get_fq_name_desc().clone(), i);
            i = i + 1;
        }
        i = 0;
        let mut f_lookup = HashMap::new();
        while i < fields.len() {
            let f = match fields.get(i).clone() {
                Some(val) => val.clone(),
                None => panic!("Error: field {} not found on {}", i, klass_name),
            };
            f_lookup.insert(f.get_fq_name_desc().clone(), i);
            i = i + 1;
        }
        // dbg!(m_lookup.clone());
        // dbg!(f_lookup.clone());
        OtKlass {
            id: 0, // This indicates that the class has not yet been loaded into a repo
            name: klass_name,
            super_name: super_klass,
            flags: flags,
            cp_entries: cp_entries.to_vec(),
            methods: methods.to_vec(),
            fields: fields.to_vec(),
            m_name_desc_lookup: m_lookup,
            f_name_desc_lookup: f_lookup,
        }
    }

    pub fn set_id(&mut self, id: usize) -> () {
        self.id = id
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_super_name(&self) -> String {
        self.super_name.to_owned()
    }

    pub fn get_methods(&self) -> Vec<OtMethod> {
        self.methods.clone()
    }

    pub fn set_native_method(
        &mut self,
        name_desc: String,
        n_code: fn(&InterpLocalVars) -> Option<JvmValue>,
    ) {
        // dbg!("Setting native code");
        // dbg!(name_desc.clone());
        match self.get_mutable_method(&name_desc) {
            Some(m2) => m2.set_native_code(n_code),
            None => panic!("Should be unreachable"),
        }
    }

    // FIXME The size in bytes of an object of this type
    pub fn obj_size(&self) -> usize {
        100
    }

    // NOTE: This is fully-qualified
    pub fn get_method_by_name_and_desc(&self, name_desc: &String) -> Option<&OtMethod> {
        dbg!(&self.m_name_desc_lookup);
        dbg!(&name_desc);
        let opt_idx = self.m_name_desc_lookup.get(name_desc);
        let idx: usize = match opt_idx {
            Some(value) => value.clone(),
            None => return None,
        };
        self.methods.get(idx)
    }

    pub fn get_mutable_method(&mut self, name_desc: &String) -> Option<&mut OtMethod> {
        for m in &mut self.methods {
            if *m.get_desc() == *name_desc {
                return Some(m);
            }
        }
        None
    }

    pub fn lookup_cp(&self, cp_idx: u16) -> CpEntry {
        let idx = cp_idx as usize;
        // dbg!(&self.cp_entries);
        match self.cp_entries.get(idx).clone() {
            Some(val) => val.clone(),
            None => panic!(
                "Error: No entry found on {} at CP index {}",
                self.name, cp_idx
            ),
        }
    }

    pub fn cp_as_string(&self, i: u16) -> String {
        match self.lookup_cp(i) {
            CpEntry::utf8 { val: s } => s,
            CpEntry::class { idx: utf_idx } => self.cp_as_string(utf_idx),
            CpEntry::methodref { clz_idx, nt_idx } => {
                self.cp_as_string(clz_idx) + "." + &self.cp_as_string(nt_idx)
            }
            CpEntry::name_and_type {
                name_idx: nidx,
                type_idx: tidx,
            } => self.cp_as_string(nidx) + ":" + &self.cp_as_string(tidx),
            _ => panic!(
                "Unimplemented stringify of CP entry found in {} at index {}",
                self.name, i
            ),
        }
    }
}

// flags: u16,
// cp_entries: Vec<CpEntry>,
// name_desc_lookup: HashMap<String, usize>,
impl fmt::Display for OtKlass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ISA {} with methods {:?}",
            self.name, self.super_name, self.m_name_desc_lookup
        )
    }
}
