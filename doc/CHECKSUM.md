
void __cdecl
method.NI::SOUND::IntegrityCheck.computeChecksumSlow_void_const__unsigned_int_
          (uint noname_0, char *arg_ch, int32_t arg_10h)

{
    uint32_t uVar1;
    uint32_t uVar2;
    uint32_t uVar3;
    uint var_10h;

    // NI::SOUND::IntegrityCheck::computeChecksumSlow(void const*, unsigned int)
    uVar1 = 0;
    if (arg_10h != 0) {
        do {
            uVar3 = *arg_ch;
            uVar2 = uVar1 * 2 ^ 0x4c11db7;
            if ((uVar1 & 0x80000000) == (uVar3 & 1)) {
                uVar2 = uVar1 * 2;
            }
            uVar1 = uVar2 * 2 ^ 0x4c11db7;
            if ((uVar2 & 0x80000000) == (uVar3 & 2)) {
                uVar1 = uVar2 * 2;
            }
            uVar2 = uVar1 * 2 ^ 0x4c11db7;
            if ((uVar1 & 0x80000000) == (uVar3 & 4)) {
                uVar2 = uVar1 * 2;
            }
            uVar1 = uVar2 * 2 ^ 0x4c11db7;
            if ((uVar2 & 0x80000000) == (uVar3 & 8)) {
                uVar1 = uVar2 * 2;
            }
            uVar2 = uVar1 * 2 ^ 0x4c11db7;
            if ((uVar1 & 0x80000000) == (uVar3 & 0x10)) {
                uVar2 = uVar1 * 2;
            }
            uVar1 = uVar2 * 2 ^ 0x4c11db7;
            if ((uVar2 & 0x80000000) == (uVar3 & 0x20)) {
                uVar1 = uVar2 * 2;
            }
            uVar2 = uVar1 * 2 ^ 0x4c11db7;
            if ((uVar1 & 0x80000000) == (uVar3 & 0x40)) {
                uVar2 = uVar1 * 2;
            }
            uVar1 = uVar2 * 2 ^ 0x4c11db7;
            if ((uVar2 & 0x80000000) == (uVar3 & 0x80)) {
                uVar1 = uVar2 * 2;
            }
            arg_ch = arg_ch + 1;
            arg_10h = arg_10h + -1;
        } while (arg_10h != 0);
    }
    return;
}
