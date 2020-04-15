use std::fmt;
use std::path::Path;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use regex::Regex;

use crate::JvmValue;
use crate::InterpLocalVars;
use crate::otfield::OtField;
use crate::otmethod::OtMethod;
use crate::otklass::OtKlass;

use ocelotter_util::file_to_bytes;

//////////// SHARED RUNTIME KLASS REPO

#[derive(Debug, Clone)]
pub enum KlassLoadingStatus {
    Mentioned {},
    Loaded { klass: OtKlass },
    Live { klass: OtKlass }
}

#[derive(Debug)]
pub struct SharedKlassRepo {
    klass_count: AtomicUsize,
    klass_lookup: HashMap<String, usize>,
    id_lookup: HashMap<usize, RefCell<KlassLoadingStatus>>,
}

impl SharedKlassRepo {

    //////////////////////////////////////////////
    // Static methods

    // FIXME This is effectively static
    fn parse_bootstrap_class(&self, cl_name: String) -> OtKlass {
        let fq_klass_fname = "./resources/lib/".to_owned() + &cl_name + ".class";
        let bytes = match file_to_bytes(Path::new(&fq_klass_fname)) {
            Ok(buf) => buf,
            _ => panic!("Error reading file {}", fq_klass_fname),
        };
        let mut parser = crate::klass_parser::OtKlassParser::of(bytes, cl_name.clone());
        parser.parse();
        parser.klass()
    }

    pub fn klass_name_from_fq(klass_name: &String) -> String {
        lazy_static! {
            static ref KLASS_NAME: Regex =
                Regex::new("((?:([a-zA-Z_$][a-zA-Z\\d_$]*(?:/[a-zA-Z_$][a-zA-Z\\d_$]*)*)/)?([a-zA-Z_$][a-zA-Z\\d_$]*))\\.").unwrap();
        }
        let caps = KLASS_NAME.captures(klass_name).unwrap();
        // Capture the package name and the class name via the use of a nexted group
        caps.get(1).map_or("".to_string(), |m| m.as_str().to_string())
    }

    pub fn klass_name_from_dotted_fq(klass_name: &String) -> String {
        lazy_static! {
            static ref KLASS_NAME_DOTTED: Regex =
                Regex::new("(?:([a-zA-Z_$][a-zA-Z\\d_$]*(?:\\.[a-zA-Z_$][a-zA-Z\\d_$]*)*)\\.)?([a-zA-Z_$][a-zA-Z\\d_$]*)").unwrap();
        }
        let caps = KLASS_NAME_DOTTED.captures(klass_name).unwrap();
        // In dotted syntax the field / method name comes after the final dot, hence no nested group
        caps.get(1).map_or("".to_string(), |m| m.as_str().to_string())
    }

    //////////////////////////////////////////////

    pub fn of() -> SharedKlassRepo {
        SharedKlassRepo {
            klass_lookup: HashMap::new(),
            id_lookup: HashMap::new(),
            klass_count: AtomicUsize::new(1),
        }
    }

    pub fn lookup_klass(&self, klass_name: &String) -> OtKlass {
        // let s = format!("{}", self);
        // dbg!(s);

        let kid = match self.klass_lookup.get(klass_name) {
            Some(id) => id,
            None => panic!("No klass called {} found in repo", klass_name),
        };
        let cell = match self.id_lookup.get(kid) {
            Some(value) => value,
            None => panic!("No klass with ID {} found in repo", kid),
        };
        match &*(cell.borrow()) {
            KlassLoadingStatus::Mentioned {} => panic!("Klass with ID {} is not loaded yet", kid),
            KlassLoadingStatus::Loaded { klass : k } => k.clone(),
            KlassLoadingStatus::Live { klass : k } => k.clone()
        }
    }

    pub fn add_klass(&mut self, k: &OtKlass) -> () {
        // First check to see if we already have this class and which state it's in
        let klass_name = k.get_name();
        let upgrade = match self.klass_lookup.get(&klass_name) {
            Some(id) => match self.id_lookup.get(id) {
                Some(value) => match &*(value.borrow()) {
                    KlassLoadingStatus::Mentioned {} => Some(id),
                    KlassLoadingStatus::Loaded { klass : _ } => None, 
                    KlassLoadingStatus::Live { klass : _ } => None 
                },
                None => panic!("No klass with ID {} found in repo", id),
            },
            None => {
                let k2: OtKlass = (*k).to_owned();
                // If it's completely new, then set its ID (which we'll use as the key for it)
                k2.set_id(self.klass_count.fetch_add(1, Ordering::SeqCst));
                let id = k2.get_id();
                // Scan for every other class the newcomer mentions
                let klasses_mentioned = k2.get_mentioned_klasses();

                self.klass_lookup.insert(k.get_name().clone(), id);
                self.id_lookup.insert(id, RefCell::new(KlassLoadingStatus::Loaded{ klass: k2 }));
                // Mention everything this class refers to
                self.mention(klasses_mentioned);
                None
            }
        };
        match upgrade {
            None => (),
            Some(id) => {
                let k2 = (*k).to_owned();
                // Set kid & Load k into map
                k2.set_id(*id);
                self.id_lookup.get(id).unwrap().replace(KlassLoadingStatus::Loaded{ klass: k2 });
            }
        }
    }

