# Item

HSIN: Native Instruments Sound Header

- afl~readItem
- afl~read_NI

## 118 RepositoryRoot
- chunk: 58 bytes
- @method.NI::SOUND::RepositoryRoot.readItem_NI::GP::Stream__NI::SOUND::ReadContext_

### Magic
data + 36 bytes
0x0E701000 ` p  `
- method.NI::SOUND::RepositoryRoot.getRepositoryMagic___const

Headers (20 bytes)
u64
u32
u32
u32

> 106 Authorization
	u64
	u32
	u32
	u32
	u64
	u32
	u32
	u32

	iVar1 = u32
	if iVar1 == 1 {
		stack:+0x18 &= 0xfffffffffffffffe
	}

u32
u32
u32
u32

## 3 Item

> 101
