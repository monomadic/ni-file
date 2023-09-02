proc BInsertBus {} {
	set id [uint16 -hex "id"]
	if {$id != 0x45} {
		error "BInsertBus must have id 0x45, found $id"
	}

	set length [uint32 "length"]
	set isDataStructured [int8 "isDataStructured"]
	if {$isDataStructured == 1} {
		set version [uint16 -hex "version"]

		# if {$version != 0x50} { error "Unsupported BParFX: v50" }

		section "privateData" {
			set privateDataLength [uint32 "privateDataLength"]
			if {$privateDataLength > 0} {
				bytes $privateDataLength "privateData"
			}
		}

		section "publicData" {
			set publicDataLength [uint32 "publicDataLength"]
			if {$publicDataLength > 0} {
				bytes $publicDataLength "publicData"
			}
		}

		section "children" {
			set chunkSize [uint32 "length"]
			set maxPos [expr [pos] + $chunkSize]
			# while {[pos] < $maxPos} {
			# 	#include "Kontakt/StructuredObjectSimple.tcl"
			# }
		}
	}
}

BInsertBus