    fn mention(&mut self, mentions: Vec<String>) -> () {
        // Loop over mentions
        let mut i = 0;
        while i < mentions.len() {
            // Check to see if we have this class already
            let klass_name = mentions.get(i).unwrap();
            match self.klass_lookup.get(klass_name) {
                // If not, add a mention
                None => {
                    let id = self.klass_count.fetch_add(1, Ordering::SeqCst);
                    self.klass_lookup.insert(klass_name.clone(), id);
                    self.id_lookup.insert(id, RefCell::new(KlassLoadingStatus::Mentioned{ }));    
                },
                Some(id) => match self.id_lookup.get(id) {
                    Some(value) => (),
                    None => panic!("No klass with ID {} found in repo", id),
                },
            }

            i = i + 1;
        }
    }

    fn run_clinit_method(&mut self, k : &OtKlass, i_callback: fn(&mut SharedKlassRepo, &OtMethod, &mut InterpLocalVars) -> Option<JvmValue>) {
        let klass_name = k.get_name();
        let m_str: String = klass_name.clone() + ".<clinit>:()V";
        let clinit = match k.get_method_by_name_and_desc(&m_str) {
            Some(value) => value.clone(),
            // FIXME Make this a clean exit
            None => panic!("Error: Clinit method not found {}", klass_name),
        };
        // FIXME Parameter passing
        let mut vars = InterpLocalVars::of(5);
        i_callback(self, &clinit, &mut vars);
    }

    // FIXME This should be changed to read in an ocelot-rt.jar (a cut down full RT)
    // and add each class one by one before fixing up the native code that we have working
//  (repo: SharedKlassRepo, meth: &OtMethod, lvt: &mut InterpLocalVars) -> Option<JvmValue>
    pub fn bootstrap(&mut self, i_callback: fn(&mut SharedKlassRepo, &OtMethod, &mut InterpLocalVars) -> Option<JvmValue>) -> () {
        // Add java.lang.Object
        let k_obj = self.parse_bootstrap_class("java/lang/Object".to_string());
        // let s = format!("{}", self);
        // dbg!(s);

        // Add j.l.O native methods (e.g. hashCode())
        k_obj.set_native_method(
            "java/lang/Object.hashCode:()I".to_string(),
            crate::native_methods::java_lang_Object__hashcode,
        );
        k_obj.set_native_method(
            "java/lang/Object.registerNatives:()V".to_string(),
            crate::native_methods::java_lang_Object__registerNatives,
        );
        self.add_klass(&k_obj);
        // FIXME Must reset the value set for the klass repo before clinit
        self.run_clinit_method(&k_obj, i_callback);

        // FIXME Add primitive arrays

        // FIXME Add java.lang.Class

        // Add wrapper classes
        let k_jli = self.parse_bootstrap_class("java/lang/Integer".to_string());
        self.add_klass(&k_jli);
        // Needs j.l.Class to run (set up primitive type .class object)
        // self.run_clinit_method(&k_jli, i_callback);

        let k_jlic = self.parse_bootstrap_class("java/lang/Integer$IntegerCache".to_string());
        self.add_klass(&k_jlic);
        // Needs j.l.Class and uses sun.* classes to do VM-protected stuff
        // self.run_clinit_method(&k_jlic, i_callback);

        // FIXME Other classes

        // Add java.lang.String
        let k_jls = self.parse_bootstrap_class("java/lang/String".to_string());
        // FIXME String only has intern() as a native method, skip for now
        self.add_klass(&k_jls);

        // Add java.lang.StringBuilder
        let k_jlsb = self.parse_bootstrap_class("java/lang/StringBuilder".to_string());
        self.add_klass(&k_jlsb);

        // FIXME Add class objects for already bootstrapped classes

        // Add java.lang.System
        let k_sys = self.parse_bootstrap_class("java/lang/System".to_string());
        k_sys.set_native_method(
            "java/lang/System.currentTimeMillis:()J".to_string(),
            crate::native_methods::java_lang_System__currentTimeMillis,
        );
        self.add_klass(&k_sys);

        // TODO Dummy up enough of java.io.PrintStream to get System.out.println() to work
        // By faking up the class so that println(Ljava/lang/Object;) fwds to native code
        // k_obj = self.parse_bootstrap_class("java/io/PrintStream".to_string());
        // k_obj.set_native_method(
        //     "println:(Ljava/lang/Object;)V".to_string(),
        //     crate::native_methods::java_io_PrintStream__println,
        // );
    }

