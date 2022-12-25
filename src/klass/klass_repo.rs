use std::fmt;
use std::path::Path;
use std::cell::RefCell;
use std::collections::HashMap;

use regex::Regex;

use crate::interpreter::values::*;
use crate::interpreter::native_methods::*;

use crate::klass::otfield::OtField;
use crate::klass::otmethod::OtMethod;
use crate::klass::otklass::OtKlass;
use crate::klass::klass_parser::OtKlassParser;
use crate::klass::options::Options;

// use ocelotter_util::file_to_bytes;
// use ocelotter_util::ZipFiles;


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

    // We keep a mutable reference to the shared klass repo b/c we're the only thread allowed to modify it.
    pub fn start(options: Options) {
        let mut repo = SharedKlassRepo::of();
        repo.bootstrap(exec_method);

        let fq_klass_name = options.fq_klass_name();
        let f_name = options.f_name();

        if let Some(file) = &options.classpath {
            ZipFiles::new(file)
                .into_iter()
                .filter(|f| matches!(f, Ok((name, _)) if name.ends_with(".class")))
                .for_each(|z| {
                    if let Ok((name, bytes)) = z {
                        let mut parser = OtKlassParser::of(bytes, name);
                        parser.parse();
                        repo.add_klass(&parser.klass());
                    }
                });
            //Not using a classpath jar, just a class
        } else {
            let bytes = file_to_bytes(Path::new(&fq_klass_name))
                .unwrap_or_else(|_| panic!("Problem reading {}", &fq_klass_name));
            let mut parser = OtKlassParser::of(bytes, fq_klass_name);
            parser.parse();
            let k = parser.klass();
            repo.add_klass(&k);
        }
    }

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

    fn run_clinit_method(&mut self, klass_name: &String, i_callback: fn(&mut SharedKlassRepo, &OtMethod, &mut InterpLocalVars) -> Option<JvmValue>) {
        let m_str = klass_name.to_owned() + ".<clinit>:()V";
        let k = self.lookup_klass(klass_name);
        let clinit = match k.get_method_by_name_and_desc(&m_str) {
            Some(value) => value.clone(),
            // FIXME Make this a clean exit
            None => panic!("Error: Clinit method not found {}", klass_name),
        };
        // FIXME Parameter passing
        let mut vars = InterpLocalVars::of(5);
        i_callback(self, &clinit, &mut vars);
    }

    fn install_native_method(&mut self, klass_name: &String, name_desc: &String,
        n_code: fn(&InterpLocalVars) -> Option<JvmValue> ) -> () {
        let k = self.lookup_klass(klass_name);
        let fq_name = klass_name.to_owned() +"."+ &name_desc;

        k.set_native_method(fq_name, n_code);
        self.klass_lookup.get(klass_name).unwrap().replace(KlassLoadingStatus::Live{ klass: k });
    }

//    fn double_mapper_factory(tfm: fn(f64) -> f64) -> fn(&InterpLocalVars) -> Option<JvmValue> {
//        |args: &InterpLocalVars| -> Option<JvmValue> {
//            let d = match args.load(0) {
//                JvmValue::Double(v) => v,
//                x => panic!("Non-double value {} of type {} encountered in Math", x, x.name())
//            };
//
//            Some(JvmValue::Double {val: tfm(d)})
//        }
//    }

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

//        self.install_native_method(&"java/lang/Object".to_string(), &"getClass:()Ljava/lang/Class;".to_string(), java_lang_Object__getClass);
        self.install_native_method(&"java/lang/Object".to_string(), &"hashCode:()I".to_string(), java_lang_Object__hashcode);
//        self.install_native_method(&"java/lang/Object".to_string(), &"clone:()Ljava/lang/Object;".to_string(), java_lang_Object__clone);
        self.install_native_method(&"java/lang/Object".to_string(), &"notify:()V".to_string(), java_lang_Object__notify);
        self.install_native_method(&"java/lang/Object".to_string(), &"notifyAll:()V".to_string(), java_lang_Object__notifyAll);
        self.install_native_method(&"java/lang/Object".to_string(), &"wait:(J)V".to_string(), java_lang_Object__wait);


//        public static final native java.lang.Class forName(java.lang.String) throws java.lang.ClassNotFoundException;
//        public final native java.lang.Object newInstance() throws java.lang.InstantiationException, java.lang.IllegalAccessException;

        self.install_native_method(&"java/lang/Class".to_string(), &"getName:()Ljava/lang/String;".to_string(), java_lang_Class__getName);
//        public final native java.lang.String getName();
//        public final native java.lang.Class getSuperclass();
//        public final native java.lang.Class[] getInterfaces();
//        public final native java.lang.ClassLoader getClassLoader();
//        public final native boolean isInterface();

        self.install_native_method(&"java/lang/Compiler".to_string(), &"compileClass:(Ljava/lang/Class;)Z".to_string(), java_lang_Compiler__compileClass);
        self.install_native_method(&"java/lang/Compiler".to_string(), &"compileClasses:(Ljava/lang/String;)Z".to_string(), java_lang_Compiler__compileClasses);
