int K4PatchLib::BFile::Open(BFileName *filename, bool param_2, bool param_3, ulong fiveEightValue, int *param_5, int *param_6) {
    int iVar5;
    int iVar6;

    BFileName local_24;
    BFileName local_3c;
    BFileName local_50;

    iVar5 = BFileName::GetPathType(filename);
    if (iVar5 == 1) {
        if (param_5 == nullptr) {
            iVar5 = Open_OS(this, filename, param_2, param_3, fiveEightValue);
            return iVar5;
        }
        *param_5 = 0;
        iVar5 = Open_OS(this, filename, param_2, param_3, fiveEightValue);
        if (iVar5 != 0) {
            return iVar5;
        }
        iVar5 = this->field0_0x0->function_10(this);
        *param_5 = iVar5;
        return 0;
    }
    iVar6 = BFileName::GetPathType(filename);
    iVar5 = (uint)(iVar6 == 2) * 9;
    if (iVar6 != 2) {
        return iVar5;
    }
    if (param_2) {
        return iVar5;
    }
    BFileName::BFileName(&local_24);
    BFileName::BFileName(&local_3c);
    // ... (more code, omitted for brevity)

    return iVar5;
}
