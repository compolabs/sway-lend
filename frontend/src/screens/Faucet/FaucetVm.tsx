import React, { useMemo } from "react";
import { useVM } from "@src/hooks/useVM";
import { makeAutoObservable, reaction } from "mobx";
import { RootStore, useStores } from "@stores";
import { TokenAbi__factory } from "@src/contracts";
import { EXPLORER_URL, TOKENS_BY_ASSET_ID, TOKENS_LIST } from "@src/constants";
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
  BTC: 1,
  USDC: 10000,
  SWAY: 1000,
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
    const { walletToRead, addressInput } = this.rootStore.accountStore;
    if (walletToRead == null || addressInput == null) return;
    const tokens = TOKENS_LIST.filter((v) => v.symbol !== "ETH");
    try {
      const tokensContracts = tokens.map((b) =>
        TokenAbi__factory.connect(b.assetId, walletToRead)
      );
      const response = await Promise.all(
        tokensContracts.map((v) =>
          v.functions.already_minted(addressInput).get()
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

  actionTokenAssetId: string | null = null;
  setActionTokenAssetId = (l: string | null) => (this.actionTokenAssetId = l);

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
    this.setActionTokenAssetId(assetId);
    const { accountStore, notificationStore } = this.rootStore;
    const wallet = await accountStore.getWallet();
    if (wallet == null) return;
    const tokenContract = TokenAbi__factory.connect(assetId, wallet);

    try {
      const { transactionResult } = await tokenContract.functions
        .mint()
        .txParams({ gasPrice: 1 })
        .call();
      if (transactionResult != null) {
        this.setAlreadyMintedTokens([...this.alreadyMintedTokens, assetId]);
        const token = TOKENS_BY_ASSET_ID[assetId];
        this.rootStore.notificationStore.toast(
          `You have successfully minted ${token.symbol}`,
          {
            link: `${EXPLORER_URL}/transaction/${transactionResult.transactionId}`,
            linkTitle: "View on Explorer",
            type: "success",
            title: "Transaction is completed!",
          }
        );
      }
      await this.rootStore.accountStore.updateAccountBalances();
    } catch (e) {
      const errorText = e?.toString();
      console.log(errorText);
      notificationStore.toast(errorText ?? "", { type: "error" });
    } finally {
      this.setActionTokenAssetId(null);
      this._setLoading(false);
    }
  };
}
