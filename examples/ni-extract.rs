//
//  Extract raw InternalPresetData from an NISD container.
//

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

use color_eyre::eyre::Result;
use ni_file::{kontakt::objects::BPatchHeader, nis::schema::Repository, NIFile};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-extract <FILE>");
        return Ok(());
    };

    let file = File::open(&path)?;

    match NIFile::read(file)? {
        NIFile::NISoundContainer(container) => {
            println!("Detected format: NISound Container\n");

            let repository = Repository::from(container);

            match repository.infer_schema() {
                ni_file::nis::schema::NISObject::AppSpecific(_) => todo!(),
                ni_file::nis::schema::NISObject::Repository(_) => todo!(),
                ni_file::nis::schema::NISObject::BNISoundPreset(p) => {
                    std::fs::write("bni-preset.kon", &p.patch()?.data)?;
                    println!("write bni-preset.kon");
                }
                ni_file::nis::schema::NISObject::Preset(_) => todo!(),
                ni_file::nis::schema::NISObject::PresetChunkItem(_) => todo!(),
                ni_file::nis::schema::NISObject::Unknown => todo!(),
            };
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
        NIFile::NKSContainer(nks) => {
            println!("Detected format: Kontakt Container\n");
            match nks.header {
                BPatchHeader::BPatchHeaderV1(_) => {
                    std::fs::write("kon1.xml", &nks.decompressed_preset()?)?
                }
                BPatchHeader::BPatchHeaderV2(ref h) => {
                    if h.is_monolith {
                        unimplemented!("NKSv2 Monolith {:?}", h.is_monolith);
                    } else {
                        let filename =
                            format!("{:?}.{:?}.xml", h.app_signature, h.patch_type).to_lowercase();
                        std::fs::write(filename, &nks.decompressed_preset()?)?;
                    }
                }
                BPatchHeader::BPatchHeaderV42(ref h) => {
                    let filename =
                        format!("{:?}.{:?}.kon", h.app_signature, h.patch_type).to_lowercase();
                    std::fs::write(filename, &nks.decompressed_preset()?)?;
                }
            }
        }
        _ => todo!(),
    }

    Ok(())
}
