//
//  Extract raw InternalPresetData from an NISD container.
//

use ni_file::{nks::nksfile::NKSFile, NIFileType, NISound};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-extract <FILE>");
        return Ok(());
    };

    let file = std::fs::read(&path)?;

    match NIFileType::detect(file.as_slice())? {
        NIFileType::NISound => {
            // read the repository
            let repo = NISound::read(file.as_slice())?;

            println!("Detected NISound version: {}", repo.nisound_version()?);

            println!(
                "Writing preset chunk for {:?} {:?}",
                repo.authoring_application(),
                repo.preset_version()
            );

            let chunk = repo.chunk()?;
            std::fs::write("chunk", &chunk)?;
        }
        NIFileType::NIMonolith => todo!(),
        NIFileType::NICompressedWave => todo!(),
        NIFileType::KoreSound => todo!(),
        NIFileType::Kontakt1 => todo!(),
        NIFileType::NKS => {
            let nks = NKSFile::read(file.as_slice())?;
            std::fs::write("chunk", &nks.data)?;
        }
        NIFileType::KontaktResource => todo!(),
        NIFileType::KontaktCache => todo!(),
        NIFileType::Unknown => todo!(),
    }

    Ok(())
}
