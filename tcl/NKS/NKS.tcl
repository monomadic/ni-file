proc NKS {} {
	set magic [hex 4 "magic"]

	switch $magic {
		0x5EE56EB3 {
			include "NKS/BPatchHeaderV1.tcl"
			bytes eof "zlibCompressedData"
		}

		0x1290A87F {
			# note: this is decompressed length in v2
			set zlibLength [uint32 "zlibLength"]
			set headerVersion [uint16 -hex "headerVersion"]

			switch $headerVersion {
				256 { include "NKS/NKSv2.tcl" }
				272 { include "NKS/NKSv42.tcl" }
				default { error "invalid headerversiom $headerVersion" }
			}
		}
		default {
				error "NKS magic must be 0x1290A87F, found $magic"
		}
	}
}

section "NKS" {
	NKS
}
