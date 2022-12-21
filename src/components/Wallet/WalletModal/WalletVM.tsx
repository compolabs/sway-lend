import React, { useMemo } from "react";
import { useVM } from "@src/hooks/useVM";
import { makeAutoObservable } from "mobx";
import { RootStore, useStores } from "@stores";
import copy from "copy-to-clipboard";
import centerEllipsis from "@src/utils/centerEllipsis";
import BN from "@src/utils/BN";

const ctx = React.createContext<WalletVM | null>(null);

interface WalletVMProps {
  children: React.ReactNode;
}

export const WalletVMProvider: React.FC<WalletVMProps> = ({ children }) => {
  const rootStore = useStores();
  const store = useMemo(() => new WalletVM(rootStore), [rootStore]);
  return <ctx.Provider value={store}>{children}</ctx.Provider>;
};

export const useWalletVM = () => useVM(ctx);

class WalletVM {
  rootStore: RootStore;

  headerExpanded: boolean = true;
  setHeaderExpanded = (state: boolean) => (this.headerExpanded = state);

  constructor(rootStore: RootStore) {
    this.rootStore = rootStore;
    makeAutoObservable(this);
  }

  handleCopyAddress = () => {
    const { accountStore, notificationStore } = this.rootStore;
    if (accountStore.address) {
      copy(accountStore.address ?? "");
      notificationStore.notify("Your address was copied", {
        type: "success",
        title: "Congratulations!",
      });
    } else {
      notificationStore.notify("There is no address", { type: "error" });
    }
  };

  get totalInvestmentAmount() {
    const { balances } = this.rootStore.accountStore;
    const balancesAmount = balances.reduce(
      (acc, b) => acc.plus(b.usdEquivalent ?? 0),
      BN.ZERO
    );
    return balancesAmount.plus(BN.ZERO).toFormat(2);
  }

  handleLogOut = async () => {
    const { accountStore, settingsStore } = this.rootStore;
    settingsStore.setLoginModalOpened(false);
    accountStore.setAddress(null);
  };

  get signInInfo() {
    const { address } = this.rootStore.accountStore;
    return `
        Fuel Wallet: ${centerEllipsis(address ?? "", 10)}`;
  }
}
