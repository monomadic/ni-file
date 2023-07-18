use crate::{read_bytes::ReadBytesExt, Error, NIFileError};

pub struct FM8EffectSettings;

impl FM8EffectSettings {
    pub fn print<R: ReadBytesExt>(mut reader: R) -> Result<(), Error> {
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

        Ok(())
    }
}
