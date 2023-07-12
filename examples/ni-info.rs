use color_eyre::eyre::Result;
use ni_file::{self, nks::v42::KontaktV42, NIFileType, NISound};

pub fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-info <FILE_GLOB>");
        return Ok(());
    };

    let paths: Vec<std::path::PathBuf> = wax::Glob::new(&path)
        .unwrap()
        .walk(".")
        .flatten()
        .map(|entry| entry.into_path())
        .filter(|entry| entry.is_file())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect();

    // repository containers (used in most instruments)
    for path in paths {
        let file = std::fs::read(&path)?;
        println!("\n{}:", path.as_os_str().to_str().unwrap());

        match NIFileType::detect(&file) {
            NIFileType::NISound => {
                let sound = NISound::read(file.as_slice())?;
                println!("format:\t\t\tNISound {}", sound.nisound_version()?);

                println!(
                    "authoring_app:\t\t{:?} {}",
                    sound.authoring_application()?,
                    sound.preset_version()?
                );
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
                KontaktV42::read(file.as_slice())?;
            }
            NIFileType::Kontakt42 => {
                println!("format:\t\tKontakt42");
            }
            NIFileType::Unknown => {
                println!("format:\t\tunknown");
            }
            NIFileType::KontaktResource => todo!(),
            NIFileType::KontaktCache => todo!(),
        };
    }

    Ok(())
}
