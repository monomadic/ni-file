
void K4PO::K4PL_ProgramContainer::dbgPrint() {
    std::cerr << "name: " << this->name << std::endl;
    std::cerr << "volume: " << this->volume << std::endl;
    std::cerr << "pan: " << this->pan << std::endl;
}

int K4PO::K4PL_ProgramContainer::getNumParams(void) {
  return 3;
}

void K4PO::K4PL_ProgramContainer::read(Stream *stream) {
    std::wstring tmp_name;

    tmp_name = stream->readWString();
    this->name.assign(tmp_name);

    this->volume = stream->readF32();
    this->pan = stream->readF32();
}
