proc ZoneList {} {
	section "ZoneList" {
		# set id [uint16 -hex "id"]
		# if {$id != 0x34} {
		# 	error "ZoneList must have id 0x34, found $id"
		# }

		set length [uint32 "length"]

		if { $length > 0 } {
			set arrayLength [uint32 "arrayLength"]

			for { set i 0 } { $i < $arrayLength } { incr i } {
				include "Kontakt/ZoneData.tcl"
			}
		}
	}
}

ZoneList
