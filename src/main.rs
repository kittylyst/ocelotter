use std::env;
use std::path::Path;

use ocelotter_runtime::klass_parser::*;
use ocelotter_runtime::JvmValue::*;
use ocelotter_util::file_to_bytes;

use ocelotter::exec_method;
use ocelotter::CONTEXT;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    // FIXME In reality, need to bootstrap rt.jar
    CONTEXT.lock().unwrap().get_repo().bootstrap();

    let f_name = args[1].clone();

    let fq_klass_name = f_name.clone() + ".class";
    let bytes = match file_to_bytes(Path::new(&fq_klass_name)) {
        Ok(buf) => buf,
        _ => panic!("Error reading {}", f_name),
    };
    let mut parser = OtKlassParser::of(bytes, fq_klass_name.clone());
    parser.parse();
    let mut k = parser.klass();
    CONTEXT.lock().unwrap().get_repo().add_klass(&mut k);

    // FIXME Real main() signture required, dummying for ease of testing
    let main_str: String = f_name.clone() + ".main2:([Ljava/lang/String;)I";
    let meth = k.get_method_by_name_and_desc(main_str);

    let opt_ret = exec_method(meth);
    let ret = match opt_ret {
        Some(value) => value,
        None => panic!("Error executing ".to_owned() + &f_name + " - no value returned"),
    };
    let ret_i = match ret {
        Int { val: i } => i,
        _ => panic!("Error executing ".to_owned() + &f_name + " - non-int value returned"),
    };
    println!("Ret: {}", ret_i);
}
