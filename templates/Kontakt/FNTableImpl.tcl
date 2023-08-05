proc FNTableImpl {} {
	set id [uint16 -hex "id"]
	if {$id != 0x4b} {
		error "FNTableImpl must have id 0x4b, found $id"
	}

	set length [uint32 "length"]
	set version [uint16 -hex "version"]


	if {$version != 0x02} { error "Unsupported FNTableImpl: v$version" }

	set fileCount [uint32 "fileCount"]

	for { set i 0 } { $i < $fileCount } { incr i } {
		set pathSegments [uint32 "pathSegments"]

		for { set i 0 } { $i < $pathSegments } { incr i } {
			uint8 "segmentType"
			set length [uint32 "len"]
			utf16 [expr $length * 2] "name"
		}
	}
}

FNTableImpl
