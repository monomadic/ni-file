proc ZoneDataV95 {} {
	sectionname "ZoneDataV95"
	int32 "sampleStart"
	int32 "sampleEnd"
	int32 "sampleStartModRange"
	int16 "lowVelocity"
	int16 "highVelocity"
	int16 "lowKey"
	int16 "highKey"
	int16 "fadeLowVelocity"
	int16 "fadeHighVelocity"
	int16 "fadeLowKey"
	int16 "fadeHighKey"
	int16 "rootKey"
	float "zoneVolume"
	float "zonePan"
	float "zoneTune"
	int32 "filenameId"
	int32 "sampleDataType"
	int32 "sampleRate"
	int8 "numChannels"
	int32 "numFrames"
	int32 "reserved1"
	int32 "reserved2"
	int32 "rootNote"
	float "tuning"
	int8 "reserved3"
	int32 "reserved4"
}

proc ZoneDataV98 {} {
	section "ZoneDataV98" {
		int32 "sampleStart"
		int32 "sampleEnd"
		int32 "sampleStartModRange"
		int16 "lowVelocity"
		int16 "highVelocity"
		int16 "lowKey"
		int16 "highKey"
		int16 "fadeLowVelocity"
		int16 "fadeHighVelocity"
		int16 "fadeLowKey"
		int16 "fadeHighKey"
		int16 "rootKey"
		float "zoneVolume"
		float "zonePan"
		float "zoneTune"
		int32 "filenameId"
		int32 "sampleDataType"
		int32 "sampleRate"
		int8 "numChannels"
		int32 "numFrames"
		int32 "reserved1"
		int32 "rootNote"
		float "tuning"
		int8 "reserved3"
		int32 "reserved4"
	}
}

proc ZoneData {} {
	section -collapsed "ZoneData" {
		set isDataStructured [int8 "isDataStructured"]
		set version [uint16 -hex "version"]

		set privateDataLength [uint32 "privateDataLength"]
		if {$privateDataLength > 0} {
			bytes $privateDataLength "privateData"
		}

		set pubDataLength [uint32 "pubDataLength"]

		if {$version < 0x96} {
			ZoneDataV95
		}

		if {$version >= 0x96 && $version < 0x99} {
			ZoneDataV98
		}

		if {$version >= 0x99} {
			error "Unsupported ZoneData v$version"
		}

		set childrenDataLength [uint32 "childrenDataLength"]
		if {$childrenDataLength > 0} {
			bytes $childrenDataLength "childrenData"
		}
	}
}

ZoneData
