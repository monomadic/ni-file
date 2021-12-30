# Native Instruments File Formats

```
Tag   Type
hsin  Native Instruments Segment Header
DSIN  Native Instruments Segment Data
4KIN  Native Instruments Kontakt 4
```

## Typical Block Layout

### Kontakt 4
hsin // file header?
  dsin [type=118]
hsin // file type information
  4kin [type=3] // denoting a contact file?
hsin // library metadata tags
  dsin [type=108]
hsin
  dsin [type=121]
hsin
  dsin [type=116]
hsin // footer, metadata?
  4kin [type=4]

### Massive
hsin // file header?
  dsin [type=118]
hsin
  dsin [type=101]
hsin
  dsin [type=108]
hsin
  dsin [type=121]
hsin
  dsin [type=116]
