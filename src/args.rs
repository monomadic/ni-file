use std::path::PathBuf;
use structopt::StructOpt;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("NI Extractor v0.0.1\n");

    let args = Args::from_args();
    let input = args.input;

    info!("Reading {:?}", input);

    let buffer = std::fs::read(input)?;

    let _ = crate::extractor::read(&buffer)?;

    // crate::extract::read(&buffer)?;

    if !args.skip_write {
        // std::fs::write(output, crate::ni_file::NIFile::from(segment).preset)?;
    }

    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "extract", about = "RSDK Extractor")]
struct Args {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Skip writing files to disk
    #[structopt(long = "skip-write")]
    skip_write: bool,
}
