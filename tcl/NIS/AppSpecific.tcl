uint32 "version"
uint32 "AuthoringApplication"
set length [uint32 "len"]
utf16 [expr $length * 2] "name"
