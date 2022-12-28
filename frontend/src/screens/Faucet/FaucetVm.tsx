import React, { useMemo } from "react";
import { useVM } from "@src/hooks/useVM";
import { makeAutoObservable, reaction } from "mobx";
import { RootStore, useStores } from "@stores";
import { TokenAbi__factory } from "@src/contracts";
import {
  NODE_URL,
  SEED,
  TOKENS_BY_ASSET_ID,
  TOKENS_LIST,
} from "@src/constants";
import { Provider, Wallet } from "fuels";
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
  BUSD: 10000,
  USDC: 10000,
  USDT: 10000,
};

class FaucetVM {
  public rootStore: RootStore;

  loading: boolean = false;
  private _setLoading = (l: boolean) => (this.loading = l);

  alreadyMintedTokens: string[] = [];
  private setAlreadyMintedTokens = (l: string[]) =>
    (this.alreadyMintedTokens = l);

  constructor(rootStore: RootStore) {
    this.rootStore = rootStore;
    this.checkTokensThatAlreadyBeenMinted().then();
    reaction(
      () => this.rootStore.accountStore.address,
      () => this.checkTokensThatAlreadyBeenMinted()
    );
    makeAutoObservable(this);
  }

  checkTokensThatAlreadyBeenMinted = async () => {
    const { address } = this.rootStore.accountStore;
    if (address == null) return;
    const checkWallet = Wallet.fromSeed(SEED, "", new Provider(NODE_URL));
    const wallet = Wallet.fromAddress(address, new Provider(NODE_URL));
    const b256Address = {
      value: wallet.address.toB256(),
    };
    const tokens = TOKENS_LIST.filter((v) => v.symbol !== "ETH");
    try {
      const tokensContracts = tokens.map((b) =>
        TokenAbi__factory.connect(b.assetId, checkWallet)
      );
      const response = await Promise.all(
        tokensContracts.map((v) =>
          v.functions.already_minted(b256Address).simulate()
        )
      );
      if (response.length > 0) {
        const v = response.reduce(
          (acc, v, index) =>
            v.value ? [...acc, tokens[index].assetId] : [...acc],
          [] as string[]
        );
        this.setAlreadyMintedTokens(v);
      }
    } catch (e) {
      console.log(e);
    }
  };

  get faucetTokens() {
    const { accountStore, pricesStore } = this.rootStore;
    if (accountStore.assetBalances == null) return [];
    return TOKENS_LIST.map((b) => {
      const balance = accountStore.findBalanceByAssetId(b.assetId);
      const price =
        pricesStore.tokensPrices != null
          ? pricesStore.tokensPrices[b.assetId]
          : BN.ZERO;
      const mintAmount = new BN(faucetAmounts[b.symbol] ?? 0);
      const mintAmountDollar = mintAmount.times(price);
      const formatBalance = BN.formatUnits(
        balance?.balance ?? BN.ZERO,
        b.decimals
      );
      const balanceDollar = formatBalance.times(price);
      return {
        ...TOKENS_BY_ASSET_ID[b.assetId],
        ...balance,
        formatBalance,
        price,
        balanceDollar,
        mintAmount,
        mintAmountDollar,
      };
    });
  }

  mint = async (assetId?: string) => {
    if (assetId == null || this.alreadyMintedTokens.includes(assetId)) return;
    this._setLoading(true);
    const { accountStore, notificationStore } = this.rootStore;
    const { address } = accountStore;
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
      this.setAlreadyMintedTokens([...this.alreadyMintedTokens, assetId]);
    } catch (e) {
      console.log(e);
      notificationStore.notify(
        `You have already minted ${TOKENS_BY_ASSET_ID[assetId].symbol}`,
        { type: "error" }
      );
    } finally {
      this._setLoading(false);
    }
  };

  //todo change it to this way when its possible

  //todo add notification to mint eth when there any eth on balance

  // checkTokensThatAlreadyBeenMinted1 = async () => {
  //   const { address } = this.rootStore.accountStore;
  //   if (address == null) return;
  //   const wallet = Wallet.fromAddress(address, new Provider(NODE_URL));
  //   const b256Address = {
  //     value: wallet.address.toB256(),
  //   };
  //   try {
  //     const tokensContracts = TOKENS_LIST.filter((v) => v.symbol === "ETH").map(
  //         (b) => TokenAbi__factory.connect(b.assetId, wallet)
  //     );
  //     const response = await Promise.all(
  //         tokensContracts.map((v) =>
  //             v.functions.already_minted(b256Address).simulate()
  //         )
  //     );
  //     console.log(response);
  //   } catch (e) {
  //     console.log(e);
  //   }
  // };
}
