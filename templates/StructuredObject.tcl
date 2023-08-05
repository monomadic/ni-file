proc chunk {} {
	section "Chunk" {
		set id [uint16 -hex "id"]
		set length [uint32 "length"]

		switch -- $id {
			6 {
				sectionname "BParScript"
				object $length
			}
			40 {
				sectionname "BProgram"
				object $length
			}
			50 {
				sectionname "VoiceGroups"
				bytes $length "data"
			}
			51 {
				sectionname "GroupList"
				bytes $length "data"
			}
			52 {
				sectionname "ZoneList"
				bytes $length "data"
			}
			58 {
				sectionname "BParameterArraySer<BParFX,8>"
				object $length
			}
			69 {
				sectionname "BInsertBus"
				object $length
			}
			71 {
				sectionname "SaveSettings"
				object $length
			}
			75 {
				sectionname "FNTableImpl"
				bytes $length "data"
			}
			78 {
				sectionname "QuickBrowseData"
				object $length
			}
			default {
				bytes $length "data"
			}
		}
	}
}

proc object { length } {
	set isDataStructured [int8 "isDataStructured"]

	if {$isDataStructured == 1} {
		uint16 -hex "version"

		section "privateData" {
			set privateDataLength [uint32 "privateDataLength"]
			if {$privateDataLength > 0} {
				bytes $privateDataLength "privateData"
			}
		}

		section "publicData" {
			set publicDataLength [uint32 "publicDataLength"]
			bytes $publicDataLength "publicData"
		}

		section "children" {
			set chunkSize [uint32 "length"]
			set maxPos [expr [pos] + $chunkSize]
			while {[pos] < $maxPos} {
				chunk
			}
		}
	} else {
		bytes [expr $length-1] "rawData"
	}
}

while {![end]} { chunk }
