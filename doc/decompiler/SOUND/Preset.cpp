
void NI::SOUND::Preset::exportProperties(Preset *this, Properties *param_1) {
    std::string ni_factory_flag_key = "ni_factory_flag";
    bool somebool = this->somebool;
    Properties::append<bool>(param_1, ni_factory_flag_key, somebool);

    std::string authoring_app_key = "authoring-app";
    unsigned int field30_0x24 = this->field30_0x24;
    Properties::append<unsigned int>(param_1, authoring_app_key, field30_0x24);

    std::string authoring_app_version_key = "authoring-app-version";
    std::string version_string = GP::VersionNumber::toString();
    Properties::append<std::string>(param_1, authoring_app_version_key, version_string);

    Authorization::exportProperties((Authorization *)this, param_1);
}
