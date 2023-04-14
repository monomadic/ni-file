use ni_file::prelude::*;
use ni_file::{NIContainer, NIFileType};

pub fn main() -> Result<()> {
    // lets read a Maschine file.
    // let file =
    //     include_bytes!("../tests/files/nicontainer/file/maschine/001-standard.mxfx").as_slice();

    // lets read an FM8 file.
    // let file = include_bytes!("../tests/data/files/fm8/001-fm7.nfm8").as_slice();

    let file = include_bytes!("../tests/data/nisound/file/kontakt/7.1.3.0/002-single-sample-2.nki");

    // // lets read a Massive 1.0.0.0 file.
    // let file = include_bytes!("../tests/data/files/massive/000-new.nmsv").as_slice();

    // make sure this is a valid NISound container
    if NIFileType::detect(file) == NIFileType::NISound {
        // read the repository
        let repo = NIContainer::read(file.as_slice())?;
        let chunk = repo.chunk()?;
        let preset = repo.preset()?;
        println!(
            "Writing preset chunk for {:?} {}",
            preset.authoring_app, preset.version
        );

        std::fs::write("chunk", &chunk)?;
    } else {
        println!("no file detected.")
    }

    Ok(())
}
