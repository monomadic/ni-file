use crate::Error;
use binread::{io::Cursor, prelude::*, NullWideString};
/**
 * NiContainer
 *  holds embedded segments
 */
use rctree::Node;
use std::io::prelude::*;

pub struct NIHeaderSegment {
    data: NIDataSegment,
    children: Node<NIDataSegment>,
}

pub struct NIDataSegment {
    id: u32,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum SegmentType {
    Item,
    Authorization,
    AudioSampleItem,
    AutomationParameters,
    AppSpecific,
    Bank,
    BankContainer,
    Resources,
    SoundInfoItem,
    Preset,
    EncryptionItem,
    PresetChunkItem,
    PresetContainer,
    PresetInner,
    SubtreeItem,
    BinaryChunkItem,
    ExternalFileReference,
    InternalResourceReferenceItem,
    ControllerAssignments,
    ModuleBank,
    Module,
    GenericItem(Box<SegmentType>),
    Unknown(u32),
}

impl From<u32> for SegmentType {
    fn from(id: u32) -> Self {
        match id {
            1 => SegmentType::Item,
            // 3 => SegmentType::Maybe("KontaktFile".into()),
            100 => SegmentType::Bank,
            101 => SegmentType::Preset,
            102 => SegmentType::BankContainer,
            103 => SegmentType::PresetContainer,
            104 => SegmentType::BinaryChunkItem,
            106 => SegmentType::Authorization,
            108 => SegmentType::SoundInfoItem,
            109 => SegmentType::PresetChunkItem, // occurs in first header of deflated kontakt
            110 => SegmentType::ExternalFileReference,
            111 => SegmentType::Resources,
            112 => SegmentType::AudioSampleItem,
            113 => SegmentType::InternalResourceReferenceItem,
            114 => SegmentType::PictureItem,
            115 => SegmentType::SubtreeItem,
            116 => SegmentType::EncryptionItem,
            117 => SegmentType::AppSpecific,
            // 118 => SegmentType::FileHeader,
            120 => SegmentType::AutomationParameters,
            121 => SegmentType::ControllerAssignments,
            122 => SegmentType::Module,
            123 => SegmentType::ModuleBank,
            _ => SegmentType::Unknown(id),
        }
    }
}

pub fn read(buf: &[u8]) -> Result<(), Error> {
    let mut cursor = Cursor::new(buf);
    _header(&mut cursor)
}

pub fn header(cursor: &mut Cursor<&[u8]>) -> Result<(), Error> {
    // let hsin: HSINHeader = cursor.read_le()?;
    // println!("{:#?}", hsin);

    // let mut indent = 0;
    // println!("<{:?}>", hsin.tag);

    _header(cursor)?;

    Ok(())
}

pub fn _header(cursor: &mut Cursor<&[u8]>) -> Result<(), Error> {
    info!("reading header segment");

    let size: u32 = cursor.read_le()?;
    info!("reported segment size: {} bytes", size);

    // todo: we don't really need to create a new buffer
    // use seek(-4) instead
    let mut segment = vec![0; size as usize - 4];
    cursor.read_exact(&mut segment)?;
    let mut segment_cursor: Cursor<&[u8]> = Cursor::new(&segment);

    let hsin: HSIN = segment_cursor.read_ne().unwrap();
    info!("read hsin header (20 bytes) {}", hsin.to_string());

    // temp

    // let checksum: [u32;4] = [
    //     segment_cursor.read_le()?,
    //     segment_cursor.read_le()?,
    //     segment_cursor.read_le()?,
    //     segment_cursor.read_le()?,
    // ];

    // let checksum = checksum
    //     .iter()
    //     .map(|h| format!("{:08x}", h))
    //     .collect::<String>();

    // let checksum: [u16;8] = [
    //     segment_cursor.read_le()?,
    //     segment_cursor.read_le()?,
    //     segment_cursor.read_le()?,
    //     segment_cursor.read_le()?,
    //     segment_cursor.read_le()?,
    //     segment_cursor.read_le()?,
    //     segment_cursor.read_le()?,
    //     segment_cursor.read_le()?,
    // ];

    // let checksum = checksum
    //     .iter()
    //     .map(|h| format!("{:04x}", h))
    //     .collect::<String>();

    // warn!("checksum: {}", checksum);

    let mut checksum = vec![0; 16];
    segment_cursor.read_exact(&mut checksum)?;
    warn!(
        "checksum: {}",
        checksum
            .iter()
            .map(|h| format!("{:x}", h))
            .collect::<String>()
    );

    // data segment
    data_segment(&mut segment_cursor)?;

    // child segments
    let d: u32 = segment_cursor.read_le()?;
    warn!("unknown u32 values (index?) d {:?}", (d));

    let child_count: u32 = segment_cursor.read_le()?;
    info!("{} children reported", child_count);

    for _i in 0..child_count {
        let index: u32 = segment_cursor.read_le()?;
        info!("index: {}", index);

        let magic: String = crate::strings::read_utf8(&mut segment_cursor, 4)?;
        info!("read next magic {:?}", &magic);

        let segment_id: u32 = segment_cursor.read_le()?;
        let segment_type: SegmentType = segment_id.into();

        // DEBUG DUMP STUFF
        let mut file = std::fs::File::create(format!("output/{}.hsin", segment_id)).unwrap();
        let mut c = segment_cursor.clone();
        let mut d = segment_cursor.clone();
        let s: u32 = c.read_le()?;
        info!("taking block {} bytes", s);
        let mut buffer = vec![0; s as usize];
        d.read_exact(&mut buffer)?;
        file.write_all(&buffer).unwrap();
        // END DEBUG DUMP

        header(&mut segment_cursor)?;
    }

    if segment_cursor.has_data_left()? {
        error!(
            "header segment has {} unused bytes remaining!",
            segment_cursor.remaining_slice().len()
        );
    }

    Ok(())
}

fn data_segment(cursor: &mut Cursor<&[u8]>) -> Result<(), Error> {
    info!("reading dsin block");
    let mut dsin_pointer = cursor.clone();

    // read header
    let dsin: DSIN = cursor.read_le().unwrap();
    info!("read dsin header (20 bytes) {}", dsin.to_string());

    // let size: u32 = cursor.read_le()?;
    // info!("reported data segment size: {} bytes", size);

    // seek back 4 bytes
    // cursor.seek(binread::io::SeekFrom::Current(-20));

    let size = dsin.size;

    // read data chunk (temporary)
    let mut segment = vec![0; size as usize - 20];
    cursor.read_exact(&mut segment)?;
    let mut segment_cursor: Cursor<&[u8]> = Cursor::new(&segment);

    info!("reading data segment of {} bytes", size - 20);

    // if dsin.id == 1 {
    //     info!("terminator block found; returning early");
    //     return Ok(());
    // }

    // let mut segment = vec![0; size as usize - 4];
    // cursor.read_exact(&mut segment)?;
    // let mut segment_cursor: Cursor<&[u8]> = Cursor::new(&segment);

    // let segment_copy = segment.clone(); // remove later, used for dumps

    // let magic: String = crate::strings::read_utf8(&mut segment_cursor, 4)?;
    // info!("read magic {:?}", &magic);

    // let segment_id: u32 = segment_cursor.read_le()?;
    // let segment_type: SegmentType = segment_id.into();

    // info!("segment id: {} {:?}", segment_id, segment_type);

    // let unknown: u32 = segment_cursor.read_le()?;

    {
        // DEBUG DUMP STUFF
        let mut file = std::fs::File::create(format!(
            "output/{}.{}.dsin",
            dsin.tag.into_iter().collect::<String>(),
            dsin.id
        ))
        .unwrap();
        let mut buffer = vec![0; size as usize];
        dsin_pointer.read_exact(&mut buffer)?;
        file.write_all(&buffer).unwrap();
        // END DEBUG DUMP
    }

    match dsin.id {
        1 => {
            info!("1: empty segment detected. doing nothing.");
            // data_segment(&mut segment_cursor);
            let a: u32 = segment_cursor.read_le()?;

            if a != 1 {
                error!("1: single unknown (usually  1) {:?}", a);
            }
        }
        108 => {
            info!("108: library metadata strings");
            let block: DataSegment = segment_cursor.read_ne().unwrap();
            // let block: Block108 = segment_cursor.read_ne().unwrap();
            info!(
                "108: remaining bytes {:?}",
                segment_cursor.remaining_slice()
            );
        }
        109 => {
            info!("109: internal preset?");
            // kill off terminator
            data_segment(&mut segment_cursor);

            // kontakt
        }
        115 => {
            info!("115: compressed segment?");
            data_segment(&mut segment_cursor);
            // info!("115: remaining bytes {:?}", segment_cursor.remaining_slice());
            let a: u16 = segment_cursor.read_le()?;
            info!("115: a unknown: {:?}", a);

            let compressed_file = segment_cursor.remaining_slice();

            let mut file = std::fs::File::create("output/compressed").unwrap();
            file.write_all(&compressed_file).unwrap();

            let (_, deflated) = crate::deflate::deflate(&compressed_file, 11).unwrap();
            let mut file = std::fs::File::create("output/deflated").unwrap();
            file.write_all(&deflated).unwrap();
        }
        116 => {
            info!("116: preset wrapper? used in kontakt");
            data_segment(&mut segment_cursor);
            info!(
                "116: remaining bytes {:?}",
                segment_cursor.remaining_slice()
            );
        }
        118 => {
            info!("118: ???");
        }
        121 => {
            info!("121: possibly compressed preset");
        }
        _ => {
            warn!("??: unhandled preset {}", dsin.id);
        }
    }

    if segment_cursor.has_data_left()? {
        error!(
            "data block has {} unused bytes remaining!",
            segment_cursor.remaining_slice().len()
        );
    }

    Ok(())
}

#[derive(BinRead, Debug)]
struct DataSegment {
    size: u64,
    tag: [char; 4],
    // #[br(magic = b"DSIN", assert(id==108))]
    id: u32,
    unknown: u32,

