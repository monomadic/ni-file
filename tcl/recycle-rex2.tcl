# REX Loop Recycle 2 File Format
#
# - audio appears to be DWOP (Differential Waveform Oppression) lossless data.
#		- this is why silence is compressed
#	- chunks are RIFF-like, padded to align on word boundaries (2 bytes).

big_endian

proc pad2 {size} {
  set padding [expr {$size % 2}]
  if {$padding > 0} {
    entry "zeroPadding" "" $padding
    move $padding
  }
}

proc str {name} {
	set length [uint32]
	if {$length > 0} {
		ascii $length $name
	}
}

proc chunk {} {
	section "Chunk" {
		set magic [ascii 4 "magic"]
		set length [uint32 "length"]

		switch $magic {
			"CAT " {
				sectionname "Category"
				category $length
			}

			"HEAD" {
				sectionname "REX2HeadChunk"
				bytes $length "data"
			}

			"CREI" {
				sectionname "REX2CreatorInfoChunk"
				str "name"
				str "copyright"
				str "url"
				str "email"
				str "description"
			}

			"GLOB" {
				uint32 "numSlices"
				uint16 "?"
				uint16 "?"
				uint16 "?"
				uint32 "?"
				uint32 "?"
				uint16 "?"
				uint16 "?"
			}

			"SLCE" {
				sectionname "REX2SliceChunk"
				uint32 "?"
				uint32 "?"
				uint8 "?"
				uint8 "?"
				uint8 "?"
			}

			"SINF" {
				sectionname "SoundInfo"
				uint8 "channels?"
				uint8 "?"
				uint32 "soundHz"
				uint32 "?"
				uint32 "?"
				uint16 "?"
				uint16 "?"

			}

			default {
				bytes $length "data"
			}
		}

		pad2 $length
	}
}

proc category { length } {
	set maxPos [expr [pos] + $length]
	set categoryType [ascii 4 "categoryType"]
	sectionname "Category: $categoryType"
	while {[pos] < $maxPos} {
		chunk
	}
}

chunk

# int SET_PREVIEW_TEMPO(void* this, Handle handle, int tempo) {
#   Handle this_00;
#
#   if (!Validate(this, handle)) {
#     return 0xca;
#   }
#
#   if (tempo < 20000 || tempo > 99999) {
#     return 0xd2;
#   }
#
#   this_00 = handle ^ 0xc0a44431;
#   if (this_00 == NULL) {
#     this_00 = handle;
#   }
#
#   SetTempo(this_00, tempo);
#
#   return 1;
# }
