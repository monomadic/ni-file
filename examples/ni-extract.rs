//
//  Extract raw InternalPresetData from an NISD container.
//

use std::{fs::File, io::Cursor};

use color_eyre::eyre::Result;
use ni_file::{
    nis::{items::PresetChunkItem, ItemContainer, ItemID},
    NIFile,
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-extract <FILE>");
        return Ok(());
    };

    let file = File::open(&path)?;

    match NIFile::read(file)? {
        NIFile::NISContainer(repo) => {
            println!("Detected NISound version: {}", repo.nisound_version()?);
            println!(
                "Authoring Application: {:?} {}\n",
                repo.authoring_application()?,
                repo.preset_version()?
            );

            let preset = repo.preset_raw()?;
            std::fs::write("inner-container", &preset)?;
            println!("Wrote: inner-container");

            let item = ItemContainer::read(Cursor::new(preset))?;
            match item.find(&ItemID::PresetChunkItem) {
                Some(preset_item_frame) => {
                    let preset_chunk_item: PresetChunkItem =
                        preset_item_frame.clone().try_into()?;
                    std::fs::write("preset-chunk", &preset_chunk_item.chunk())?;
                    println!("Wrote: preset_chunk");
                }
                None => todo!(),
            }
        }
        NIFile::NKSContainer(nks) => {
            std::fs::write("chunk", &nks.compressed_patch_data)?;
        }
        _ => todo!(),
    }

    Ok(())
}
