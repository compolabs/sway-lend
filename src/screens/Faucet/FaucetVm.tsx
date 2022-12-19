import React, { useMemo } from "react";
import { useVM } from "@src/hooks/useVM";
import { makeAutoObservable } from "mobx";
import { RootStore, useStores } from "@stores";
import { TOKENS_BY_SYMBOL } from "@src/constants";
import { TokenContractAbi__factory } from "@src/contracts";
import { Wallet } from "fuels";

const ctx = React.createContext<FaucetVM | null>(null);

interface IProps {
  children: React.ReactNode;
}

export const FaucetVMProvider: React.FC<IProps> = ({ children }) => {
  const rootStore = useStores();
  const store = useMemo(() => new FaucetVM(rootStore), [rootStore]);
  return <ctx.Provider value={store}>{children}</ctx.Provider>;
};

export const useFaucetVM = () => useVM(ctx);

class FaucetVM {
  public rootStore: RootStore;

  tokenContract: any | null = null;
  setTokenContract = (v: any | null) => (this.tokenContract = v);

  constructor(rootStore: RootStore) {
    this.rootStore = rootStore;

    makeAutoObservable(this);
  }

  mint = async () => {
    //todo add mint call
    const { address } = this.rootStore.accountStore;
    if (address == null) return;
    const wallet = Wallet.fromPrivateKey(
      "02037c54d7f1b4d316d7e7f303c6e964f218057a26b0a4bc2ada1bd1ff6240a2",
      "https://node-beta-1.fuel.network/graphql"
    );
    const contract = TokenContractAbi__factory.connect(
      TOKENS_BY_SYMBOL.USDT.assetId,
      wallet
    );
    console.log(contract);
  };
}
