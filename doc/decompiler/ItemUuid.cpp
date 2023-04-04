ssize_t __thiscall
NI::SOUND::ItemUuid::write(ItemUuid *this,Stream *stream,void *param_2,size_t param_3)

{
  GP::Stream::writeU32(stream,this->field_u32);
  GP::Stream::writeU16(stream,this->field1_0x4);
  GP::Stream::writeU16(stream,this->field2_0x6);
  GP::Stream::writeU8(stream,this->field3_0x8);
  GP::Stream::writeU8(stream,this->field4_0x9);
  GP::Stream::writeU8(stream,this->field5_0xa);
  GP::Stream::writeU8(stream,this->field6_0xb);
  GP::Stream::writeU8(stream,this->field7_0xc);
  GP::Stream::writeU8(stream,this->field8_0xd);
  GP::Stream::writeU8(stream,this->field9_0xe);
  GP::Stream::writeU8(stream,this->field10_0xf);
  return 1;
}
