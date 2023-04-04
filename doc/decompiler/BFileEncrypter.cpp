// The function takes a buffer and its length as input, and performs a bitwise XOR operation with a key to encrypt the buffer.
void K4PatchLib::BFileEncrypter::EncryptBuffer(BFileEncrypter *this, uchar *buffer, int length) {
  int keyBase = this->keyBase;

  if (keyBase != 0 && length > 0) {
    int keyOffset = this->keyOffset;

    while (length > 0) {
      *buffer ^= *(byte *)(keyBase + keyOffset % 0x10000);
      keyOffset++;
      this->keyOffset = keyOffset;
      length--;
      buffer++;
    }
  }
}

void K4PatchLib::BFileEncrypter::DecryptBuffer(BFileEncrypter *this, uchar *buffer, int length) {
  int keyBase = this->keyBase;

  if (keyBase != 0 && length > 0) {
    int keyOffset = this->keyOffset;

    while (length > 0) {
      *buffer ^= *(byte *)(keyBase + keyOffset % 0x10000);
      keyOffset++;
      this->keyOffset = keyOffset;
      length--;
      buffer++;
    }
  }
}

uint32_t K4PatchLib::BCheckPatchHeader::GetHeaderSizePreNIS(BFile *file) {
  int headerVersion = GetHeaderVersionPreNIS(file);
  uint32_t headerSize;

  switch (headerVersion) {
    case 1:
      headerSize = 0x24;
      break;
    case 2:
      headerSize = 0xAA;
      break;
    case 3:
      headerSize = 0xDE;
      break;
    default:
      headerSize = 0;
      break;
  }

  return headerSize;
}

bool K4PatchLib::BCheckPatchHeader::CheckPreV2(void *buffer, int bufferSize) {
  int fileHeaderVersion;
  bool isValidHeader;
  BFile fileMemory[21];

  BFileMem::BFileMem((BFileMem *)fileMemory);
  int openResult = BFileMem::OpenMem((BFileMem *)fileMemory, buffer, bufferSize, false);

  if (openResult == 0) {
    fileHeaderVersion = GetHeaderVersionPreNIS(fileMemory);
    isValidHeader = (fileHeaderVersion == 1);
  } else {
    isValidHeader = false;
  }

  BFileMem::~BFileMem((BFileMem *)fileMemory);
  return isValidHeader;
}

byte K4PatchLib::BCheckPatchHeader::GetHeaderVersionPreNIS(BFile *file) {
  uint32_t originalPosition;
  int iteration;
  uint magic;
  ushort local_2c;

  originalPosition = file->GetPosition(file);
  file->SetPosition(file, 0, 0);
  file->Read(file, &magic, 0x24);
  file->SetPosition(file, originalPosition, 0);
  iteration = 0;

  while (iteration <= 1) {
    if (magic == 0xa4d6e55a || magic == 0xab85ef01 || magic == 0xb36ee55e ||
        magic == 0x10874353 || magic == 0x74b5a69b || magic == 0x7fa89012) {
      if (local_2c < 0x100) {
        return 1;
      }
      return (0x10f < local_2c) ? 2 : 0;
    }

    magic = (magic << 24) | (magic >> 8);
    local_2c = (local_2c >> 8) | (local_2c << 8);
    iteration++;
  }

  return 0;
}





int32_t K4PatchLib::BCheckPatchHeader::GetHeaderVersionPreNIS(int32_t* arg1)
{
    int32_t originalPosition = arg1->GetPosition(arg1);
    arg1->SetPosition(arg1, 0, 0);
    int32_t magic;
    arg1->Read(arg1, &magic, 0x24);
    arg1->SetPosition(arg1, originalPosition, 0);
    int32_t iteration = 0;
    int32_t local_2c;
    int32_t result = 0;

    while (true)
    {
        if ((magic == 0x7fa89012) || (magic == 0x74b5a69b) || (magic == 0x10874353) ||
            (magic == 0xa4d6e55a) || (magic == 0xab85ef01) || (magic == 0xb36ee55e))
        {
            if (local_2c < 0x100)
            {
                result = 1;
            }
            else
            {
                result = (local_2c > 0x10f) ? 2 : 0;
            }
            break;
        }

        magic = (magic << 24) | (magic >> 8);
        local_2c = (local_2c << 8) | (local_2c >> 8);

        iteration++;
        if (iteration >= 2)
        {
					result = 0;
					break;
				}
	}

	return result;
	}

uint32_t K4PatchLib::BFile::GetCreatorID(BFileName *filename) {
    char toFSRefResult;
    short catalogInfoResult;
    uint32_t creatorID = 0x3f3f3f3f;
    FSRef fileFSRef[80];
    uint32_t catalogCreatorID;

    toFSRefResult = BFileName::ToFSRef(filename, fileFSRef);

    if (toFSRefResult != 0) {
        catalogInfoResult = _FSGetCatalogInfo(fileFSRef, 0x800, nullptr, &catalogCreatorID, nullptr, nullptr);

        if (catalogInfoResult == 0) {
            creatorID = catalogCreatorID;
        }
    }
    return creatorID;
}
