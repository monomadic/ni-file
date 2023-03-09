# Bin 4.2

- `afl~writeItem` for write functions.

- `Preset` contains app type (kontakt=2, guitarrig=1, fm8=8)

Inheritance Model

- kontakt-5.4-demo.nki:
	- RepositoryRoot(118) -> Authorization(106) -> Item(1)
		- BNISoundPreset(3) -> Preset(101) -> Authorization(106) -> Item(1)
			- SoundInfoItem(108) -> Item(1)
			- ControllerAssignments (121) -> Item(1)
			- EncryptionItem(116) -> SubtreeItem(115) -> Item(1)
			- BNISoundHeader(4) -> Item(1)

- 000-default.nfm8
	- RepositoryRoot(118) -> Authorization(106) -> Item(1)
		- Preset(101) -> Authorization(106) -> Item(1)
			- SoundInfoItem(108) -> Item(1)
			-
			-

external libs:
- ElastiquePro stretching engine
    https://licensing.zplane.de/uploads/SDK/ELASTIQUE-PRO/V3/manual/elastique_pro_v3_sdk_documentation.pdf
- Boost
- OpenSSL
- Xerces 3.018 xml

NGL native instruments gui library
DB database

zlib section could actually be the compressor
https://en.wikipedia.org/wiki/Zlib
https://en.wikipedia.org/wiki/Deflate
https://docs.rs/inflate/0.4.5/inflate/

as simple DEFLATE
https://en.wikipedia.org/wiki/Deflate

9535  0x002c1b60 0x002c1b60 LOCAL  FUNC 0        NI::GP::ZLibStorage::getPos64() const
9536  0x002c1b90 0x002c1b90 LOCAL  FUNC 0        NI::GP::ZLibStorage::seek64(long long, NI::GP::IStorage::eSeekMode)
9609  0x002c3a60 0x002c3a60 LOCAL  FUNC 0        NI::GP::ZLibStorage::write(void const*, unsigned long)
9610  0x002c3b50 0x002c3b50 LOCAL  FUNC 0        NI::GP::ZLibStorage::read(void*, unsigned long)
9648  0x002c5240 0x002c5240 LOCAL  FUNC 0        NI::GP::ZLibStorage::rewind()
9675  0x002c6970 0x002c6970 LOCAL  FUNC 0        NI::GP::ZLibStorage::deInit()
9676  0x002c6a00 0x002c6a00 LOCAL  FUNC 0        NI::GP::ZLibStorage::~ZLibStorage()
9677  0x002c6a60 0x002c6a60 LOCAL  FUNC 0        NI::GP::ZLibStorage::~ZLibStorage()
31924 0x01023f07 0x01023f07 LOCAL  FUNC 0        typeinfo name for NI::GP::ZLibStorage
39093 0x0155d600 0x0155d600 LOCAL  FUNC 0        vtable for NI::GP::ZLibStorage
39115 0x0155d8c4 0x0155d8c4 LOCAL  FUNC 0        typeinfo for NI::GP::ZLibStorage

Reading segments could be this enum:
NI::DB::Item::Item(unsigned int) // could be a 'hsin'
NI::DB::Item::Item(NI::DB::Property::Key const&, NI::DB::Variant const&)
NI::DB::Item::Item(NI::DB::Property const&)

Then attributes are dsin blocks
NI::DB::Item::setAttribute(NI::DB::Attribute const&)

