//
//  Extract raw InternalPresetData from an NISD container.
//

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

use color_eyre::eyre::Result;
use ni_file::{nis::items::RepositoryRootContainer, nks::header::BPatchHeader, NIFile};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-extract <FILE>");
        return Ok(());
    };

    let file = File::open(&path)?;

    match NIFile::read(file)? {
        NIFile::NISContainer(container) => {
            let repository = RepositoryRootContainer::try_from(&container)?;

            if let Ok(root) = repository.properties() {
                println!("NISound {}", root.nisound_version);
            }

            if let Some(app_specific) = repository.app_specific() {
                println!("\nKontakt multi detected.");
                let app = app_specific?;
                let props = app.properties()?;
                let inner = RepositoryRootContainer(props.subtree_item.item()?);

                if let Some(preset) = inner.kontakt_preset() {
                    println!("\nKontakt instrument detected.");

                    if let Some(preset) = preset?.preset_data() {
                        std::fs::write("kontakt.nkm.kon", &preset?)?;
                    }
                }
            }

            if let Some(preset) = repository.kontakt_preset() {
                println!("\nKontakt instrument detected.");

                if let Some(preset) = preset?.preset_data() {
                    std::fs::write("kontakt.nki.kon", &preset?)?;
                }
            }

            // if let Some(subtree) = repo.subtree_item() {
            //     std::fs::write("subtree_item.nki", &subtree?.inner_data)?;
            //     println!("Wrote: subtree_item.nki");
            // }

            // if let Some(single) = repo.encryption_item() {
            //     let preset = single.preset_raw()?;
            //     std::fs::write("inner.nki", &preset)?;
            //     println!("Wrote: inner.nki");
            // }

            // let item = ItemContainer::read(Cursor::new(preset))?;
            // match item.find(&ItemID::PresetChunkItem) {
            //     Some(preset_item_frame) => {
            //         let preset_chunk_item: PresetChunkItem =
            //             preset_item_frame.clone().try_into()?;
            //         std::fs::write("preset.chunk", &preset_chunk_item.chunk())?;
            //         println!("Wrote: preset.chunk");
            //     }
            //     None => todo!(),
            // }
        }
        NIFile::Monolith(container) => {
            println!("Detected format:\t\tMonolith (FileContainer Archive)\n");

            let mut file = File::open(&path)?;
            for item in container.items {
                println!("Writing: {}", &item.filename);

                file.seek(SeekFrom::Start(
                    item.file_start_offset + container.file_section_offset,
                ))?;
                let mut output = File::create(&item.filename)?;

                let mut buf = vec![0u8; item.file_size as usize];
                file.read_exact(&mut buf)?;

                output.write_all(&buf)?;
            }
        }
        NIFile::NKSContainer(nks) => match nks.header {
            BPatchHeader::BPatchHeaderV1(_) => todo!(),
            BPatchHeader::BPatchHeaderV2(_) => todo!(),
            BPatchHeader::BPatchHeaderV42(ref h) => {
                let filename =
                    format!("{:?}.{:?}.kon", h.app_signature, h.patch_type).to_lowercase();
                std::fs::write(filename, &nks.preset_data()?)?;
            }
        },
        //  => std::fs::write("preset.xml", v1.preset_xml()?)?,
        // KontaktPreset::Kon2(_) => todo!(),
        // KontaktPreset::Kon4(v42) => {
        //     std::fs::write("chunk_data.kontakt", v42.decompress_patch_data()?)?
        // }
        _ => todo!(),
    }

    Ok(())
}
