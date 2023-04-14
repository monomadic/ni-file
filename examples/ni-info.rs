use color_eyre::eyre::Result;
use ni_file::{self, NIContainer, NIFileType};

pub fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-info <FILE_GLOB>");
        return Ok(());
    };

    let paths: Vec<std::path::PathBuf> = glob::glob(&path)
        .expect("glob error")
        .filter_map(|path| path.ok())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect();

    // repository containers (used in most instruments)
    for path in paths {
        let file = std::fs::read(&path)?;

        match NIFileType::detect(&file) {
            NIFileType::NISound => {
                println!("format:\t\tNISound");

                let container = NIContainer::read(file.as_slice())?;
                let preset = container.preset()?;

                println!("authoring_app:\t{:?}", preset.authoring_app);
                println!("version:\t{}", preset.version);
            }
            NIFileType::NIMonolith => {
                println!("format:\t\tNIMonolith");
            }
            NIFileType::NICompressedWave => todo!(),
            NIFileType::KoreSound => todo!(),
            NIFileType::Kontakt1 => {
                println!("format:\t\tKontakt1");
            }
            NIFileType::Kontakt2 => {
                println!("format:\t\tKontakt2");
            }
            NIFileType::Kontakt42 => {
                println!("format:\t\tKontakt42");
            }
            NIFileType::NIPresetKontakt => todo!(),
            NIFileType::FM8Preset => todo!(),
            NIFileType::Unknown => todo!(),
        };
    }

    Ok(())
}
