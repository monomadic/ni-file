use ni_file::detect;
use std::path::PathBuf;
use structopt::StructOpt;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Command::from_args();
    match args {
        Command::Extract { path } => {
            let buffer = std::fs::read(path)?;
            detect::filetype(&buffer);
        }
        Command::Convert { path } => {
            let buffer = std::fs::read(path)?;
            match detect::filetype(&buffer) {
                detect::NIFileType::NIContainer => {
                    println!("{}", ni_file::ni_container::read(&buffer)?.to_xml());
                }
                detect::NIFileType::NIKontaktMonolith => todo!(),
                detect::NIFileType::KoreSound => todo!(),
                detect::NIFileType::Unknown => todo!(),
            }
        }
        Command::Info { path } => {
            let buffer = std::fs::read(path)?;
            detect::filetype(&buffer);
        }
    };

    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "ni-file", about = "NI Extractor")]
enum Command {
    Extract { path: PathBuf },
    Convert { path: PathBuf },
    Info { path: PathBuf },
}
