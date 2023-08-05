proc FNTablePreK51 {} {
	set id [uint16 -hex "id"]
	if {$id != 0x3d} {
		error "FNTablePreK51 must have id 0x3d, found $id"
	}

	set length [uint32 "length"]

	uint32 -hex "unknown"

	set fileCount [uint32 "fileCount"]


	for { set i 0 } { $i < $fileCount } { incr i } {
		section -collapsed "file" {
			set pathSegments [uint32 "pathSegments"]

			for { set s 0 } { $s < $pathSegments } { incr s } {
				uint8 "segmentType"
				set length [uint32 "len"]
				utf16 [expr $length * 2] "name"
			}
		}
	}


	for { set i 0 } { $i < $fileCount } { incr i } {
		float "a"
		float "b"
	}

	float "c"
}

FNTablePreK51
