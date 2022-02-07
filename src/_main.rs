#![feature(cursor_remaining)]
#![feature(buf_read_has_data_left)]

#[macro_use]
extern crate log;

mod app_version;
mod args;
mod cb;
mod container;
mod deflate;
mod detect;
mod extract;
mod extractor;
mod fm8;
mod kontakt;
mod lz77;
mod monolith;
mod ni_file;
mod offset;
mod output;
mod strings;
mod structures;
mod checksum;

pub type Error = Box<dyn std::error::Error>;

fn main() {
    // set up a logger with default level 'info'
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    match args::run() {
        Ok(_) => info!("done"),
        Err(e) => error!("{}", e), // todo: print error properly
    }
}