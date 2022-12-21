import React from "react";
import { useStores } from "@stores";
import Button from "@components/Button";
import { observer } from "mobx-react-lite";
import { useFaucetVM } from "@screens/Faucet/FaucetVm";

interface IProps {}

const FaucetBtn: React.FC<IProps> = () => {
  const { accountStore, settingsStore } = useStores();
  const vm = useFaucetVM();

  switch (true) {
    case accountStore.address == null:
      return (
        <Button onClick={() => settingsStore.setLoginModalOpened(true)} fixed>
          Connect wallet
        </Button>
      );
    //todo add logic for not big mint

    case vm.loading:
      return (
        <Button disabled fixed>
          Loading
        </Button>
      );
    default:
      return (
        <Button onClick={vm.mint} fixed>
          Get 10 {vm.tokenForMint.symbol}
        </Button>
      );
  }
};
export default observer(FaucetBtn);
