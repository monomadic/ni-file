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

            println!("polyphonyNumVoices {:?}", reader.read_f32_le()?);
            println!("unisonNumVoices {:?}", reader.read_f32_le()?);
            println!("unisonDetune {:?}", reader.read_f32_le()?);
            println!("pitchMasterTune {:?}", reader.read_f32_le()?);
            println!("polyphonyMono {:?}", reader.read_f32_le()?);
            println!("2 {:?}", reader.read_f32_le()?);
            println!("3 {:?}", reader.read_f32_le()?);
            println!("4 {:?}", reader.read_f32_le()?);
            println!("portamento {:?}", reader.read_f32_le()?);
            println!("3 {:?}", reader.read_f32_le()?);
            println!("5 {:?}", reader.read_f32_le()?);
            println!("7 {:?}", reader.read_f32_le()?);
            println!("9 {:?}", reader.read_f32_le()?);
            println!("11 {:?}", reader.read_f32_le()?);
            println!("13 {:?}", reader.read_f32_le()?);
            println!("15 {:?}", reader.read_f32_le()?);
            println!("17 {:?}", reader.read_f32_le()?);
            println!("10 {:?}", reader.read_f32_le()?);
            println!("11 {:?}", reader.read_f32_le()?);
            println!("12 {:?}", reader.read_f32_le()?);
            println!("13 {:?}", reader.read_f32_le()?);
            println!("13 {:?}", reader.read_f32_le()?);
            println!("14 {:?}", reader.read_f32_le()?);
            println!("15 {:?}", reader.read_f32_le()?);
            println!("16 {:?}", reader.read_f32_le()?);
            println!("17 {:?}", reader.read_f32_le()?);
            println!("18 {:?}", reader.read_f32_le()?);
            println!("19 {:?}", reader.read_f32_le()?);
            println!("20 {:?}", reader.read_f32_le()?);
            println!("21 {:?}", reader.read_f32_le()?);
            println!("22 {:?}", reader.read_f32_le()?);

            println!("Overdrive: On/Off {:?}", reader.read_f32_le()?);
            println!("Overdrive: Drive {:?}", reader.read_f32_le()?);
            println!("Overdrive: Tone {:?}", reader.read_f32_le()?);
            println!("Overdrive: Bass {:?}", reader.read_f32_le()?);
            println!("Overdrive: Volume {:?}", reader.read_f32_le()?);

            println!("TubeAmp: On/Off {:?}", reader.read_f32_le()?);
            println!("TubeAmp: Drive {:?}", reader.read_f32_le()?);
            println!("TubeAmp: Volume {:?}", reader.read_f32_le()?);

            println!("Cabinet: On/Off {:?}", reader.read_f32_le()?);
            println!("Cabinet: Type {:?}", reader.read_f32_le()?);
            println!("Cabinet: Size {:?}", reader.read_f32_le()?);
            println!("Cabinet: Air {:?}", reader.read_f32_le()?);
            println!("Cabinet: Bass {:?}", reader.read_f32_le()?);
            println!("Cabinet: Treble {:?}", reader.read_f32_le()?);

            println!("ShelfEQ: On/Off {:?}", reader.read_f32_le()?);
            println!("ShelfEQ: Low Frequency {:?}", reader.read_f32_le()?);
            println!("ShelfEQ: Low Gain {:?}", reader.read_f32_le()?);
            println!("ShelfEQ: High Frequency {:?}", reader.read_f32_le()?);
            println!("ShelfEQ: High Gain {:?}", reader.read_f32_le()?);
            println!("ShelfEQ: Volume {:?}", reader.read_f32_le()?);

            println!("PeakEQ: On/Off {:?}", reader.read_f32_le()?);
            println!("PeakEQ: Frequency 1 {:?}", reader.read_f32_le()?);
            println!("PeakEQ: Gain 1 {:?}", reader.read_f32_le()?);
            println!("PeakEQ: Quality 1 {:?}", reader.read_f32_le()?);
            println!("PeakEQ: Frequency 2 {:?}", reader.read_f32_le()?);
            println!("PeakEQ: Gain 2 {:?}", reader.read_f32_le()?);
            println!("PeakEQ: Quality 2 {:?}", reader.read_f32_le()?);
            println!("PeakEQ: Volume {:?}", reader.read_f32_le()?);

            println!("TalkWah: On/Off {:?}", reader.read_f32_le()?);
            println!("TalkWah: Mouth {:?}", reader.read_f32_le()?);
            println!("TalkWah: ModWheel {:?}", reader.read_f32_le()?);
            println!("TalkWah: Size {:?}", reader.read_f32_le()?);
            println!("TalkWah: Bright {:?}", reader.read_f32_le()?);

            println!("Phaser: On/Off  {:?}", reader.read_f32_le()?);
            println!("Phaser: Modulation Rate {:?}", reader.read_f32_le()?);
            println!("Phaser: Color {:?}", reader.read_f32_le()?);
            println!("Phaser: Rotate {:?}", reader.read_f32_le()?);
            println!("Phaser: Sweep Minimum {:?}", reader.read_f32_le()?);
            println!("Phaser: Sweep Maximum {:?}", reader.read_f32_le()?);
            println!("Phaser: MIDI Tempo Sync {:?}", reader.read_f32_le()?);
            println!("Phaser: Dry/Wet {:?}", reader.read_f32_le()?);
            println!("Phaser: Invert {:?}", reader.read_f32_le()?);
            println!("Phaser: Notches {:?}", reader.read_f32_le()?);

            println!("Flanger: On/Off {:?}", reader.read_f32_le()?);
            println!("Flanger: Modulation Rate {:?}", reader.read_f32_le()?);
            println!("Flanger: Static {:?}", reader.read_f32_le()?);
            println!("Flanger: Modulation Depth {:?}", reader.read_f32_le()?);
            println!("Flanger: Color {:?}", reader.read_f32_le()?);
            println!("Flanger: Rotate {:?}", reader.read_f32_le()?);
            println!("Flanger: MIDI Tempo Sync {:?}", reader.read_f32_le()?);
            println!("Flanger: Invert {:?}", reader.read_f32_le()?);
            println!("Flanger: Dry/Wet {:?}", reader.read_f32_le()?);

            println!("Tremolo: On/Off {:?}", reader.read_f32_le()?);
            println!("Tremolo: Rate {:?}", reader.read_f32_le()?);
            println!("Tremolo: Intensity {:?}", reader.read_f32_le()?);
            println!("Tremolo: MIDI Tempo Sync {:?}", reader.read_f32_le()?);
            println!("Tremolo: Stereo Panning {:?}", reader.read_f32_le()?);
            println!("Tremolo: Pulse Width {:?}", reader.read_f32_le()?);
            println!("Tremolo: Attack {:?}", reader.read_f32_le()?);
            println!("Tremolo: Decay {:?}", reader.read_f32_le()?);

            println!("Reverb: On/Off {:?}", reader.read_f32_le()?);
            println!("Reverb: Dry/Wet {:?}", reader.read_f32_le()?);
            println!("Reverb: Bright {:?}", reader.read_f32_le()?);
            println!("Reverb: RoomSize {:?}", reader.read_f32_le()?);
            println!("Reverb: Treble {:?}", reader.read_f32_le()?);

            println!("PsycheDelay: On/Off {:?}", reader.read_f32_le()?);
            println!("PsycheDelay: Dry/Wet {:?}", reader.read_f32_le()?);
            println!("PsycheDelay: Delay Time {:?}", reader.read_f32_le()?);
            println!("PsycheDelay: Reverse Delay {:?}", reader.read_f32_le()?);
            println!("PsycheDelay: Delay Detune {:?}", reader.read_f32_le()?);
            println!("PsycheDelay: Feedback {:?}", reader.read_f32_le()?);

            println!("PsycheDelay: ? {:?}", reader.read_f32_le()?);

            println!("PsycheDelay: Delay Pitch Shift {:?}", reader.read_f32_le()?);
            println!("PsycheDelay: MIDI Tempo Sync {:?}", reader.read_f32_le()?);
            println!("PsycheDelay: Stereo {:?}", reader.read_f32_le()?);

            println!("Delay: Delay Time {:?}", reader.read_f32_le()?);
            println!("Delay: Diffusion {:?}", reader.read_f32_le()?);
            println!("Delay: Modulation Depth {:?}", reader.read_f32_le()?);
            println!("Delay: Feedback {:?}", reader.read_f32_le()?);
            println!("Delay: High Cut {:?}", reader.read_f32_le()?);
            println!("Delay: Low Cut {:?}", reader.read_f32_le()?);
            println!("Delay: Invert {:?}", reader.read_f32_le()?);
            println!("Delay: On/Off {:?}", reader.read_f32_le()?);
            println!("Delay: Dry/Wet {:?}", reader.read_f32_le()?);
            println!("Delay: Modulation Rate {:?}", reader.read_f32_le()?);
            println!("Delay: MIDI Tempo Sync {:?}", reader.read_f32_le()?);
            println!("Delay: Sync Delays {:?}", reader.read_f32_le()?);

            println!("73 {:?}", reader.read_f32_le()?);
            println!("74 {:?}", reader.read_f32_le()?);
            println!("75 {:?}", reader.read_f32_le()?);
            println!("76 {:?}", reader.read_f32_le()?);
            println!("77 {:?}", reader.read_f32_le()?);
            println!("78 {:?}", reader.read_f32_le()?);
            println!("79 {:?}", reader.read_f32_le()?);
            println!("80 {:?}", reader.read_f32_le()?);
            println!("Unison Pan {:?}", reader.read_f32_le()?);
            println!("Unison Dynamic On/Off {:?}", reader.read_f32_le()?);

            println!("Morph X {:?}", reader.read_f32_le()?);
            println!("Morph y {:?}", reader.read_f32_le()?);
            println!("Morph Random X {:?}", reader.read_f32_le()?);
            println!("Morph Random Y {:?}", reader.read_f32_le()?);
            println!("Morph Random Seed {:?}", reader.read_f32_le()?);

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

            println!("FXAmount {:?}", reader.read_f32_le()?);
            println!("1 {:?}", reader.read_f32_le()?);
            println!("2 {:?}", reader.read_f32_le()?);
            println!("3 {:?}", reader.read_f32_le()?);
            println!("4 {:?}", reader.read_f32_le()?);
            println!("5 {:?}", reader.read_f32_le()?);
            println!("6 {:?}", reader.read_f32_le()?);
            println!("7 {:?}", reader.read_f32_le()?);
            println!("8 {:?}", reader.read_f32_le()?);
            println!("9 {:?}", reader.read_f32_le()?);
            println!("10 {:?}", reader.read_f32_le()?);
            println!("11 {:?}", reader.read_f32_le()?);
            println!("12 {:?}", reader.read_f32_le()?);
            println!("13 {:?}", reader.read_f32_le()?);
            println!("14 {:?}", reader.read_f32_le()?);
            println!("15 {:?}", reader.read_f32_le()?);
            println!("16 {:?}", reader.read_f32_le()?);
            println!("17 {:?}", reader.read_f32_le()?);
            println!("18 {:?}", reader.read_f32_le()?);
            println!("19 {:?}", reader.read_f32_le()?);
            println!("20 {:?}", reader.read_f32_le()?);
            println!("21 {:?}", reader.read_f32_le()?);
            println!("22 {:?}", reader.read_f32_le()?);
            println!("23 {:?}", reader.read_f32_le()?);
            println!("24 {:?}", reader.read_f32_le()?);
            println!("25 {:?}", reader.read_f32_le()?);
            println!("26 {:?}", reader.read_f32_le()?);
            println!("27 {:?}", reader.read_f32_le()?);
            println!("28 {:?}", reader.read_f32_le()?);
            println!("29 {:?}", reader.read_f32_le()?);
            println!("30 {:?}", reader.read_f32_le()?);
            println!("31 {:?}", reader.read_f32_le()?);
            println!("32 {:?}", reader.read_f32_le()?);
            println!("33 {:?}", reader.read_f32_le()?);
            println!("34 {:?}", reader.read_f32_le()?);
            println!("35 {:?}", reader.read_f32_le()?);
            println!("36 {:?}", reader.read_f32_le()?);
            println!("37 {:?}", reader.read_f32_le()?);
            println!("38 {:?}", reader.read_f32_le()?);
            println!("39 {:?}", reader.read_f32_le()?);
            println!("40 {:?}", reader.read_f32_le()?);
            println!("41 {:?}", reader.read_f32_le()?);
            println!("42 {:?}", reader.read_f32_le()?);
            println!("filter2Mode {:?}", reader.read_f32_le()?);
            println!("44 {:?}", reader.read_f32_le()?);
            println!("45 {:?}", reader.read_f32_le()?);
            println!("46 {:?}", reader.read_f32_le()?);
            println!("filter1Resonance {:?}", reader.read_f32_le()?);
            println!("48 {:?}", reader.read_f32_le()?);
            println!("49 {:?}", reader.read_f32_le()?);
            println!("50 {:?}", reader.read_f32_le()?);
            println!("51 {:?}", reader.read_f32_le()?);
            println!("52 {:?}", reader.read_f32_le()?);
            println!("53 {:?}", reader.read_f32_le()?);
            println!("54 {:?}", reader.read_f32_le()?);
            println!("55 {:?}", reader.read_f32_le()?);
            println!("56 {:?}", reader.read_f32_le()?);
            println!("57 {:?}", reader.read_f32_le()?);
            println!("58 {:?}", reader.read_f32_le()?);
            println!("59 {:?}", reader.read_f32_le()?);

            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);

            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);

            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);

            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);

            println!("1 {:?}", reader.read_u32_le()?);
            println!("2 {:?}", reader.read_u32_le()?);
            println!("3 {:?}", reader.read_u32_le()?);
            println!("4 {:?}", reader.read_u32_le()?);
            println!("5 {:?}", reader.read_u32_le()?);
            println!("6 {:?}", reader.read_u32_le()?);
            println!("7 {:?}", reader.read_u32_le()?);
            println!("8 {:?}", reader.read_u32_le()?);
            println!("9 {:?}", reader.read_u32_le()?);
            println!("9 {:?}", reader.read_u32_le()?);

            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);
            println!("? {}", reader.read_u8()?);

            println!("ampModPBUp {:?}", reader.read_i8()?);
            println!("1 {:?}", reader.read_i8()?);
            println!("2 {:?}", reader.read_i8()?);
            println!("3 {:?}", reader.read_i8()?);
            println!("4 {:?}", reader.read_i8()?);
            println!("5 {:?}", reader.read_i8()?);
            println!("6 {:?}", reader.read_i8()?);
            println!("7 {:?}", reader.read_i8()?);
            println!("8 {:?}", reader.read_i8()?);
            println!("ampModPBDown {:?}", reader.read_i8()?);
            println!("13 {:?}", reader.read_i8()?);
            println!("14 {:?}", reader.read_i8()?);
            println!("15 {:?}", reader.read_i8()?);
            println!("16 {:?}", reader.read_i8()?);
            println!("17 {:?}", reader.read_i8()?);
            println!("18 {:?}", reader.read_i8()?);
            println!("19 {:?}", reader.read_i8()?);
            println!("20 {:?}", reader.read_i8()?);
            println!("ampModAT {:?}", reader.read_i8()?);
            println!("22 {:?}", reader.read_i8()?);
            println!("23 {:?}", reader.read_i8()?);
            println!("14 {:?}", reader.read_i8()?);
            println!("15 {:?}", reader.read_i8()?);
            println!("16 {:?}", reader.read_i8()?);
            println!("17 {:?}", reader.read_i8()?);
            println!("18 {:?}", reader.read_i8()?);
            println!("19 {:?}", reader.read_i8()?);
            println!("ampModMod {:?}", reader.read_i8()?);
            println!("21 {:?}", reader.read_i8()?);
            println!("22 {:?}", reader.read_i8()?);
            println!("23 {:?}", reader.read_i8()?);
            println!("14 {:?}", reader.read_i8()?);
            println!("15 {:?}", reader.read_i8()?);
            println!("16 {:?}", reader.read_i8()?);
            println!("17 {:?}", reader.read_i8()?);
            println!("18 {:?}", reader.read_i8()?);
            println!("ampModBreath {:?}", reader.read_i8()?);
            println!("20 {:?}", reader.read_i8()?);
            println!("21 {:?}", reader.read_i8()?);
            println!("22 {:?}", reader.read_i8()?);
            println!("23 {:?}", reader.read_i8()?);

            println!("22 {:?}", reader.read_i8()?);
            println!("22 {:?}", reader.read_i8()?);
            println!("22 {:?}", reader.read_i8()?);
            println!("23 {:?}", reader.read_i8()?);
            println!("14 {:?}", reader.read_i8()?);
            println!("15 {:?}", reader.read_i8()?);
            println!("16 {:?}", reader.read_i8()?);
            println!("17 {:?}", reader.read_i8()?);
            println!("18 {:?}", reader.read_i8()?);
            println!("19 {:?}", reader.read_i8()?);
            println!("23 {:?}", reader.read_i8()?);
            println!("14 {:?}", reader.read_i8()?);
            println!("15 {:?}", reader.read_i8()?);
            println!("16 {:?}", reader.read_i8()?);
            println!("17 {:?}", reader.read_i8()?);
            println!("18 {:?}", reader.read_i8()?);
            println!("19 {:?}", reader.read_i8()?);
            println!("23 {:?}", reader.read_i8()?);
            println!("14 {:?}", reader.read_i8()?);
            println!("15 {:?}", reader.read_i8()?);
            println!("16 {:?}", reader.read_i8()?);
            println!("17 {:?}", reader.read_i8()?);
            println!("18 {:?}", reader.read_i8()?);
            println!("19 {:?}", reader.read_i8()?);
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
