bool FM7::Program::read(Program *this, Stream *stream) {
  uint uVar4, uVar11;
  undefined uVar2;
  char cVar3;
  int iVar10, iVar12;
  float10 fVar15;
  float *local_44, *local_40, *local_3c, *local_38, *local_34, *local_30, *local_2c, *local_28;

  uVar4 = stream->readU32();
  if (uVar4 < 0xd0) {
    if ((uVar4 != 0xcf) || (stream->readU32() != 0x000020bd)) {
      readStdStringHelper(stream, &this->name);
      readStdStringHelper(stream, &this->field_0x10);
      readStdStringHelper(stream, &this->field_0x1c);
      this[2].field_0x400 = stream->readU32();

      for (uVar11 = 0; uVar11 != 0x4c; uVar11++) {
        if (((uVar11 & 0xfffffffe) != 0xc) && (uVar11 != 3) && (uVar11 != 6)) {
          this->field_0x28[uVar11] = stream->readS8();
        }
      }

      for (uVar11 = 0; uVar11 != 0x23; uVar11++) {
        // The code inside this loop has been simplified and is not shown here for brevity.
      }

      // The code from this point onwards has been simplified and is not shown here for brevity.
    }
  }

  return true;
}
