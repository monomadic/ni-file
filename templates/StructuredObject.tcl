# proc SaveSettings { length, version } {
# 	# if $version == 0x10 {
# 	# 	error "Unsupported SaveSettings version $version"
# 	#
# 	# } else {
# 	# 	error "Unsupported SaveSettings version $version"
# 	# }
# }

set objectNames [dict create \
		0x28 "BProgram" \
		0x3d "FileNameListPreK51" \
	]

proc chunk {} {
	section "Chunk" {
		set id [uint16 -hex "id"]

		if {$id == 0x28} { sectionname "BProgram" }
		if {$id == 0x3a} { sectionname "BParameterArraySer<BParFX,8>" }
		if {$id == 0x3d} { sectionname "FileNameListPreK51" }

		if {$id == 0x47} {
			sectionname "SaveSettings"

			set length [uint32 "length"]

			set isDataStructured [int8 "isDataStructured"]
			set version [uint16 -hex "version"]
			#bytes [expr $length - 3]
			uint32 "BFNTrns"
			int32 "BFNOrig"
			int32 "?"
			int8 "?"
			int8 "?"
			int8 "?"
		} elseif {$id == 0x4b} {
			sectionname "FNTableImpl"

			set length [uint32 "length"]
			object $length
		} else {
			set length [uint32 "length"]
			object $length
		}
	}
}

proc object { length } {
	set isDataStructured [int8 "isDataStructured"]

	if {$isDataStructured == 1} {
		uint16 -hex "version"

		section "privateData" {
			set privateDataLength [uint32 "privateDataLength"]
			bytes $privateDataLength "privateData"
		}

		section "publicData" {
			set publicDataLength [uint32 "publicDataLength"]
			bytes $publicDataLength "publicData"
		}

		section "children" {
			set chunkSize [uint32 "length"]
			set maxPos [expr [pos] + $chunkSize]
			while {[pos] < $maxPos} {
				section "child" {
					set id [uint16 -hex "id"]
					set length [uint32 "length"]

					if {$id == 0x3a} { sectionname "BParameterArraySer<BParFX,8>" }

					set isDataStructured [int8 "isDataStructured"]
					if {$isDataStructured == 1} {
						# uint16 -hex "version"
						#
						# section "privateData" {
						# 	set privateDataLength [uint32 "privateDataLength"]
						# 	bytes $privateDataLength "privateData"
						# }
						#
						# section "publicData" {
						# 	set publicDataLength [uint32 "publicDataLength"]
						# 	bytes $publicDataLength "publicData"
						# }
						#
						# section "childData" {
						# 	set childDataLength [uint32 "childDataLength"]
						# 	bytes $childDataLength "childData"
						# }
						bytes [expr $length-1] "StructuredData"
					} else {
						bytes [expr $length-1] "rawData"
					}
				}
			}

			# set childrenDataLength [uint32 "childrenDataLength"]
			# #bytes $childrenDataLength "childrenData"
			#
			# # uint16 -hex "id"
			# # set length [uint32 "length"]
			# # object $length
			#
			# set maxPos [expr [pos] + $childrenDataLength]
			# while {[pos] < $maxPos} {
			# 	uint16 -hex "id"
			# 	set length [uint32 "length"]
			# # 	object $length
			# #
			# # 	# section "object" {
			# # 	# 	chunk
			# # 	# }
			# }
		}
	} else {
		bytes [expr $length-1] "rawData"
	}
}

# while { [chunk] } {}
while {![end]} { chunk }
