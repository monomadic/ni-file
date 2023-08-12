//
//  Extract raw InternalPresetData from an NISD container.
//

use std::io::Cursor;

use color_eyre::eyre::Result;
use ni_file::{nks::nksfile::NKSFile, NIFileType, Repository};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-extract <FILE>");
        return Ok(());
    };

    let file = std::fs::read(&path)?;

    match NIFileType::detect(Cursor::new(&file))? {
        NIFileType::NISound => {
            // read the repository
            let repo = Repository::read(Cursor::new(file))?;

            println!("Detected NISound version: {}", repo.nisound_version()?);

            println!(
                "Writing preset chunk for {:?} {:?}",
                repo.authoring_application(),
                repo.preset_version()
            );

            let preset = repo.preset_raw()?;
            std::fs::write("preset", &preset)?;
            println!("Wrote: inner preset");
        }
        NIFileType::NIMonolith => todo!(),
        NIFileType::NICompressedWave => todo!(),
        NIFileType::KoreSound => todo!(),
        NIFileType::Kontakt1 => todo!(),
        NIFileType::NKS => {
            let nks = NKSFile::read(Cursor::new(file))?;
            std::fs::write("chunk", &nks.compressed_patch_data)?;
        }
        NIFileType::KontaktResource => todo!(),
        NIFileType::KontaktCache => todo!(),
        NIFileType::Unknown => todo!(),
    }

    Ok(())
}
