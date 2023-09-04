proc ZoneList {} {
	section "ZoneList" {
		# set id [uint16 -hex "id"]
		# if {$id != 0x34} {
		# 	error "ZoneList must have id 0x34, found $id"
		# }

		set arrayLength [uint32 "arrayLength"]

		if { $arrayLength > 0 } {

			for { set i 0 } { $i < $arrayLength } { incr i } {
				section "ZoneData" {
					uint32 "?"
					include "Kontakt/StructuredObjectRaw.tcl"
				}
			}
		}
	}
}

ZoneList
