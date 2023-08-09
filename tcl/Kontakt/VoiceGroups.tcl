proc VoiceGroups {} {
	set isDataStructured [int8 "isDataStructured"]
	set version [uint16 -hex "version"]

	if {$version == 0x60} {
		VoiceLimit
	}

	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
}

proc VoiceLimit {} {
	set length [uint32 "len"]
	utf16 [expr $length * 2] "name"

	int16 "kill mode"
	uint8 "prefer released"
	int32 "max_num_voices"
	int32 "ms_fade_time"
	int32 "exclusion_group"
}
