//
//  Extract raw InternalPresetData from an NISD container.
//

use ni_file::{NIFileType, NISound};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-extract <FILE>");
        return Ok(());
    };

    let file = std::fs::read(&path)?;

    // make sure this is a valid NISound container
    if NIFileType::detect(file.as_slice())? == NIFileType::NISound {
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
    } else {
        println!("error: file is not a valid nisound container.")
    }

    Ok(())
}
