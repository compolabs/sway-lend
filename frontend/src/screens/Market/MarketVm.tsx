import React, { PropsWithChildren, useMemo } from "react";
import { makeAutoObservable } from "mobx";
import { RootStore, useStores } from "@stores";
import { useVM } from "@src/hooks/useVM";

const ctx = React.createContext<MarketVM | null>(null);

export const MarketVMProvider: React.FC<PropsWithChildren> = ({ children }) => {
  const rootStore = useStores();
  const store = useMemo(() => new MarketVM(rootStore), [rootStore]);
  return <ctx.Provider value={store}>{children}</ctx.Provider>;
};

export const useMarketVM = () => useVM(ctx);

class MarketVM {
  constructor(private rootStore: RootStore) {
    makeAutoObservable(this);
  }
}
