import styled from "@emotion/styled";
import React from "react";
import SwitchButtons from "@components/SwitchButtons";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import { observer } from "mobx-react-lite";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
`;

const ModeSwitch: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  return (
    <Root>
      <SwitchButtons
        values={["+ Deposit mode", "- Loan mode"]}
        active={vm.mode}
        onActivate={(v) => vm.setMode(v)}
      />
    </Root>
  );
};
export default observer(ModeSwitch);
