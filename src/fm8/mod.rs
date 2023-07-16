use crate::{read_bytes::ReadBytesExt, Error, NIFileError};

#[derive(Debug)]
pub struct FM8Preset;

impl FM8Preset {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let mut magic = reader.read_bytes(4)?;
        magic.reverse();
        assert_eq!(
            magic, b"FM8E",
            "Stream does not appear to be an FM8 Ensemble",
        );

        let major_version = reader.read_u32_le()?;
        println!("major version {:?}", major_version);

        // major version < 208
        if major_version < 0xD0 {
            // major version < 202
            if major_version < 0xCA {
                // FM7???
                println!("morph_bottomleft_name {}", read_string(&mut reader)?);
                println!("morph_bottomright_name {}", read_string(&mut reader)?);
                println!("morph_topright_name {}", read_string(&mut reader)?);
                println!("morph_topleft_name {}", reader.read_u32_le()?);
            } else {
                println!("minor version {:?}", reader.read_u32_le()?);

                println!("morph_bottomleft_name {}", read_string(&mut reader)?);
                println!("morph_bottomright_name {}", read_string(&mut reader)?);
                println!("morph_topright_name {}", read_string(&mut reader)?);
                println!("morph_topleft_name {}", read_string(&mut reader)?);
            }

            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);

            println!("numPolyphonyVoices {:?}", reader.read_f32_le()?);
            println!("numUnisonVoices {:?}", reader.read_f32_le()?);
            println!("unisonDetune {:?}", reader.read_f32_le()?);
            println!("pitchMasterTune {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("portamento {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("qualityAnalog {:?}", reader.read_f32_le()?);
            println!("qualityDigital {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("_ {:?}", reader.read_f32_le()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
            // println!("_ {:?}", reader.read_f32_le()?);
        } else {
            panic!("Newer version than supported");
        }

        Ok(Self)
    }
}

pub fn read_string<R: ReadBytesExt>(mut reader: R) -> Result<String, Error> {
    let strlen = reader.read_u32_le()?;
    String::from_utf8(reader.read_bytes(strlen as usize)?)
        .map_err(|_| NIFileError::Generic("failed to read string".to_owned()))
}

#[test]
fn test_fm8_preset_read() -> Result<(), Error> {
    let file = include_bytes!("../../tests/chunks/fm8/1.2.0.1010/000");
    FM8Preset::read(file.as_slice())?;

    Ok(())
}

// pub enum FM8CategoryNames {
//     None,
//     Bass,
//     Bell,
//     Brass,
//     Drum,
//     Ensemble,
//     Ethnic,
//     Flute,
//     FX,
//     Guitar,
//     Lead,
//     Loop,
//     Organ,
//     Pad,
//     Percuss,
//     Piano,
//     Reed,
//     String,
//     Synth,
//     Vocal,
//     Separator,
//     Acoustic,
//     Analog,
//     Atmo,
//     Bright,
//     Clean,
//     Cold,
//     Dark,
//     Detuned,
//     Digital,
//     Distorted,
//     Electric,
//     Full,
//     Hard,
//     High,
//     Long,
//     Low,
//     Metallic,
//     Muted,
//     Noisy,
//     Short,
//     Soft,
//     Sweep,
//     Warm,
// }
