section "BPatchHeaderV2" {
	set headerMagic [hex 4 "headerMagic"]
	if {$headerMagic != 0x722A013E} {
		error "NKS headerMagic must be 0x722A013E, found $headerMagic"
	}

	set patchType [uint16 "patchType"]

	uint8 "patchVersionMinorC"
	uint8 "patchVersionMinorB"
	uint8 "patchVersionMinorA"
	uint8 "patchVersionMajor"

	ascii 4 "appSignature"

	unixtime32 "creationDate"

	uint32 "?"

	uint16 "numZones"
	uint16 "numGroups"
	uint16 "numInstruments"

	uint16 "u16?"
	uint16 "u16?"

	uint32 "patchType"
	uint8 "u8?"
	uint16 "u16?"
	uint32 "u32?"
	uint8 "u8?"
	uint32 "icon"

	#bytes 104 "strings"
	ascii 8 "author"
	ascii 3 "?"
	ascii 86 "url"
	ascii 7 "?"

	hex 4 "checksum?"
	uint32 "patchLevel?"
}

include "NKS/NKSv2Block.tcl"
