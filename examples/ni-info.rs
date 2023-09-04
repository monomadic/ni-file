use std::{fs::File, io::Cursor};
use tracing::instrument;

use color_eyre::eyre::Result;
use ni_file::{
    self, fm8::FM8Preset, kontakt::instrument::KontaktInstrument, nifile::NIFile,
    nks::header::NKSHeader,
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
    if let Some(filename_table) = instrument.filename_table() {
        println!("\nFilename table:");
        let filename_table = filename_table?;

        for (index, filename) in &filename_table {
            println!("{}:\t{}", index, filename);
        }

        if let Some(program) = instrument.program() {
            if let Some(zones) = program?.zones() {
                println!("\nZones:");
                for zone in zones? {
                    let zone_data = zone.public_params()?;
                    if let Some(filename) = filename_table.get(&(zone_data.filename_id as u32)) {
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

#[instrument]
pub fn main() -> Result<()> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing();

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

            use ni_file::nis::AuthoringApplication::*;
            match repository.authoring_application()? {
                FM8 => {
                    let raw_preset = Cursor::new(repository.preset_raw()?);
                    FM8Preset::read(raw_preset)?;
                }
                _ => (),
            }
        }
        NIFile::FileContainer => {
            println!("Detected format:\t\tFileContainer (Monolith)");
        }
        NIFile::NKSContainer(nks) => {
            println!("Detected format:\t\tNKS (Native Instruments Kontakt Sound) Container");

            use NKSHeader::*;
            match &nks.header {
                BPatchHeaderV2(h) => {
                    println!("\nBPatchHeaderV2:");
                    println!("  type:\t\t\t{:?}", h.patch_type);
                    println!("  kontakt_version:\t{}", h.app_version);
                    println!("  author:\t\t\t{}", h.author);
                    println!("  zones:\t\t\t{}", h.number_of_zones);
                    println!("  groups:\t\t\t{}", h.number_of_groups);
                    println!("  instruments:\t\t{}", h.number_of_instruments);
                    println!("  created_at:\t\t{}", h.created_at);
                }
                BPatchHeaderV42(h) => {
                    println!("\nBPatchHeaderV42:");
                    println!("  type:\t\t\t{:?}", h.patch_type);
                    println!("  kontakt_version:\t{}", h.app_version);
                    println!("  author:\t\t{}", h.author);
                    println!("  zones:\t\t{}", h.number_of_zones);
                    println!("  groups:\t\t{}", h.number_of_groups);
                    println!("  instruments:\t\t{}", h.number_of_instruments);
                    println!("  created_at:\t\t{}", h.created_at);
                }
            }

            print_kontakt_instrument(nks.instrument()?)?;
        }
    };

    Ok(())
}
