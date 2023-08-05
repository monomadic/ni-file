proc GroupList {} {
	section "GroupList" {
		set id [uint16 -hex "id"]
		if {$id != 0x33} {
			error "GroupList must have id 0x33, found $id"
		}

		set length [uint32 "length"]
		set length [uint32 "length"]
		set isDataStructured [int8 "isDataStructured"]

		if {$isDataStructured == 1} {
			set version [uint16 -hex "version"]

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
				while {[pos] < $maxPos} {
					include "StructuredObjectSimple.tcl"
				}
			}
		}
	}
}

GroupList
