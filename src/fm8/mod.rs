// mod arp;
mod fx;
mod matrix;

use crate::{
    fm8::{fx::FM8EffectSettings, matrix::FM8Matrix},
    read_bytes::ReadBytesExt,
    Error, NIFileError,
};

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
            println!("--4 {:?}", reader.read_f32_le()?);
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

            FM8EffectSettings::print(&mut reader)?;

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
            println!("---4 {:?}", reader.read_f32_le()?);
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
            println!("X Noise Cutoff {:?}", reader.read_f32_le()?);
            println!("X Noise Reso {:?}", reader.read_f32_le()?);
            println!("X Noise Amp {:?}", reader.read_f32_le()?);
            println!("X Saturator Gain {:?}", reader.read_f32_le()?);
            println!("X Saturator Level {:?}", reader.read_f32_le()?);
            println!("X Saturator Asym {:?}", reader.read_f32_le()?);
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

            println!("keyscaling {:?}", reader.read_i8()?);
            println!("keyscaling {:?}", reader.read_i8()?);

            println!("A Env: Key Scaling {:?}", reader.read_i8()?);
            println!("B Env: Key Scaling {:?}", reader.read_i8()?);
            println!("C Env: Key Scaling {:?}", reader.read_i8()?);
            println!("D Env: Key Scaling {:?}", reader.read_i8()?);
            println!("E Env: Key Scaling {:?}", reader.read_i8()?);
            println!("F Env: Key Scaling {:?}", reader.read_i8()?);
            println!("X Env: Key Scaling {:?}", reader.read_i8()?);
            println!("Z Env: Key Scaling {:?}", reader.read_i8()?);
            println!("? Env: Key Scaling {:?}", reader.read_i8()?);

            println!("A Env: Vel Scaling {:?}", reader.read_i8()?);
            println!("B Env: Vel Scaling {:?}", reader.read_i8()?);
            println!("C Env: Vel Scaling {:?}", reader.read_i8()?);
            println!("D Env: Vel Scaling {:?}", reader.read_i8()?);
            println!("E Env: Vel Scaling {:?}", reader.read_i8()?);
            println!("F Env: Vel Scaling {:?}", reader.read_i8()?);
            println!("X Env: Vel Scaling {:?}", reader.read_i8()?);
            println!("Z Env: Vel Scaling {:?}", reader.read_i8()?);
            println!("? Env: Vel Scaling {:?}", reader.read_i8()?);

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

            println!("u8-49 {:?}", reader.read_u8()?);
            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-48 {:?}", reader.read_u8()?);

            println!("u8-49 {:?}", reader.read_u8()?);
            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-48 {:?}", reader.read_u8()?);

            println!("u8-49 {:?}", reader.read_u8()?);
            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-48 {:?}", reader.read_u8()?);

            println!("u8-49 {:?}", reader.read_u8()?);
            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-48 {:?}", reader.read_u8()?);

            println!("u8-49 {:?}", reader.read_u8()?);
            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-48 {:?}", reader.read_u8()?);

            println!("u8-49 {:?}", reader.read_u8()?);
            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-48 {:?}", reader.read_u8()?);

            println!("u8-49 {:?}", reader.read_u8()?);
            println!("u8-46 {:?}", reader.read_u8()?);
            println!("u8-47 {:?}", reader.read_u8()?);
            println!("u8-48 {:?}", reader.read_u8()?);

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

            println!("A Ratio {:?}", reader.read_f32_le()?);
            println!("B Ratio {:?}", reader.read_f32_le()?);
            println!("C Ratio {:?}", reader.read_f32_le()?);
            println!("D Ratio {:?}", reader.read_f32_le()?);
            println!("E Ratio {:?}", reader.read_f32_le()?);
            println!("F Ratio {:?}", reader.read_f32_le()?);
            println!("138 {:?}", reader.read_f32_le()?);
            println!("150 {:?}", reader.read_f32_le()?);
            println!("161 {:?}", reader.read_f32_le()?);
            println!("A Offset (Hz) {:?}", reader.read_f32_le()?);
            println!("B Offset (Hz) {:?}", reader.read_f32_le()?);
            println!("C Offset (Hz) {:?}", reader.read_f32_le()?);
            println!("D Offset (Hz) {:?}", reader.read_f32_le()?);
            println!("E Offset (Hz) {:?}", reader.read_f32_le()?);
            println!("F Offset (Hz) {:?}", reader.read_f32_le()?);
            println!("232 {:?}", reader.read_f32_le()?);
            println!("244 {:?}", reader.read_f32_le()?);
            println!("257 {:?}", reader.read_f32_le()?);
            println!("269 {:?}", reader.read_f32_le()?);
            println!("281 {:?}", reader.read_f32_le()?);
            println!("293 {:?}", reader.read_f32_le()?);
            println!("304 {:?}", reader.read_f32_le()?);
            println!("308 {:?}", reader.read_f32_le()?);
            println!("320 {:?}", reader.read_f32_le()?);
            println!("332 {:?}", reader.read_f32_le()?);

            println!("344 {:?}", reader.read_u32_le()?);
            println!("356 {:?}", reader.read_u32_le()?);
            println!("368 {:?}", reader.read_u32_le()?);
            println!("380 {:?}", reader.read_u32_le()?);
            println!("392 {:?}", reader.read_u32_le()?);
            println!("403 {:?}", reader.read_u32_le()?);
            println!("407 {:?}", reader.read_u32_le()?);
            println!("419 {:?}", reader.read_u32_le()?);
            println!("431 {:?}", reader.read_u32_le()?);
            println!("443 {:?}", reader.read_u32_le()?);
            println!("455 {:?}", reader.read_u32_le()?);
            println!("467 {:?}", reader.read_u32_le()?);
            println!("479 {:?}", reader.read_u32_le()?);
            println!("4001 {:?}", reader.read_u32_le()?);
            println!("4102 {:?}", reader.read_u32_le()?);

            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);
            println!("509 {:?}", reader.read_i8()?);

            println!("4802 {:?}", reader.read_u32_le()?);
            println!("4904 {:?}", reader.read_u32_le()?);
            println!("5006 {:?}", reader.read_u32_le()?);
            println!("5108 {:?}", reader.read_u32_le()?);
            println!("5210 {:?}", reader.read_u32_le()?);
            println!("5312 {:?}", reader.read_u32_le()?);
            println!("5414 {:?}", reader.read_u32_le()?);
            println!("5516 {:?}", reader.read_u32_le()?);
            println!("5617 {:?}", reader.read_u32_le()?);
            println!("5714 {:?}", reader.read_u32_le()?);
            println!("5816 {:?}", reader.read_u32_le()?);
            println!("5918 {:?}", reader.read_u32_le()?);
            println!("6020 {:?}", reader.read_u32_le()?);
            println!("6122 {:?}", reader.read_u32_le()?);
            println!("6223 {:?}", reader.read_u32_le()?);
            println!("6317 {:?}", reader.read_u32_le()?);
            println!("6419 {:?}", reader.read_u32_le()?);
            println!("6521 {:?}", reader.read_u32_le()?);
            println!("6623 {:?}", reader.read_u32_le()?);
            println!("6725 {:?}", reader.read_u32_le()?);
            println!("6827 {:?}", reader.read_u32_le()?);
            println!("6929 {:?}", reader.read_u32_le()?);
            println!("7031 {:?}", reader.read_u32_le()?);
            println!("7132 {:?}", reader.read_u32_le()?);
            println!("7229 {:?}", reader.read_u32_le()?);
            println!("7331 {:?}", reader.read_u32_le()?);
            println!("7433 {:?}", reader.read_u32_le()?);
            println!("7535 {:?}", reader.read_u32_le()?);
            println!("7637 {:?}", reader.read_u32_le()?);
            println!("7738 {:?}", reader.read_u32_le()?);
            println!("7832 {:?}", reader.read_u32_le()?);
            println!("7934 {:?}", reader.read_u32_le()?);
            println!("8036 {:?}", reader.read_u32_le()?);
            println!("8138 {:?}", reader.read_u32_le()?);
            println!("8240 {:?}", reader.read_u32_le()?);
            println!("8342 {:?}", reader.read_u32_le()?);
            println!("8444 {:?}", reader.read_u32_le()?);
            println!("8546 {:?}", reader.read_u32_le()?);
            println!("8647 {:?}", reader.read_u32_le()?);
            println!("8744 {:?}", reader.read_u32_le()?);
            println!("8846 {:?}", reader.read_u32_le()?);
            println!("8948 {:?}", reader.read_u32_le()?);
            println!("9050 {:?}", reader.read_u32_le()?);
            println!("9152 {:?}", reader.read_u32_le()?);
            println!("9253 {:?}", reader.read_u32_le()?);
            println!("9347 {:?}", reader.read_u32_le()?);
            println!("9449 {:?}", reader.read_u32_le()?);
            println!("9551 {:?}", reader.read_u32_le()?);
            println!("9653 {:?}", reader.read_u32_le()?);
            println!("9755 {:?}", reader.read_u32_le()?);
            println!("9857 {:?}", reader.read_u32_le()?);
            println!("9959 {:?}", reader.read_u32_le()?);
            println!("10061 {:?}", reader.read_u32_le()?);
            println!("10162 {:?}", reader.read_u32_le()?);
            println!("10259 {:?}", reader.read_u32_le()?);
            println!("10361 {:?}", reader.read_u32_le()?);
            println!("10463 {:?}", reader.read_u32_le()?);
            println!("10565 {:?}", reader.read_u32_le()?);
            println!("10667 {:?}", reader.read_u32_le()?);
            println!("10768 {:?}", reader.read_u32_le()?);
            println!("10862 {:?}", reader.read_u32_le()?);
            println!("10964 {:?}", reader.read_u32_le()?);
            println!("11066 {:?}", reader.read_u32_le()?);
            println!("11168 {:?}", reader.read_u32_le()?);
            println!("11270 {:?}", reader.read_u32_le()?);
            println!("11372 {:?}", reader.read_u32_le()?);
            println!("11474 {:?}", reader.read_u32_le()?);
            println!("11576 {:?}", reader.read_u32_le()?);
            println!("11677 {:?}", reader.read_u32_le()?);
            println!("11774 {:?}", reader.read_u32_le()?);
            println!("11876 {:?}", reader.read_u32_le()?);
            println!("11978 {:?}", reader.read_u32_le()?);
            println!("12080 {:?}", reader.read_u32_le()?);
            println!("12182 {:?}", reader.read_u32_le()?);
            println!("12283 {:?}", reader.read_u32_le()?);
            println!("12377 {:?}", reader.read_u32_le()?);
            println!("12479 {:?}", reader.read_u32_le()?);
            println!("12581 {:?}", reader.read_u32_le()?);
            println!("12683 {:?}", reader.read_u32_le()?);
            println!("12785 {:?}", reader.read_u32_le()?);
            println!("12887 {:?}", reader.read_u32_le()?);
            println!("12989 {:?}", reader.read_u32_le()?);
            println!("13091 {:?}", reader.read_u32_le()?);
            println!("13192 {:?}", reader.read_u32_le()?);
            println!("13289 {:?}", reader.read_u32_le()?);
            println!("13391 {:?}", reader.read_u32_le()?);
            println!("13493 {:?}", reader.read_u32_le()?);
            println!("13595 {:?}", reader.read_u32_le()?);
            println!("13697 {:?}", reader.read_u32_le()?);
            println!("13798 {:?}", reader.read_u32_le()?);
            println!("13892 {:?}", reader.read_u32_le()?);
            println!("13994 {:?}", reader.read_u32_le()?);
            println!("14096 {:?}", reader.read_u32_le()?);
            println!("14198 {:?}", reader.read_u32_le()?);
            println!("14300 {:?}", reader.read_u32_le()?);
            println!("14402 {:?}", reader.read_u32_le()?);
            println!("14504 {:?}", reader.read_u32_le()?);
            println!("14606 {:?}", reader.read_u32_le()?);
            println!("14707 {:?}", reader.read_u32_le()?);
            println!("14804 {:?}", reader.read_u32_le()?);
            println!("14906 {:?}", reader.read_u32_le()?);
            println!("15008 {:?}", reader.read_u32_le()?);
            println!("15110 {:?}", reader.read_u32_le()?);
            println!("15212 {:?}", reader.read_u32_le()?);
            println!("15313 {:?}", reader.read_u32_le()?);
            println!("15407 {:?}", reader.read_u32_le()?);
            println!("15509 {:?}", reader.read_u32_le()?);
            println!("15611 {:?}", reader.read_u32_le()?);
            println!("15713 {:?}", reader.read_u32_le()?);
            println!("15815 {:?}", reader.read_u32_le()?);
            println!("15917 {:?}", reader.read_u32_le()?);
            println!("16019 {:?}", reader.read_u32_le()?);
            println!("16121 {:?}", reader.read_u32_le()?);
            println!("16222 {:?}", reader.read_u32_le()?);
            println!("16319 {:?}", reader.read_u32_le()?);
            println!("16421 {:?}", reader.read_u32_le()?);
            println!("16523 {:?}", reader.read_u32_le()?);
            println!("16625 {:?}", reader.read_u32_le()?);
            println!("16727 {:?}", reader.read_u32_le()?);
            println!("16828 {:?}", reader.read_u32_le()?);
            println!("16922 {:?}", reader.read_u32_le()?);
            println!("17024 {:?}", reader.read_u32_le()?);
            println!("17126 {:?}", reader.read_u32_le()?);
            println!("17228 {:?}", reader.read_u32_le()?);
            println!("17330 {:?}", reader.read_u32_le()?);
            println!("17432 {:?}", reader.read_u32_le()?);
            println!("17534 {:?}", reader.read_u32_le()?);
            println!("17636 {:?}", reader.read_u32_le()?);
            println!("17737 {:?}", reader.read_u32_le()?);
            println!("17834 {:?}", reader.read_u32_le()?);
            println!("17936 {:?}", reader.read_u32_le()?);
            println!("18038 {:?}", reader.read_u32_le()?);
            println!("18140 {:?}", reader.read_u32_le()?);
            println!("18242 {:?}", reader.read_u32_le()?);
            println!("18343 {:?}", reader.read_u32_le()?);
            println!("18437 {:?}", reader.read_u32_le()?);
            println!("18539 {:?}", reader.read_u32_le()?);
            println!("18641 {:?}", reader.read_u32_le()?);
            println!("18743 {:?}", reader.read_u32_le()?);
            println!("18845 {:?}", reader.read_u32_le()?);
            println!("18947 {:?}", reader.read_u32_le()?);
            println!("19049 {:?}", reader.read_u32_le()?);
            println!("19151 {:?}", reader.read_u32_le()?);
            println!("19252 {:?}", reader.read_u32_le()?);
            println!("19349 {:?}", reader.read_u32_le()?);
            println!("19451 {:?}", reader.read_u32_le()?);
            println!("19553 {:?}", reader.read_u32_le()?);
            println!("19655 {:?}", reader.read_u32_le()?);
            println!("19757 {:?}", reader.read_u32_le()?);
            println!("19858 {:?}", reader.read_u32_le()?);
            println!("19952 {:?}", reader.read_u32_le()?);
            println!("20054 {:?}", reader.read_u32_le()?);
            println!("20156 {:?}", reader.read_u32_le()?);
            println!("20258 {:?}", reader.read_u32_le()?);
            println!("20360 {:?}", reader.read_u32_le()?);
            println!("20462 {:?}", reader.read_u32_le()?);
            println!("20564 {:?}", reader.read_u32_le()?);
            println!("20666 {:?}", reader.read_u32_le()?);
            println!("20767 {:?}", reader.read_u32_le()?);
            println!("20864 {:?}", reader.read_u32_le()?);
            println!("20966 {:?}", reader.read_u32_le()?);
            println!("21068 {:?}", reader.read_u32_le()?);
            println!("21170 {:?}", reader.read_u32_le()?);
            println!("21272 {:?}", reader.read_u32_le()?);
            println!("21373 {:?}", reader.read_u32_le()?);

            println!("21467 {:?}", reader.read_i8()?);
            println!("21467 {:?}", reader.read_i8()?);
            println!("21467 {:?}", reader.read_i8()?);

            FM8Matrix::print(&mut reader)?;

            println!("FM Matrix A {:?}", reader.read_i8()?);
            println!("393 {:?}", reader.read_u8()?);

            // -1
            println!("382 {:?}", reader.read_i32_le()?);
            println!("384 {:?}", reader.read_i32_le()?);
            println!("386 0x{:x}", reader.read_u32_be()?);
            println!("387 {:?}", reader.read_i32_le()?);
            println!("381 {:?}", reader.read_i32_le()?);
            println!("383 {:?}", reader.read_i32_le()?);
            println!("385 {:?}", reader.read_i32_le()?);
            println!("387 {:?}", reader.read_i32_le()?);
            println!("389 {:?}", reader.read_u32_le()?);
            println!("391 {:?}", reader.read_u32_le()?);
            println!("395 {:?}", reader.read_u32_le()?);

            println!("393 {:?}", reader.read_u8()?);
            println!("393 {:?}", reader.read_u8()?);
            println!("399 {:?}", reader.read_i8()?);
            println!("3910 {:?}", reader.read_i8()?);
            println!("3911 {:?}", reader.read_i8()?);
            println!("3912 {:?}", reader.read_i8()?);
            println!("3913 {:?}", reader.read_i8()?);
            println!("3914 {:?}", reader.read_i8()?);
            println!("3915 {:?}", reader.read_i8()?);
            println!("3916 {:?}", reader.read_i8()?);
            println!("3917 {:?}", reader.read_i8()?);
            println!("3918 {:?}", reader.read_i8()?);
            println!("3919 {:?}", reader.read_i8()?);
            println!("3920 {:?}", reader.read_i8()?);
            println!("3921 {:?}", reader.read_i8()?);
            println!("3922 {:?}", reader.read_i8()?);
            println!("3923 {:?}", reader.read_i8()?);
            println!("3924 {:?}", reader.read_i8()?);

            println!("Tempo Note Length {:?}", reader.read_i8()?);
            println!("3926 {:?}", reader.read_i8()?);
            println!("Tempo Triplets/Dotted {:?}", reader.read_i8()?); // 0: Off 1: Triplets 2: Dotted
            println!("Expression Velocity On/Off {:?}", reader.read_i8()?);
            println!("Expression Velocity {:?}", reader.read_i8()?);
            println!("Expression Accent {:?}", reader.read_i8()?);
            println!("3931 {:?}", reader.read_i8()?);
            println!("3932 {:?}", reader.read_i8()?);
            println!("Expression Split {:?}", reader.read_i8()?);
            println!("406 {:?}", reader.read_i8()?);
            println!("406 {:?}", reader.read_i8()?);
            println!("Tempo Shuffle {:?}", reader.read_i8()?);
            println!("406 {:?}", reader.read_i8()?);

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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_fm8_preset_read() -> Result<(), Error> {
        let file = File::open("../../tests/patchdata/fm8/1.2.0.1010/000")?;
        FM8Preset::read(file)?;

        Ok(())
    }
}
