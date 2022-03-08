use binread::{io::Cursor, prelude::*};
use std::io::prelude::*;

#[derive(Debug)]
pub enum NIData {
    Unknown(u32),
    Version(Version),
    HeaderInfoA(u32),
    HeaderInfoB(u32),
}

impl NIData {
    pub fn read(id: u32, buffer: &[u8]) -> BinResult<Self> {
        Ok(match id {
            101 => version(buffer)?,
            106 => HeaderInfoA(buffer)?,
            118 => HeaderInfoB(buffer)?,
            _ => NIData::Unknown(id),
        })
    }
}

#[derive(PartialEq, BinRead, Debug, Clone)]
pub enum SegmentType {
    AppSpecific,
    AudioSampleItem,
    Authorization,
    AutomationParameters,
    Bank,
    BankContainer,
    BinaryChunkItem,
    ControllerAssignments,
    EncryptionItem,
    ExternalFileReference,
    GenericItem(Box<SegmentType>),
    InternalResourceReferenceItem,
    Item,
    Module,
    ModuleBank,
    PictureItem,
    Preset,
    PresetChunkItem,
    PresetContainer,
    PresetInner,
    Resources,
    SoundInfoItem,
    SubtreeItem,
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

impl SegmentType {
    pub fn binread<R: Read + Seek>(
        reader: &mut R,
        _ro: &binread::ReadOptions,
        _: (),
    ) -> BinResult<SegmentType> {
        let id: u32 = reader.read_le()?;
        Ok(id.into())
    }
}

fn version(buffer: &[u8]) -> BinResult<NIData> {
    println!("segment {} : {}", format_hex(buffer), format_ascii(buffer));

    let mut reader = Cursor::new(buffer);

    let version: Version = reader.read_le()?;

    println!("{:?}", version);

    Ok(NIData::Version(version))
}

// major-version, minor-version, patch-version
#[derive(BinRead, Debug)]
pub struct Version {
    pub unknown_a: u32,
    pub unknown_b: u8,

    #[br(parse_with = read_app)]
    pub app: App,

    pub unknown_c: u32,

    #[br(parse_with = pascal_string_utf16)]
    pub version: String,
}

#[derive(BinRead, Debug, Clone)]
pub enum App {
    GuitarRig,
    Kontakt,
    Reaktor,
    Maschine,
    Massive,
    FM8,
    Unknown(u32),
}

fn read_app<R: Read + Seek>(reader: &mut R, _ro: &binread::ReadOptions, _: ()) -> BinResult<App> {
    let id: u32 = reader.read_le()?;

    // NI::PA::APPID
    Ok(match id {
        1 => App::GuitarRig,
        2 => App::Kontakt,
        3 => App::Kore,
        4 => App::Reaktor,
        5 => App::Maschine,
        6 => App::Absynthe,
        7 => App::Massive,
        8 => App::FM8,
        9 => App::Battery,
        10 => App::KKontrol,
        11 => App::SC,
        12 => App::FXF_01,
        13 => App::FXF_02,
        14 => App::FXF_03,
        15 => App::FXF_04,
        16 => App::FXF_05,
        17 => App::FXF_06,
        18 => App::FXF_07,
        19 => App::FXF_08,
        20 => App::FXF_09,
        21 => App::FXF_10,
        22 => App::FXF_01,
        23 => App::FXF_02,
        24 => App::FXF_03,
        25 => App::FXF_04,
        26 => App::FXF_05,
        27 => App::FXF_06,
        28 => App::FXF_07,
        29 => App::FXF_08,
        20 => App::FXF_09,
        21 => App::Traktor,
        _ => App::Unknown(id),
    })
}

fn HeaderInfoA(buffer: &[u8]) -> BinResult<NIData> {
    Ok(NIData::HeaderInfoA(118))
}

fn HeaderInfoB(buffer: &[u8]) -> BinResult<NIData> {
    Ok(NIData::HeaderInfoB(106))
}

fn format_hex(buffer: &[u8]) -> String {
    format!(
        "{}",
        &buffer
            .iter()
            .map(|x| format!("{:02x} ", x))
            .collect::<String>()
    )
}

fn format_ascii(buffer: &[u8]) -> String {
    format!("{}", String::from_utf8_lossy(buffer).to_string())
    // format!("{}", &buffer.iter().map(|x| if x.is_ascii() {'s'} else {' '})).collect::<String>())
}

fn pascal_string_utf16<R: Read + Seek>(
    reader: &mut R,
    _ro: &binread::ReadOptions,
    _: (),
) -> BinResult<String> {
    let size: u32 = reader.read_le()?;

    info!("string length {}", size);

    if size == 0 {
        return Ok(String::new());
    }

    let string: String = reader.read_le::<binread::NullWideString>()?.into_string();

    Ok(string)
}
