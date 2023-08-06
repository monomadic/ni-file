proc chunk {} {
	section "Chunk" {
		set id [hex 1 "id"]
		bytes 1
		set length [uint32 "length"]
		#bytes $length "data"

		switch $id {
			0x28 {
				#bytes $length "data"
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
# chunk
# chunk
