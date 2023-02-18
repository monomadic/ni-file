# Terminology

- `Repository`
	- `Item`
		- `ItemHeader` ('hsin') // NI::SOUND::ItemHeader::write(NI::GP::Stream&)
			@ u64 FrameSize
			@ u32
			@ u32 DomainID 0x6e697368 'hsin'
			@ u32
			@ u32
			@ uuid method.NI::SOUND::ItemUuid.write_NI::GP::Stream__const
		- `ItemFrameStack`
			- `ItemFrame` (Size, DomainID, ItemID)
				@ u64 FrameSize
				@ u32 DomainID 0x4e495344 "DSIN"
				@ u32 ItemID
				@ u32 Version
				.getFrameSize()
				.isFrameForBase() { DomainID != 'DSIN' || ItemID != 1 }



- `ItemFrame`

- `ItemHeader` - the chunk before an Item (eg. preheader)
- `BNISoundHeader` 4KIN

- `DomainID` eg. 'NISD' 'nish'
- `ItemID` u8 id (eg 0x72; 'r'; 114 = PresetChunkItem etc)
