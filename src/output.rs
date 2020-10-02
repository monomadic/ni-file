use crate::{structures::{NIAppVersion, parse_app_version, parse_metadata, NIMetaData}, ni::{DSINValue, NISegment, take_block}};

pub fn print_segment(segment: &NISegment) {
    println!("[{}:{}]", segment.tag, segment.unknown_1);

    print_data_segment(&segment.data);

    for child in &segment.children {
        print_segment(&child);
    }
}

fn print_data_segment(segment: &DSINValue) {
    print!("  [{}:{} ({} bytes)]", segment.tag, segment.id, segment.data.len());
    
    match segment.id {
        101 => print_app_version(parse_app_version(segment.data).unwrap().1),
        108 => print_metadata(parse_metadata(segment.data).unwrap().1),
        115 => {
            let (_, deflated_data) = crate::deflate::deflate(segment.data, 11).unwrap();
            let (_, deflated_data) = deflated_data.split_at(12);
            let (_, preset) = take_block(deflated_data).expect("preset to parse");
            print_segment(&preset);
        },
        _ => (),
    }

    // if segment.id == 115 {
    //     use std::io::Write;
    //     let mut buffer = std::fs::File::create("output/dsin.data").unwrap();
    //     let (_, deflated_file) = crate::deflate::deflate(&segment.data, 11).unwrap();
    //     buffer.write_all(&deflated_file).unwrap();
    // }

    if segment.id == 109 {
        use std::io::Write;
        let mut buffer = std::fs::File::create("output/preset.data").unwrap();
        buffer.write_all(&segment.data).unwrap();
    }

    print!("\n");
    
    if let Some(data) = segment.child.clone() {
        print_data_segment(&*data);
    }
}

fn print_metadata(meta: NIMetaData) {
    print!(" 108: metadata {:?}", meta);
}

fn print_app_version(av: NIAppVersion) {
    print!(" 101: version detected. ");

    match av.application_id {
        1 => print!("Guitar Rig"),
        2 => print!("Kontakt?"),
        4 => print!("Reaktor ENS?"),
        7 => print!("Massive?"),
        8 => print!("FM8"),
        _ => print!("UNKNOWN"),
    }

    print!(" {} ({} {} {} {})", av.version, av.unknown_1, av.commercial, av.application_id, av.unknown_4);

    match av.commercial {
        0 => print!(" USER"),
        1 => print!(" COMMERCIAL"),
        _ => print!(" UNKNOWN WRITE STATUS")
    }
}

// fn ds_106_app_version(data: &[u8])