    #[br(parse_with = binread::until_eof)]
    data: Vec<u8>,
}

#[derive(BinRead, Debug)]
struct Block108 {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    name_size_wide: u32,
    name: NullWideString,
}

#[derive(BinRead, Debug)]
struct SegmentHeader {
    size: u64,
    tag: [char; 4],
    // #[br(magic = b"DSIN", assert(id==108))]
    id: u32,
    unknown: u32,
}

#[derive(BinRead, Debug)]
struct HSIN {
    a: u32,
    b: u32,
    tag: [char; 4],
    // #[br(magic = b"hsin")]
    c: u32,
    d: u32,
}

impl ToString for HSIN {
    fn to_string(&self) -> String {
        let magic: String = self.tag.into_iter().collect();

        format!("[{}-{}-{}-{}-{}]", self.a, self.b, magic, self.c, self.d)
    }
}

#[derive(BinRead, Debug)]
struct DSIN {
    size: u32,
    b: u32,
    tag: [char; 4],
    id: u32,
    d: u32,
}

impl ToString for DSIN {
    fn to_string(&self) -> String {
        let magic: String = self.tag.into_iter().collect();

        format!(
            "[size:{}-{}-{}-{}-{}]",
            self.size, self.b, magic, self.id, self.d
        )
    }
}
