use crate::{structures::{NIAppVersion, parse_app_version}, ni::{DSINValue, NISegment}};

pub fn print_segment(segment: NISegment) {
    println!("[{}:{}]", segment.tag, segment.unknown_1);

    print_data_segment(segment.data);

    for child in segment.children {
        print_segment(child);
    }
}

fn print_data_segment(segment: DSINValue) {
    print!("  [{}:{} ({} bytes)]", segment.tag, segment.id, segment.data.len());
    
    match segment.id {
        101 => print_app_version(parse_app_version(segment.data).unwrap().1),
        115 => print!(" 115: compressed preset detected"),
        _ => (),
    }

    print!("\n");
    
    if let Some(data) = segment.child {
        print_data_segment(*data);
    }
}

fn print_app_version(av: NIAppVersion) {
    print!(" 101: version detected [{}] {} {} {} {}, application: ", av.version, av.unknown_1, av.unknown_2, av.unknown_3, av.unknown_4);

    match av.unknown_3 {
        2 => print!("Kontakt?"),
        4 => print!("Reaktor ENS?"),
        7 => print!("Massive?"),
        _ => print!("UNKNOWN"),
    }
}

// fn ds_106_app_version(data: &[u8])