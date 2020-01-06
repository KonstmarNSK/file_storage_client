use std::{env, io};
use std::io::BufRead;

mod storage;
mod filesystem;
mod auth;

fn main() {
    env_logger::init().unwrap();
    let mountpoint = env::args_os().nth(1).unwrap();

    let storage = auth::authenticate("".to_string(), "".to_string());

    let bg_session = filesystem::mount(&mountpoint, storage);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        println!("you entered: {}", s);
        if &s == "unmount" {
            return;
        }
    }
}
