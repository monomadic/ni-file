use std::path::PathBuf;
use structopt::StructOpt;

use ni_file::detect::NIFileType;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    let buffer = std::fs::read(args.input)?;

    match ni_file::detect::filetype(&buffer) {
        NIFileType::NIContainer => {
            //println!("{}", ni_file::ni_container::read(&buffer)?.to_xml());
            Ok(())
        }
        NIFileType::NIKontaktMonolith => todo!(),
        NIFileType::KoreSound => todo!(),
        NIFileType::Unknown => todo!(),
        NIFileType::Kontakt2 => todo!(),
    }
}

#[derive(StructOpt)]
#[structopt(name = "ni-file", about = "NI Extractor")]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}
