use crate::ni;
use ni::NISegment;

pub(crate) fn dump(input: &[u8]) -> Vec<u8> {
    // match ni::read(input) {
    //     Ok((_, segment)) => search_compressed_segment(&segment),
    //     Err(e) => Vec::new(),
    // }
    // crate::ni_file::
    Vec::new()
}

// fn search_segment_by_id(segment: &NISegment, id: u32) -> Option<&NISegment> {
//     if segment.id == id {
//         Some(segment)
//         // let (_, deflated_file) = crate::deflate::deflate(&segment.data.data, 11).unwrap();
//         // return deflated_file;
//     } else {
//         for child in segment.children {
//             search_segment_by_id(child, id)
//         }
//     }

//     None
// }
