use ncw::NcwReader;
use std::fs::File;

use color_eyre::eyre::Result;
use ni_file::{
    self,
    kontakt::{KontaktInstrument, KontaktPreset},
    nifile::NIFile,
    nis::{AppSpecific, ItemID},
    nks::header::BPatchHeader,
};

fn print_kontakt_instrument(instrument: KontaktInstrument) {
    match instrument.header {
        BPatchHeader::BPatchHeaderV1(ref h) => {
            println!("\nBPatchHeaderV1:");
            println!("  created_at:\t\t{}", h.created_at);
            println!("  samples_size:\t\t{}", h.samples_size);
        }
        BPatchHeader::BPatchHeaderV2(ref h) => {
            println!("\nBPatchHeaderV2:");
            println!("  signature:\t\t{}", h.app_signature);
            println!("  type:\t\t\t{:?}", h.patch_type);
            println!("  kontakt_version:\t{}", h.app_version);
            println!("  author:\t\t{}", h.author);
            println!("  zones:\t\t{}", h.number_of_zones);
            println!("  groups:\t\t{}", h.number_of_groups);
            println!("  instruments:\t\t{}", h.number_of_instruments);
            println!("  created_at:\t\t{}", h.created_at);
        }
        BPatchHeader::BPatchHeaderV42(h) => {
            println!("\nBPatchHeaderV42:");
            println!("  signature:\t\t{}", h.app_signature);
            println!("  type:\t\t\t{:?}", h.patch_type);
            println!("  kontakt_version:\t{}", h.app_version);
            println!("  author:\t\t{}", h.author);
            println!("  zones:\t\t{}", h.number_of_zones);
            println!("  groups:\t\t{}", h.number_of_groups);
            println!("  instruments:\t\t{}", h.number_of_instruments);
            println!("  created_at:\t\t{}", h.created_at);
        }
    }

    match instrument.preset {
        KontaktPreset::Kon1(_) => todo!(),
        KontaktPreset::Kon2(_) => todo!(),
        KontaktPreset::Kon3(_) => todo!(),
        KontaktPreset::Kon4(_) => todo!(),
        KontaktPreset::Kon7(_) => todo!(),
    }
}

// fn _print_kontakt_instrument(instrument: KontaktChunkSet) -> Result<()> {
//     println!("\nKontakt Data:");
//     if let Some(Ok(program)) = instrument.program() {
//         if let Ok(params) = program.public_params() {
//             println!("\nProgram:");
//             println!("  name:\t\t{}", params.name);
//             println!("  credits:\t{}", params.instrument_credits);
//             println!("  author:\t{}", params.instrument_author);
//         }
//     }
//     if let Some(filename_table) = instrument.filename_tables()? {
//         println!("\nFilename tables:");
//
//         println!("\nOther:");
//         for (index, filename) in &filename_table.other_filetable {
//             println!("{}:\t{}", index, filename);
//         }
//
//         println!("\nSamples:");
//         for (index, filename) in &filename_table.sample_filetable {
//             println!("{}:\t{}", index, filename);
//         }
//
//         println!("\nSpecial:");
//         for (index, filename) in &filename_table.special_filetable {
//             println!("{}:\t{}", index, filename);
//         }
//
//         if let Some(program) = instrument.program() {
//             if let Some(zones) = program?.zones() {
//                 println!("\nZones:");
//                 for zone in zones? {
//                     let zone_data = zone.public_params()?;
//                     if let Some(filename) = filename_table
//                         .sample_filetable
//                         .get(&(zone_data.filename_id as u32))
//                     {
//                         println!("Zone: {}", filename);
//                     }
//                 }
//             } else {
//                 println!("\nNo zones found!");
//             }
//         } else {
//             println!("\nNo program found!");
//         }
//     } else {
//         println!("\nNo filename table found!");
//     }
//
//     Ok(())
// }

