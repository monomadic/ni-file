# filename groups are:
# - other
# - sample
# - special

proc BFileName {} {
	section "BFileName" {
		set pathSegments [uint32 "pathSegments"]

		if {$pathSegments > 0} {
		for { set i 0 } { $i < $pathSegments } { incr i } {
			set segmentType [uint8 "segmentType"]
			switch $segmentType {
				1 {
					# drive letter
					# uint8 "?"
					# uint16 "?"
					set length [uint32 "len"]
					utf16 [expr $length * 2] "name"
				}
				2 {
					set length [uint32 "len"]
					utf16 [expr $length * 2] "name"
				}
				4 {
					set length [uint32 "len"]
					utf16 [expr $length * 2] "name"
				}
				5 {
					#uint8 "?"
					# set length [uint32 "len"]
					# utf16 [expr $length * 2] "name"
				}

				6 {
					uint8 "?"
					set length [uint32 "len"]
					utf16 [expr $length * 2] "name"
				}
				default {
					exit "unknown segmentType {$segmentType}"
				}
			}
		}

		}
	}
}

proc FNTableImpl {} {
	set id [uint16 -hex "id"]
	if {$id != 0x4b} {
		error "FNTableImpl must have id 0x4b, found $id"
	}

	set length [uint32 "length"]
	set version [uint16 -hex "version"]

	if {$version != 0x02} { error "Unsupported FNTableImpl: v$version" }

	set tableCount [uint32 "tableCount"]

	section "absolutePath" {
		BFileName
	}

	set fileCount [uint32 "fileCount"]

	section "filenameTable" {
		for { set i 0 } { $i < $fileCount } { incr i } {
			BFileName
		}
	}

	section "filenameTable" {
		for { set i 0 } { $i < $fileCount } { incr i } {
			uint16 "?"
			uint16 "?"
			uint16 "?"
			uint16 "?"
		}
	}

	section "filenameTable" {
		for { set i 0 } { $i < $fileCount } { incr i } {
			uint16 "?"
			uint16 "?"
		}
	}


	section "resourcesTable" {
		set fileCount [uint32 "fileCount"]
		section "filenameTable" {
			for { set i 0 } { $i < $fileCount } { incr i } {
				BFileName
			}
		}
	}

	uint16 "?"
}

FNTableImpl
