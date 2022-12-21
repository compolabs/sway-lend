import React, { useMemo } from "react";
import { useVM } from "@src/hooks/useVM";
import { makeAutoObservable } from "mobx";
import { RootStore, useStores } from "@stores";
import { TokenAbi__factory } from "@src/contracts";
import { IToken, TOKENS_BY_SYMBOL, TOKENS_LIST } from "@src/constants";
import { Wallet } from "fuels";
import Balance from "@src/entities/Balance";

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

  //todo add type
  tokenContract: any | null = null;
  setTokenContract = (v: any | null) => (this.tokenContract = v);

  tokenForMint: IToken = TOKENS_BY_SYMBOL.USDT;
  setTokenForMint = (v: IToken) => (this.tokenForMint = v);

  loading: boolean = false;
  private _setLoading = (l: boolean) => (this.loading = l);

  error: string | null = null;
  setError = (l: string | null) => (this.error = l);

  constructor(rootStore: RootStore) {
    this.rootStore = rootStore;

    makeAutoObservable(this);
  }

  get tokensForMint() {
    return TOKENS_LIST.map((v) => new Balance({ ...v })).filter(
      ({ symbol }) => symbol !== "ETH"
    );
  }

  mint = async () => {
    //todo add mint call and wallet auth
    const { address } = this.rootStore.accountStore;
    if (address == null || window.FuelWeb3 == null) return;
    //todo add signing from account store
    const wallet = Wallet.fromAddress(address, window.FuelWeb3?.getProvider());
    const tokenContract = TokenAbi__factory.connect(
      this.tokenForMint.assetId,
      wallet
    );

    this._setLoading(true);
    try {
      const v = await tokenContract.functions
        .mint()
        .txParams({ gasPrice: 1 })
        .call();
      console.log(v);
    } catch (e) {
      console.log(e);
      this.setError("Something went wrong");
    } finally {
      this._setLoading(false);
    }
  };
}
