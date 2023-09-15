# Native Instruments File Formats.

All information in this document was obtained through legal means protected by Fair Use laws.

It might be worth reading [TERMINOLOGY.md](TERMINOLOGY.md) which describes terms used throughout the code and this documentation.

## Ecosystem

There is some overlap between the many file formats NI has used across the decades. In general, one or more presets are wrapped in a container format.

Kontakt originally had its own container format, called NKS (Native Instruments Kontakt Sound), and embedded inside was a compressed XML document with the actual preset data. Eventually the preset data was replaced with a custom RIFF-like binary chunk document.

Eventually all NI products used one single format, NIS (Native Instruments Sound), sometimes also referred to as NISound. This is a a slightly more sturdy (though still terrible) binary chunk format similar to EBML. Kontakt still used the same chunk format as previously, without the NKS container.

## Containers

- [NISound](containers/NIS.md) Generic container format for all modern NI types.
- [FileContainer](containers/FileContainer.md) Kontakt monolith (Kontakt 5.1+)
- [NKS](containers/NKS.md) Kontakt Instrument container (Kontakt 1-4).

## Presets

Embedded presets look like IFF files. They start with a 16-bit id followed by a 32-bit length, followed by the data.

- [Kontakt](presets/Kontakt.md) Kontakt 4.22+

## Other
