use ncw::NcwReader;
use std::{fs::File, io::Cursor};

use color_eyre::eyre::Result;
use ni_file::{
    self, fm8::FM8Preset, kontakt::instrument::KontaktInstrument, nifile::NIFile,
    nis::AuthoringApplication, nks::nksfile::NKSContainer,
};

fn print_kontakt_instrument(instrument: KontaktInstrument) -> Result<()> {
    println!("\nKontakt Data:");
    if let Some(Ok(program)) = instrument.program() {
        if let Ok(params) = program.public_params() {
            println!("\nProgram:");
            println!("  name:\t\t{}", params.name);
            println!("  credits:\t{}", params.instrument_credits);
            println!("  author:\t{}", params.instrument_author);
        }
    }
    if let Some(filename_table) = instrument.filename_tables()? {
        println!("\nFilename tables:");

        println!("\nOther:");
        for (index, filename) in &filename_table.other_filetable {
            println!("{}:\t{}", index, filename);
        }

        println!("\nSamples:");
        for (index, filename) in &filename_table.sample_filetable {
            println!("{}:\t{}", index, filename);
        }

        println!("\nSpecial:");
        for (index, filename) in &filename_table.special_filetable {
            println!("{}:\t{}", index, filename);
        }

        if let Some(program) = instrument.program() {
            if let Some(zones) = program?.zones() {
                println!("\nZones:");
                for zone in zones? {
                    let zone_data = zone.public_params()?;
                    if let Some(filename) = filename_table
                        .sample_filetable
                        .get(&(zone_data.filename_id as u32))
                    {
                        println!("Zone: {}", filename);
                    }
                }
            } else {
                println!("\nNo zones found!");
            }
        } else {
            println!("\nNo program found!");
        }
    } else {
        println!("\nNo filename table found!");
    }

    Ok(())
}

pub fn main() -> Result<()> {
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-info <FILE_GLOB>");
        return Ok(());
    };

    let file = File::open(&path)?;

    match NIFile::read(file)? {
        NIFile::NISContainer(repository) => {
            println!("Detected format:\t\t\tNIS (Native Instruments Sound) Container");
            println!("\nNIS Data:");
            println!("  NIS Version: {}", repository.nisound_version()?);
            println!("  Preset Version: {}", repository.preset_version()?);
            println!(
                "  Authoring Application: {:?}",
                repository.authoring_application()?,
            );

            if let Ok(h) = repository.nks_header() {
                println!("\nBPatchHeaderV42:");
                println!("  type:\t\t\t{:?}", h.patch_type);
                println!("  kontakt_version:\t{}", h.app_version);
                println!("  author:\t\t{}", h.author);
                println!("  zones:\t\t{}", h.number_of_zones);
                println!("  groups:\t\t{}", h.number_of_groups);
                println!("  instruments:\t\t{}", h.number_of_instruments);
                println!("  created_at:\t\t{}", h.created_at);
            }

            print_kontakt_instrument(repository.instrument()?)?;

            match repository.authoring_application()? {
                AuthoringApplication::FM8 => {
                    let raw_preset = Cursor::new(repository.preset_raw()?);
                    FM8Preset::read(raw_preset)?;
                }
                _ => (),
            }
        }
        NIFile::KontaktResource => {
            println!("Detected format:\t\tKontaktResource");
        }
        NIFile::NICache => {
            println!("Detected format:\t\tNICache");
        }
        NIFile::NICompressedWave => {
            println!("Detected format:\t\tNICompressedWave");
            let file = File::open(&path)?;
            let ncw = NcwReader::read(&file)?;
            println!("\nNCW:");
            println!("  channels:\t\t{}", ncw.header.channels);
            println!("  bits_per_sample:\t{}", ncw.header.bits_per_sample);
            println!("  sample_rate:\t\t{}", ncw.header.sample_rate);
        }
        NIFile::Monolith(container) => {
            println!("Detected format:\t\tMonolith (FileContainer Archive)\n");
            println!("Files:");
            for item in container.items {
                println!("  {}", item.filename);
            }
        }
        NIFile::NKSContainer(nks) => {
            println!("Detected format:\t\tNKS (Native Instruments Kontakt Sound) Container");

            match nks {
                NKSContainer::V1(v1) => {
                    let h = v1.header;
                    println!("\nBPatchHeaderV1:");
                    println!("  created_at:\t\t{}", h.created_at);
                }
                NKSContainer::V2(v2) => {
                    let h = v2.header;
                    println!("\nBPatchHeaderV2:");
                    println!("  type:\t\t\t{:?}", h.patch_type);
                    println!("  kontakt_version:\t{}", h.app_version);
                    println!("  author:\t\t\t{}", h.author);
                    println!("  zones:\t\t\t{}", h.number_of_zones);
                    println!("  groups:\t\t\t{}", h.number_of_groups);
                    println!("  instruments:\t\t{}", h.number_of_instruments);
                    println!("  created_at:\t\t{}", h.created_at);
                }
                NKSContainer::V42(v42) => {
                    let h = v42.header;
                    println!("\nBPatchHeaderV42:");
                    println!("  type:\t\t\t{:?}", h.patch_type);
                    println!("  kontakt_version:\t{}", h.app_version);
                    println!("  author:\t\t{}", h.author);
                    println!("  zones:\t\t{}", h.number_of_zones);
                    println!("  groups:\t\t{}", h.number_of_groups);
                    println!("  instruments:\t\t{}", h.number_of_instruments);
                    println!("  created_at:\t\t{}", h.created_at);

                    // print_kontakt_instrument(v42.instrument()?)?;
                }
            }
        }
    };

    Ok(())
}
