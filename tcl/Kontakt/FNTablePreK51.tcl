# sampleFileName
# sampleTimeStamp
# sampleContainerOfs

# 20 bytes
proc BFileName {} {
	set numSegments [uint8 "numSegments"]
	BFileNameSegment

	# for { set i 0 } { $i < $numSegments } { incr i } {
	# 	BFileNameSegment
	# }
}

proc BFileNameSegment {} {
		set segmentType [uint8 "segmentType"]
		set length [uint32 "strlen"]
		if { $length > 0 } {
			utf16 [expr $length * 2] "pathSegment"
		}
}

proc FileName {} {
	section "FileName" {
		set segmentCount [uint8 "segmentCount?"]

		for { set i 0 } { $i < $segmentCount + 1 } { incr i } {
			section "fileSegment" {
				set segmentType [uint8 "segmentType"]

				switch $segmentType {
					0 {
						uint8 "?"
						uint8 "?"
					}
					2 {
						set length [uint32 "strlen"]
						if { $length > 0 } {
							utf16 [expr $length * 2] "pathSegment"
						}
					}
					4 {
						set length [uint32 "strlen"]
						utf16 [expr $length * 2] "pathSegment"
						uint8 "?"
						uint8 "?"
						uint16 "?"
						uint32 "?"
						uint8 "?"
						uint8 "?"
					}
				}
			}
		}
	}
}

proc FNTablePreK51 {} {
	set id [uint16 -hex "id"]
	if {$id != 0x3d} {
		error "FNTablePreK51 must have id 0x3d, found $id"
	}

	set length [uint32 "length"]

	set u [uint32 "version?"]
	set fileCount [uint32 "fileCount"]

	FileName
}

FNTablePreK51
