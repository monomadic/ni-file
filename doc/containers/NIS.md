# Native Instruments Sound Containers

The most modern repository/container format for almost all types of NI files.

Each [`NISound`] is like a mini database of sorts, and you can read these repositories with low
level structs (embedded [`Item`](crate::nisound::Item)s) or use high-level structs such as [`NISound`]. It is
recommended and much easier to use the latter unless you are dealing with filetypes still
undocumented by the library.

## ItemHeader
