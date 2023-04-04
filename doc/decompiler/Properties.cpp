
#include <string>

bool NI::SOUND::Properties::getValue(const ushort* key, NI::GP::String& value) const
{
    std::u16string local_str = u"&DAT_0020978c"; // This might be an unknown or placeholder value.

    if (key != nullptr) {
        size_t key_length = 0;
        while (key[key_length] != 0) {
            key_length++;
        }
        local_str.assign(key, key_length);
    }

    const int* property_node = *reinterpret_cast<const int**>(this);
    if (property_node == reinterpret_cast<const int*>(this)) {
        return false;
    }

    uint local_str_len = local_str.length();
    do {
        uint property_key_len = *reinterpret_cast<const uint*>(property_node + 2);
        uint min_len = (local_str_len < property_key_len) ? local_str_len : property_key_len;
        uint idx = 0;

        if (min_len != 0) {
            do {
                ushort property_key_char = *(reinterpret_cast<const ushort*>(property_node[2]) + idx * 2);
                if (property_key_char != local_str[idx]) {
                    break;
                }
                idx++;
            } while (idx < min_len);
        }

        if (local_str_len == property_key_len) {
            value.assign(reinterpret_cast<const NI::GP::String*>(property_node + 3));
            return true;
        }

        property_node = reinterpret_cast<const int*>(*property_node);
    } while (property_node != reinterpret_cast<const int*>(this));

    return false;
}