    pub fn lookup_static_field(&self, klass_name: &String, idx: u16) -> OtField {
        let current_klass = self.lookup_klass(klass_name);

        // Lookup the Fully-Qualified field name from the CP index
        let fq_name_desc = current_klass.cp_as_string(idx);
        let target_klass_name = &SharedKlassRepo::klass_name_from_fq(&fq_name_desc);
        let target_klass = self.lookup_klass(&target_klass_name);

        let opt_f = target_klass.get_static_field_by_name_and_desc(&fq_name_desc);

        match opt_f {
            Some(f) => f.clone(),
            None => panic!(
                "No static field {} found on klass {} ",
                fq_name_desc.clone(),
                target_klass_name
            ),
        }
    }

    pub fn lookup_instance_field(&self, klass_name: &String, idx: u16) -> OtField {
        let current_klass = self.lookup_klass(klass_name);

        // Lookup the Fully-Qualified field name from the CP index
        let fq_name_desc = current_klass.cp_as_string(idx);
        let target_klass_name = &SharedKlassRepo::klass_name_from_fq(&fq_name_desc);
        let target_klass = self.lookup_klass(&target_klass_name);

        let opt_f = target_klass.get_instance_field_by_name_and_desc(&fq_name_desc);

        match opt_f {
            Some(f) => f.clone(),
            None => panic!(
                "No instance field {} found on klass {} ",
                fq_name_desc.clone(),
                target_klass_name
            ),
        }
    }

    // FIXME Lookup offset properly
    pub fn get_field_offset(&self, kid: usize, f: OtField) -> usize {
        0
    }

    pub fn put_static(&self, klass_name: String, f: OtField, v: JvmValue) -> () {
        // FIXME Handle storage properly
    }

    pub fn lookup_method_exact(&self, klass_name: &String, fq_name_desc: String) -> OtMethod {
        let kid = match self.klass_lookup.get(klass_name) {
            Some(id) => id,
            None => panic!("No klass called {} found in repo", klass_name),
        };
        match self.id_lookup.get(kid) {
            Some(cell) => match &*(cell.borrow()) {
                KlassLoadingStatus::Mentioned {} => panic!("Klass with ID {} is not loaded yet", kid),
                KlassLoadingStatus::Loaded { klass : k } => k.get_method_by_name_and_desc(&fq_name_desc).unwrap().clone(),
                KlassLoadingStatus::Live { klass : k } => k.get_method_by_name_and_desc(&fq_name_desc).unwrap().clone(),
            },
            None => panic!("No klass with ID {} found in repo", kid),
        }
    }

    // m_idx is IDX in CP of current class
    pub fn lookup_method_virtual(&self, klass_name: &String, m_idx: u16) -> OtMethod {
        // Get klass
        let kid = match self.klass_lookup.get(klass_name) {
            Some(id) => id,
            None => panic!("No klass called {} found in repo", klass_name),
        };
        match self.id_lookup.get(kid) {
            Some(cell) => match &*(cell.borrow()) {
                KlassLoadingStatus::Mentioned {} => panic!("Klass with ID {} is not loaded yet", kid),
                KlassLoadingStatus::Loaded { klass : k } => k.get_method_by_offset_virtual(m_idx),
                KlassLoadingStatus::Live { klass : k } => k.get_method_by_offset_virtual(m_idx),
            }
            None => panic!("No klass with ID {} found in repo", kid),
        }
    }
}

//     klass_lookup: HashMap<String, usize>,
//    id_lookup: HashMap<usize, OtKlass>,
impl fmt::Display for SharedKlassRepo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} with klasses {:?}",
            self.klass_count, self.id_lookup
        )
    }
}

impl Clone for SharedKlassRepo {
    fn clone(&self) -> SharedKlassRepo {
        SharedKlassRepo {
            klass_lookup: self.klass_lookup.clone(),
            id_lookup: self.id_lookup.clone(),
            klass_count: AtomicUsize::new(self.klass_count.fetch_add(0, Ordering::SeqCst)),
        }
    }
}

/////////////////////////////////////////////////////////////////
