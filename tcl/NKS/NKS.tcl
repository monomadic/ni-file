proc NKS {} {
	set magic [hex 4 "magic"]
	if {$magic != 0x1290A87F} {
		error "NKS magic must be 0x1290A87F, found $magic"
	}

	set zlibLength [uint32 "zlibLength"]
	set headerVersion [uint16 -hex "headerVersion"]

	switch $headerVersion {
		256 { include "NKS/NKSv2.tcl" }
		272 { include "NKS/NKSv42.tcl" }
		default { error "invalid headerversiom $headerVersion" }
	}
}

section "NKS" {
	NKS
}
