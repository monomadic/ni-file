# ResultType

0 = SUCCESS
1 = SUCCESS_WITHIN_DEPTH
2 = CORRUPT_DATA
3 = CORRUPT_ITEM_FRAME
4 = CORRUPT_ITEM_HEADER
5 = ACTION_CANCELLED
6 = VERSION_MISMATCH
7 = UNRESOLVED_REFS
8 = INTERNAL_ERROR
9 = FILE_INACCESSIBLE
10 = EXTERNAL_ITEM_ERROR
11 = UNKNOWN_ITEM
12 = ITEM_INCOMPLETE
13 = DECOMPRESSION_FAILED
14 = CHECKSUM_MISMATCH_ERROR
15 = DECRYPTION_FAILED
> 15 = UNDEFINED(n)


int32_t NI::SOUND::Item::read(int32_t* arg1, int32_t* arg2)
{
    void var_94;
    void* var_ac = &var_94;
    NI::SOUND::ReadContext::ReadContext();
    int32_t eax = NI::SOUND::Item::read(arg1, arg2, &var_94);
    void* var_ac_2 = &var_94;
    NI::SOUND::ReadContext::~ReadContext();
    return eax;
}
