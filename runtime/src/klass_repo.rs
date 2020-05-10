use std::fmt;
use std::path::Path;
use std::cell::RefCell;
use std::collections::HashMap;

use regex::Regex;

use crate::JvmValue;
use crate::InterpLocalVars;
use crate::otfield::OtField;
use crate::otmethod::OtMethod;
use crate::otklass::OtKlass;

use ocelotter_util::file_to_bytes;
use ocelotter_util::ZipFiles;

//////////// SHARED RUNTIME KLASS REPO

#[derive(Debug, Clone)]
pub enum KlassLoadingStatus {
    Mentioned {},
    Loaded { klass: OtKlass },
    Live { klass: OtKlass }
}

#[derive(Debug)]
pub struct SharedKlassRepo {
    klass_lookup: HashMap<String, RefCell<KlassLoadingStatus>>,
}

impl SharedKlassRepo {

    //////////////////////////////////////////////
    // Static methods

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
        }
    }

    pub fn lookup_klass(&self, klass_name: &String) -> OtKlass {
        // let s = format!("{}", self);
        // dbg!(s);

        match self.klass_lookup.get(klass_name) {
            Some(cell) => match &*(cell.borrow()) {
                KlassLoadingStatus::Mentioned {} => panic!("Klass {} is not loaded yet", klass_name),
                KlassLoadingStatus::Loaded { klass : k } => k.clone(),
                KlassLoadingStatus::Live { klass : k } => k.clone()
            },
            None => panic!("No klass called {} found in repo", klass_name),
        }
    }

    pub fn add_klass(&mut self, k: &OtKlass) -> () {
        // First check to see if we already have this class and which state it's in
        let klass_name = k.get_name();
        let upgrade = match self.klass_lookup.get(&klass_name) {
            Some(value) => match &*(value.borrow()) {
                KlassLoadingStatus::Mentioned {} => true,
                KlassLoadingStatus::Loaded { klass : _ } => false, 
                KlassLoadingStatus::Live { klass : _ } => false 
            },
            None => {
                let k2: OtKlass = (*k).to_owned();
                // Scan for every other class the newcomer mentions
                let klasses_mentioned = k2.get_mentioned_klasses();

                self.klass_lookup.insert(k.get_name().clone(), RefCell::new(KlassLoadingStatus::Loaded{ klass: k2 }));
                // Mention everything this class refers to
                self.mention(klasses_mentioned);
                false
            }
        };
        if upgrade {
            let k2 = (*k).to_owned();
            // Load k into map
            self.klass_lookup.get(&klass_name).unwrap().replace(KlassLoadingStatus::Loaded{ klass: k2 });
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
                    self.klass_lookup.insert(klass_name.clone(), RefCell::new(KlassLoadingStatus::Mentioned{ }));
                },
                Some(value) => (),
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

    // This reads in classes.jar and adds each class one by one before fixing up
    // the bits of native code that we have working
    //
    // An interpreter callback, i_callback is needed to run the static initializers
    pub fn bootstrap(&mut self, i_callback: fn(&mut SharedKlassRepo, &OtMethod, &mut InterpLocalVars) -> Option<JvmValue>) -> () {
        let file = "resources/lib/classes.jar";
        ZipFiles::new(file)
        .into_iter()
        .filter(|f| match f {
            Ok((name, _)) if name.ends_with(".class") => true,
            _ => false,
        })
        .for_each(|z| {
            if let Ok((name, bytes)) = z {
                let mut parser = crate::klass_parser::OtKlassParser::of(bytes, name);
                parser.parse();
                self.add_klass(&parser.klass());
            }
        });

        {
            let klass_name = "java/lang/Object".to_string();
            let k_obj = self.lookup_klass(&klass_name);
            k_obj.set_native_method(
                "java/lang/Object.hashCode:()I".to_string(),
                crate::native_methods::java_lang_Object__hashcode,
            );
            self.klass_lookup.get(&klass_name).unwrap().replace(KlassLoadingStatus::Live{ klass: k_obj });
        }

        {
            let klass_name = "java/lang/System".to_string();
            let k_sys = self.lookup_klass(&klass_name);
            k_sys.set_native_method(
                "java/lang/System.currentTimeMillis:()J".to_string(),
                crate::native_methods::java_lang_System__currentTimeMillis,
            );
            self.klass_lookup.get(&klass_name).unwrap().replace(KlassLoadingStatus::Live{ klass: k_sys });
        }

        // TODO Get enough of java.io.PrintStream working to get System.out.println() to work
        //     crate::native_methods::java_io_PrintStream__println,

        // let s = format!("{:?}", self.klass_lookup);
        // dbg!(s);
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
        match self.klass_lookup.get(klass_name) {
            Some(cell) => match &*(cell.borrow()) {
                KlassLoadingStatus::Mentioned {} => panic!("Klass with ID {} is not loaded yet", klass_name),
                KlassLoadingStatus::Loaded { klass : k } => k.get_method_by_name_and_desc(&fq_name_desc).unwrap().clone(),
                KlassLoadingStatus::Live { klass : k } => k.get_method_by_name_and_desc(&fq_name_desc).unwrap().clone(),
            },
            None => panic!("No klass with ID {} found in repo", klass_name),
        }
    }

    // m_idx is IDX in CP of current class
    pub fn lookup_method_virtual(&self, klass_name: &String, m_idx: u16) -> OtMethod {
        match self.klass_lookup.get(klass_name) {
            Some(cell) => match &*(cell.borrow()) {
                KlassLoadingStatus::Mentioned {} => panic!("Klass with ID {} is not loaded yet", klass_name),
                KlassLoadingStatus::Loaded { klass : k } => k.get_method_by_offset_virtual(m_idx),
                KlassLoadingStatus::Live { klass : k } => k.get_method_by_offset_virtual(m_idx),
            }
            None => panic!("No klass with ID {} found in repo", klass_name),
        }
    }
}

impl fmt::Display for SharedKlassRepo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:#?}",
            self.klass_lookup
        )
    }
}

impl Clone for SharedKlassRepo {
    fn clone(&self) -> SharedKlassRepo {
        SharedKlassRepo {
            klass_lookup: self.klass_lookup.clone(),
        }
    }
}

/////////////////////////////////////////////////////////////////