30627 0x00f99f80 0x00f99f80 LOCAL  FUNC 0        NI::DB::Item::Item(unsigned int)
30628 0x00f99f90 0x00f99f90 LOCAL  FUNC 0        NI::DB::Item::setProperty(NI::DB::Property const&)
30629 0x00f99fc0 0x00f99fc0 LOCAL  FUNC 0        NI::DB::Item::erase()
30630 0x00f99ff0 0x00f99ff0 LOCAL  FUNC 0        NI::DB::Item::clearAttributes()
30631 0x00f9a010 0x00f9a010 LOCAL  FUNC 0        NI::DB::Item::clearAttribute(NI::DB::Attribute const&)
30632 0x00f9a040 0x00f9a040 LOCAL  FUNC 0        NI::DB::Item::setAttribute(NI::DB::Attribute const&)
30633 0x00f9a070 0x00f9a070 LOCAL  FUNC 0        NI::DB::Item::hasAttribute(NI::DB::Attribute const&) const
30634 0x00f9a0a0 0x00f9a0a0 LOCAL  FUNC 0        NI::DB::Item::getAttributes() const
30635 0x00f9a0d0 0x00f9a0d0 LOCAL  FUNC 0        NI::DB::Item::clearProperty(NI::DB::Property const&)
30636 0x00f9a100 0x00f9a100 LOCAL  FUNC 0        NI::DB::Item::setProperty(NI::DB::Property::Key const&, NI::DB::Variant const&)
30637 0x00f9a140 0x00f9a140 LOCAL  FUNC 0        NI::DB::Item::clearProperty(NI::DB::Property::Key const&)
30638 0x00f9a170 0x00f9a170 LOCAL  FUNC 0        NI::DB::Item::getProperty(NI::DB::Property::Key const&) const
30639 0x00f9a1a0 0x00f9a1a0 LOCAL  FUNC 0        NI::DB::Item::getProperties() const
30640 0x00f9a1d0 0x00f9a1d0 LOCAL  FUNC 0        NI::DB::Item::getIdentifier() const
30641 0x00f9a1f0 0x00f9a1f0 LOCAL  FUNC 0        NI::DB::Item::Item(unsigned int)
30642 0x00f9a200 0x00f9a200 LOCAL  FUNC 0        NI::DB::Item::Item(NI::DB::Property::Key const&, NI::DB::Variant const&)
30643 0x00f9a290 0x00f9a290 LOCAL  FUNC 0        NI::DB::Item::Item(NI::DB::Property const&)

