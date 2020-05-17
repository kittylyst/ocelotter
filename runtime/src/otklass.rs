use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;

use crate::constant_pool::CpEntry;
use crate::otfield::OtField;
use crate::otmethod::OtMethod;
use crate::InterpLocalVars;
use crate::JvmValue;

//////////// RUNTIME KLASS AND RELATED HANDLING

#[derive(Debug, Clone)]
pub struct OtKlass {
    id: Cell<usize>,
    name: String,
    super_name: String,
    flags: u16,
    cp_entries: Vec<CpEntry>,
    methods: Vec<OtMethod>,
    i_fields: Vec<OtField>,
    s_fields: Vec<OtField>,
    s_field_vals: Vec<Cell<JvmValue>>,
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
        let mut s_fields = Vec::new();
        let mut s_field_vals = Vec::new();
        let mut i_fields = Vec::new();
        while i < fields.len() {
            let f = match fields.get(i) {
                Some(val) => val.clone(),
                None => panic!("Error: field {} not found on {}", i, klass_name),
            };
            let f_name = f.get_fq_name_desc();
            if f.is_static() {
                let default_val = f.get_default();
                s_fields.push(f);
                s_field_vals.push(Cell::new(default_val));
            } else {
                i_fields.push(f);
            }
            f_lookup.insert(f_name, i);
            i = i + 1;
        }
        // dbg!(m_lookup.clone());
        // dbg!(f_lookup.clone());
        OtKlass {
            id: Cell::new(0), // This indicates that the class has not yet been loaded into a repo
            name: klass_name,
            super_name: super_klass,
            flags: flags,
            cp_entries: cp_entries.to_vec(),
            methods: methods.to_vec(),
            i_fields: i_fields.to_vec(),
            s_fields: s_fields.to_vec(),
            s_field_vals: s_field_vals.to_vec(),
            // FIXME
            m_name_desc_lookup: m_lookup,
            f_name_desc_lookup: f_lookup,
        }
    }

    /////////////////////////////////////

    pub fn parse_sig_for_args(signature : String) -> Vec<JvmValue> {
        let mut out: Vec<JvmValue> = Vec::new();
        let mut chars = signature.chars();

        while let Some(next) = chars.next() {
            let indicative_char = match next {
                '(' => continue,
                ')' => break,
                'Z' => 'Z',
                'B' => 'B',
                'S' => 'S',
                'I' => 'I',
                'J' => 'J',
                'F' => 'F',
                'D' => 'D',
                'C' => 'C',
                'L' => {
                    // advance through the object type
                    while let Some(lbrac) = chars.next() {
                        if lbrac == ';' {
                            break;
                        }
                    };
                    'A'
                },
                '[' => {
                    // advance through the array type
                    while let Some(lbrac) = chars.next() {
                        if lbrac == 'L' {
                            // advance through the object type
                            while let Some(lbrac) = chars.next() {
                                if lbrac == ';' {
                                    break;
                                }
                            };
                            break;
                        }
                        if lbrac != '[' {
                            break;
                        }
                    };
                    'A'
                },
                x => panic!("Illegal type {} seen when trying to parse {}", x, signature)
            };

            out.push(JvmValue::default_value(indicative_char));
        }

        out
    }

    /////////////////////////////////////

    pub fn make_default_values(&self) -> Vec<JvmValue> {
        let mut out: Vec<JvmValue> = Vec::new();
        let mut i = 0;
        while i < self.i_fields.len() {
            match self.i_fields.get(i) {
                Some(f) => out.push(f.get_default()),
                None => panic!("Error: field {} not found on {}", i, self.name),
            };
            i = i + 1;
        }
        out
    }

    pub fn set_id(&self, new_id: usize) -> () {
        self.id.set(new_id)
    }

    pub fn get_id(&self) -> usize {
        self.id.get()
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
        &self,
        name_desc: String,
        n_code: fn(&InterpLocalVars) -> Option<JvmValue>,
    ) {
        match self.get_method_by_name_and_desc(&name_desc) {
            Some(m2) => m2.set_native_code(n_code),
            None => {
                panic!("Should be unreachable - trying to store native code in a non-existant method")
            }
        }
    }

    pub fn get_mentioned_klasses(&self) -> Vec<String> {
        let mut i = 0;
        let mut out = Vec::new();
        while i < self.cp_entries.len() {
            let o_klass_name = match self.cp_entries.get(i).unwrap() {
                CpEntry::class { idx: utf_idx } => Some(self.cp_as_string(*utf_idx)),
                _ => None
            };
            match o_klass_name {
                None => (),
                Some(s) => out.push(s)
            };
            i = i + 1;
        }
        out
    }

    pub fn get_instance_field_offset(&self, f: &OtField) -> usize {
        let mut i = 0;
        while i < self.i_fields.len() {
            let c_f = match self.i_fields.get(i) {
                Some(f) => f,
                None => panic!("Should be unreachable, field should always exist"),
            };
            if c_f.get_fq_name_desc() == f.get_fq_name_desc() {
                return i;
            }
            i = i + 1;
        }
        panic!("Field {} not found on {}", f, self)
    }

    pub fn get_static_field_offset(&self, f: &OtField) -> usize {
        let mut i = 0;
        while i < self.s_fields.len() {
            let c_f = match self.s_fields.get(i) {
                Some(f) => f,
                None => panic!("Should be unreachable, field should always exist"),
            };
            if c_f.get_fq_name_desc() == f.get_fq_name_desc() {
                return i;
            }
            i = i + 1;
        }
        panic!("Field {} not found on {}", f, self)
    }


    pub fn get_static(&self, f: &OtField) -> JvmValue {
        let idx = self.get_static_field_offset(f);
        self.s_field_vals.get(idx).unwrap().get().clone()
    }

    pub fn put_static(&self, f: &OtField, v: JvmValue) -> () {
        let idx = self.get_static_field_offset(f);
        self.s_field_vals.get(idx).unwrap().set(v);
    }


    pub fn get_method_by_offset_virtual(&self, m_idx: u16) -> OtMethod {
        // If present, return value at specific offset
        // let offset = self.get_method_offset(f);

        // Otherwise walk up to subclass & retry

        // FIXME DUMMY
        OtMethod::of(
            "DUMMY_KLASS".to_string(),
            "DUMMY_METH".to_string(),
            "DUMMY_DESC".to_string(),
            0,
            1,
            2,
        )
    }

    // NOTE: This is fully-qualified
    pub fn get_method_by_name_and_desc(&self, name_desc: &String) -> Option<&OtMethod> {
        let opt_idx = self.m_name_desc_lookup.get(name_desc);
        let idx: usize = match opt_idx {
            Some(value) => value.clone(),
            None => return None,
        };
        self.methods.get(idx)
    }

    // NOTE: This is fully-qualified
    pub fn get_static_field_by_name_and_desc(&self, name_desc: &String) -> Option<&OtField> {
//        dbg!(&name_desc);
        let opt_idx = self.f_name_desc_lookup.get(name_desc);
        let idx: usize = match opt_idx {
            Some(value) => value.clone(),
            None => return None,
        };
        self.s_fields.get(idx)
    }

    // NOTE: This is fully-qualified
    pub fn get_instance_field_by_name_and_desc(&self, name_desc: &String) -> Option<&OtField> {
//        dbg!(&name_desc);
        let opt_idx = self.f_name_desc_lookup.get(name_desc);
        let idx: usize = match opt_idx {
            Some(value) => value.clone(),
            None => return None,
        };
        self.i_fields.get(idx)
    }

    pub fn lookup_cp(&self, cp_idx: u16) -> CpEntry {
        let idx = cp_idx as usize;
        match self.cp_entries.get(idx).clone() {
            Some(val) => val.clone(),
            None => panic!(
                "Error: No entry found on {} at CP index {}",
                self.name, cp_idx
            ),
        }
    }

    pub fn get_method_arg_count(&self, cp_idx: u16) -> u8 {
        let cp_entry = self.lookup_cp(cp_idx);
        let name_and_type = match cp_entry {
            CpEntry::methodref {
                clz_idx: _,
                nt_idx: ntidx,
            } => self.lookup_cp(ntidx),
            _ => panic!(
                "Attempt to count args of non-method in {} at index {} where {:?}",
                self.name, cp_idx, self.cp_entries.get(cp_idx as usize)
            ),
        };
        let type_signature = match name_and_type {
            CpEntry::name_and_type {
                name_idx: _,
                type_idx: tidx,
            } => self.lookup_cp(tidx),
            _ => panic!(
                "Attempt to count args of non-method in {} at index {} where {:?}",
                self.name, cp_idx, self.cp_entries.get(cp_idx as usize)
            ),
        };
        match type_signature {
            CpEntry::utf8 {
                val: sig,
            } => OtKlass::parse_sig_for_args(sig).len() as u8,
            _ => panic!(
                "Attempt to count args of non-method in {} at index {} found {}",
                self.name, cp_idx, type_signature.name()
            ),
        }
    }

    pub fn cp_as_string(&self, i: u16) -> String {
        match self.lookup_cp(i) {
            CpEntry::utf8 { val: s } => s,
            CpEntry::class { idx: utf_idx } => self.cp_as_string(utf_idx),
            CpEntry::fieldref { clz_idx, nt_idx } => {
                self.cp_as_string(clz_idx) + "." + &self.cp_as_string(nt_idx)
            }
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
            "{} ISA {} with methods {:?} and constants {:?}",
            self.name, self.super_name, self.m_name_desc_lookup, self.cp_entries
        )
    }
}
