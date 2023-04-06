void __thiscall NI::SOUND::Bank::importProperties(Bank *this,Properties *properties) {
  Properties::getValue(properties,"bankname",(String *)&this->bankname);
  Item::importProperties((Properties *)this);
  return;
}
