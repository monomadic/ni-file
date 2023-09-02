set version [int32 "version?"]
if {$version < 3} {
	int32 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	uint8 "?"
	float "?"
	float "?"
	uint8 "bool"
	uint8 "bool"
	int32 "?"
	float "?"
	int16 "?"
	int16 "?"
	int32 "?"
	int32 "?"
	int32 "?"
	uint8 "?"
	uint8 "bool"
	uint64 "?"
	section "array" {
		# SER::ARRAY::WriteSparse<>(param_1,(array *)&this->field_0x508,param_2);
		uint8 "bitmask1"
		uint8 "bitmask2"
	}
	float "?"
	float "?"
	uint8 "bool"
	int32 "?"
	float "?"
	float "?"
	int32 "?"
}


# void ReadSparse(Stream stream, array& arr, ReadContext* context) {
#
#   byte bitmask1 = stream.readU8();
#   byte bitmask2 = stream.readU8();
#
#   for (int i = 0; i < 16; i++) {
#     if (getBit(bitmask1, i) || getBit(bitmask2, i)) {
#       Read(stream, arr[i], context);
#     }
#     else {
#       // Mark array element as empty
#       arr[i].pointer = nullptr;
#       arr[i].id = 0xFFFF0000;
#       arr[i].count = 0xFFFF;
#       arr[i].flags = 0xFF;
#     }
#   }
# }
#
# The key points:
# It reads two bitmasks from the stream to determine which array elements are populated.
# It iterates over the array, reading populated elements from the stream and setting empty elements to default values.
# The bitmasks are used to check if an element is populated - it checks the ith bit.
# Empty elements are marked by setting pointer and id fields to sentinel values.
# So in summary, it is reading a sparsely populated array by first reading bitmasks to know which elements to read from the stream.
