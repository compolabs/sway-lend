import React, { useMemo } from "react";
import { useVM } from "@src/hooks/useVM";
import { makeAutoObservable } from "mobx";
import { RootStore, useStores } from "@stores";
import { FuelWeb3Provider } from "@fuel-wallet/sdk";

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
    if (address == null || window.FuelWeb3 == null) return;
    // const wallet = Wallet.fromPrivateKey(
    //   "02037c54d7f1b4d316d7e7f303c6e964f218057a26b0a4bc2ada1bd1ff6240a2",
    //   "https://node-beta-1.fuel.network/graphql"
    // );
    // const contract = TokenContractAbi__factory.connect(
    //   TOKENS_BY_SYMBOL.USDT.assetId,
    //   wallet
    // );
    // console.log(contract);
    console.log("getProvider", window.FuelWeb3.getProvider());
    console.log(
      "getWallet",
      window.FuelWeb3.getWallet(
        "fuel1lw8hf2fvwmmwwlkn3z2q8854lwshud4dazp6487wn6jkmuz0jfwqce9v0j"
      )
    );
    const provider = new FuelWeb3Provider(window.FuelWeb3);
    console.log(provider);
    // const wallet = Wallet.fromAddress(address, provider);
    // console.log(wallet);
  };
}
