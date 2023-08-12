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
            let file = File::open(&path)?;
            let sound = Repository::read(file)?;
            println!("format:\t\t\tNISound {}", sound.nisound_version()?);

            println!(
                "authoring_app:\t\t{:?} {}",
                sound.authoring_application()?,
                sound.preset_version()?
            );

            use ni_file::nisound::AuthoringApplication::*;
            match sound.authoring_application()? {
                FM8 => {
                    let raw_preset = Cursor::new(sound.preset_raw()?);
                    FM8Preset::read(raw_preset)?;
                }
                _ => (),
            }
        }
        NIFileType::NIMonolith => {
            println!("format:\t\tNIMonolith");
        }
        NIFileType::NICompressedWave => todo!(),
        NIFileType::KoreSound => todo!(),
        NIFileType::Kontakt1 => {
            println!("format:\t\tKontakt1");
        }
        NIFileType::NKS => {
            println!("format:\t\t\tNKS Container (Kontakt)");
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
        NIFileType::Unknown => {
            println!("format:\t\tunknown");
        }
        NIFileType::KontaktResource => todo!(),
        NIFileType::KontaktCache => todo!(),
    };

    Ok(())
}
