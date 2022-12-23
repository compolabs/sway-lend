import React, { useMemo } from "react";
import { useVM } from "@src/hooks/useVM";
import { makeAutoObservable } from "mobx";
import { RootStore, useStores } from "@stores";
import { TokenAbi__factory } from "@src/contracts";
import { IToken, TOKENS_BY_SYMBOL, TOKENS_LIST } from "@src/constants";
import { Wallet } from "fuels";
import BN from "@src/utils/BN";

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

const faucetAmounts: Record<string, number> = {
  ETH: 0.5,
  LINK: 1000,
  UNI: 1000,
  BNB: 5,
  BTC: 1,
  BUSD: 100,
  USDC: 100,
  UST: 100,
};

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

  get faucetTokens() {
    if (this.rootStore.accountStore.assetBalances == null) return [];
    return TOKENS_LIST.map((b) => {
      const balance = this.rootStore.accountStore.findBalanceByAssetId(
        b.assetId
      );
      const mintAmount = new BN(faucetAmounts[b.symbol] ?? 0);
      const mintAmountDollar = mintAmount.times(balance?.defaultPrice ?? 0);
      const formatBalance = BN.formatUnits(
        balance?.balance ?? BN.ZERO,
        b.decimals
      );
      const balanceDollar = formatBalance.times(balance?.defaultPrice ?? 0);
      return {
        ...balance,
        formatBalance,
        balanceDollar,
        mintAmount,
        mintAmountDollar,
      };
    });
  }

  mint = async (assetId?: string) => {
    //todo add mint call and wallet auth
    if (assetId == null) return;
    this._setLoading(true);
    const { address } = this.rootStore.accountStore;
    if (address == null || window.FuelWeb3 == null) return;
    //todo add signing from account store
    const wallet = Wallet.fromAddress(address, window.FuelWeb3?.getProvider());
    const tokenContract = TokenAbi__factory.connect(assetId, wallet);

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
