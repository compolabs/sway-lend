import React, { PropsWithChildren, useMemo } from "react";
import { makeAutoObservable } from "mobx";
import { RootStore, useStores } from "@stores";
import { useVM } from "@src/hooks/useVM";
import { IToken, TOKENS_BY_ASSET_ID, TOKENS_BY_SYMBOL } from "@src/constants";
import BN from "@src/utils/BN";

const ctx = React.createContext<DashboardVm | null>(null);

export const DashboardVMProvider: React.FC<PropsWithChildren> = ({
  children,
}) => {
  const rootStore = useStores();
  const store = useMemo(() => new DashboardVm(rootStore), [rootStore]);
  return <ctx.Provider value={store}>{children}</ctx.Provider>;
};

export const useDashboardVM = () => useVM(ctx);

type ActionType = "supply" | "borrow";

class DashboardVm {
  public rootStore: RootStore;

  collaterals: IToken[] = [
    TOKENS_BY_SYMBOL.SWAY,
    TOKENS_BY_SYMBOL.ETH,
    TOKENS_BY_SYMBOL.BTC,
    TOKENS_BY_SYMBOL.LINK,
    TOKENS_BY_SYMBOL.UNI,
  ];

  action: ActionType | null = null;
  setAction = (l: ActionType | null) => (this.action = l);

  tokenAmount: BN | null = null;
  setTokenAmount = (l: BN | null) => (this.tokenAmount = l);

  actionTokenAssetId: string | null = null;
  setActionTokenAssetId = (l: string | null) => (this.actionTokenAssetId = l);

  constructor(rootStore: RootStore) {
    this.rootStore = rootStore;
    makeAutoObservable(this);
  }

  isThereUserData = false;

  get tokenBtnsClick(): Record<string, [() => void]> {
    return this.collaterals.reduce((acc, v) => {
      const c = () => {
        console.log("click click", v.symbol);
      };
      return { ...acc, [v.assetId]: [c] };
    }, {} as Record<string, [() => void]>);
  }

  get actionName() {
    if (this.action == null) return "";
    switch (this.action) {
      case "borrow":
        return "Borrow";
      case "supply":
        return "Supply";
    }
  }

  get token() {
    if (this.actionTokenAssetId == null) return TOKENS_BY_SYMBOL.USDC;
    return TOKENS_BY_ASSET_ID[this.actionTokenAssetId];
  }
}