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

const INDENT_SIZE: usize = 2;

pub fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("usage: kontakt-info <FILE_GLOB>");
        return Ok(());
    };

    let file = File::open(&path)?;
    let kontakt = KontaktChunks::read(file)?;

    for chunk in &kontakt.0 {
        print_chunk(chunk, INDENT_SIZE)?;
    }

    Ok(())
}

fn print_chunk(chunk: &Chunk, indent: usize) -> Result<(), Report> {
    print!("{:indent$}", " ");
    print!("[0x{:X}] ", chunk.id);
    match &chunk.into_object()? {
        KontaktObject::Program(program) => {
            println!("Program v{:X}:", program.version());

            let params = program.params()?;
            print!("{:>indent$}", " ");
            println!("name:\t\t\t{}", params.name);

            print!("{:>indent$}", " ");
            println!("library_id:\t\t{}\n", params.library_id);

            for chunk in program.children() {
                print_chunk(chunk, indent + INDENT_SIZE)?;
            }
        }
        KontaktObject::BParFX(bparfx) => {
            println!("BParFX v{:X}", bparfx.version());
            println!("{}", format_hex(&bparfx.0.public_data));
            for chunk in &bparfx.0.children {
                print_chunk(chunk, indent + INDENT_SIZE)?;
            }
        }
        KontaktObject::BParScript(script) => {
            println!("BParScript v{:X} {:?}", script.0.version, script.params()?);
        }
        KontaktObject::BParFXSendLevel(fx) => {
            println!("BParFXSendLevel {:?}", fx);
        }
        KontaktObject::FNTableImpl(filetable) => print_filetable(&filetable),
        KontaktObject::FileNameListPreK51(fnl) => {
            println!("FileNameListPreK51:");
            for (_i, path) in &fnl.filenames {
                print!("{:>indent$}", " ");
                println!("{path}");
            }
        }
        KontaktObject::SlotList(list) => {
            println!("SlotList:");
            for (_, prog) in &list.slots {
                print!("{:>indent$}", " ");
                println!("params: {:?}", prog.params()?);
                for chunk in &prog.0.children {
                    // print!("{:>indent$}", " ");
                    // println!(" 0x{:X}", &chunk.id);
                    print_chunk(chunk, indent + INDENT_SIZE)?;
                }
            }
            println!("");
        }
        KontaktObject::PrivateRawObject(po) => {
            print!("PrivateRawObject: ");
            println!("0x{}", format_hex(po.data()));
        }
        KontaktObject::ProgramList(list) => {
            println!("ProgramList {list:?}");
            for program in &list.programs {
                print_kontakt_program(program)?;
                for chunk in program.children() {
                    print_chunk(chunk, indent + INDENT_SIZE)?;
                }
            }
        }
        KontaktObject::BProgramContainer(pc) => {
            println!("ProgramContainer {pc:?}");
        }
        KontaktObject::LoopArray(loops) => {
            println!("LoopArray ({} items)", loops.items.len());
            for l in &loops.items {
                print!("{:>indent$}", " ");
                println!("{:?}", l);
            }
        }
        KontaktObject::Bank(bank) => {
            println!("Bank {:?}", bank.params()?);
            let slots = bank.slot_list()?.slots;

            print!("{:>indent$}", " ");
            println!("  slots: {}\n", slots.len());

            for chunk in &bank.0.children {
                print_chunk(chunk, indent + INDENT_SIZE)?;
            }
        }
        KontaktObject::VoiceGroups(groups) => {
            println!("VoiceGroups {:?}", groups);
            for vg in &groups.groups {
                if let Some(g) = vg {}
            }
        }
        KontaktObject::VoiceGroup(vg) => {
            println!("VoiceGroup {:?}", vg);
        }
        KontaktObject::GroupList(gl) => {
            println!("GroupList");

            print!("{:>indent$}", " ");
            println!("  groups: {}\n", gl.groups.len());
            for group in &gl.groups {
                print!("{:>indent$}", " ");
                println!("Group v{:X}", group.0.version);
                let p = group.params()?;
                print!("{:>indent$}", " ");
                println!("name: {}", p.name);
                for chunk in &group.0.children {
                    print_chunk(chunk, indent + INDENT_SIZE)?;
                }
            }
        }
        KontaktObject::StartCriteriaList(scl) => {
            println!("StartCriteriaList: {}", scl.group_starts)
        }
        KontaktObject::BParameterArraySerBParFX8(pa) => {
            println!("BParamArrayBParFX8 v{:X}", pa.version);
            print!("{:>indent$}", " ");
            println!("params ({}):", pa.len());

            for chunk in &pa.items {
                if let Some(chunk) = chunk {
                    print_chunk(chunk, indent + INDENT_SIZE)?;
                }
            }
        }
        KontaktObject::ZoneList(zones) => {
            println!("ZoneList ({} zones)", zones.zones.len());

            for zone in &zones.zones {
                print!("{:>indent$}", " ");
                println!("ZoneData v{:X}", zone.0.version);
                for chunk in &zone.0.children {
                    print_chunk(&chunk, indent + INDENT_SIZE)?;
                }
            }
        }
        _ => println!("Unsupported {:?}", chunk.into_object()?),
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
    println!("FNTableImpl:");

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

            print_filetable(&p.filetable);
        }
        KontaktPreset::Unsupported(chunks) => {
            for chunk in &chunks.0 {
                println!("Chunk(0x{:X})", chunk.id);
            }
        }
    };

    Ok(())
}

pub fn format_hex(buffer: &[u8]) -> String {
    format!(
        "{}",
        &buffer
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>()
    )
}
