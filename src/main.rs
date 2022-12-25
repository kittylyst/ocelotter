#[macro_use]
extern crate lazy_static;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
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

    // Advanced option handling would go here

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let k_keep = thread::spawn(move || {
        SharedKlassRepo::start(options, tx)
    });

    let f_name = rx.recv().unwrap();
    let j_main = thread::spawn(move || {
        start_new_jthread(f_name)
    });

    j_main.join().unwrap();
    // k_keep.clean_shutdown();
    k_keep.join().unwrap();
}
