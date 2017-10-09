use std;
use std::sync::Mutex;
use config::Config;
use worker::Worker;

pub fn run() {
    let config_path = match std::env::args().nth(1) {
        Some(v) => v,
        None => panic!(
            "Path to config file is required as the first command line argument"
        )
    };
    let config = Config::load_from_file(config_path).unwrap();

    for _ in 0..16 {
        let mut w = Worker::new(config.clone());
        std::thread::spawn(move || w.run());
    }

    deadlock();
}

fn deadlock() {
    let m = Mutex::new(false);
    let _h1 = m.lock().unwrap();
    let _h2 = m.lock().unwrap();
}