## Important Symbols
44701 0x01682450 0x01682450 LOCAL  FUNC 0        NI::DB::DefaultAttributeSets::init(bool)::s_strDefaultKAD
45122 0x016f0538 0x016f0538 LOCAL  FUNC 0        NI::DB::DefaultAttributes::SoundFile
45123 0x016f053c 0x016f053c LOCAL  FUNC 0        NI::DB::DefaultAttributes::Performance
45124 0x016f0540 0x016f0540 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MIDIFile
45125 0x016f0544 0x016f0544 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Component
45126 0x016f0548 0x016f0548 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineProject
45127 0x016f054c 0x016f054c LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineSong
45128 0x016f0550 0x016f0550 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineGroup
45129 0x016f0554 0x016f0554 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineTrack
45130 0x016f0558 0x016f0558 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschinePattern
45131 0x016f055c 0x016f055c LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineFX
45132 0x016f0560 0x016f0560 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Sample
45133 0x016f0564 0x016f0564 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineSamplerType
45134 0x016f0568 0x016f0568 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineInputType
45135 0x016f056c 0x016f056c LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineMidiOutType
45136 0x016f0570 0x016f0570 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineInstrumentBank
45137 0x016f0574 0x016f0574 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineFXBank
45138 0x016f0578 0x016f0578 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MaschineMIDIBank
45139 0x016f057c 0x016f057c LOCAL  FUNC 0        NI::DB::DefaultAttributes::Instrument
45140 0x016f0580 0x016f0580 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Effect
45141 0x016f0584 0x016f0584 LOCAL  FUNC 0        NI::DB::DefaultAttributes::MIDI
45142 0x016f0588 0x016f0588 LOCAL  FUNC 0        NI::DB::DefaultAttributes::VSTPlugIn
45143 0x016f058c 0x016f058c LOCAL  FUNC 0        NI::DB::DefaultAttributes::AudioUnit
45144 0x016f0590 0x016f0590 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Internal
45145 0x016f0594 0x016f0594 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Favorite
45146 0x016f0598 0x016f0598 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Flagged
45147 0x016f059c 0x016f059c LOCAL  FUNC 0        NI::DB::DefaultAttributes::BuiltIn
45148 0x016f05a0 0x016f05a0 LOCAL  FUNC 0        NI::DB::DefaultAttributes::InvalidFile
45149 0x016f05a4 0x016f05a4 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Kore1Synth
45150 0x016f05a8 0x016f05a8 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Kore1Lead
45151 0x016f05ac 0x016f05ac LOCAL  FUNC 0        NI::DB::DefaultAttributes::Kore1Pad
45152 0x016f05b0 0x016f05b0 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Kore2SynthLead
45153 0x016f05b4 0x016f05b4 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Kore2SynthPad
45154 0x016f05b8 0x016f05b8 LOCAL  FUNC 0        NI::DB::DefaultAttributes::Kore2SynthMisc
45155 0x016f05bc 0x016f05bc LOCAL  FUNC 0        NI::DB::DefaultAttributes::s_collKore1Instruments
45156 0x016f05d4 0x016f05d4 LOCAL  FUNC 0        NI::DB::DefaultAttributes::s_collKore1EffectTypes
45157 0x016f05ec 0x016f05ec LOCAL  FUNC 0        NI::DB::DefaultAttributes::s_collKore1Sources
45158 0x016f0604 0x016f0604 LOCAL  FUNC 0        NI::DB::DefaultAttributes::s_collKore1Modes
45159 0x016f061c 0x016f061c LOCAL  FUNC 0        NI::DB::DefaultAttributes::s_collKore1Timbres
45160 0x016f0634 0x016f0634 LOCAL  FUNC 0        NI::DB::DefaultAttributes::s_collKore1Characteristics
45161 0x016f064c 0x016f064c LOCAL  FUNC 0        NI::DB::DefaultAttributes::s_collKore1Articulations
45162 0x016f0664 0x016f0664 LOCAL  FUNC 0        NI::DB::DefaultAttributes::s_collKore1Applications
45163 0x016f067c 0x016f067c LOCAL  FUNC 0        NI::DB::DefaultAttributes::s_collKore1Genres
45164 0x016f0694 0x016f0694 LOCAL  FUNC 0        NI::DB::DefaultProperties::s_mapStringColor
45165 0x016f06ac 0x016f06ac LOCAL  FUNC 0        NI::DB::DefaultProperties::s_mapColorString
45166 0x016f06c4 0x016f06c4 LOCAL  FUNC 0        NI::DB::DefaultProperties::Name
45167 0x016f06c8 0x016f06c8 LOCAL  FUNC 0        NI::DB::DefaultProperties::URL
45168 0x016f06cc 0x016f06cc LOCAL  FUNC 0        NI::DB::DefaultProperties::Vendor
45169 0x016f06d0 0x016f06d0 LOCAL  FUNC 0        NI::DB::DefaultProperties::Author
45170 0x016f06d4 0x016f06d4 LOCAL  FUNC 0        NI::DB::DefaultProperties::Bank
45171 0x016f06d8 0x016f06d8 LOCAL  FUNC 0        NI::DB::DefaultProperties::Banks
45172 0x016f06dc 0x016f06dc LOCAL  FUNC 0        NI::DB::DefaultProperties::Color
45173 0x016f06e0 0x016f06e0 LOCAL  FUNC 0        NI::DB::DefaultProperties::DateModified
45174 0x016f06e4 0x016f06e4 LOCAL  FUNC 0        NI::DB::DefaultProperties::Rating
45175 0x016f06e8 0x016f06e8 LOCAL  FUNC 0        NI::DB::DefaultProperties::Comment
45176 0x016f06ec 0x016f06ec LOCAL  FUNC 0        NI::DB::DefaultProperties::Inputs
45177 0x016f06f0 0x016f06f0 LOCAL  FUNC 0        NI::DB::DefaultProperties::Outputs
45178 0x016f06f4 0x016f06f4 LOCAL  FUNC 0        NI::DB::DefaultProperties::UseCount
45179 0x016f06f8 0x016f06f8 LOCAL  FUNC 0        NI::DB::DefaultProperties::Flag
45180 0x016f06fc 0x016f06fc LOCAL  FUNC 0        NI::DB::DefaultProperties::Components
45181 0x016f0700 0x016f0700 LOCAL  FUNC 0        NI::DB::DefaultProperties::Attributised
45182 0x016f0704 0x016f0704 LOCAL  FUNC 0        NI::DB::DefaultProperties::ExternalComponents

