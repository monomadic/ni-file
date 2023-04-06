
void K4PatchLib::BMonolith::MonolithWriter::WriteToDisk(MonolithWriter *instance, BFileName *fileName, BPatchHeaderV42 *patch, BPatchHeaderV2 *param_3) {
    BPatchHeaderV42 *localPatchHeader;
    int fileOpenStatus;
    undefined fileBuffer[8];
    int iStack_44, iStack_40;
    undefined2 uStack_3c;
    undefined4 uStack_38, uStack_34, uStack_30, uStack_2c;
    BFileName localFilename;

    localPatchHeader = (BPatchHeaderV42 *)param_3;
    if (patch == NULL) {
        localPatchHeader = patch;
    }
    instance->patchHeader = localPatchHeader;
    HItemFolder::CalculateFutureFilePos((HItemFolder *)&instance->itemFolder);

    fileBuffer[4] = 0;
    fileBuffer[0] = PTR_vtable_001fa388 + 8;
    BFileName::BFileName(&localFilename);

    fileBuffer[4] = 1;
    iStack_44 = 0;
    iStack_40 = -1;
    uStack_2c = 0;
    uStack_34 = 0;
    uStack_38 = 0xffffffff;
    uStack_3c = 0;
    uStack_30 = 2;

    fileOpenStatus = BFile::Open((BFile *)fileBuffer, fileName, true, true, 0x58585858, NULL, NULL);
    if (fileOpenStatus == 0) {
        if (patch != NULL) {
            BFile::Write((BFile *)fileBuffer, patch, (uint)param_3);
        }
        HItemFolder::Write((HItemFolder *)&instance->itemFolder, (BFile *)fileBuffer, instance->field4_0x10, instance->field0_0x0, instance->field1_0x4, instance->callback, instance->field3_0xc);
    }

    fileBuffer[0] = PTR_vtable_001fa388 + 8;
    (**(code **)(PTR_vtable_001fa388 + 0x1c))((BFile *)fileBuffer);
    BFileName::~BFileName(&localFilename);
    return;
}
