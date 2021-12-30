#[macro_use]
extern crate log;

mod args;
mod cb;
mod container;
mod deflate;
mod extract;
mod extractor;
mod fm8;
mod kontakt;
mod ni_file;
mod offset;
mod output;
mod strings;
mod structures;

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
