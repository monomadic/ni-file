// SoundInfoItem
// children: none
// psuedocode of method.NI::SOUND::GenericItem_NI::SOUND::SoundInfoItem_.readItem_NI::GP::Stream__NI::SOUND::ReadContext_
// ..
// if (struct_base+8 == "DSIN" && struct_base+12 == 108) break;
// else { move struct_base + 20 bytes }
//
// psuedo of method.NI::SOUND::Preset.readItem_NI::GP::Stream__NI::SOUND::ReadContext_
// u32
// bool
// u32
// VersionNumber // method.NI::SOUND::AuthoringApplicationInfo.readVersion_NI::GP::Stream__NI::PA::VersionNumber_
//   u32
//   WString

// +20b SoundInfo
// - name
// - vendor
// - author
