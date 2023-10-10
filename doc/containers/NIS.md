# Native Instruments Sound Containers

The most modern repository/container format for almost all types of NI files.

Each [`NISound`] is like a mini database of sorts, and you can read these repositories with low
level structs (embedded [`Item`](crate::nisound::Item)s) or use high-level structs such as [`NISound`]. It is
recommended and much easier to use the latter unless you are dealing with filetypes still
undocumented by the library.

## Container Format

### Domains and IDs

Each Item (which could be thought of as a record in a document store db or an element in an xml document) is identified by two properties: its domain and ID. The ID is a u16 number and the domain, a [FOURCC](https://en.wikipedia.org/wiki/FourCC), allows for application specific objects to be defined without breaking the format.

The common domain is `nisd`, or Native Instruments Sound Domain. Other examples include `kon4` for Kontakt42 and `MAS2` for maschine. Most objects will be `nisd`.

The type is used to determine which children might be present and the structure of the Items properties.

## Structs and Objects

## Schemas

NIS Documents have patterns to them (schemas) and this is generally how a document is defined. There is no descriptor that directly defines what the archive contains, document types are defined by the first object directly under a `RepositoryRoot`.

You can use the `ni-tree` cli tool to view an NIS object structure. As an example, here is the output from a standard single instrument preset from the FM8 synthesizer:

```
> ni-tree preset.nfm8

NISound 1.3.0

RepositoryRoot
 Preset, Authorization, Item
  SoundInfoItem, Item
  ControllerAssignments, Item
  EncryptionItem, SubtreeItem, Item
```

A `RepositoryRoot` object lies at the top of the hierarchy. This object only contains a single child, which dictates its purpose. The `RepositoryRoot` has properties which define what version of NISound document this is, but nothing about its purpose or application. You can use the `ni-info` tool to read more about the properties of each object:

```
> ni-info preset.nfm8

RepositoryRoot:
  version:            NISound 1.3.0
  repository_magic:   0
  repository_type:    1
...
```

The object directly under the root is a `Preset`, and the preset contains three standard objects including `EncryptionItem`, whose property contains a `SubtreeItem`, which wraps the inner document containing the actual preset data.

This may sound complicated, and it is for a simple preset structure, and this is more evidence of Native Instruments over-engineered attempts at vendor lockin through complexity. But once fully reverse engineered and understood, it is manageable.

### Preset

By far the most common schema. Represents a single instrument preset, embedded inside another NISound container inside the `EncryptionItem` object.

### BNISoundPreset

Essentially the Kontakt specific version of `Preset`, as it includes a `BNISoundHeader` as a child. A special domain was created for Kontakt presets specifically as they require a special header (a legacy from the NKI days) to determine what Kontakt schema and instrument layout to use.

### AppSpecific

This defers the content to an application. One example is Kontakt, where an AppSpecific schema usually contains multi instruments.
