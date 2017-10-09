#![feature(never_type)]

extern crate sendgrid;
extern crate redis;
extern crate threadpool;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod init;
mod config;
mod worker;
mod mail;

fn main() {
    init::run();
}
