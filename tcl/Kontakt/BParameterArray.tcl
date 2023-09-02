proc BParameterArray8 {} {
	set id [uint16 -hex "id"]
	if {$id != 0x3a} {
		error "BParameterArray must have id 0x3a, found $id"
	}

	set length [uint32 "length"]
	set isDataStructured [int8 "isDataStructured"]
	set version [uint16 -hex "version"]

	if {$version == 0x10} { error "unsupported version 0x10" }

	for { set i 0 } { $i < 8 } { incr i } {
		set hasItem [int8 "hasItem"]
		if { $hasItem } {
			include "Kontakt/StructuredObjectSimple.tcl"
		}
	}
}

BParameterArray8