//        public static final native java.lang.Object command(java.lang.Object);
        self.install_native_method(&"java/lang/Compiler".to_string(), &"enable:()V".to_string(), java_lang_Compiler__enable);
        self.install_native_method(&"java/lang/Compiler".to_string(), &"disable:()V".to_string(), java_lang_Compiler__disable);
        
        self.install_native_method(&"java/lang/Runtime".to_string(), &"freeMemory:()J".to_string(), java_lang_Runtime__freeMemory);
        self.install_native_method(&"java/lang/Runtime".to_string(), &"totalMemory:()J".to_string(), java_lang_Runtime__totalMemory);
        self.install_native_method(&"java/lang/Runtime".to_string(), &"gc:()V".to_string(), java_lang_Runtime__gc);
        self.install_native_method(&"java/lang/Runtime".to_string(), &"runFinalization:()V".to_string(), java_lang_Runtime__runFinalization);
        self.install_native_method(&"java/lang/Runtime".to_string(), &"traceInstructions:(Z)V".to_string(), java_lang_Runtime__traceInstructions);
        self.install_native_method(&"java/lang/Runtime".to_string(), &"traceMethodCalls:(Z)V".to_string(), java_lang_Runtime__traceMethodCalls);

        self.install_native_method(&"java/lang/System".to_string(), &"currentTimeMillis:()J".to_string(), java_lang_System__currentTimeMillis);
        self.install_native_method(&"java/lang/System".to_string(), &"arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V".to_string(), java_lang_System__arraycopy);

        // Load j.l.Math native methods
//        let sin_f = SharedKlassRepo::double_mapper_factory(|i: f64| -> f64 { i.sin() });
//        self.install_native_method(&"java/lang/Math".to_string(), &"sin:(D)D".to_string(), sin_f);
        self.install_native_method(&"java/lang/Math".to_string(), &"sin:(D)D".to_string(), java_lang_Math__sin);
        self.install_native_method(&"java/lang/Math".to_string(), &"cos:(D)D".to_string(), java_lang_Math__cos);
        self.install_native_method(&"java/lang/Math".to_string(), &"tan:(D)D".to_string(), java_lang_Math__tan);
        self.install_native_method(&"java/lang/Math".to_string(), &"asin:(D)D".to_string(), java_lang_Math__asin);
        self.install_native_method(&"java/lang/Math".to_string(), &"acos:(D)D".to_string(), java_lang_Math__acos);
        self.install_native_method(&"java/lang/Math".to_string(), &"atan:(D)D".to_string(), java_lang_Math__atan);
        self.install_native_method(&"java/lang/Math".to_string(), &"exp:(D)D".to_string(), java_lang_Math__exp);
        self.install_native_method(&"java/lang/Math".to_string(), &"log:(D)D".to_string(), java_lang_Math__log);
        self.install_native_method(&"java/lang/Math".to_string(), &"sqrt:(D)D".to_string(), java_lang_Math__sqrt);
//public static final native double IEEEremainder(double, double);
        self.install_native_method(&"java/lang/Math".to_string(), &"ceil:(D)D".to_string(), java_lang_Math__ceil);
        self.install_native_method(&"java/lang/Math".to_string(), &"floor:(D)D".to_string(), java_lang_Math__floor);
//public static final native double rint(double);
        self.install_native_method(&"java/lang/Math".to_string(), &"atan2:(DD)D".to_string(), java_lang_Math__atan2);
        self.install_native_method(&"java/lang/Math".to_string(), &"pow:(DD)D".to_string(), java_lang_Math__pow);

        // TODO Get enough of java.io.PrintStream working to get System.out.println() to work

        // // private native void open(String name) throws IOException;
        // self.install_native_method(&"java/io/FileOutputStream".to_string(), &"open:(Ljava/lang/String;)V".to_string(), java_io/_FileOutputStream__open);
        
        // // public native void write(int b) throws IOException;
        // self.install_native_method(&"java/io/FileOutputStream".to_string(), &"write:(I)V".to_string(), java_io/_FileOutputStream__write);

        // // private native void writeBytes(byte b[], int off, int len) throws IOException;
        // self.install_native_method(&"java/io/FileOutputStream".to_string(), &"writeBytes:([BII])V".to_string(), java_io/_FileOutputStream__writeBytes);

        // // public native void close() throws IOException;
        // self.install_native_method(&"java/io/FileOutputStream".to_string(), &"close:()V".to_string(), java_io/_FileOutputStream__close);

        // // private static native FileDescriptor initSystemFD(FileDescriptor fdObj, int desc);
        self.install_native_method(&"java/io/FileDescriptor".to_string(), &"initSystemFD:(Ljava/io/FileDescriptor;I)Ljava/io/FileDescriptor;".to_string(), java_io_FileDescriptor__initSystemFD);

        // let s = format!("{:?}", self.klass_lookup);
        // dbg!(s);

        // All native methods are installed for the bootstrap classes 
        // Now, we need to run the static initializers in the right order
        self.run_clinit_method(&"java/io/FileDescriptor".to_string(), i_callback);

        // // This requires the file descriptor handling to already exist
        // self.run_clinit_method(&"java/lang/System".to_string(), i_callback);
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
