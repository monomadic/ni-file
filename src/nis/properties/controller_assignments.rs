/*
void NI::SOUND::ControllerAssignments::exportProperties(Properties *param_1) {
  auto this_00 = *(ControllerGroupCollection **)(this + 0x14);
  auto numGroups = ControllerGroupCollection::getNumGroups(this_00);

  for (uint groupIdx = 0; groupIdx < numGroups; ++groupIdx) {
    auto group = ControllerGroupCollection::getGroup(this_00, groupIdx);
    auto groupSize = ControllerGroupAssignments::getGroupSize(group);

    for (uint assignIdx = 0; assignIdx < groupSize; ++assignIdx) {
      auto assignment = ControllerGroupAssignments::getAssignment(group, assignIdx);

      uint hostParamIdx = ControllerAssignment::getHostParameterIdx(assignment);
      uint clientParamIdx = ControllerAssignment::getClientParameterIdx(assignment);
      float minVal = static_cast<float>(ControllerAssignment::getMinValue(assignment));
      float maxVal = static_cast<float>(ControllerAssignment::getMaxValue(assignment));

      auto name = ControllerAssignment::getUseAutonaming(assignment)
                  ? "autonaming"
                  : ControllerAssignment::getAssignmentName(assignment)->c_str();

      // Append properties
      param_1->append(format("ctrl-%d-host-idx", groupIdx, assignIdx), hostParamIdx);
      param_1->append(format("ctrl-%d-client-idx", groupIdx, assignIdx), clientParamIdx);
      param_1->append(format("ctrl-%d-name", groupIdx, assignIdx), name);
      param_1->append(format("ctrl-%d-min", groupIdx, assignIdx), minVal);
      param_1->append(format("ctrl-%d-max", groupIdx, assignIdx), maxVal);
    }
  }
  Item::exportProperties(param_1);
}
*/
