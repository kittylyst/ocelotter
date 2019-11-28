#[derive(Debug)]
pub struct SharedKlassRepo {
    klass_count: AtomicUsize,
    klass_lookup: HashMap<String, usize>,
    id_lookup: HashMap<usize, OtKlass>,
}

impl SharedKlassRepo {
    pub fn of() -> SharedKlassRepo {
        SharedKlassRepo {
            klass_lookup: HashMap::new(),
            id_lookup: HashMap::new(),
            klass_count: AtomicUsize::new(1),
        }
    }

    pub fn lookup_klass(&self, klass_name: &String) -> &OtKlass {
        let s = format!("{}", self);
        dbg!(s);

        let kid = self.klass_lookup.get(klass_name).expect(&format!("No klass called {} found in repo", klass_name));
        self.id_lookup.get(kid).expect(&format!("No klass with ID {} found in repo", kid))
    }

    pub fn add_klass(&mut self, k: &OtKlass) -> () {
        k.set_id(self.klass_count.fetch_add(1, Ordering::SeqCst));
        let id = k.get_id();
        let k2: OtKlass = (*k).to_owned();

        self.klass_lookup.insert(k.get_name().clone(), id);
        self.id_lookup.insert(id, k2);
    }

    fn add_bootstrap_class(&mut self, cl_name: String) {
        let mut k: OtKlass = // Pull the bytes off disk and parse them to an OtKlass
        self.add_klass(&mut k);
    }

    pub fn bootstrap(&mut self, i_callback: fn(&OtMethod, &mut InterpLocalVars) -> Option<JvmValue>) -> () {
        // Add java.lang.Object
        self.add_bootstrap_class("java/lang/Object".to_string());
        // let s = format!("{}", self);
        // dbg!(s);
        let k_obj = self.lookup_klass(&"java/lang/Object".to_string());
        // ...

        ()
    }
}

impl fmt::Display for SharedKlassRepo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} with klasses {:?}",
            self.klass_count, self.id_lookup
        )
    }
}
