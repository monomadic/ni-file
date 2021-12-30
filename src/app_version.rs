use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct AppVersion {
    app: App,
    version: String,
    license: License,
}

impl ToString for AppVersion {
    fn to_string(&self) -> String {
        format!("{:?} v{} - {:?}", self.app, self.version, self.license)
    }
}

#[derive(Debug, Clone)]
pub enum App {
    GuitarRig,
    Kontakt,
    Reaktor,
    Massive,
    FM8,
    Unknown(u32),
}

impl From<u32> for App {
    fn from(id: u32) -> Self {
        match id {
            1 => App::GuitarRig,
            2 => App::Kontakt,
            4 => App::Reaktor,
            7 => App::Massive,
            8 => App::FM8,
            _ => App::Unknown(id),
        }
    }
}

#[derive(Debug, Clone)]
pub enum License {
    User,
    Commercial,
    Unknown(u8),
}

impl From<u8> for License {
    fn from(id: u8) -> Self {
        match id {
            0 => License::User,
            1 => License::Commercial,
            _ => License::Unknown(id),
        }
    }
}

pub(crate) fn read(mut buf: &[u8]) {
    let _unknown = buf.read_u32::<LittleEndian>().unwrap();
    let blocksize = buf.read_u64::<LittleEndian>().unwrap();
    let (_current_segment, buffer) = buf.split_at((blocksize - 8) as usize);

    info!("{}", read_app_version(buffer).to_string());
}

fn read_app_version(mut buf: &[u8]) -> AppVersion {
    let license: License = buf.read_u8().unwrap().into();
    let _unknown = buf.read_u32::<LittleEndian>().unwrap();
    let app: App = buf.read_u32::<LittleEndian>().unwrap().into();
    let _string_count = buf.read_u32::<LittleEndian>().unwrap();
    let char_count = buf.read_u32::<LittleEndian>().unwrap();
    let (ver, _buffer) = buf.split_at(char_count as usize);
    let version: String = std::str::from_utf8(&ver).unwrap().into();

    AppVersion {
        app,
        version,
        license,
    }
}
