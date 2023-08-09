section "RepositoryRoot" {
		set version [uint32 -hex "nisound-version"]
		if { $version == 1 } {
			hex 4 "repository-magic"
			uint32 "repository-version"
			uint32 "repository-type"

			section "referenced-repository" {
				set version [uint32 -hex "version"]
				if { $version == 1 } {
					uint32 "?"
					set strlen [uint32 "strlen"]
					if { $strlen > 0 } {
						ascii $strlen
					}

					set strlen [uint32 "strlen"]
					if { $strlen > 0 } {
						ascii $strlen
					}

					section "SNPID" {
						set strlen [uint32 "strlen"]
						if { $strlen > 0 } {
							ascii [expr $strlen * 2] "wstring"
						}
					}
				}

				section "referenced-item-uuid" {
					uint32 "?"
					uint16 "?"
					uint16 "?"
					uint8 "?"
					uint8 "?"
					uint8 "?"
					uint8 "?"
					uint8 "?"
					uint8 "?"
					uint8 "?"
					uint8 "?"
				}
			}
		}
}
