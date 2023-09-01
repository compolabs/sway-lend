import React, { useMemo } from "react";
import { useVM } from "@src/hooks/useVM";
import { makeAutoObservable, reaction } from "mobx";
import { RootStore, useStores } from "@stores";
import {
  EXPLORER_URL,
  TOKENS_BY_ASSET_ID,
  TOKENS_BY_SYMBOL,
  TOKENS_LIST,
} from "@src/constants";
import BN from "@src/utils/BN";
import { LOGIN_TYPE } from "@stores/AccountStore";
import { TokenFactoryAbi__factory } from "@src/contracts";
import { hashMessage } from "fuels";

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
  LINK: 50,
  UNI: 50,
  BTC: 0.01,
  USDC: 300,
  SWAY: 5,
  COMP: 5,
};

class FaucetVM {
  public rootStore: RootStore;

  loading: boolean = false;
  private _setLoading = (l: boolean) => (this.loading = l);

  // alreadyMintedTokens: string[] = [];
  // private setAlreadyMintedTokens = (l: string[]) =>
  //   (this.alreadyMintedTokens = l);

  constructor(rootStore: RootStore) {
    this.rootStore = rootStore;
    this.checkTokensThatAlreadyBeenMinted().then();
    reaction(
      () => this.rootStore.accountStore.address,
      () => this.checkTokensThatAlreadyBeenMinted()
    );
    makeAutoObservable(this);
  }

  updateFaucetStateWhenVersionChanged = async () => {
    this.setInitialized(false);
    await this.checkTokensThatAlreadyBeenMinted();
    this.setInitialized(true);
  };

  rejectUpdateStatePromise?: () => void;
  setRejectUpdateStatePromise = (v: any) => (this.rejectUpdateStatePromise = v);

  checkTokensThatAlreadyBeenMinted = async () => {
    // console.log(this.rootStore.settingsStore.faucetTokens);
    // const { walletToRead, addressInput } = this.rootStore.accountStore;
    // if (walletToRead == null || addressInput == null) return;
    // // const tokens = TOKENS_LIST.filter((v) => v.symbol !== "ETH");
    // const tokens = TOKENS_LIST.filter((v) => v.symbol === "USDC");
    // if (this.rejectUpdateStatePromise != null) this.rejectUpdateStatePromise();
    //
    // const tokensContracts = tokens.map((b) => {
    //   console.log(b.symbol, b.assetId);
    //   return TokenContractAbi__factory.connect(b.assetId, walletToRead);
    // });
    // const promise = new Promise((resolve, reject) => {
    //   this.rejectUpdateStatePromise = reject;
    //   resolve(
    //     Promise.all(
    //       tokensContracts.map((v) =>
    //         v.functions.already_minted(addressInput).simulate()
    //       )
    //     )
    //   );
    // });
    // promise
    //   .catch((v) => {
    //     console.log("update faucet data error", v);
    //   })
    //   .then((value: any) => {
    //     if (value.length > 0) {
    //       const v = value.reduce(
    //         (acc: any, v: any, index: number) =>
    //           v.value ? [...acc, tokens[index].assetId] : [...acc],
    //         [] as string[]
    //       );
    //       this.setAlreadyMintedTokens(v);
    //     }
    //   })
    //   .finally(() => {
    //     this.setInitialized(true);
    //     this.setRejectUpdateStatePromise(undefined);
    //   });
    this.setInitialized(true);
  };

  actionTokenAssetId: string | null = null;
  setActionTokenAssetId = (l: string | null) => (this.actionTokenAssetId = l);

  initialized: boolean = false;
  private setInitialized = (l: boolean) => (this.initialized = l);

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
    // const alreadyMintedTokens = this.rootStore.settingsStore.faucetTokens[th];

    if (assetId == null) return;
    //   console.log("return 1");
    //   return;
    // }
    if (window?.fuel == null) return;
    if (this.rootStore.accountStore.loginType === LOGIN_TYPE.FUEL_WALLET) {
      const addedAssets: Array<any> = await window?.fuel.assets();
      if (
        addedAssets != null &&
        !addedAssets.some((v) => v.assetId === assetId)
      ) {
        await this.addAsset(assetId);
      }
    }
    this._setLoading(true);
    this.setActionTokenAssetId(assetId);
    const { accountStore, notificationStore } = this.rootStore;
    const { tokenFactory } = this.rootStore.settingsStore.currentVersionConfig;
    const wallet = await accountStore.getWallet();
    console.log(wallet);
    console.log("wallet in  mint", wallet);
    // const wallet2 = Wallet.fromPrivateKey(
    //   process.env.REACT_APP_SECRET!,
    //   accountStore.provider
    // );
    if (wallet == null) return;
    const tokenFactoryContract = TokenFactoryAbi__factory.connect(
      tokenFactory,
      wallet
    );

    try {
      const token = TOKENS_BY_ASSET_ID[assetId];
      const amount = BN.parseUnits(faucetAmounts[token.symbol], token.decimals);
      const hash = hashMessage(token.symbol);
      const userAddress = wallet.address.toB256();

      const { transactionResult } = await tokenFactoryContract.functions
        .mint({ value: userAddress }, hash, amount.toString())
        .txParams({ gasPrice: 2 })
        .call();
      if (transactionResult != null) {
        // this.setAlreadyMintedTokens([...this.alreadyMintedTokens, assetId]);
        const token = TOKENS_BY_ASSET_ID[assetId];
        this.rootStore.notificationStore.toast(
          `You have successfully minted ${token.symbol}`,
          {
            link: `${EXPLORER_URL}/transaction/${transactionResult.id}`,
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

  addAsset = async (assetId: string) => {
    if (assetId === TOKENS_BY_SYMBOL.ETH.assetId || window.fuel == null) return;
    const token = TOKENS_BY_ASSET_ID[assetId];
    const asset = {
      name: token.name,
      assetId: token.assetId,
      imageUrl: window.location.origin + token.logo,
      symbol: token.symbol,
      isCustom: true,
    };
    return window?.fuel.addAsset(asset);
  };
}
