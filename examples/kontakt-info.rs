use std::fs::File;

use color_eyre::eyre::{Report, Result};
use ni_file::kontakt::{objects::FNTableImpl, Chunk, KontaktChunks, KontaktObject};

const INDENT_SIZE: usize = 2;
const INDENT_CHAR: char = ' ';

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
    let indent_str = INDENT_CHAR.to_string().repeat(indent);

    print!("{}[0x{:X}] ", indent_str, chunk.id);
    match &chunk.into_object()? {
        KontaktObject::Program(program) => {
            println!("Program v{:X}", program.version());
            for chunk in program.children() {
                print_chunk(chunk, indent + INDENT_SIZE)?;
            }
        }
        KontaktObject::BParFX(bparfx) => {
            print!("BParFX v{:X} ", bparfx.version());
            println!("{}", format_hex(&bparfx.0.private_data));
            for chunk in &bparfx.0.children {
                print_chunk(chunk, indent + INDENT_SIZE)?;
            }
        }
        KontaktObject::BParScript(script) => {
            println!(
                "BParScript v{:X} {}",
                script.0.version,
                script.params()?.description.unwrap_or_default()
            );
        }
        KontaktObject::BParFXSendLevel(fx) => {
            println!("BParFXSendLevel {:?}", fx);
        }
        KontaktObject::QuickBrowseData(q) => {
            println!(
                "QuickBrowseData v{:X} {} ({} children)",
                q.0.version,
                q.params()?.unknown,
                q.0.children.len()
            );
        }
        KontaktObject::SaveSettings(_) => {
            println!("SaveSettings");
        }
        KontaktObject::BInsertBus(bus) => {
            println!("InsertBus v{:X} {:?}", bus.0.version, bus.params()?);
            for chunk in &bus.0.children {
                print_chunk(chunk, indent + INDENT_SIZE)?;
            }
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
                for chunk in &prog.0.children {
                    print_chunk(chunk, indent + INDENT_SIZE)?;
                }
            }
        }
        KontaktObject::BParameterArraySerBParInternalMod16(arr) => {
            println!("InternalModArray16 v{:X}", arr.0.version);
            for chunk in &arr.children()? {
                print_chunk(chunk, indent + INDENT_SIZE)?;
            }
        }
        KontaktObject::BParInternalMod(imod) => {
            println!("InternalMod v{:X}", imod.0.version);
        }
        KontaktObject::PrivateRawObject(po) => {
            print!("PrivateRawObject: ");
            println!("0x{}", format_hex(po.data()));
        }
        KontaktObject::ProgramList(list) => {
            println!("ProgramList {list:?}");
            for program in &list.programs {
                println!("Program v{:X}", program.version());
                for chunk in program.children() {
                    print_chunk(chunk, indent + INDENT_SIZE)?;
                }
            }
        }
        KontaktObject::BProgramContainer(pc) => {
            println!("ProgramContainer {pc:?}");
            for chunk in &pc.0.children {
                print_chunk(chunk, indent + INDENT_SIZE)?;
            }
        }
        KontaktObject::LoopArray(loops) => {
            println!("LoopArray ({} items)", loops.items.len());
            let indent = indent + INDENT_SIZE;
            let indent_str = INDENT_CHAR.to_string().repeat(indent);
            for l in &loops.items {
                println!("{}{:?}", indent_str, l);
            }
        }
        KontaktObject::Bank(bank) => {
            println!("Bank {:?}", bank.params()?);
            for chunk in &bank.0.children {
                print_chunk(chunk, indent + INDENT_SIZE)?;
            }
        }
        KontaktObject::VoiceGroups(groups) => {
            println!("VoiceGroups {:?}", groups);
            for vg in &groups.groups {
                if let Some(_g) = vg {}
            }
        }
        KontaktObject::VoiceGroup(vg) => {
            println!("VoiceGroup {:?}", vg);
        }
        KontaktObject::GroupList(gl) => {
            println!("GroupList ({} groups)", gl.groups.len());
            let indent = indent + INDENT_SIZE;
            let indent_str = INDENT_CHAR.to_string().repeat(indent);
            for group in &gl.groups {
                println!(
                    "{}Group v{:X} {:?}",
                    indent_str,
                    group.0.version,
                    group.params()?
                );
                for chunk in &group.0.children {
                    print_chunk(chunk, indent + INDENT_SIZE)?;
                }
            }
        }
        KontaktObject::BParFXDelay(fx) => {
            println!("BParamArrayBParFX8 v{:X}", fx.0.version);
        }
        KontaktObject::StartCriteriaList(scl) => {
            println!("StartCriteriaList ({} items)", scl.items.len());
            for item in &scl.items {
                println!("{}StartCriteria v{:?}", indent_str, item)
            }
        }
        KontaktObject::BParameterArraySerBParFX8(pa) => {
            println!("BParamArrayBParFX8 v{:X}", pa.version);
            for chunk in &pa.items {
                if let Some(chunk) = chunk {
                    print_chunk(chunk, indent + INDENT_SIZE)?;
                }
            }
        }
        KontaktObject::ZoneList(zones) => {
            println!("ZoneList ({} zones)", zones.zones().len());
            let indent = indent + INDENT_SIZE;
            let indent_str = INDENT_CHAR.to_string().repeat(indent);
            for zone in zones.zones() {
                println!("{}ZoneData v{:X}", indent_str, zone.0.version);
                for chunk in &zone.0.children {
                    print_chunk(&chunk, indent + INDENT_SIZE)?;
                }
            }
        }
        _ => println!("!!! {:?}", chunk.into_object()?),
    };

    Ok(())
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

pub fn format_hex(buffer: &[u8]) -> String {
    format!(
        "{}",
        &buffer
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>()
    )
}
