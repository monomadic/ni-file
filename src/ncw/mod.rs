//
// NCW is basically just Differential PCM (DPCM).
// I'm not sure why NI claimed to have their own format for the sake of it,
// except to lock you in.
//

mod reader;
mod wav;

pub use self::reader::NcwReader;
pub use self::wav::write_wav;
