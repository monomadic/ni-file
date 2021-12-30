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



## Massive
<hsin size=2816 checksum=667104fc447a0d55cbb5c9b79fec660a unknown=(1, 1, 0)>
    <DSIN#FileHeader size=150>
    </DSIN#FileHeader>
    <pre-DSIN#Maybe( "Massive File") unknown=(1, 0, 0)>
        <hsin size=2606 checksum=d0d839644b0b674275099fb90330a851 unknown=(1, 1, 0)>
            <DSIN#Maybe( "Massive File") size=105>
            </DSIN#Maybe( "Massive File")>
            <pre-DSIN#Unknown(108) unknown=(1, 0, 0)>
                <hsin size=420 checksum=1c2a71d24a240942470608b1e1e3c132 unknown=(1, 1, 0)>
                    <DSIN#Unknown(108) size=372>
                    </DSIN#Unknown(108)>
                </hsin>
            </pre-DSIN#Unknown(108)>
            <pre-DSIN#Unknown(121) unknown=(1, 0, 2)>
                <hsin size=104 checksum=dd896a594df25087f61a0cbbaef37013 unknown=(1, 1, 0)>
                    <DSIN#Unknown(121) size=56>
                    </DSIN#Unknown(121)>
                </hsin>
            </pre-DSIN#Unknown(121)>
            <pre-DSIN#Unknown(116) unknown=(1, 0, 1)>
                <hsin size=1893 checksum=8f93ef004365152878e91ea78475038f unknown=(1, 1, 0)>
                    <DSIN#Unknown(116) size=1845>
                    </DSIN#Unknown(116)>
                </hsin>
            </pre-DSIN#Unknown(116)>
        </hsin>
    </pre-DSIN#Maybe( "Massive File")>
</hsin>

## FM8
<hsin size=2784 checksum=0f798e3749c7c3c023dea08759f45fd2 unknown=(1, 1, 0)>
    <DSIN#FileHeader size=150>
    </DSIN#FileHeader>
    <pre-DSIN#Maybe( "Container?") unknown=(1, 0, 0)>
        <hsin size=2574 checksum=37f8e13046c014000e00a2a89120d5c8 unknown=(1, 1, 0)>
            <DSIN#Maybe( "Container?") size=109>
            </DSIN#Maybe( "Container?")>
            <pre-DSIN#Unknown(108) unknown=(1, 0, 0)>
                <hsin size=996 checksum=ddd73b204a5b2d028cc863bdfbc39baa unknown=(1, 1, 0)>
                    <DSIN#Unknown(108) size=948>
                    </DSIN#Unknown(108)>
                </hsin>
            </pre-DSIN#Unknown(108)>
            <pre-DSIN#Unknown(121) unknown=(1, 0, 2)>
                <hsin size=104 checksum=c28beb464487e99a28323d84a136ba76 unknown=(1, 1, 0)>
                    <DSIN#Unknown(121) size=56>
                    </DSIN#Unknown(121)>
                </hsin>
            </pre-DSIN#Unknown(121)>
            <pre-DSIN#Unknown(116) unknown=(1, 0, 1)>
                <hsin size=1281 checksum=820de2c34e9c2c7380416bb9a4be15c3 unknown=(1, 1, 0)>
                    <DSIN#Unknown(116) size=1233>
                    </DSIN#Unknown(116)>
                </hsin>
            </pre-DSIN#Unknown(116)>
        </hsin>
    </pre-DSIN#Maybe( "Container?")>
</hsin>