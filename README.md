# Native Instruments File Format

Native Instruments file format support for rust. Very basic support at the moment, mostly surrounding the container format (called a Container).

Anyone who wants to do this with me, please get in touch. I'm on telegram at @deathdisco.

## Progress

After several years doing this part time, the container format is around 80% reversed, not all of this is reflected in the code. Currently you can:

- [x] read container metadata (NISound version, App type etc)
- [x] search / traverse container hierarchy with inspection
- [x] access inner presets (the actual raw preset data of each app)

More is coming very shortly, please check the examples directory for usage.


## FAQ

Q: Can I extract .wav files from kontakt files?
A: Not **yet** but there is functionality in the code for extracting samples from monoliths.

Q: Can I extract any meaningful information besides library and file metadata from any preset types?
A: Not **yet** but this is coming very very soon. The container format took years to reverse engineer part-time but that was most of the tricky work done. This work also opens the door for other reversers to document simpler internal preset structures.

Q: Will there be write support?
A: Yes, eventually, but not until reading the majority of files are done.

Q: Are compressed samples/presets supported?
A: Yes, files mostly use FastLZ compression internally and tdecompression is supported by the library.

Q: Is decryption supported?
A: No, but this will be eventually implemented and supported through legal means.
