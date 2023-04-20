use ni_file::{NIFileType, NISound};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = include_bytes!("../tests/data/nisound/file/kontakt/7.1.3.0/002-single-sample-2.nki");

    // make sure this is a valid NISound container
    if NIFileType::detect(file) == NIFileType::NISound {
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
        println!("no file detected.")
    }

    Ok(())
}
