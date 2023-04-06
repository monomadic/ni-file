#include <iostream>

void K4PO::K4PL_Group::dbgPrint() const {
    std::cout << "name: " << name << std::endl
              << "volume: " << volume << std::endl
              << "pan: " << pan << std::endl
              << "tune: " << tune << std::endl
              << "keyTracking: " << keyTracking << std::endl
              << "reverse: " << reverse << std::endl
              << "releaseTrigger: " << releaseTrigger << std::endl
              << "releaseTriggerNoteMonophonic: " << releaseTriggerNoteMonophonic << std::endl
              << "rlsTrigCounter: " << rlsTrigCounter << std::endl
              << "midiChannel: " << midiChannel << std::endl
              << "voiceGroupIdx: " << voiceGroupIdx << std::endl
              << "fxIdxAmpSplitPoint: " << fxIdxAmpSplitPoint << std::endl
              << "muted: " << muted << std::endl
              << "soloed: " << soloed << std::endl
              << "interpQuality: " << interpQuality << std::endl;
}