pub fn main() -> Result<()> {
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-info <FILE_GLOB>");
        return Ok(());
    };

    let file = File::open(&path)?;

    match NIFile::read(file)? {
        NIFile::NISContainer(repository) => {
            if let Some(root) = repository.find_repository_root() {
                let root = root?;
                println!("RepositoryRoot:");
                println!("  version:            NISound {}", root.nisound_version);
                println!("  repository_magic:   {}", root.repository_magic);
                println!("  repository_type:    {}", root.repository_type);
            } else {
                println!("\nNot a RepositoryRoot");
            }

            if let Some(preset) = repository.item().find_preset_item() {
                let preset = preset?;
                println!("\nPreset:");
                println!(
                    "  authoring_app:\t{:?} {}",
                    preset.authoring_app, preset.version
                );
                println!("  is_factory_preset:\t{}", preset.is_factory_preset);
            }

            if let Some(preset) = repository.item().find_kontakt_preset_item() {
                let preset = preset?;
                println!("\nKontakt Preset:");
                println!(
                    "  authoring_app:\t{:?} {}",
                    preset.authoring_app, preset.version
                );
                println!("  is_factory_preset:\t{}", preset.is_factory_preset);

                if let Some(kontakt_preset) = repository.item().extract_kontakt_preset() {
                    print_kontakt_instrument(kontakt_preset?);
                }
            }

            // match repository.detect() {
            //     ni_file::nis::RepositoryType::KontaktPreset => todo!(),
            //     ni_file::nis::RepositoryType::AppSpecific => todo!(),
            //     ni_file::nis::RepositoryType::Preset => todo!(),
            //     ni_file::nis::RepositoryType::Unknown => todo!(),
            // }

            // multi
            if let Some(app_specific) = repository.item().find(&ItemID::AppSpecific) {
                let app = AppSpecific::try_from(&app_specific.data)?;
                println!("\nAppSpecific:");
                println!("  authoring_app:\t{:?} {}", app.authoring_app, app.version);
            }
        }
        NIFile::KontaktResource => {
            println!("Detected format:\tKontaktResource");
        }
        NIFile::NICache => {
            println!("Detected format:\tNICache");
        }
        NIFile::NICompressedWave => {
            println!("Detected format:\tNICompressedWave");
            let file = File::open(&path)?;
            let ncw = NcwReader::read(&file)?;
            println!("\nNCW:");
            println!("  channels:\t\t{}", ncw.header.channels);
            println!("  bits_per_sample:\t{}", ncw.header.bits_per_sample);
            println!("  sample_rate:\t\t{}", ncw.header.sample_rate);
        }
        NIFile::Monolith(container) => {
            println!("Detected format:\tMonolith (FileContainer Archive)\n");
            println!("Files:");
            for item in container.items {
                println!("  {}", item.filename);
            }
        }
        NIFile::NKSContainer(nks) => {
            println!("Detected format:\tNKS (Native Instruments Kontakt Sound) Container");

            match nks.header {
                BPatchHeader::BPatchHeaderV1(ref h) => {
                    println!("\nBPatchHeaderV1:");
                    println!("  created_at:\t\t{}", h.created_at);
                    println!("  samples_size:\t\t{}", h.samples_size);
                }
                BPatchHeader::BPatchHeaderV2(ref h) => {
                    println!("\nBPatchHeaderV2:");
                    println!("  signature:\t\t{}", h.app_signature);
                    println!("  type:\t\t\t{:?}", h.patch_type);
                    println!("  kontakt_version:\t{}", h.app_version);
                    println!("  author:\t\t{}", h.author);
                    println!("  zones:\t\t{}", h.number_of_zones);
                    println!("  groups:\t\t{}", h.number_of_groups);
                    println!("  instruments:\t\t{}", h.number_of_instruments);
                    println!("  created_at:\t\t{}", h.created_at);
                }
                BPatchHeader::BPatchHeaderV42(ref h) => {
                    println!("\nBPatchHeaderV42:");
                    println!("  signature:\t\t{}", h.app_signature);
                    println!("  type:\t\t\t{:?}", h.patch_type);
                    println!("  kontakt_version:\t{}", h.app_version);
                    println!("  author:\t\t{}", h.author);
                    println!("  zones:\t\t{}", h.number_of_zones);
                    println!("  groups:\t\t{}", h.number_of_groups);
                    println!("  instruments:\t\t{}", h.number_of_instruments);
                    println!("  created_at:\t\t{}", h.created_at);
                }
            }

            match nks.preset()? {
                KontaktPreset::Kon1(kon1) => {
                    println!("\nKon1:");
                    println!("\n{}", kon1.preset);
                }
                KontaktPreset::Kon2(kon2) => {
                    println!("\nKon2:");
                    println!("\n{}", kon2);
                }
                KontaktPreset::Kon3(kon3) => {
                    println!("\nKon3:");
                    println!("\n{}", kon3);
                }
                KontaktPreset::Kon4(kon4) => {
                    println!("\nKon4:");
                    println!("\n{:?}", kon4);
                    // print_kontakt_instrument(kon4.()?)?;
                }
                KontaktPreset::Kon7(_) => todo!(),
            }
        }
        NIFile::FM8Preset => {
            println!("Detected format:\tFM8 Preset\n");
        }
    };

    Ok(())
}
