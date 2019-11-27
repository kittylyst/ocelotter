use std::env;
use std::path::Path;

use ocelotter_runtime::klass_parser::*;
use ocelotter_runtime::InterpLocalVars;
use ocelotter_runtime::JvmValue::*;
use ocelotter_util::file_to_bytes;

use ocelotter::exec_method;
use ocelotter_runtime::SharedKlassRepo;
use ocelotter_runtime::HEAP;
use ocelotter_runtime::REPO;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    // FIXME In reality, need to bootstrap rt.jar
    let mut bare_repo = SharedKlassRepo::of();
    bare_repo.bootstrap(ocelotter::exec_method);
    *REPO.lock().unwrap() = bare_repo;

    let f_name = args[1].clone();

    let fq_klass_name = f_name.clone() + ".class";
    let bytes = match file_to_bytes(Path::new(&fq_klass_name)) {
        Ok(buf) => buf,
        _ => panic!("Error reading {}", f_name),
    };
    let mut parser = OtKlassParser::of(bytes, fq_klass_name.clone());
    parser.parse();
    let k = parser.klass();
    REPO.lock().unwrap().add_klass(&k);

    // FIXME Real main() signture required, dummying for ease of testing
    let main_str: String = f_name.clone() + ".main2:([Ljava/lang/String;)I";
    let main = k.get_method_by_name_and_desc(&main_str)
                      .expect(&format!("Error: Main method not found {}", main_str.clone())).clone();

    // FIXME Parameter passing
    let mut vars = InterpLocalVars::of(5);
    let ret = exec_method(&main, &mut vars).expect(&format!("Error: When executing {} - no value returned", f_name));
    let ret_i = match ret {
        Int { val: i } => i,
        _ => panic!("Error executing ".to_owned() + &f_name + " - non-int value returned"),
    };
    println!("Ret: {}", ret_i);
}
