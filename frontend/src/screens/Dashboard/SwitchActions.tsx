import React from "react";
import SwitchButtons from "@components/SwitchButtons";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import { observer } from "mobx-react-lite";

interface IProps {}

const SwitchActions: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  return (
    <SwitchButtons
      values={["Deposit", "Borrow"]}
      active={vm.mode}
      onActivate={(v) => {
        vm.setMode(v);
        vm.setAction(null);
        vm.setActionTokenAssetId(null);
      }}
    />
  );
};
export default observer(SwitchActions);
