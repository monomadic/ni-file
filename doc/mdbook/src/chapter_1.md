# Native Instruments File Formats

This book documents and references the various file formats used by Native Instruments (NI) over the years. It is the result of a sole developer and years of painstaking research and reverse engineering.

## Format Evolution

Native Instruments has multiple file formats with some common features. Typically, these formats serve as containers for preset data.

### Kontakt and NKS

In its early versions, Kontakt used a proprietary container known as NKS (Native Instruments Kontakt Sound). This contained a compressed XML file storing the actual preset information. Starting from version 4.22, Kontakt switched from XML to a custom binary format resembling RIFF, while keeping the NKS container.

### Introduction of NIS

Later, NI standardized to a single container format, NIS (Native Instruments Sound), for all products (starting with Kontakt 5.1). NIS is similar to EBML and more robust than NKS. The switch likely aimed to facilitate metadata searching across all NI applications. Although NIS replaced NKS, the internal binary chunk-based preset format hasn't changed in the latest Kontakt versions, merely introducing new versions of the old chunk types.

### FileContainer and Monoliths

Although the Kontakt UI refers to monoliths interchangably between all versions of Kontakt, NKS supports monoliths but NIS does not - instead a new container was introduced, internally referred to as FileContainer.

### Single and Multi

Presets can be single-instrument or multi instrument.

## Purpose of NIFile Library

Given this ecosystem's complexity, the NIFile library aims to simplify interactions with these formats with a simple interface, but also provide access to the lower level, direct structures and complexities of the internal files when needed.

## Container Formats

- [NKS](./containers/NKS.md) Kontakt Instrument container (Kontakt 1-4).
- [NISound](containers/NIS.md) Generic container format for all modern NI types.
- [FileContainer](containers/FileContainer.md) Kontakt monolith (Kontakt 5.1+)

## Presets

Embedded presets look like IFF files. They start with a 16-bit id followed by a 32-bit length, followed by the data.

- [Kontakt](./kontakt.md) Kontakt 4.22+
- [FM8](presets/FM8.md) NI's DX7 emulator / FM synth

## Other

- [NCW](other/NCW.md) NI Compressed Wave
