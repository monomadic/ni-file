use ncw::NcwReader;
use std::fs::File;

use color_eyre::eyre::{Report, Result};
use ni_file::{
    kontakt::{objects::filename_list::FNTableImpl, KontaktPreset},
    nifile::NIFile,
    nis::{items::RepositoryRootContainer, Preset},
    nks::header::BPatchHeader,
};

pub fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-info <FILE_GLOB>");
        return Ok(());
    };

    let file = File::open(&path)?;

    match NIFile::read(file)? {
        NIFile::NISContainer(container) => {
            let repository = RepositoryRootContainer::try_from(&container)?;

            if let Ok(root) = repository.properties() {
                println!("RepositoryRoot:");
                println!("  version:            NISound {}", root.nisound_version);
                println!("  repository_magic:   {}", root.repository_magic);
                println!("  repository_type:    {}", root.repository_type);
                println!("");
            } else {
                println!("Not a RepositoryRoot\n");
            }

            // regular preset
            if let Some(preset) = repository.preset() {
                println!("\nPreset detected");
                print_preset_properties(preset?.properties()?);
            }

            // kontakt preset
            if let Some(preset) = repository.kontakt_preset() {
                println!("\nKontakt instrument detected.");

                let preset = preset?;

                print_preset_properties(preset.properties()?.preset);

                if let Some(header) = preset.header() {
                    print_kontakt_header(&BPatchHeader::BPatchHeaderV42(header?.0));
                }

                if let Some(preset) = preset.preset() {
                    print_kontakt_preset(&preset?)?;
                }
            }

            // multi
            if let Some(app_specific) = repository.app_specific() {
                println!("\nKontakt multi detected.");

                let app = app_specific?;
                let props = app.properties()?;

                println!("\nAppSpecific:");
                println!(
                    "  authoring_app:\t{:?} {}",
                    props.authoring_app, props.version
                );

                let inner = RepositoryRootContainer(props.subtree_item.item()?);
                if let Some(preset) = inner.kontakt_preset() {
                    let preset = preset?;

                    println!("\nKontakt preset detected.");
                    print_preset_properties(preset.properties()?.preset);

                    if let Some(header) = preset.header() {
                        print_kontakt_header(&BPatchHeader::BPatchHeaderV42(header?.0));
                    }

                    if let Some(preset) = preset.preset() {
                        print_kontakt_preset(&preset?)?;
                    }
                }
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
            print_kontakt_header(&nks.header);
            print_kontakt_preset(&nks.preset()?)?;
        }
        NIFile::FM8Preset => {
            println!("Detected format:\tFM8 Preset\n");
        }
    };

    Ok(())
}

fn print_preset_properties(preset: Preset) {
    println!("\nPreset:");
    println!(
        "  authoring_app:\t{:?} {}",
        preset.authoring_app, preset.version
    );
    println!("  is_factory_preset:\t{}", preset.is_factory_preset);
}

fn print_filetable(ft: &FNTableImpl) {
    println!("\nFNTableImpl:");

    println!("  Special Table:");
    if ft.special_filetable.is_empty() {
        println!("    (none)");
    }
    for (_i, path) in &ft.special_filetable {
        println!("    {path}");
    }

    println!("\n  Sample Table:");
    if ft.sample_filetable.is_empty() {
        println!("    (none)");
    }
    for (_i, path) in &ft.sample_filetable {
        println!("    {path}");
    }

    println!("\n  Other Table:");
    if ft.other_filetable.is_empty() {
        println!("    (none)");
    }
    for (_i, path) in &ft.other_filetable {
        println!("    {path}");
    }
    println!("");
}

fn print_kontakt_header(header: &BPatchHeader) {
    match header {
        BPatchHeader::BPatchHeaderV1(ref h) => {
            println!("\nBPatchHeaderV1:");
            println!("  created_at:\t\t{}", h.created_at);
            println!("  samples_size:\t\t{}", h.samples_size);
        }
        BPatchHeader::BPatchHeaderV2(ref h) => {
            println!("\nBPatchHeaderV2:");
            println!("  signature:\t\t{}", h.app_signature);
            println!("  type:\t\t\t{:?}", h.patch_type);
            println!("  is_monolith:\t\t{:?}", h.is_monolith);
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
            println!("  is_monolith:\t\t{:?}", h.is_monolith);
            println!("  kontakt_version:\t{}", h.app_version);
            println!("  author:\t\t{}", h.author);
            println!("  zones:\t\t{}", h.number_of_zones);
            println!("  groups:\t\t{}", h.number_of_groups);
            println!("  instruments:\t\t{}", h.number_of_instruments);
            println!("  created_at:\t\t{}", h.created_at);
        }
    }
    println!("");
}

fn print_kontakt_preset(preset: &KontaktPreset) -> Result<()> {
    match preset {
        KontaktPreset::KontaktV1(kon1) => {
            println!("\nKon1:");
            println!("\n{}", kon1.preset);
        }
        KontaktPreset::KontaktV2(kon2) => {
            println!("\nKon2:");
            println!("\n{}", kon2.preset);
        }
        KontaktPreset::KontaktV42(kon4) => {
            let program = &kon4.program;
            println!("\nProgram 0x{:X}:", program.version());

            let program = program.public_params()?;
            println!("  name:\t\t{}", program.name);
            println!("  library_id:\t{}", program.library_id);

            println!("\nFileNameListPreK51:");
            for (_i, path) in &kon4.filetable.filenames {
                println!("  {path}");
            }
        }
        KontaktPreset::Kon5(p) => {
            let program = &p.program;
            println!("\nProgram 0x{:X}:", program.version());

            let program = program.public_params()?;
            println!("  name:\t\t{}", program.name);
            println!("  library_id:\t{}", program.library_id);

            print_filetable(&p.filetable);
        }
        KontaktPreset::Kon6(kon6) => {
            let program = &kon6.program;
            println!("\nProgram 0x{:X}:", program.version());
            println!("  chunks:\t{}", kon6.chunks.len());

            let program = program.public_params()?;
            println!("  name:\t\t{}", program.name);
            println!("  library_id:\t{}", program.library_id);

            // print_filetable(&kon6.filetable);
        }
        KontaktPreset::Kon7(kon7) => {
            let program = &kon7.program;
            println!("\nProgram 0x{:X}:", program.version());
            println!("  chunks:\t{}", kon7.chunks.len());

            let program = program.public_params()?;
            println!("  name:\t\t{}", program.name);
            println!("  library_id:\t{}", program.library_id);
        }
        KontaktPreset::KontaktMulti(p) => {
            println!("\nBank:");

            print_filetable(&p.filetable);
        }
    };

    Ok(())
}
