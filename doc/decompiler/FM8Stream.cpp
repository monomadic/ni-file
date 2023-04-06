
void FM8Stream::readStdStringHelper(Stream *stream, basic_string *str) {
  ulong length;
  void *buffer;
  int foundIndex;

  length = stream->readS32();
  if ((int)length < 10000) {
    int bufferSize = length + 1;
    buffer = new char[bufferSize];
    stream->readRaw(buffer, length);
    ((char *)buffer)[length] = 0;
    str->assign((char *)buffer);
    delete[] buffer;

    if (stream->field5_0x8 == 1) {
      foundIndex = str->find("\r\n", 0, 2);
      while (foundIndex != -1) {
        str->replace(foundIndex, 2, "\n");
        foundIndex = str->find("\r\n", 0, 2);
      }

      basic_string *current, *end;
      current = (((byte)*str) & 1) == 0 ? str + 1 : *(basic_string **)(str + 8);

      while (true) {
        uint strLen;
        end = (((byte)*str) & 1) == 0 ? str + 1 : *(basic_string **)(str + 8);
        strLen = (((byte)*str) & 1) == 0 ? (uint)((byte)*str >> 1) : *(uint *)(str + 4);

        if (current == end + strLen) break;

        byte value = (byte)*current;
        if (value < 0xD6) {
          if (value == 0xC4) {
            *current = (basic_string)0x80;
          }
        } else if (value < 0xDC) {
          if (value == 0xD6) {
            *current = (basic_string)0x85;
          }
        } else if (value < 0xF6) {
          if (value == 0xDC) {
            *current = (basic_string)0x86;
          } else if (value == 0xDF) {
            *current = (basic_string)0xA7;
          } else if (value == 0xE4) {
            *current = (basic_string)0x8A;
          }
        } else {
          if (value == 0xF6) {
            *current = (basic_string)0x9A;
          } else if (value == 0xFC) {
            *current = (basic_string)0x9F;
          }
        }
        current++;
      }
    }
  }
  return;
}
