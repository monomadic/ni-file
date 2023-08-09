# sampleFileName
# sampleTimeStamp
# sampleContainerOfs

proc FNTablePreK51 {} {
	set id [uint16 -hex "id"]
	if {$id != 0x3d} {
		error "FNTablePreK51 must have id 0x3d, found $id"
	}

	set length [uint32 "length"]
	set u [uint32 "unknown_u32"]
	set fileCount [uint32 "fileCount"]

	section "fileSegment" {
		if {$u == 1} {
			uint8 "?"
			uint8 "segmentType"
			set length [uint32 "strlen"]
			utf16 [expr $length * 2] "pathSegment"
			uint8 "?"
			uint8 "?"
			uint16 "?"
			uint32 "?"
			uint8 "?"
			uint8 "?"
		} else {
			uint32 "?"
		}
	}

	section "fileSegment" {
		uint8 "segmentType"
		set length [uint32 "strlen"]
		utf16 [expr $length * 2] "pathSegment"
	}

	section "fileSegment" {
		uint8 "segmentType"
		set length [uint32 "strlen"]
		if { $length > 0 } {
			utf16 [expr $length * 2] "pathSegment"
		}
	}

	# section "fileSegment" {
	# 	uint8 "segmentType"
	# 	set length [uint32 "strlen"]
	# 	if { $length > 0 } {
	# 		utf16 [expr $length * 2] "pathSegment"
	# 	}
	# }

	# uint8 "segmentType"
	# uint16 "segmentType"
	# uint16 "segmentType"
	# uint16 "segmentType"
	# uint16 "segmentType"


	# section "fileSegment" {
	# 	uint8 "segmentType"
	# 	set length [uint32 "strlen"]
	# 	utf16 [expr $length * 2] "pathSegment"
	# }

	# for { set i 0 } { $i < $fileCount } { incr i } {
	# 	section -collapsed "file" {
	# 		set pathSegments [uint32 "pathSegments"]
	#
	# 		for { set s 0 } { $s < $pathSegments } { incr s } {
	# 			uint8 "segmentType"
	# 			set length [uint32 "len"]
	# 			utf16 [expr $length * 2] "name"
	# 		}
	# 	}
	# }
	#
	#
	# for { set i 0 } { $i < $fileCount } { incr i } {
	# 	float "a"
	# 	float "b"
	# }
	#
	# float "c"
}

FNTablePreK51
