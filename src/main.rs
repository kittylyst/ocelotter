use std::path::Path;

use ocelotter_runtime::klass_parser::*;
use ocelotter_runtime::klass_repo::SharedKlassRepo;
use ocelotter_runtime::InterpLocalVars;
use ocelotter_runtime::JvmValue::*;
use ocelotter_util::file_to_bytes;
use structopt::StructOpt;

use ocelotter::exec_method;
use ocelotter_util::ZipFiles;
use options::Options;

mod options;

pub fn main() {
    // Parse any command-line arguments
    let options = Options::from_args();

    let mut repo = SharedKlassRepo::of();
    repo.bootstrap(exec_method);

    let fq_klass_name = options.fq_klass_name();
    let f_name = options.f_name();

    if let Some(file) = &options.classpath {
        ZipFiles::new(file)
            .into_iter()
            .filter(|f| match f {
                Ok((name, _)) if name.ends_with(".class") => true,
                _ => false,
            })
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

    // FIXME Real main() signature required, dummying for ease of testing
    let main_str: String = f_name.clone() + ".main2:([Ljava/lang/String;)I";
    let main_klass = repo.lookup_klass(&f_name);
    let main = main_klass
        .get_method_by_name_and_desc(&main_str)
        .unwrap_or_else(|| panic!("Error: Main method not found {}", main_str.clone()));

    // FIXME Parameter passing
    let mut vars = InterpLocalVars::of(5);

    let ret = exec_method(&mut repo, main, &mut vars)
        .map(|return_value| match return_value {
            Int(i) => i,
            _ => panic!("Error executing {} - non-int value returned", &f_name),
        })
        .unwrap_or_else(|| panic!("Error executing {} - no value returned", &f_name));

    println!("Ret: {}", ret);
}
