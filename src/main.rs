#[macro_use]
extern crate lazy_static;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use structopt::StructOpt;

use klass::klass_repo::SharedKlassRepo;
use klass::options::Options;
use klass::otklass::OtKlassComms;
use interpreter::thread::start_new_jthread;
use crate::klass::otklass::OtKlass;

pub mod klass;
pub mod interpreter;

pub fn main() {
    // Parse any command-line arguments
    let options = Options::from_args();

    // Advanced option handling would go here

    // Handle the "send fname, get klass back"
    let (tx_fname, rx_fname): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (tx_klass, rx_klass): (Sender<OtKlass>, Receiver<OtKlass>) = mpsc::channel();

    // General comms
    let (tx, rx): (Sender<OtKlassComms>, Receiver<OtKlassComms>) = mpsc::channel();

    let k_keep = thread::spawn(move || {
        SharedKlassRepo::start(options, tx_fname, rx)
    });

    let f_name = rx_fname.recv().unwrap();
    let j_main = thread::spawn(move || {
        start_new_jthread(f_name, tx)
    });

    j_main.join().unwrap();
    // k_keep.clean_shutdown();
    k_keep.join().unwrap();
}

// mod tests