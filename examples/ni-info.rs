use std::{fs::File, io::Cursor};
use tracing::instrument;

use color_eyre::eyre::Result;
use ni_file::{
    self,
    fm8::FM8Preset,
    nks::{header::NKSHeader, nksfile::NKSFile},
    NIFileType, Repository,
};

#[instrument]
pub fn main() -> Result<()> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing();

    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-info <FILE_GLOB>");
        return Ok(());
    };

    match NIFileType::detect(File::open(&path)?)? {
        NIFileType::NISound => {
            println!("Detected format:\t\t\tNIS (Native Instruments Sound) Container");

            let file = File::open(&path)?;
            let repository = Repository::read(file)?;

            println!("Version: {}", repository.nisound_version()?);
            println!(
                "Authoring Application: {:?} {}\n",
                repository.authoring_application()?,
                repository.preset_version()?
            );

            use ni_file::nisound::AuthoringApplication::*;
            match repository.authoring_application()? {
                FM8 => {
                    let raw_preset = Cursor::new(repository.preset_raw()?);
                    FM8Preset::read(raw_preset)?;
                }
                _ => (),
            }
        }
        NIFileType::FileContainer => {
            println!("Detected format:\t\tFileContainer (Monolith)");
        }
        NIFileType::NICompressedWave => todo!(),
        NIFileType::KoreSound => todo!(),
        NIFileType::NKS => {
            println!("Detected format:\t\t\tNKS (Native Instruments Kontakt Sound) Container");
            let file = File::open(&path)?;
            let nks = NKSFile::read(file)?;

            use NKSHeader::*;
            match nks.header {
                BPatchHeaderV2(h) => {
                    println!("type:\t\t\t{:?}", h.patch_type);
                    println!(
                        "kontakt_version:\t{}.{}.{}.{}",
                        h.app_version.major,
                        h.app_version.minor_1,
                        h.app_version.minor_2,
                        h.app_version.minor_3
                    );
                    println!("author:\t\t\t{}", h.author);
                    println!("zones:\t\t\t{}", h.number_of_zones);
                    println!("groups:\t\t\t{}", h.number_of_groups);
                    println!("instruments:\t\t{}", h.number_of_instruments);
                    println!("created_at:\t\t{}", h.created_at);
                }
                BPatchHeaderV42(h) => {
                    println!("type:\t\t\t{:?}", h.patch_type);
                    println!(
                        "kontakt_version:\t{}.{}.{}.{}",
                        h.app_version.major,
                        h.app_version.minor_1,
                        h.app_version.minor_2,
                        h.app_version.minor_3
                    );
                    println!("author:\t\t\t{}", h.author);
                    println!("zones:\t\t\t{}", h.number_of_zones);
                    println!("groups:\t\t\t{}", h.number_of_groups);
                    println!("instruments:\t\t{}", h.number_of_instruments);
                    println!("created_at:\t\t{}", h.created_at);
                }
            }
        }
        _ => {
            println!("format:\t\tunknown");
        }
    };

    Ok(())
}
