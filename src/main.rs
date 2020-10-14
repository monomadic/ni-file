use std::{io, path::PathBuf};

mod cb;
mod cmd;
mod deflate;
mod fm8;
mod kontakt;
mod ni;
mod ni_file;
mod offset;
mod output;
mod strings;
mod structures;

use ni_file::NIFile;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "astryx")]
struct Opt {
    /// Input file
    file: String,
    
    /// Command
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// dump the internet preset
    Dump {
        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },
    Print,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let file = std::fs::read(opt.file)?;

    match ni::read(&file) {
        Ok((_, segment)) => {
            match opt.command {
                Command::Dump { output } => {
                    std::fs::write(output, NIFile::from(segment).preset)?;
                }

                Command::Print => {
                    output::print_segment(&segment); // todo: don't use stdout inside this fn
                },
            };
        }
        Err(e) => println!("error: {:?}", e),
    };

    Ok(())
}
