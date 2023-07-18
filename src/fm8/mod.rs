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

        // FM8Program

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

            println!("PB Mode {:?}", reader.read_f32_le()?);
            println!("Transpose {:?}", reader.read_f32_le()?);
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

            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);

            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);

            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);

            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);

            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);

            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);
            println!("? {}", reader.read_i8()?);

            println!("1 {:?}", reader.read_i32_le()?);
            println!("2 {:?}", reader.read_i32_le()?);
            println!("3 {:?}", reader.read_i32_le()?);
            println!("4 {:?}", reader.read_i32_le()?);
            println!("5 {:?}", reader.read_i32_le()?);
            println!("6 {:?}", reader.read_i32_le()?);
            println!("7 {:?}", reader.read_i32_le()?);
            println!("8 {:?}", reader.read_i32_le()?);

            println!("A AmpMod PB Up {:?}", reader.read_i8()?);
            println!("B AmpMod PB Up {:?}", reader.read_i8()?);
            println!("C AmpMod PB Up {:?}", reader.read_i8()?);
            println!("D AmpMod PB Up {:?}", reader.read_i8()?);
            println!("E AmpMod PB Up {:?}", reader.read_i8()?);
            println!("F AmpMod PB Up {:?}", reader.read_i8()?);
            println!("X AmpMod PB Up {:?}", reader.read_i8()?);
            println!("Z AmpMod PB Up {:?}", reader.read_i8()?);

            println!("? {:?}", reader.read_i8()?);

            println!("A AmpMod PB Dn {:?}", reader.read_i8()?);
            println!("B AmpMod PB Dn {:?}", reader.read_i8()?);
            println!("C AmpMod PB Dn {:?}", reader.read_i8()?);
            println!("D AmpMod PB Dn {:?}", reader.read_i8()?);
            println!("E AmpMod PB Dn {:?}", reader.read_i8()?);
            println!("F AmpMod PB Dn {:?}", reader.read_i8()?);
            println!("X AmpMod PB Dn {:?}", reader.read_i8()?);
            println!("Z AmpMod PB Dn {:?}", reader.read_i8()?);

            println!("6 {:?}", reader.read_i8()?);

            println!("A AmpMod PB AT {:?}", reader.read_i8()?);
            println!("B AmpMod PB AT {:?}", reader.read_i8()?);
            println!("C AmpMod PB AT {:?}", reader.read_i8()?);
            println!("D AmpMod PB AT {:?}", reader.read_i8()?);
            println!("E AmpMod PB AT {:?}", reader.read_i8()?);
            println!("F AmpMod PB AT {:?}", reader.read_i8()?);
            println!("X AmpMod PB AT {:?}", reader.read_i8()?);
            println!("Z AmpMod PB AT {:?}", reader.read_i8()?);

            println!("18 {:?}", reader.read_i8()?);

            println!("A AmpMod Mod {:?}", reader.read_i8()?);
            println!("B AmpMod Mod {:?}", reader.read_i8()?);
            println!("C AmpMod Mod {:?}", reader.read_i8()?);
            println!("D AmpMod Mod {:?}", reader.read_i8()?);
            println!("E AmpMod Mod {:?}", reader.read_i8()?);
            println!("F AmpMod Mod {:?}", reader.read_i8()?);
            println!("X AmpMod Mod {:?}", reader.read_i8()?);
            println!("Z AmpMod Mod {:?}", reader.read_i8()?);

            println!("17 {:?}", reader.read_i8()?);

            println!("A AmpMod PB Breath {:?}", reader.read_i8()?);
            println!("B AmpMod PB Breath {:?}", reader.read_i8()?);
            println!("C AmpMod PB Breath {:?}", reader.read_i8()?);
            println!("D AmpMod PB Breath {:?}", reader.read_i8()?);
            println!("E AmpMod PB Breath {:?}", reader.read_i8()?);
            println!("F AmpMod PB Breath {:?}", reader.read_i8()?);
            println!("X AmpMod PB Breath {:?}", reader.read_i8()?);
            println!("Z AmpMod PB Breath {:?}", reader.read_i8()?);

            println!("16 {:?}", reader.read_i8()?);

            println!("A AmpMod PB Ctrl 1 {:?}", reader.read_i8()?);
            println!("B AmpMod PB Ctrl 1 {:?}", reader.read_i8()?);
            println!("C AmpMod PB Ctrl 1 {:?}", reader.read_i8()?);
            println!("D AmpMod PB Ctrl 1 {:?}", reader.read_i8()?);
            println!("E AmpMod PB Ctrl 1 {:?}", reader.read_i8()?);
            println!("F AmpMod PB Ctrl 1 {:?}", reader.read_i8()?);
            println!("X AmpMod PB Ctrl 1 {:?}", reader.read_i8()?);
            println!("Z AmpMod PB Ctrl 1 {:?}", reader.read_i8()?);

            println!("19 {:?}", reader.read_i8()?);

            println!("A AmpMod PB Ctrl 2 {:?}", reader.read_i8()?);
            println!("B AmpMod PB Ctrl 2 {:?}", reader.read_i8()?);
            println!("C AmpMod PB Ctrl 2 {:?}", reader.read_i8()?);
            println!("D AmpMod PB Ctrl 2 {:?}", reader.read_i8()?);
            println!("E AmpMod PB Ctrl 2 {:?}", reader.read_i8()?);
            println!("F AmpMod PB Ctrl 2 {:?}", reader.read_i8()?);
            println!("X AmpMod PB Ctrl 2 {:?}", reader.read_i8()?);
            println!("Z AmpMod PB Ctrl 2 {:?}", reader.read_i8()?);

            println!("23 {:?}", reader.read_i8()?);

            println!("A AmpMod PB In Env {:?}", reader.read_i8()?);
            println!("B AmpMod PB In Env {:?}", reader.read_i8()?);
            println!("C AmpMod PB In Env {:?}", reader.read_i8()?);
            println!("D AmpMod PB In Env {:?}", reader.read_i8()?);
            println!("E AmpMod PB In Env {:?}", reader.read_i8()?);
            println!("F AmpMod PB In Env {:?}", reader.read_i8()?);
            println!("X AmpMod PB In Env {:?}", reader.read_i8()?);
            println!("Z AmpMod PB In Env {:?}", reader.read_i8()?);

            println!("15 {:?}", reader.read_i8()?);

            println!("A AmpMod PB LFO 1 {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 1 {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 1 {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 1 {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 1 {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 1 {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 1 {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 1 {:?}", reader.read_i8()?);

            println!("u8-2 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 1 AT {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 1 AT {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 1 AT {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 1 AT {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 1 AT {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 1 AT {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 1 AT {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 1 AT {:?}", reader.read_i8()?);

            println!("u8-3 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 1 Mod {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 1 Mod {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 1 Mod {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 1 Mod {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 1 Mod {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 1 Mod {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 1 Mod {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 1 Mod {:?}", reader.read_i8()?);

            println!("u8-4 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 1 Breath {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 1 Breath {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 1 Breath {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 1 Breath {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 1 Breath {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 1 Breath {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 1 Breath {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 1 Breath {:?}", reader.read_i8()?);

            println!("u8-6 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 1 Ctrl 1 {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 1 Ctrl 1 {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 1 Ctrl 1 {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 1 Ctrl 1 {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 1 Ctrl 1 {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 1 Ctrl 1 {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 1 Ctrl 1 {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 1 Ctrl 1 {:?}", reader.read_i8()?);

            println!("u8-5 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 1 Ctrl 2 {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 1 Ctrl 2 {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 1 Ctrl 2 {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 1 Ctrl 2 {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 1 Ctrl 2 {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 1 Ctrl 2 {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 1 Ctrl 2 {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 1 Ctrl 2 {:?}", reader.read_i8()?);

            println!("u8-7 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 2 {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 2 {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 2 {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 2 {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 2 {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 2 {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 2 {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 2 {:?}", reader.read_i8()?);

            println!("u8-8 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 2 AT {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 2 AT {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 2 AT {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 2 AT {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 2 AT {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 2 AT {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 2 AT {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 2 AT {:?}", reader.read_i8()?);

            println!("u8-9 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 2 Mod {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 2 Mod {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 2 Mod {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 2 Mod {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 2 Mod {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 2 Mod {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 2 Mod {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 2 Mod {:?}", reader.read_i8()?);

            println!("u8-10 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 2 Breath {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 2 Breath {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 2 Breath {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 2 Breath {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 2 Breath {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 2 Breath {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 2 Breath {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 2 Breath {:?}", reader.read_i8()?);

            println!("u8-11 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 2 Ctrl 1 {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 2 Ctrl 1 {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 2 Ctrl 1 {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 2 Ctrl 1 {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 2 Ctrl 1 {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 2 Ctrl 1 {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 2 Ctrl 1 {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 2 Ctrl 1 {:?}", reader.read_i8()?);

            println!("u8-12 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 2 Ctrl 2 {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 2 Ctrl 2 {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 2 Ctrl 2 {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 2 Ctrl 2 {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 2 Ctrl 2 {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 2 Ctrl 2 {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 2 Ctrl 2 {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 2 Ctrl 2 {:?}", reader.read_i8()?);

            println!("u8-21 {:?}", reader.read_u8()?);

            println!("A AmpMod PB LFO 2 Ctrl ? {:?}", reader.read_i8()?);
            println!("B AmpMod PB LFO 2 Ctrl ? {:?}", reader.read_i8()?);
            println!("C AmpMod PB LFO 2 Ctrl ? {:?}", reader.read_i8()?);
            println!("D AmpMod PB LFO 2 Ctrl ? {:?}", reader.read_i8()?);
            println!("E AmpMod PB LFO 2 Ctrl ? {:?}", reader.read_i8()?);
            println!("F AmpMod PB LFO 2 Ctrl ? {:?}", reader.read_i8()?);
            println!("X AmpMod PB LFO 2 Ctrl ? {:?}", reader.read_i8()?);
            println!("Z AmpMod PB LFO 2 Ctrl ? {:?}", reader.read_i8()?);

            println!("u8-30 {:?}", reader.read_u8()?);

            println!("A Waveform Invert {:?}", reader.read_u8()?);
            println!("B Waveform Invert {:?}", reader.read_u8()?);
            println!("C Waveform Invert {:?}", reader.read_u8()?);
            println!("D Waveform Invert {:?}", reader.read_u8()?);
            println!("E Waveform Invert {:?}", reader.read_u8()?);
            println!("F Waveform Invert {:?}", reader.read_u8()?);
            println!("X Waveform Invert {:?}", reader.read_u8()?);
            println!("Z Waveform Invert {:?}", reader.read_u8()?);

            println!("u8-39 {:?}", reader.read_u8()?);

            println!("A Waveform Pitch Env {:?}", reader.read_u8()?);
            println!("B Waveform Pitch Env {:?}", reader.read_u8()?);
            println!("C Waveform Pitch Env {:?}", reader.read_u8()?);
            println!("D Waveform Pitch Env {:?}", reader.read_u8()?);
            println!("E Waveform Pitch Env {:?}", reader.read_u8()?);
            println!("F Waveform Pitch Env {:?}", reader.read_u8()?);
            println!("X Waveform Pitch Env {:?}", reader.read_u8()?);
            println!("Z Waveform Pitch Env {:?}", reader.read_u8()?);

            println!("u8-48 {:?}", reader.read_u8()?);
            println!("u8-49 {:?}", reader.read_u8()?);

            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-48 {:?}", reader.read_u8()?);

            println!("7 {:?}", reader.read_f32_le()?);
            println!("9 {:?}", reader.read_f32_le()?);
            println!("11 {:?}", reader.read_f32_le()?);
            println!("13 {:?}", reader.read_f32_le()?);
            println!("14 {:?}", reader.read_f32_le()?);
            println!("8 {:?}", reader.read_f32_le()?);
            println!("10 {:?}", reader.read_f32_le()?);
            println!("12 {:?}", reader.read_f32_le()?);
            println!("15 {:?}", reader.read_f32_le()?);
            println!("17 {:?}", reader.read_f32_le()?);
            println!("19 {:?}", reader.read_f32_le()?);
            println!("21 {:?}", reader.read_f32_le()?);
            println!("22 {:?}", reader.read_f32_le()?);
            println!("16 {:?}", reader.read_f32_le()?);
            println!("18 {:?}", reader.read_f32_le()?);
            println!("20 {:?}", reader.read_f32_le()?);
            println!("22 {:?}", reader.read_u32_le()?);
            println!("24 {:?}", reader.read_u32_le()?);
            println!("26 {:?}", reader.read_u32_le()?);
            println!("28 {:?}", reader.read_u32_le()?);
            println!("30 {:?}", reader.read_u32_le()?);
            println!("31 {:?}", reader.read_u32_le()?);
            println!("25 {:?}", reader.read_u32_le()?);
            println!("27 {:?}", reader.read_u32_le()?);
            println!("29 {:?}", reader.read_u32_le()?);
            println!("31 {:?}", reader.read_u32_le()?);
            println!("33 {:?}", reader.read_u32_le()?);
            println!("35 {:?}", reader.read_u32_le()?);
            println!("37 {:?}", reader.read_u32_le()?);
            println!("39 {:?}", reader.read_u32_le()?);
            println!("40 {:?}", reader.read_u32_le()?);
            println!("38 {:?}", reader.read_f32_le()?);
            println!("40 {:?}", reader.read_f32_le()?);
            println!("42 {:?}", reader.read_f32_le()?);
            println!("44 {:?}", reader.read_f32_le()?);
            println!("45 {:?}", reader.read_f32_le()?);
            println!("39 {:?}", reader.read_f32_le()?);
            println!("41 {:?}", reader.read_f32_le()?);
            println!("43 {:?}", reader.read_f32_le()?);
            println!("46 {:?}", reader.read_f32_le()?);
            println!("48 {:?}", reader.read_f32_le()?);
            println!("50 {:?}", reader.read_f32_le()?);
            println!("52 {:?}", reader.read_f32_le()?);
            println!("53 {:?}", reader.read_f32_le()?);
            println!("47 {:?}", reader.read_f32_le()?);
            println!("49 {:?}", reader.read_f32_le()?);
            println!("51 {:?}", reader.read_f32_le()?);

            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);

            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);

            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);

            println!("waveform {:?}", reader.read_u8()?);
            println!("waveform {:?}", reader.read_u8()?);
            println!("waveform {:?}", reader.read_u8()?);
            println!("A Waveform Type {:?}", reader.read_u8()?);

            println!("B Waveform Type {:?}", reader.read_u8()?);
            println!("C Waveform Type {:?}", reader.read_u8()?);
            println!("D Waveform Type {:?}", reader.read_u8()?);
            println!("E Waveform Type {:?}", reader.read_u8()?);
            println!("F Waveform Type {:?}", reader.read_u8()?);
            println!("X Waveform Type {:?}", reader.read_u8()?);
            println!("Z Waveform Type {:?}", reader.read_u8()?);

            println!("4waveform {:?}", reader.read_u8()?);

            println!("56 {:?}", reader.read_u32_le()?);
            println!("58 {:?}", reader.read_u32_le()?);
            println!("60 {:?}", reader.read_u32_le()?);
            println!("62 {:?}", reader.read_u32_le()?);
            println!("64 {:?}", reader.read_u32_le()?);
            println!("66 {:?}", reader.read_u32_le()?);
            println!("68 {:?}", reader.read_u32_le()?);
            println!("70 {:?}", reader.read_u32_le()?);
            println!("71 {:?}", reader.read_u32_le()?);
            println!("69 {:?}", reader.read_f32_le()?);
            println!("71 {:?}", reader.read_f32_le()?);
            println!("73 {:?}", reader.read_f32_le()?);
            println!("75 {:?}", reader.read_f32_le()?);
            println!("76 {:?}", reader.read_f32_le()?);
            println!("70 {:?}", reader.read_f32_le()?);
            println!("72 {:?}", reader.read_f32_le()?);
            println!("74 {:?}", reader.read_f32_le()?);
            println!("77 {:?}", reader.read_f32_le()?);
            println!("79 {:?}", reader.read_f32_le()?);
            println!("81 {:?}", reader.read_f32_le()?);
            println!("83 {:?}", reader.read_f32_le()?);
            println!("84 {:?}", reader.read_f32_le()?);
            println!("78 {:?}", reader.read_f32_le()?);
            println!("80 {:?}", reader.read_f32_le()?);
            println!("82 {:?}", reader.read_f32_le()?);
            println!("84 {:?}", reader.read_u32_le()?);
            println!("86 {:?}", reader.read_u32_le()?);
            println!("88 {:?}", reader.read_u32_le()?);
            println!("90 {:?}", reader.read_u32_le()?);
            println!("92 {:?}", reader.read_u32_le()?);
            println!("93 {:?}", reader.read_u32_le()?);
            println!("87 {:?}", reader.read_u32_le()?);
            println!("89 {:?}", reader.read_u32_le()?);
            println!("91 {:?}", reader.read_u32_le()?);
            println!("93 {:?}", reader.read_u32_le()?);
            println!("95 {:?}", reader.read_u32_le()?);
            println!("97 {:?}", reader.read_u32_le()?);
            println!("99 {:?}", reader.read_u32_le()?);
            println!("101 {:?}", reader.read_u32_le()?);
            println!("102 {:?}", reader.read_u32_le()?);
            println!("99 {:?}", reader.read_u32_le()?);
            println!("101 {:?}", reader.read_u32_le()?);
            println!("103 {:?}", reader.read_u32_le()?);
            println!("105 {:?}", reader.read_u32_le()?);
            println!("107 {:?}", reader.read_u32_le()?);
            println!("108 {:?}", reader.read_u32_le()?);
            println!("102 {:?}", reader.read_u32_le()?);
            println!("104 {:?}", reader.read_u32_le()?);
            println!("106 {:?}", reader.read_u32_le()?);
            println!("108 {:?}", reader.read_u32_le()?);
            println!("110 {:?}", reader.read_u32_le()?);
            println!("112 {:?}", reader.read_u32_le()?);
            println!("114 {:?}", reader.read_u32_le()?);
            println!("116 {:?}", reader.read_u32_le()?);
            println!("117 {:?}", reader.read_u32_le()?);
            println!("114 {:?}", reader.read_u32_le()?);
            println!("116 {:?}", reader.read_u32_le()?);
            println!("118 {:?}", reader.read_u32_le()?);
            println!("120 {:?}", reader.read_u32_le()?);
            println!("122 {:?}", reader.read_u32_le()?);
            println!("123 {:?}", reader.read_u32_le()?);
            println!("117 {:?}", reader.read_u32_le()?);
            println!("119 {:?}", reader.read_u32_le()?);
            println!("121 {:?}", reader.read_u32_le()?);
            println!("123 {:?}", reader.read_u32_le()?);
            println!("125 {:?}", reader.read_u32_le()?);
            println!("127 {:?}", reader.read_u32_le()?);
            println!("129 {:?}", reader.read_u32_le()?);
            println!("131 {:?}", reader.read_u32_le()?);
            println!("132 {:?}", reader.read_u32_le()?);
            println!("129 {:?}", reader.read_u32_le()?);
            println!("131 {:?}", reader.read_u32_le()?);
            println!("133 {:?}", reader.read_u32_le()?);
            println!("135 {:?}", reader.read_u32_le()?);
            println!("137 {:?}", reader.read_u32_le()?);
            println!("138 {:?}", reader.read_u32_le()?);
            println!("132 {:?}", reader.read_u32_le()?);
            println!("134 {:?}", reader.read_u32_le()?);
            println!("136 {:?}", reader.read_u32_le()?);
            println!("138 {:?}", reader.read_u32_le()?);
            println!("140 {:?}", reader.read_u32_le()?);
            println!("142 {:?}", reader.read_u32_le()?);
            println!("144 {:?}", reader.read_u32_le()?);
            println!("146 {:?}", reader.read_u32_le()?);
            println!("147 {:?}", reader.read_u32_le()?);
            println!("144 {:?}", reader.read_u32_le()?);
            println!("146 {:?}", reader.read_u32_le()?);
            println!("148 {:?}", reader.read_u32_le()?);
            println!("150 {:?}", reader.read_u32_le()?);
            println!("152 {:?}", reader.read_u32_le()?);
            println!("153 {:?}", reader.read_u32_le()?);
            println!("147 {:?}", reader.read_u32_le()?);
            println!("149 {:?}", reader.read_u32_le()?);
            println!("151 {:?}", reader.read_u32_le()?);
            println!("153 {:?}", reader.read_u32_le()?);
            println!("155 {:?}", reader.read_u32_le()?);
            println!("157 {:?}", reader.read_u32_le()?);
            println!("159 {:?}", reader.read_u32_le()?);
            println!("161 {:?}", reader.read_u32_le()?);
            println!("162 {:?}", reader.read_u32_le()?);
            println!("159 {:?}", reader.read_u32_le()?);
            println!("161 {:?}", reader.read_u32_le()?);
            println!("163 {:?}", reader.read_u32_le()?);
            println!("165 {:?}", reader.read_u32_le()?);
            println!("167 {:?}", reader.read_u32_le()?);
            println!("168 {:?}", reader.read_u32_le()?);
            println!("162 {:?}", reader.read_u32_le()?);
            println!("164 {:?}", reader.read_u32_le()?);
            println!("166 {:?}", reader.read_u32_le()?);
            println!("168 {:?}", reader.read_u32_le()?);
            println!("170 {:?}", reader.read_u32_le()?);
            println!("172 {:?}", reader.read_u32_le()?);
            println!("174 {:?}", reader.read_u32_le()?);
            println!("176 {:?}", reader.read_u32_le()?);
            println!("177 {:?}", reader.read_u32_le()?);
            println!("174 {:?}", reader.read_u32_le()?);
            println!("176 {:?}", reader.read_u32_le()?);
            println!("178 {:?}", reader.read_u32_le()?);
            println!("180 {:?}", reader.read_u32_le()?);
            println!("182 {:?}", reader.read_u32_le()?);
            println!("183 {:?}", reader.read_u32_le()?);
            println!("177 {:?}", reader.read_u32_le()?);
            println!("179 {:?}", reader.read_u32_le()?);
            println!("181 {:?}", reader.read_u32_le()?);
            println!("183 {:?}", reader.read_u32_le()?);
            println!("185 {:?}", reader.read_u32_le()?);
            println!("187 {:?}", reader.read_u32_le()?);
            println!("189 {:?}", reader.read_u32_le()?);
            println!("191 {:?}", reader.read_u32_le()?);
            println!("192 {:?}", reader.read_u32_le()?);
            println!("189 {:?}", reader.read_u32_le()?);
            println!("191 {:?}", reader.read_u32_le()?);
            println!("193 {:?}", reader.read_u32_le()?);
            println!("195 {:?}", reader.read_u32_le()?);
            println!("197 {:?}", reader.read_u32_le()?);
            println!("198 {:?}", reader.read_u32_le()?);
            println!("192 {:?}", reader.read_u32_le()?);
            println!("194 {:?}", reader.read_u32_le()?);
            println!("196 {:?}", reader.read_u32_le()?);
            println!("198 {:?}", reader.read_u32_le()?);
            println!("200 {:?}", reader.read_u32_le()?);
            println!("202 {:?}", reader.read_u32_le()?);
            println!("204 {:?}", reader.read_u32_le()?);
            println!("206 {:?}", reader.read_u32_le()?);
            println!("207 {:?}", reader.read_u32_le()?);
            println!("204 {:?}", reader.read_u32_le()?);
            println!("206 {:?}", reader.read_u32_le()?);
            println!("208 {:?}", reader.read_u32_le()?);
            println!("210 {:?}", reader.read_u32_le()?);
            println!("212 {:?}", reader.read_u32_le()?);
            println!("213 {:?}", reader.read_u32_le()?);
            println!("207 {:?}", reader.read_u32_le()?);
            println!("209 {:?}", reader.read_u32_le()?);
            println!("211 {:?}", reader.read_u32_le()?);
            println!("213 {:?}", reader.read_u32_le()?);
            println!("215 {:?}", reader.read_u32_le()?);
            println!("217 {:?}", reader.read_u32_le()?);
            println!("219 {:?}", reader.read_u32_le()?);
            println!("221 {:?}", reader.read_u32_le()?);
            println!("222 {:?}", reader.read_u32_le()?);
            println!("219 {:?}", reader.read_u32_le()?);
            println!("221 {:?}", reader.read_u32_le()?);
            println!("223 {:?}", reader.read_u32_le()?);
            println!("225 {:?}", reader.read_u32_le()?);
            println!("227 {:?}", reader.read_u32_le()?);
            println!("228 {:?}", reader.read_u32_le()?);
            println!("222 {:?}", reader.read_u32_le()?);
            println!("224 {:?}", reader.read_u32_le()?);
            println!("226 {:?}", reader.read_u32_le()?);
            println!("228 {:?}", reader.read_u32_le()?);
            println!("230 {:?}", reader.read_u32_le()?);
            println!("232 {:?}", reader.read_u32_le()?);
            println!("234 {:?}", reader.read_u32_le()?);
            println!("236 {:?}", reader.read_u32_le()?);
            println!("237 {:?}", reader.read_u32_le()?);
            println!("234 {:?}", reader.read_u32_le()?);
            println!("236 {:?}", reader.read_u32_le()?);
            println!("238 {:?}", reader.read_u32_le()?);
            println!("240 {:?}", reader.read_u32_le()?);
            println!("242 {:?}", reader.read_u32_le()?);
            println!("243 {:?}", reader.read_u32_le()?);
            println!("237 {:?}", reader.read_u32_le()?);
            println!("239 {:?}", reader.read_u32_le()?);
            println!("241 {:?}", reader.read_u32_le()?);
            println!("243 {:?}", reader.read_u32_le()?);
            println!("245 {:?}", reader.read_u32_le()?);
            println!("247 {:?}", reader.read_u32_le()?);
            println!("249 {:?}", reader.read_u32_le()?);
            println!("251 {:?}", reader.read_u32_le()?);
            println!("252 {:?}", reader.read_u32_le()?);
            println!("249 {:?}", reader.read_u32_le()?);
            println!("251 {:?}", reader.read_u32_le()?);
            println!("253 {:?}", reader.read_u32_le()?);
            println!("255 {:?}", reader.read_u32_le()?);
            println!("257 {:?}", reader.read_u32_le()?);
            println!("258 {:?}", reader.read_u32_le()?);
            println!("252 {:?}", reader.read_u32_le()?);
            println!("254 {:?}", reader.read_u32_le()?);
            println!("256 {:?}", reader.read_u32_le()?);
            println!("258 {:?}", reader.read_u32_le()?);
            println!("260 {:?}", reader.read_u32_le()?);
            println!("262 {:?}", reader.read_u32_le()?);
            println!("264 {:?}", reader.read_u32_le()?);
            println!("266 {:?}", reader.read_u32_le()?);
            println!("267 {:?}", reader.read_u32_le()?);
            println!("264 {:?}", reader.read_u32_le()?);
            println!("266 {:?}", reader.read_u32_le()?);
            println!("268 {:?}", reader.read_u32_le()?);
            println!("270 {:?}", reader.read_u32_le()?);
            println!("272 {:?}", reader.read_u32_le()?);
            println!("273 {:?}", reader.read_u32_le()?);
            println!("267 {:?}", reader.read_u32_le()?);
            println!("269 {:?}", reader.read_u32_le()?);
            println!("271 {:?}", reader.read_u32_le()?);
            println!("273 {:?}", reader.read_u32_le()?);
            println!("275 {:?}", reader.read_u32_le()?);
            println!("277 {:?}", reader.read_u32_le()?);
            println!("279 {:?}", reader.read_u32_le()?);
            println!("281 {:?}", reader.read_u32_le()?);
            println!("282 {:?}", reader.read_u32_le()?);
            println!("279 {:?}", reader.read_u32_le()?);
            println!("281 {:?}", reader.read_u32_le()?);
            println!("283 {:?}", reader.read_u32_le()?);
            println!("285 {:?}", reader.read_u32_le()?);
            println!("287 {:?}", reader.read_u32_le()?);
            println!("288 {:?}", reader.read_u32_le()?);
            println!("282 {:?}", reader.read_u32_le()?);
            println!("284 {:?}", reader.read_u32_le()?);
            println!("286 {:?}", reader.read_u32_le()?);
            println!("288 {:?}", reader.read_u32_le()?);
            println!("290 {:?}", reader.read_u32_le()?);
            println!("292 {:?}", reader.read_u32_le()?);
            println!("294 {:?}", reader.read_u32_le()?);
            println!("296 {:?}", reader.read_u32_le()?);
            println!("297 {:?}", reader.read_u32_le()?);
            println!("294 {:?}", reader.read_u32_le()?);
            println!("296 {:?}", reader.read_u32_le()?);
            println!("298 {:?}", reader.read_u32_le()?);
            println!("300 {:?}", reader.read_u32_le()?);
            println!("302 {:?}", reader.read_u32_le()?);
            println!("303 {:?}", reader.read_u32_le()?);
            println!("297 {:?}", reader.read_u32_le()?);
            println!("299 {:?}", reader.read_u32_le()?);
            println!("301 {:?}", reader.read_u32_le()?);
            println!("303 {:?}", reader.read_u32_le()?);
            println!("305 {:?}", reader.read_u32_le()?);
            println!("307 {:?}", reader.read_u32_le()?);
            println!("309 {:?}", reader.read_u32_le()?);
            println!("311 {:?}", reader.read_u32_le()?);
            println!("312 {:?}", reader.read_u32_le()?);
            println!("309 {:?}", reader.read_u32_le()?);
            println!("311 {:?}", reader.read_u32_le()?);
            println!("313 {:?}", reader.read_u32_le()?);
            println!("315 {:?}", reader.read_u32_le()?);
            println!("317 {:?}", reader.read_u32_le()?);
            println!("318 {:?}", reader.read_u32_le()?);
            println!("312 {:?}", reader.read_u32_le()?);
            println!("314 {:?}", reader.read_u32_le()?);
            println!("316 {:?}", reader.read_u32_le()?);
            println!("318 {:?}", reader.read_u32_le()?);
            println!("320 {:?}", reader.read_u32_le()?);
            println!("322 {:?}", reader.read_u32_le()?);
            println!("324 {:?}", reader.read_u32_le()?);
            println!("326 {:?}", reader.read_u32_le()?);
            println!("327 {:?}", reader.read_u32_le()?);
            println!("324 {:?}", reader.read_u32_le()?);
            println!("326 {:?}", reader.read_u32_le()?);
            println!("328 {:?}", reader.read_u32_le()?);
            println!("330 {:?}", reader.read_u32_le()?);
            println!("332 {:?}", reader.read_u32_le()?);
            println!("333 {:?}", reader.read_u32_le()?);
            println!("327 {:?}", reader.read_u32_le()?);
            println!("329 {:?}", reader.read_u32_le()?);
            println!("331 {:?}", reader.read_u32_le()?);
            println!("333 {:?}", reader.read_u32_le()?);
            println!("335 {:?}", reader.read_u32_le()?);
            println!("337 {:?}", reader.read_u32_le()?);
            println!("339 {:?}", reader.read_u32_le()?);
            println!("341 {:?}", reader.read_u32_le()?);
            println!("342 {:?}", reader.read_u32_le()?);
            println!("339 {:?}", reader.read_u32_le()?);
            println!("341 {:?}", reader.read_u32_le()?);
            println!("343 {:?}", reader.read_u32_le()?);
            println!("345 {:?}", reader.read_u32_le()?);
            println!("347 {:?}", reader.read_u32_le()?);
            println!("348 {:?}", reader.read_u32_le()?);
            println!("342 {:?}", reader.read_u32_le()?);
            println!("344 {:?}", reader.read_u32_le()?);
            println!("346 {:?}", reader.read_u32_le()?);
            println!("348 {:?}", reader.read_u32_le()?);
            println!("350 {:?}", reader.read_u32_le()?);
            println!("352 {:?}", reader.read_u32_le()?);
            println!("354 {:?}", reader.read_u32_le()?);
            println!("356 {:?}", reader.read_u32_le()?);
            println!("357 {:?}", reader.read_u32_le()?);
            println!("354 {:?}", reader.read_u32_le()?);
            println!("356 {:?}", reader.read_u32_le()?);
            println!("358 {:?}", reader.read_u32_le()?);
            println!("360 {:?}", reader.read_u32_le()?);
            println!("362 {:?}", reader.read_u32_le()?);
            println!("363 {:?}", reader.read_u32_le()?);
            println!("357 {:?}", reader.read_u32_le()?);
            println!("359 {:?}", reader.read_u32_le()?);
            println!("361 {:?}", reader.read_u32_le()?);
            println!("363 {:?}", reader.read_u32_le()?);
            println!("365 {:?}", reader.read_u32_le()?);
            println!("367 {:?}", reader.read_u32_le()?);
            println!("369 {:?}", reader.read_u32_le()?);
            println!("371 {:?}", reader.read_u32_le()?);
            println!("372 {:?}", reader.read_u32_le()?);
            println!("369 {:?}", reader.read_u32_le()?);
            println!("371 {:?}", reader.read_u32_le()?);
            println!("373 {:?}", reader.read_u32_le()?);
            println!("375 {:?}", reader.read_u32_le()?);
            println!("377 {:?}", reader.read_u32_le()?);
            println!("378 {:?}", reader.read_u32_le()?);
            println!("372 {:?}", reader.read_u32_le()?);
            println!("374 {:?}", reader.read_u32_le()?);
            println!("376 {:?}", reader.read_u32_le()?);
            println!("378 {:?}", reader.read_u32_le()?);
            println!("380 {:?}", reader.read_u32_le()?);

            println!("393 {:?}", reader.read_u8()?);

            // -1
            println!("382 {:?}", reader.read_i32_le()?);
            println!("384 {:?}", reader.read_i32_le()?);
            println!("386 {:?}", reader.read_i32_le()?);
            println!("387 {:?}", reader.read_i32_le()?);
            println!("381 {:?}", reader.read_i32_le()?);
            println!("383 {:?}", reader.read_i32_le()?);
            println!("385 {:?}", reader.read_i32_le()?);
            println!("387 {:?}", reader.read_i32_le()?);

            println!("393 {:?}", reader.read_u8()?);

            println!("389 {:?}", reader.read_u32_le()?);
            println!("391 {:?}", reader.read_u32_le()?);

            println!("393 {:?}", reader.read_u8()?);
            println!("393 {:?}", reader.read_u8()?);

            println!("395 {:?}", reader.read_u32_le()?);

            println!("397 {:?}", reader.read_u8()?);
            println!("397 {:?}", reader.read_u8()?);
            println!("397 {:?}", reader.read_u8()?);
            println!("397 {:?}", reader.read_u8()?);
            println!("397 {:?}", reader.read_u8()?);
            println!("397 {:?}", reader.read_u8()?);
            println!("397 {:?}", reader.read_u8()?);
            println!("397 {:?}", reader.read_u8()?);
            println!("397 {:?}", reader.read_u8()?);
            println!("397 {:?}", reader.read_u8()?);
            println!("397 {:?}", reader.read_u8()?);
            println!("398 {:?}", reader.read_i16_le()?);
            println!("400 {:?}", reader.read_i16_le()?);
            println!("402 {:?}", reader.read_i16_le()?);
            println!("404 {:?}", reader.read_i16_le()?);
            println!("405 {:?}", reader.read_i16_le()?);
            println!("400 {:?}", reader.read_i16_le()?);
            println!("402 {:?}", reader.read_i16_le()?);
            println!("404 {:?}", reader.read_i16_le()?);
            println!("406 {:?}", reader.read_i16_le()?);

            println!("arpKeySync2 {:x}", reader.read_u8()?);
            println!("arpKeySync3 {:x}", reader.read_u8()?);

            println!("Arpeggiator BPM {:?}", reader.read_f32_le()?);

            println!("step - 412 {:?}", reader.read_u8()?);

            for i in 1..33 {
                println!("Step {:?}", reader.read_i32_le()?);
                println!("Step Sequencer {} - On/Off {:?}", i, reader.read_i8()?);
                println!("Step Sequencer {} - Tie {:?}", i, reader.read_i8()?);
                println!("Step Sequencer {} - Accent {:?}", i, reader.read_i8()?);
                println!("Step Sequencer {} - Note Order {:?}", i, reader.read_i8()?);
                println!("Step Sequencer {} - Octave {:?}", i, reader.read_i8()?);
                println!("Step Sequencer {} - Transpose {:?}", i, reader.read_i8()?);
            }
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
