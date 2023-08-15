# "product-bindings"
# "watermark"
# "authorization-level"
section "Authorization" {
		set version [uint32 -hex "version"]
		if { $version == 1 } {
			set a [uint32 "?"]
			if { $a == 1 } {
				set b [uint32 "?"]
				section "LicenseInfo" {
					set c [uint32 "?"]
					if { $c == 1 } {
						section "SNPID" {
							set strlen [uint32 "strlen"]
							if { $strlen > 0 } {
								ascii [expr $strlen * 2] "wstring"
							}
						}
					}
				}
			}
		}
}
