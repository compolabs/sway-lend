import React, { PropsWithChildren, useMemo } from "react";
import { action, makeAutoObservable, reaction } from "mobx";
import { RootStore, useStores } from "@stores";
import { useVM } from "@src/hooks/useVM";
import {
  CONTRACT_ADDRESSES,
  IToken,
  TOKENS_BY_ASSET_ID,
  TOKENS_BY_SYMBOL,
} from "@src/constants";
import BN from "@src/utils/BN";
import { MarketAbi, MarketAbi__factory } from "@src/contracts";
import { log } from "util";

const ctx = React.createContext<DashboardVm | null>(null);

export enum ACTION_TYPE {
  SUPPLY,
  BORROW,
  REPAY,
  WITHDRAW,
}

export const DashboardVMProvider: React.FC<PropsWithChildren> = ({
  children,
}) => {
  const rootStore = useStores();
  const store = useMemo(() => new DashboardVm(rootStore), [rootStore]);
  return <ctx.Provider value={store}>{children}</ctx.Provider>;
};

export const useDashboardVM = () => useVM(ctx);

class DashboardVm {
  public rootStore: RootStore;

  constructor(rootStore: RootStore) {
    this.rootStore = rootStore;
    makeAutoObservable(this);
    this.initMarketContract().then(this.updateAccountInfo);
    reaction(() => this.rootStore.accountStore.seed, this.initMarketContract);
  }

  loading: boolean = false;
  private _setLoading = (l: boolean) => (this.loading = l);

  initMarketContract = async () => {
    const { address, wallet } = this.rootStore.accountStore;
    if (address == null || wallet == null) return;
    const marketContract = MarketAbi__factory.connect(
      CONTRACT_ADDRESSES.market,
      wallet
    );
    this.setMarketContract(marketContract);
  };

  updateAccountInfo = async () => {
    const { address } = this.rootStore.accountStore;
    if (this.marketContract == null || address == null) return;
    // const addressInput = { value: address };
    const v = await this.marketContract.functions.get_user_supply_borrow({
      value: "fuel1s9a20mkrgcw9uulpjpewa38mspzavkr4arw6rhay8htumw7spdaqgvnkhd",
    });
    console.log(v);
    // const supplyAndBase=
  };

  collaterals: IToken[] = [
    TOKENS_BY_SYMBOL.SWAY,
    TOKENS_BY_SYMBOL.ETH,
    TOKENS_BY_SYMBOL.BTC,
    TOKENS_BY_SYMBOL.LINK,
    TOKENS_BY_SYMBOL.UNI,
  ];

  marketContract: MarketAbi | null = null;
  setMarketContract = (v: MarketAbi | null) => (this.marketContract = v);

  mode: 0 | 1 = 0;
  setMode = (v: 0 | 1) => (this.mode = v);

  action: ACTION_TYPE | null = null;
  setAction = (l: ACTION_TYPE | null) => (this.action = l);

  tokenAmount: BN | null = null;
  setTokenAmount = (l: BN | null) => (this.tokenAmount = l);

  actionTokenAssetId: string | null = null;
  setActionTokenAssetId = (l: string | null) => (this.actionTokenAssetId = l);

  isThereUserData = false;

  get tokenBtnsClick(): Record<string, [() => void]> {
    return this.collaterals.reduce((acc, v) => {
      const c = () => {
        console.log("click click", v.symbol);
      };
      return { ...acc, [v.assetId]: [c] };
    }, {} as Record<string, [() => void]>);
  }

  get actionToken() {
    if (this.actionTokenAssetId == null) return TOKENS_BY_SYMBOL.USDC;
    return TOKENS_BY_ASSET_ID[this.actionTokenAssetId];
  }

  get baseToken() {
    return TOKENS_BY_SYMBOL.USDC;
  }

  supplyBase = async () => {
    const { accountStore, notificationStore } = this.rootStore;
    if (
      this.action !== ACTION_TYPE.SUPPLY ||
      this.tokenAmount == null ||
      accountStore.seed == null ||
      this.marketContract == null ||
      this.tokenAmount.lte(0)
    ) {
      console.log(
        this.action !== ACTION_TYPE.SUPPLY,
        this.tokenAmount == null,
        accountStore.seed == null,
        this.marketContract == null,
        this.tokenAmount?.lte(0)
      );
      return;
    }
    this._setLoading(true);

    await this.marketContract.functions
      .supply_base()
      .callParams({
        forward: [this.tokenAmount.toString(), this.baseToken.assetId],
      })
      .txParams({ gasPrice: 1 })
      .call()
      .catch((e) =>
        notificationStore.notify("", { type: "error", title: "oops" })
      )
      .then(accountStore.updateAccountBalances)
      .finally(() => {
        notificationStore.notify(
          `You have successfully deposited ${this.formattedTokenAmount} ${this.baseToken.symbol}`,
          {
            type: "success",
            title: "Congrats!",
          }
        );
        this._setLoading(false);
      });
  };

  supplyCollateral = async () => {
    console.log("supplyCollateral");
  };
  borrowBase = async () => {
    console.log("borrowBase");
  };
  withdrawBase = async () => {
    console.log("withdrawBase");
  };

  onMaxBtnClick() {
    if (this.actionTokenAssetId == null) return null;
    if (this.action === ACTION_TYPE.SUPPLY) {
      const tokenBalance = this.rootStore.accountStore.findBalanceByAssetId(
        this.actionTokenAssetId
      );
      this.setTokenAmount(tokenBalance?.balance ?? BN.ZERO);
    }
  }

  get tokenInputBalance(): string {
    if (this.actionTokenAssetId == null) return "";
    else if (this.action === ACTION_TYPE.SUPPLY) {
      return (
        this.rootStore.accountStore.getFormattedBalance(this.actionToken) ??
        "0.00"
      );
    } else if (
      this.action === ACTION_TYPE.BORROW &&
      this.actionToken === this.baseToken
    ) {
      return "borrow base";
    } else if (
      this.action === ACTION_TYPE.WITHDRAW &&
      this.actionToken === this.baseToken
    ) {
      return "WITHDRAW base";
    } else if (this.action === ACTION_TYPE.REPAY) {
      return "return borrow";
    }
    return "";
  }

  marketAction = () => {
    if (
      this.action === ACTION_TYPE.SUPPLY &&
      this.actionTokenAssetId === this.baseToken.assetId
    ) {
      return this.supplyBase();
    }
    if (
      this.action === ACTION_TYPE.SUPPLY &&
      this.collaterals.map((v) => v.assetId).includes(this.baseToken.assetId)
    ) {
      return this.supplyCollateral();
    }
  };

  get formattedTokenAmount() {
    if (
      this.tokenAmount == null ||
      this.tokenAmount?.eq(0) ||
      this.actionTokenAssetId == null
    )
      return "0.00";
    return BN.formatUnits(
      this.tokenAmount,
      TOKENS_BY_ASSET_ID[this.actionTokenAssetId].decimals
    ).toFormat(2);
  }

  get operationName() {
    switch (this.action) {
      case ACTION_TYPE.SUPPLY:
        return "Supply";
      case ACTION_TYPE.BORROW:
        return "Borrow";
      case ACTION_TYPE.REPAY:
        return "Repay";
      case ACTION_TYPE.WITHDRAW:
        return "Withdraw";
    }
    return "";
  }
}
