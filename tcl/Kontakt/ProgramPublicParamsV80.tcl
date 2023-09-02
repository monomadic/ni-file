proc wstring {name} {
	set length [uint32]
	if {$length > 0} {
		utf16 [expr $length * 2] $name
	}
}

proc float64 {name} {
	uint64 "$name (float64)"
}

wstring "name"
float64 "num_bytes_samples_total"
int8 "transpose"
float "volume"
float "pan"
float "tune"
uint8 "low_velocity"
uint8 "high_velocity"
uint8 "low_key"
uint8 "high_key"
int16 "default_key_switch"
int32 "dfd_channel_preload_size"
int32 "library_id"
uint32 "fingerprint"
uint32 "loading_flags"
int8 "group_solo"
int32 "cat_icon_idx"
wstring "instrument_credits"
wstring "instrument_author"
wstring "instrument_url"
int16 "instrument_cat1"
int16 "instrument_cat2"
int16 "instrument_cat3"
