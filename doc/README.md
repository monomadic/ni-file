# Native Instruments File Formats.

All information in this document was obtained through legal means protected by Fair Use laws.

## Ecosystem

There is some overlap between the many file formats NI has used across the decades. In general, one or more presets are wrapped in a container format.

Kontakt originally had its own container format, called NKS (Native Instruments Kontakt Sound), and embedded inside was a compressed XML document with the actual preset data. Around Kontakt v4.22, the preset data was replaced with a custom RIFF-like binary chunk document, keeping the outer NKS container format.

Eventually all NI products (Kontakt 5.1+, etc) introduced one single container: NIS (Native Instruments Sound), sometimes also referred to as NISound. This is a a slightly more sturdy (though still terrible) binary chunk format similar to EBML. The reasoning behind this was likely when NI introduced global databases for their entire family of applications and they wanted metadata to be searchable regardless of the application the preset was intended for. NKS was replaced with NIS, but the internal preset format remains the same up until the current version of Kontakt.

To make things more complicated (which appears to be the NI way), the NIS container format does not support embedding multiple files at once. So a radically new type, a FileContainer, is used only in these cases. In the UI of Kontakt however, they refer to these as Monoliths - a named already used for a previous container format that is part of NKS. Pretty neat right?

Because of the complexity of this ecosystem, this library intends to make these files easier to deal with, for the cases where the content is well known. NIFile is a wrapper structure designed to do just that.

## Container Formats

- [NISound](containers/NIS.md) Generic container format for all modern NI types.
- [FileContainer](containers/FileContainer.md) Kontakt monolith (Kontakt 5.1+)
- [NKS](containers/NKS.md) Kontakt Instrument container (Kontakt 1-4).

## Presets

Embedded presets look like IFF files. They start with a 16-bit id followed by a 32-bit length, followed by the data.

- [Kontakt](presets/Kontakt.md) Kontakt 4.22+

## Other
