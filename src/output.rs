use crate::{structures::parse_app_version, ni::{DSINValue, NISegment}};

pub fn print_segment(segment: NISegment) {
    println!("[{}:{}]", segment.tag, segment.unknown_1);

    print_data_segment(segment.data);

    for child in segment.children {
        print_segment(child);
    }
}

fn print_data_segment(segment: DSINValue) {
    println!("  [{}:{} ({} bytes)]", segment.tag, segment.id, segment.data.len());
    
    match segment.id {
        101 => println!("version detected? {:?}", parse_app_version(segment.data)),
        _ => (),
    }
    
    if let Some(data) = segment.child {
        print_data_segment(*data);
    }
}

// fn ds_106_app_version(data: &[u8])