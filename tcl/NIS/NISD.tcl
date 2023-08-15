proc Data {} {
			section "Data" {
			set length [uint64 "length"]
			ascii 4	"domainID"
			set itemId [uint32 "itemID"]
			uint32 "version"

			switch $itemId {
				1 { sectionname "Item" }
				3		{ sectionname "BNISoundPreset" }
				4		{ sectionname "BNISoundHeader" }
				100 { sectionname "Bank" }
				101 { sectionname "Preset" }
				102 { sectionname "BankContainer" }
				103 { sectionname "PresetContainer" }
				104 { sectionname "BinaryChunkItem" }
				106 { sectionname "Authorization" }
				108 { sectionname "SoundInfoItem" }
				109 { sectionname "PresetChunkItem" }
				110 { sectionname "ExternalFileReference" }
				111 { sectionname "Resources" }
				112 { sectionname "AudioSampleItem" }
				113 { sectionname "InternalResourceReferenceItem" }
				114 { sectionname "PictureItem" }
				115 { sectionname "SubtreeItem" }
				116 { sectionname "EncryptionItem" }
				117 { sectionname "AppSpecific" }
				118 { sectionname "RepositoryRoot" }
				120 { sectionname "AutomationParameters" }
				121 { sectionname "ControllerAssignments" }
				122 { sectionname "Module" }
				123 { sectionname "ModuleBank" }
			}

			set endPos [expr [pos] - 20 + $length]

			if {$itemId == 1} {
				uint32 "data"
			} else {
				Data
			}

			set dataLength [expr $endPos - [pos]]
			if {$dataLength > 0} {
				bytes $dataLength "data"
			}
		}

}

proc Item {} {
	section "Item" {
		section -collapsed "NISoundHeader" {
			uint64 "length"
			uint32 "version"
			ascii 4	"magic"
			uint32 "unknown"
			uint32 "flags"
			bytes 16 "uuid"
		}

		Data

		uint32 "version"
		set numChildren [uint32 "numChildren"]

		section "children" {
			for { set i 0 } { $i < $numChildren } { incr i } {
				section "child" {
					uint32 "siblingIndex"
					ascii 4 "childDomainID"
					uint32 -hex "childItemID"
					Item
				}
			}
		}
	}
}

Item