## Structure
/Volumes/BUILD/NIBuild/Kontakt4_2/Kontakt/interface/AboutDialog.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/Kontakt/interface/AboutDialog.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/Kontakt/interface/bui.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/Kontakt/interface/bui.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/Kontakt/KontaktUI/Dialogs/OptAudio.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/Kontakt/KontaktUI/Dialogs/OptAudio.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/Kontakt/KontaktUI/Dialogs/OptMIDI.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/Kontakt/KontaktUI/Dialogs/OptMIDI.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/Kontakt/KontaktUI/StaticUI/Browser/MonitorListForm.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/Kontakt/KontaktUI/StaticUI/Browser/MonitorListForm.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ab/src/Dialog/ImpNSAMidiForm_AB.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ab/src/Dialog/ImpNSAMidiForm_AB.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ab/src/Dialog/ImpNSARoutingForm_AB.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ab/src/Dialog/ImpNSARoutingForm_AB.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ab/src/Dialog/ImpNSASetupDialog_AB.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ab/src/Dialog/ImpNSASetupDialog_AB.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ab/src/Dialog/ImpNSASetupSoundCardForm_AB.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ab/src/Dialog/ImpNSASetupSoundCardForm_AB.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ab/src/Init_AB.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ab/src/Init_AB.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/cp/src/Framework/NGLRASApplication_CP.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/cp/src/Framework/NGLRASApplication_CP.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/cp/src/Init_CP.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/cp/src/Init_CP.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/dsp/src/Init_DSP.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/dsp/src/Init_DSP.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/gp/src/Common/Init_GP.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/gp/src/Common/Init_GP.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/Control_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/Control_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/LabelControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/LabelControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/LevelMonitorControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/LevelMonitorControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/ListControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/ListControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/ScopeControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/ScopeControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/SelectorControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/SelectorControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/ShadeAreaControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/ShadeAreaControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/SubFormControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/SubFormControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/SwitchControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/SwitchControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Contro/ls/TextEditControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/TextEditControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/TreeControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/TreeControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/ValueEditControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Controls/ValueEditControl_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Detail/PanelItem_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Detail/PanelItem_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Detail/SelectableTextPanelItem_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Detail/SelectableTextPanelItem_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Detail/TextItem_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Detail/TextItem_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Detail/TextPanelItem_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Detail/TextPanelItem_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Fonts/Font_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Fonts/Font_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Forms/Form_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Forms/Form_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Init_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Init_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Menus/ButtonMenu_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Menus/ButtonMenu_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Menus/ButtonMenuControl2_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Menus/ButtonMenuControl2_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Menus/PopupMenu_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/ngl/src/Menus/PopupMenu_NGL.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/nixml/src/Init_XML.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/nixml/src/Init_XML.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/nixml/src/NIXML.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/uia/src/Init_UIA.cpp
/Volumes/BUILD/NIBuild/Kontakt4_2/nilibs/uia/src/Init_UIA.cpp
xercesc/dom/impl/DOMNodeIDMap.cpp
xercesc/dom/impl/DOMNodeIDMap.cpp
xercesc/dom/impl/DOMNodeImpl.cpp
xercesc/dom/impl/DOMNodeImpl.cpp
xercesc/dom/impl/DOMNodeVector.cpp
xercesc/dom/impl/DOMNodeVector.cpp
xercesc/dom/impl/DOMTypeInfoImpl.cpp
xercesc/dom/impl/DOMTypeInfoImpl.cpp
xercesc/framework/LocalFileFormatTarget.cpp
xercesc/framework/LocalFileFormatTarget.cpp
xercesc/framework/Wrapper4DOMLSInput.cpp
xercesc/framework/Wrapper4DOMLSInput.cpp
xercesc/framework/XMLBuffer.cpp
xercesc/framework/XMLBuffer.cpp
xercesc/framework/XMLBufferMgr.cpp
xercesc/framework/XMLBufferMgr.cpp
xercesc/framework/XMLFormatter.cpp
xercesc/framework/XMLFormatter.cpp
xercesc/framework/XMLGrammarPoolImpl.cpp
xercesc/framework/XMLGrammarPoolImpl.cpp
xercesc/framework/XMLRecognizer.cpp
xercesc/framework/XMLRecognizer.cpp
xercesc/internal/DGXMLScanner.cpp
xercesc/internal/DGXMLScanner.cpp
xercesc/internal/ElemStack.cpp
xercesc/internal/ElemStack.cpp
xercesc/internal/IGXMLScanner.cpp
xercesc/internal/IGXMLScanner.cpp
xercesc/internal/IGXMLScanner2.cpp
xercesc/internal/IGXMLScanner2.cpp
xercesc/internal/ReaderMgr.cpp
xercesc/internal/ReaderMgr.cpp
xercesc/internal/SGXMLScanner.cpp
xercesc/internal/SGXMLScanner.cpp
xercesc/internal/ValidationContextImpl.cpp
xercesc/internal/ValidationContextImpl.cpp
xercesc/internal/WFXMLScanner.cpp
xercesc/internal/WFXMLScanner.cpp
xercesc/internal/XMLReader.cpp
xercesc/internal/XMLReader.cpp
xercesc/internal/XMLScanner.cpp
xercesc/internal/XMLScanner.cpp
xercesc/internal/XProtoType.cpp
xercesc/internal/XProtoType.cpp
xercesc/internal/XSAXMLScanner.cpp
xercesc/internal/XSAXMLScanner.cpp
xercesc/internal/XSerializeEngine.cpp
xercesc/internal/XSerializeEngine.cpp
xercesc/parsers/AbstractDOMParser.cpp
xercesc/parsers/AbstractDOMParser.cpp
xercesc/util/FileManagers/PosixFileMgr.cpp
xercesc/util/FileManagers/PosixFileMgr.cpp
xercesc/util/MutexManagers/PosixMutexMgr.cpp
xercesc/util/MutexManagers/PosixMutexMgr.cpp
xercesc/util/PlatformUtils.cpp
xercesc/util/PlatformUtils.cpp
xercesc/util/regx/Op.cpp
xercesc/util/regx/Op.cpp
xercesc/util/regx/ParserForXMLSchema.cpp
xercesc/util/regx/ParserForXMLSchema.cpp
xercesc/util/regx/RangeToken.cpp
xercesc/util/regx/RangeToken.cpp
xercesc/util/regx/RangeTokenMap.cpp
xercesc/util/regx/RangeTokenMap.cpp
xercesc/util/regx/RegularExpression.cpp
xercesc/util/regx/RegularExpression.cpp
xercesc/util/regx/RegxParser.cpp
xercesc/util/regx/RegxParser.cpp
xercesc/util/StringPool.cpp
xercesc/util/StringPool.cpp
xercesc/util/Transcoders/MacOSUnicodeConverter/MacOSUnicodeConverter.cpp
xercesc/util/Transcoders/MacOSUnicodeConverter/MacOSUnicodeConverter.cpp
xercesc/util/XMemory.cpp
xercesc/util/XMemory.cpp
xercesc/util/XML256TableTranscoder.cpp
xercesc/util/XML256TableTranscoder.cpp
xercesc/util/XML88591Transcoder.cpp
xercesc/util/XML88591Transcoder.cpp
xercesc/util/XMLAbstractDoubleFloat.cpp
xercesc/util/XMLAbstractDoubleFloat.cpp
xercesc/util/XMLASCIITranscoder.cpp
xercesc/util/XMLASCIITranscoder.cpp
xercesc/util/XMLBigDecimal.cpp
xercesc/util/XMLBigDecimal.cpp
xercesc/util/XMLBigInteger.cpp
xercesc/util/XMLBigInteger.cpp
xercesc/util/XMLDateTime.cpp
xercesc/util/XMLDateTime.cpp
xercesc/util/XMLString.cpp
xercesc/util/XMLString.cpp
xercesc/util/XMLUCS4Transcoder.cpp
xercesc/util/XMLUCS4Transcoder.cpp
xercesc/util/XMLUri.cpp
xercesc/util/XMLUri.cpp
xercesc/util/XMLURL.cpp
xercesc/util/XMLURL.cpp
xercesc/util/XMLUTF8Transcoder.cpp
xercesc/util/XMLUTF8Transcoder.cpp
xercesc/validators/common/AllContentModel.cpp
xercesc/validators/common/AllContentModel.cpp
xercesc/validators/common/CMAny.cpp
xercesc/validators/common/CMAny.cpp
xercesc/validators/common/CMBinaryOp.cpp
xercesc/validators/common/CMBinaryOp.cpp
xercesc/validators/common/CMUnaryOp.cpp
xercesc/validators/common/CMUnaryOp.cpp
xercesc/validators/common/ContentLeafNameTypeVector.cpp
xercesc/validators/common/ContentLeafNameTypeVector.cpp
xercesc/validators/common/DFAContentModel.cpp
xercesc/validators/common/DFAContentModel.cpp
xercesc/validators/common/MixedContentModel.cpp
xercesc/validators/common/MixedContentModel.cpp
xercesc/validators/common/SimpleContentModel.cpp
xercesc/validators/common/SimpleContentModel.cpp
xercesc/validators/datatype/AbstractNumericFacetValidator.cpp
xercesc/validators/datatype/AbstractNumericFacetValidator.cpp
xercesc/validators/datatype/AbstractNumericValidator.cpp
xercesc/validators/datatype/AbstractNumericValidator.cpp
xercesc/validators/datatype/AbstractStringValidator.cpp
xercesc/validators/datatype/AbstractStringValidator.cpp
xercesc/validators/datatype/AnySimpleTypeDatatypeValidator.cpp
xercesc/validators/datatype/AnySimpleTypeDatatypeValidator.cpp
xercesc/validators/datatype/AnyURIDatatypeValidator.cpp
xercesc/validators/datatype/AnyURIDatatypeValidator.cpp
xercesc/validators/datatype/Base64BinaryDatatypeValidator.cpp
xercesc/validators/datatype/Base64BinaryDatatypeValidator.cpp
xercesc/validators/datatype/BooleanDatatypeValidator.cpp
xercesc/validators/datatype/BooleanDatatypeValidator.cpp
xercesc/validators/datatype/DateTimeValidator.cpp
xercesc/validators/datatype/DateTimeValidator.cpp
xercesc/validators/datatype/DecimalDatatypeValidator.cpp
xercesc/validators/datatype/DecimalDatatypeValidator.cpp
xercesc/validators/datatype/DoubleDatatypeValidator.cpp
xercesc/validators/datatype/DoubleDatatypeValidator.cpp
xercesc/validators/datatype/ENTITYDatatypeValidator.cpp
xercesc/validators/datatype/ENTITYDatatypeValidator.cpp
xercesc/validators/datatype/FloatDatatypeValidator.cpp
xercesc/validators/datatype/FloatDatatypeValidator.cpp
xercesc/validators/datatype/HexBinaryDatatypeValidator.cpp
xercesc/validators/datatype/HexBinaryDatatypeValidator.cpp
xercesc/validators/datatype/IDDatatypeValidator.cpp
xercesc/validators/datatype/IDDatatypeValidator.cpp
xercesc/validators/datatype/IDREFDatatypeValidator.cpp
xercesc/validators/datatype/IDREFDatatypeValidator.cpp
xercesc/validators/datatype/ListDatatypeValidator.cpp
xercesc/validators/datatype/ListDatatypeValidator.cpp
xercesc/validators/datatype/NameDatatypeValidator.cpp
xercesc/validators/datatype/NameDatatypeValidator.cpp
xercesc/validators/datatype/NCNameDatatypeValidator.cpp
xercesc/validators/datatype/NCNameDatatypeValidator.cpp
xercesc/validators/datatype/NOTATIONDatatypeValidator.cpp
xercesc/validators/datatype/NOTATIONDatatypeValidator.cpp
xercesc/validators/datatype/QNameDatatypeValidator.cpp
xercesc/validators/datatype/QNameDatatypeValidator.cpp
xercesc/validators/datatype/StringDatatypeValidator.cpp
xercesc/validators/datatype/StringDatatypeValidator.cpp
xercesc/validators/datatype/UnionDatatypeValidator.cpp
xercesc/validators/datatype/UnionDatatypeValidator.cpp
xercesc/validators/DTD/DTDAttDefList.cpp
xercesc/validators/DTD/DTDAttDefList.cpp
xercesc/validators/DTD/DTDElementDecl.cpp
xercesc/validators/DTD/DTDElementDecl.cpp
xercesc/validators/DTD/DTDScanner.cpp
xercesc/validators/DTD/DTDScanner.cpp
xercesc/validators/DTD/DTDValidator.cpp
xercesc/validators/DTD/DTDValidator.cpp
xercesc/validators/schema/ComplexTypeInfo.cpp
xercesc/validators/schema/ComplexTypeInfo.cpp
xercesc/validators/schema/identity/XercesXPath.cpp
xercesc/validators/schema/identity/XercesXPath.cpp
xercesc/validators/schema/identity/XPathMatcher.cpp
xercesc/validators/schema/identity/XPathMatcher.cpp
xercesc/validators/schema/NamespaceScope.cpp
xercesc/validators/schema/NamespaceScope.cpp
xercesc/validators/schema/SchemaAttDefList.cpp
xercesc/validators/schema/SchemaAttDefList.cpp
xercesc/validators/schema/SchemaElementDecl.cpp
xercesc/validators/schema/SchemaElementDecl.cpp
xercesc/validators/schema/SchemaValidator.cpp
xercesc/validators/schema/SchemaValidator.cpp
xercesc/validators/schema/SubstitutionGroupComparator.cpp
xercesc/validators/schema/SubstitutionGroupComparator.cpp
xercesc/validators/schema/TraverseSchema.cpp
xercesc/validators/schema/TraverseSchema.cpp

## PNG Chunk Types
IHDR IDAT IEND PLTE bKGD cHRM gAMA hIST iCCP oFFs pCAL sCAL pHYs sBIT sPLT sRGB tEXt tIME tRNS zTXt

## Other refs
