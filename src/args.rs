use std::path::PathBuf;
use structopt::StructOpt;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Command::from_args();
    match args {
        Command::Extract { path } => {
            let buffer = std::fs::read(path)?;
            crate::detect::filetype(&buffer);
        }
        Command::Convert { path } => {
            let buffer = std::fs::read(path)?;
            crate::detect::filetype(&buffer);
        }
        Command::Info { path } => {
            let buffer = std::fs::read(path)?;
            crate::detect::filetype(&buffer);
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
