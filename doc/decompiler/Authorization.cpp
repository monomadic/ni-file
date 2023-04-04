void __thiscall NI::SOUND::Authorization::importProperties(Authorization *this,Properties *props) {
  uint authLevel;

  LicenseInfo::importProperties(this->licenseInfo,props);
  Properties::getValue(props,"authorization-level",&authLevel);
  this->authLevel = authLevel;
  Item::importProperties((Properties *)this);
  return;
}

undefined __thiscall NI::SOUND::Authorization::addProductBinding(Authorization *this,SNPID pid)
{
  undefined result;

  this->authLevel = 1;
  result = LicenseInfo::addProductBinding(this->licenseInfo,&pid);
  return result;
}
