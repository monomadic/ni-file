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

Block
