AuthoringApplicationInfo {
	int32_t* fn createVersionString(&self, a: int32_t, b: int32_t, c: int32_t, d: int32_t) {
		self[0] = a;
		self[1] = b;
		self[2] = c;
		self[3] = d;
		self[4] = 1;
	}

	bool readVersion(stream: &Stream, version_number: &VersionNumber) {
		u32 value_a = stream.readU32();

		if value_a != 1 {
			return false;
		}

		let version_string = stream.readWString();
		let version_string_ascii = version_string.toAscii();
		int128t let version_number = version_string_ascii.toVersionNumber();

		if (version_string_ascii & 1) != 0 {
		} \
	}
}
