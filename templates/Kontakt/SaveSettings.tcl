proc SaveSettings {} {
		sectionname "SaveSettings"
		set length [uint32 "length"]
		set isDataStructured [int8 "isDataStructured"]
		set version [uint16 -hex "version"]
		uint32 "BFNTrns"
		int32 "BFNOrig"
		int32 "?"
		int8 "?"
		int8 "?"
		int8 "?"
}
