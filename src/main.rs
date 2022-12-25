#[macro_use]
extern crate lazy_static;

use std::thread;

use structopt::StructOpt;

use klass::klass_repo::SharedKlassRepo;
use klass::options::Options;
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
