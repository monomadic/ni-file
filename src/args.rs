use std::path::PathBuf;
use structopt::StructOpt;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("NI Extractor v0.0.1\n");

    let args = Args::from_args();
    let input = args.input;
    let buffer = std::fs::read(input)?;

    // try to detect the type of file we're dealing with
    match crate::detect::filetype(&buffer) {
        crate::detect::NIFileType::NIContainer => {
            let container = ni_file::ni_container::read(&buffer)?;
            info!("hsin id {}", container.id);
            info!("{:?}", container);
        }
        crate::detect::NIFileType::NIKontaktMonolith => todo!(),
        crate::detect::NIFileType::KoreSound => todo!(),
        crate::detect::NIFileType::Unknown => todo!(),
    }
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "extract", about = "RSDK Extractor")]
struct Args {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}
