use ni_file::detect;
use std::path::PathBuf;
use structopt::StructOpt;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    let buffer = std::fs::read(args.input)?;

    match detect::filetype(&buffer) {
        detect::NIFileType::NIContainer => {
            //println!("{}", ni_file::ni_container::read(&buffer)?.to_xml());
            Ok(())
        }
        detect::NIFileType::NIKontaktMonolith => todo!(),
        detect::NIFileType::KoreSound => todo!(),
        detect::NIFileType::Unknown => todo!(),
    }
}

#[derive(StructOpt)]
#[structopt(name = "ni-file", about = "NI Extractor")]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}
