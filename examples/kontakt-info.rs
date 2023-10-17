use std::fs::File;

use color_eyre::eyre::{Report, Result};
use ni_file::{
    kontakt::{
        objects::{BPatchHeader, FNTableImpl, Program},
        schemas::KontaktPreset,
        Chunk, KontaktChunks, KontaktObject,
    },
    nis::Preset,
};

pub fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: kontakt-info <FILE_GLOB>");
        return Ok(());
    };

    let file = File::open(&path)?;
    let kontakt = KontaktChunks::read(file)?;

    for (i, chunk) in kontakt.0.iter().enumerate() {
        print!("{i} ");
        print_chunk(chunk)?;
    }

    Ok(())
}

fn print_chunk(chunk: &Chunk) -> Result<(), Report> {
    print!("0x{:X} ", chunk.id);
    match &chunk.into_object()? {
        KontaktObject::Program(program) => {
            print_kontakt_program(program)?;
            for (i, chunk) in program.children().iter().enumerate() {
                print!("{i} ");
                print_chunk(chunk)?;
            }
        }
        KontaktObject::BParFX(bparfx) => {
            println!("BParFX v{:X}", bparfx.version());
        }
        KontaktObject::FNTableImpl(filetable) => print_filetable(&filetable),
        KontaktObject::FileNameListPreK51(fnl) => {
            println!("\nFileNameListPreK51:");
            for (_i, path) in &fnl.filenames {
                println!("  {path}");
            }
        }
        KontaktObject::SlotList(list) => {
            println!("SlotList {list:?}");
            for (_, prog) in &list.slots {
                println!("  {:?}", prog.params()?);
                for chunk in &prog.0.children {
                    print_chunk(chunk)?;
                }
            }
        }
        KontaktObject::LoopArray(looparray) => println!("{looparray:?}"),
        KontaktObject::Bank(bank) => {
            println!("Bank {:?}", bank.params()?);
            let slots = bank.slot_list()?.slots;
            println!("  slots: {}\n", slots.len());

            for (_i, slot) in slots {
                println!("{:?}", &slot.params()?);
            }
        }
        KontaktObject::VoiceGroups(vg) => println!("VoiceGroups {:?}", vg),
        KontaktObject::GroupList(gl) => {
            println!("GroupList");
            println!("  groups: {}\n", gl.groups.len());
            for group in &gl.groups {
                println!("  Group v{:X}", group.0.version);
                let p = group.params()?;
                println!("  name {}", p.name);
                println!("");
                for chunk in &group.0.children {
                    print_chunk(chunk)?;
                }
            }
            println!("");
        }
        KontaktObject::StartCriteriaList(scl) => {
            println!("StartCriteriaList: {}", scl.group_starts)
        }
        KontaktObject::BParameterArraySerBParFX8(pa) => {
            println!("BParamArrayBParFX8 v{:X}", pa.version);
            println!("  params: {}", pa.len());
            for (i, param) in pa.params.iter().enumerate() {
                if let Some(param) = param {
                    println!("  - [{}] BParFX v{:X}", i, param.version());
                }
            }
        }
        _ => println!(
            "Unsupported Chunk(0x{:x}) {:?}",
            &chunk.id,
            chunk.into_object()?
        ),
    };

    println!("");
    Ok(())
}

fn print_kontakt_program(program: &Program) -> Result<(), Report> {
    println!("Program v{:X}:", program.version());

    let params = program.params()?;
    println!("  name:\t\t\t{}", params.name);
    println!("  library_id:\t\t{}", params.library_id);
    println!("  children:\t\t{}", program.children().len());
    println!("");

    // if let Some(zones) = program.zones() {
    //     let zones = zones?;
    //
    //     println!("ZoneList:");
    //     println!("  zones:\t\t{}\n", &zones.len());
    //
    //     // for zone in zones {
    //     //     let p = zone.public_params()?;
    //     //     println!(
    //     //         "  ZoneData: start={} end={}",
    //     //         &p.sample_start, &p.sample_end
    //     //     );
    //     // }
    // }
    //
    // println!("");
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
            println!("  kontakt_version:\t{}", h.patch_version);
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
            println!("  kontakt_version:\t{}", h.patch_version);
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
        KontaktPreset::KontaktV42(p) => {
            print_kontakt_program(&p.program)?;

            println!("\nFileNameListPreK51:");
            for (_i, path) in &p.filetable.filenames {
                println!("  {path}");
            }
        }
        KontaktPreset::Kon5(p) => {
            print_kontakt_program(&p.program)?;
            print_filetable(&p.filetable);
        }
        KontaktPreset::Kon6(p) => {
            print_kontakt_program(&p.program)?;
            print_filetable(&p.filetable);
        }
        KontaktPreset::Kon7(p) => {
            print_kontakt_program(&p.program)?;
            print_filetable(&p.filetable);
        }
        KontaktPreset::NKM(p) => {
            println!("\nBank:");

            let bank = &p.bank.params()?;
            println!("  master_volume:\t{}", bank.master_volume);
            println!("  master_tune:\t\t{}", bank.master_tune);
            println!("  master_tempo:\t\t{}", bank.master_tempo);
            println!("  name:\t\t\t{}", bank.name);

            // let slotlist = &p.bank.slot_list()?.params()?;
            // println!("\nSlotList:");
            // println!("  slots:\t{:?}", slotlist);

            print_filetable(&p.filetable);
        }
        KontaktPreset::Unsupported(chunks) => {
            for chunk in &chunks.0 {
                // println!("{:?} {:x}", chunk.into_type()?, chunk.id);
                println!("Chunk(0x{:X})", chunk.id);
            }
        }
    };

    Ok(())
}
