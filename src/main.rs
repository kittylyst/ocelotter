use std::path::Path;
use std::thread;

use structopt::StructOpt;

// use ocelotter_runtime::JvmValue::*;
// use ocelotter_util::file_to_bytes;
//
// use ocelotter::exec_method;
// use ocelotter_util::ZipFiles;
// use ocelotter_runtime::options::Options;

use klass::klass_parser::*;
use klass::klass_repo::SharedKlassRepo;
use klass::options::Options;
use interpreter::InterpLocalVars;
use interpreter::thread::start_new_jthread;

pub mod klass;
pub mod interpreter;

pub fn main() {
    // Parse any command-line arguments
    let options = Options::from_args();

    let k_keep = thread::spawn(move || {
        SharedKlassRepo::start(options)
    });

    let j_main = thread::spawn(start_new_jthread(f_name));

    j_main.join().unwrap();
    // k_keep.clean_shutdown();
    k_keep.join().unwrap();
}
