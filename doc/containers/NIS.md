# Native Instruments Sound Containers

The most modern repository/container format for almost all types of NI files.

Each [`NISound`] is like a mini database of sorts, and you can read these repositories with low
level structs (embedded [`Item`](crate::nisound::Item)s) or use high-level structs such as [`NISound`]. It is
recommended and much easier to use the latter unless you are dealing with filetypes still
undocumented by the library.

## Container Format

## Structs and Objects

## Schemas

Schemas are defined by the first object under a `RepositoryRoot`. For example, encountering a `Preset` object, there are a few configurations for its child objects, that embed a preset item.

### Preset

By far the most common schema. Represents a single instrument preset, embedded inside another NISound container inside the `EncryptionItem` object.

### BNISoundPreset

Essentially the Kontakt specific version of `Preset`, as it includes a `BNISoundHeader` as a child.

### AppSpecific

This defers the content to an application. One example is Kontakt, where an AppSpecific schema usually contains multi instruments.
