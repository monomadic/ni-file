use color_eyre::eyre::Result;
use ni_file::{self, NIFileType, NISound};

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
                println!("format:\t\t\tNISound");

                let sound = NISound::read(file.as_slice())?;

                println!("nisound_version:\t{}", sound.nisound_version()?);
                println!("authoring_app:\t\t{:?}", sound.authoring_application()?);
                println!("preset_version:\t\t{}", sound.preset_version()?);
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
            NIFileType::Unknown => todo!(),
        };
    }

    Ok(())
}
