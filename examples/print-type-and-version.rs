use ni_file::{NIContainer, NIFileType, Preset, RepositoryRoot};

#[allow(dead_code)]
fn setup_logger() {
    let _ = log::set_logger(&loggy::Loggy {
        prefix: "",
        show_time: false,
        show_thread: true,
    });
    log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // uncomment this to use debug logging:
    // setup_logger();

    // // lets read a Kontakt 7 file.
    // let file = include_bytes!("../tests/data/files/kontakt-7/000-default.nki").as_slice();

    // // lets read an FM8 file.
    // let file = include_bytes!("../tests/data/files/fm8/001-fm7.nfm8").as_slice();

    // lets read a Massive 1.0.0.0 file.
    let file = include_bytes!("../tests/data/files/massive/000-new.nmsv").as_slice();

    // make sure this is a valid NIContainer
    if NIFileType::detect(file) == NIFileType::NISound {
        // read the repository
        let repo = NIContainer::read(file)?;

        // parse the data fields of the base frame
        let root: RepositoryRoot = repo.root()?;

        // print the major version number
        println!(
            "NISound Version: {}.{}.{}",
            root.major_version(),
            root.minor_version(),
            root.patch_version(),
        );

        let preset: Preset = repo.preset()?;
        println!(
            "Authoring Application: {:?} {}",
            preset.authoring_app, preset.version
        );

        println!("Compressed: {:?}", preset.is_compressed);
    }

    Ok(())
}
