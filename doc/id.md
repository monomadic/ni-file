segment ids appear to be bitshifted or flags.

0000 0001 termination dsins
0000 0011 kontakt 4KIN version block
0000 0100 kontakt 4KIN "Kontakt (null)" signature block?
0111 0110 almmost all kontakt files topmost header
0110 1010 -> second inner dsin of ^
0110 1100 metadata for library (kontakt)
0111 0100 kontakt preset, compressed segment

method.NI::SOUND::Preset.isPresetChunkEncrypted___const
method.NI::SOUND::EncryptionTraits.getEncryptionKey_NI::PA::SNPID_const_
NI::GP::HASH::getMD5Binary(void const, unsigned long)

.getDomainID()
  - returns 'DSIN' - NI Sound Domain

starts with RepositoryRoot

afl~NI::SOUND
  BinaryChunkItem
  BankContainer
  SoundInfo2
  SoundInfoItem
    NI::SOUND::SoundInfoItem::getItemID() -> const;
      0x6c ; 'l' ; 108
  PresetChunkItem
  PresetContainer
  Preset
  PictureItem
    .getItemID() -> 0x72 ; 'r' ; 114
    .getDomainID() -> 'DSIN'
  ModuleBank
  EncryptionItem
  SubtreeItem
  GenericItem_

  .fastlz_compress
  .fastlz1_compress
  .fastlz2_compress
  .fastlz_decompress

afl~sym.NI::SOUND

