proc Block {} {
	section "Block" {
		set magic [hex 4 "magic"]
		bytes 10 "header"

		switch $magic {
			0x54AC705E {
				set numItems [uint32 "numItems"]
				uint32 "padding"

				for { set i 0 } { $i < $numItems } { incr i } {
					section "BlockItem" {
						set itemLength [uint16 "itemLength"]
						uint32 "referencePointer"
						set referenceType [uint16 "referenceType"]
						bytes [expr $itemLength - 8] "content"
						# switch $referenceType {
						# 	1 { Block }
						# }
					}
				}
			}

			# 0x0AF8CC16 {
			# 	set numItems [uint32 "numItems"]
			# 	uint8 "padding"
			#
			# 	set len [uint32 "len"]
			# 	uint32 "?"
			# 	uint32 "?"
			# 	bytes $len "data"
			# }
			#
			# 0xFA05E92A {
			# 	set len [uint32 "len"]
			# 	bytes 4 "padding"
			# 	bytes $len "data"
			# }
			#

			default {
				#error "unknown chunk $magic"
			}
		}
	}
}

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

	uint32 "u32?"
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

Block
Block
Block
# Block
# Block
# Block
# Block
