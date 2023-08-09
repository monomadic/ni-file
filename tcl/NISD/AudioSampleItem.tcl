# "sample-rate"
# "num-samples"
# "num-channels"

section "AudioSampleItem" {
		set version [uint32 -hex "version"]
		if { $version == 1 } {
			set version2 [uint32 -hex "version2"]
			if { $version2 == 1 } {
				uint32 "?"
				# AB::AudioBlock::load
			}

		}
}
