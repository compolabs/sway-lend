import React, { PropsWithChildren, useMemo } from "react";
import { makeAutoObservable, reaction } from "mobx";
import { RootStore, useStores } from "@stores";
import { useVM } from "@src/hooks/useVM";
import {
  CONTRACT_ADDRESSES,
  IToken,
  TOKENS_BY_ASSET_ID,
  TOKENS_BY_SYMBOL,
} from "@src/constants";
import BN from "@src/utils/BN";
import {
  MarketAbi,
  MarketAbi__factory,
  OracleAbi__factory,
} from "@src/contracts";
import { MarketBasicsOutput } from "@src/contracts/MarketAbi";
import { Contract } from "fuels";

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
    this.initMarketContract();
    this.updateMarketState().then(() => this.setInitialized(true));

    reaction(() => this.rootStore.accountStore.seed, this.initMarketContract);
  }

  loading: boolean = false;
  private _setLoading = (l: boolean) => (this.loading = l);

  initialized: boolean = false;
  private setInitialized = (l: boolean) => (this.initialized = l);

  suppliedBalance: BN | null = null;
  setSuppliedBalance = (l: BN | null) => (this.suppliedBalance = l);

  utilization: BN | null = null;
  setUtilization = (l: BN | null) => (this.supplyRate = l);

  supplyRate: BN | null = null;
  setSupplyRate = (l: BN | null) => (this.supplyRate = l);

  borrowRate: BN | null = null;
  setBorrowRate = (l: BN | null) => (this.borrowRate = l);

  marketBasic: MarketBasicsOutput | null = null;
  setMarketBasic = (l: MarketBasicsOutput | null) => (this.marketBasic = l);

  borrowedBalance: BN | null = null;
  setBorrowedBalance = (l: BN | null) => (this.borrowedBalance = l);

  maxBorrowBaseTokenAmount: BN | null = null;
  setMaxBorrowBaseTokenAmount = (l: BN | null) =>
    (this.maxBorrowBaseTokenAmount = l);

  collateralBalances: Record<string, BN> | null = null;
  setCollateralBalances = (l: Record<string, BN> | null) =>
    (this.collateralBalances = l);

  initMarketContract = () => {
    const { address, wallet } = this.rootStore.accountStore;
    if (address == null || wallet == null) return;
    const marketContract = MarketAbi__factory.connect(
      CONTRACT_ADDRESSES.market,
      wallet
    );
    this.setMarketContract(marketContract);
  };

  updateMarketState = () =>
    Promise.all([
      this.updateAccountInfo(),
      this.updateSupplyAndBorrowRates(),
      this.updateMarketBasic(),
      this.updateMaxBorrowAmount(),
      this.updateUserCollateralBalances(),
    ]);

  updateAccountInfo = async () => {
    const { addressInput } = this.rootStore.accountStore;
    if (this.marketContract == null || addressInput == null) return;
    const { value } = await this.marketContract.functions
      .get_user_supply_borrow(addressInput)
      .get();
    if (value == null) return;
    this.setSuppliedBalance(new BN(value[0].toString()));
    this.setBorrowedBalance(new BN(value[1].toString()));
  };

  updateMarketBasic = async () => {
    const { addressInput } = this.rootStore.accountStore;
    if (this.marketContract == null || addressInput == null) return;
    const { value } = await this.marketContract.functions
      .get_market_basics()
      .get();
    this.setMarketBasic(value);
  };

  updateMaxBorrowAmount = async () => {
    const { addressInput } = this.rootStore.accountStore;
    if (this.marketContract == null || addressInput == null) return;
    const oracle = new Contract(
      CONTRACT_ADDRESSES.priceOracle,
      OracleAbi__factory.abi
    );
    const { value } = await this.marketContract.functions
      .available_to_borrow(addressInput)
      .txParams({ gasLimit: (1e8).toString() })
      .addContracts([oracle])
      .get();
    this.setMaxBorrowBaseTokenAmount(new BN(value.toString()));
  };

  //fn get_user_collateral(address: Address, asset: ContractId)
  updateUserCollateralBalances = async () => {
    const { addressInput } = this.rootStore.accountStore;
    if (this.marketContract == null || addressInput == null) return;
    const collaterals = this.collaterals;

    const functions = collaterals.map((b) =>
      this.marketContract?.functions
        .get_user_collateral(addressInput, {
          value: b.assetId,
        })
        .get()
    );
    const data = await Promise.all(functions);
    if (data.length > 0) {
      const v = data.reduce((acc, res, index) => {
        if (res == null) return acc;
        const assetId = collaterals[index].assetId;
        return { ...acc, [assetId]: new BN(res.value.toString()) };
      }, {});
      this.setCollateralBalances(v);
    }
  };

  updateSupplyAndBorrowRates = async () => {
    const { addressInput } = this.rootStore.accountStore;
    if (this.marketContract == null || addressInput == null) return;
    const { value } = await this.marketContract.functions
      .get_utilization()
      .get();
    this.setUtilization(new BN(value.toString()));
    const [borrow, supply] = await Promise.all([
      this.marketContract.functions.get_borrow_rate(value).get(),
      this.marketContract.functions.get_supply_rate(value).get(),
    ]);
    this.setBorrowRate(new BN(borrow.value.toString()));
    this.setSupplyRate(new BN(supply.value.toString()));
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
      this.tokenAmount == null ||
      accountStore.seed == null ||
      this.marketContract == null ||
      this.tokenAmount.lte(0)
    )
      return;

    this._setLoading(true);

    await this.marketContract.functions
      .supply_base()
      .callParams({
        forward: {
          amount: this.tokenAmount.toString(),
          assetId: this.baseToken.assetId,
        },
      })
      .txParams({ gasPrice: 1 })
      .call()
      .catch((err) => {
        console.log("err", err);
        err != null &&
          notificationStore.notify("", { type: "error", title: "oops" });
        return;
      })
      .then(accountStore.updateAccountBalances)
      .then(this.updateMarketState)
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
  withdrawBase = async () => {
    const { accountStore, notificationStore } = this.rootStore;
    if (
      this.action !== ACTION_TYPE.WITHDRAW ||
      this.tokenAmount == null ||
      this.marketContract == null ||
      this.tokenAmount.lte(0)
    )
      return;

    this._setLoading(true);
    const am = this.tokenAmount.toString();

    await this.marketContract.functions
      .withdraw_base(am)
      .txParams({ gasPrice: 1 })
      .call()
      .catch((err) => {
        console.log("err", err);
        err != null &&
          notificationStore.notify("", { type: "error", title: "oops" });
        return;
      })
      .then(accountStore.updateAccountBalances)
      .then(this.updateMarketState)
      .finally(() => {
        notificationStore.notify(
          `You have successfully withdrawn ${this.formattedTokenAmount} ${this.baseToken.symbol}`,
          {
            type: "success",
            title: "Congrats!",
          }
        );
        this._setLoading(false);
      });
  };

  supplyCollateral = async () => {
    const { accountStore, notificationStore } = this.rootStore;
    if (
      this.tokenAmount == null ||
      this.actionTokenAssetId == null ||
      this.marketContract == null ||
      this.tokenAmount.eq(0)
    )
      return;

    this._setLoading(true);
    await this.marketContract.functions
      .supply_collateral()
      .callParams({
        forward: {
          assetId: this.actionTokenAssetId,
          amount: this.tokenAmount.toString(),
        },
      })
      .txParams({ gasPrice: 1 })
      .call()
      .catch((err) => {
        console.log("err", err);
        err != null &&
          notificationStore.notify("", { type: "error", title: "oops" });
        return;
      })
      .then(accountStore.updateAccountBalances)
      .then(this.updateMarketState)
      .finally(() => {
        notificationStore.notify(
          `You have successfully deposited ${this.formattedTokenAmount} ${this.actionToken.symbol}`,
          {
            type: "success",
            title: "Congrats!",
          }
        );
        this._setLoading(false);
      });
  };
  withdrawCollateral = async () => {
    const { accountStore, notificationStore } = this.rootStore;
    if (
      this.action !== ACTION_TYPE.WITHDRAW ||
      this.tokenAmount == null ||
      this.actionTokenAssetId == null ||
      this.marketContract == null ||
      this.tokenAmount.lte(0)
    )
      return;

    this._setLoading(true);
    const am = this.tokenAmount.toString();

    await this.marketContract.functions
      .withdraw_collateral({ value: this.actionTokenAssetId }, am)
      .txParams({ gasPrice: 1 })
      .call()
      .catch((err) => {
        console.log("err", err);
        err != null &&
          notificationStore.notify("", { type: "error", title: "oops" });
        return;
      })
      .then(accountStore.updateAccountBalances)
      .then(this.updateMarketState)
      .finally(() => {
        notificationStore.notify(
          `You have successfully withdrawn ${this.formattedTokenAmount} ${this.actionToken.symbol}`,
          {
            type: "success",
            title: "Congrats!",
          }
        );
        this._setLoading(false);
      });
  };

  borrowBase = async () => {
    console.log("borrowBase");
    const { accountStore, notificationStore } = this.rootStore;
    if (
      this.tokenAmount == null ||
      this.maxBorrowBaseTokenAmount == null ||
      this.marketContract == null ||
      this.tokenAmount.lte(0)
    )
      return;

    this._setLoading(true);
    const am = this.tokenAmount.toString();

    const oracle = new Contract(
      CONTRACT_ADDRESSES.priceOracle,
      OracleAbi__factory.abi
    );
    await this.marketContract.functions
      .withdraw_base(am)
      .txParams({ gasPrice: 1, gasLimit: (1e8).toString() })
      .addContracts([oracle])
      .call()
      .catch((err) => {
        console.log("err", err);
        err != null &&
          notificationStore.notify("", { type: "error", title: "oops" });
        return;
      })
      .then(accountStore.updateAccountBalances)
      .then(this.updateMarketState)
      .finally(() => {
        notificationStore.notify(
          `You have successfully borrowed ${this.formattedTokenAmount} ${this.baseToken.symbol}`,
          {
            type: "success",
            title: "Congrats!",
          }
        );
        this._setLoading(false);
      });
  };

  onMaxBtnClick() {
    if (this.actionTokenAssetId == null) return null;
    if (this.action === ACTION_TYPE.SUPPLY) {
      const tokenBalance = this.rootStore.accountStore.findBalanceByAssetId(
        this.actionTokenAssetId
      );
      this.setTokenAmount(tokenBalance?.balance ?? BN.ZERO);
    }
    if (this.action === ACTION_TYPE.WITHDRAW) {
      if (this.actionTokenAssetId === this.baseToken.assetId) {
        this.setTokenAmount(this.suppliedBalance ?? BN.ZERO);
      } else {
        const balance =
          this.collateralBalances == null
            ? BN.ZERO
            : this.collateralBalances[this.actionTokenAssetId];
        this.setTokenAmount(balance);
      }
    }
    if (this.action === ACTION_TYPE.BORROW) {
      //because this value has 9 decimals
      this.setTokenAmount(this.maxBorrowBaseTokenAmount?.div(1000) ?? BN.ZERO);
    }
  }

  get tokenInputBalance(): string {
    if (this.actionTokenAssetId == null) return "";
    if (
      this.action === ACTION_TYPE.SUPPLY ||
      this.action === ACTION_TYPE.REPAY
    ) {
      return (
        this.rootStore.accountStore.getFormattedBalance(this.actionToken) ??
        "0.00"
      );
    }
    if (this.action === ACTION_TYPE.BORROW) {
      if (this.actionToken === this.baseToken) {
        return BN.formatUnits(
          this.maxBorrowBaseTokenAmount ?? BN.ZERO,
          9
        ).toFormat(2);
      }
    }
    if (this.action === ACTION_TYPE.WITHDRAW) {
      if (this.actionToken === this.baseToken) {
        return BN.formatUnits(
          this.suppliedBalance ?? BN.ZERO,
          this.baseToken.decimals
        ).toFormat(2);
      } else {
        const balance =
          this.collateralBalances == null
            ? BN.ZERO
            : this.collateralBalances[this.actionTokenAssetId];
        return BN.formatUnits(balance, this.baseToken.decimals).toFormat(2);
      }
    }
    // if (this.action === ACTION_TYPE.REPAY) {
    //   return "return borrow";
    // }
    return "";
  }

  marketAction = () => {
    if (this.action === ACTION_TYPE.SUPPLY) {
      if (this.actionTokenAssetId === this.baseToken.assetId) {
        return this.supplyBase();
      } else {
        return this.supplyCollateral();
      }
    }
    if (this.action === ACTION_TYPE.WITHDRAW) {
      if (this.actionTokenAssetId === this.baseToken.assetId) {
        return this.withdrawBase();
      } else {
        return this.withdrawCollateral();
      }
    }
    if (this.action === ACTION_TYPE.BORROW) {
      return this.borrowBase();
    }
    if (this.action === ACTION_TYPE.REPAY) {
      return this.supplyBase();
    }
  };

  get marketActionMainBtnState() {
    if (!this.initialized) return false;
    if (
      this.tokenAmount == null ||
      this.tokenAmount.eq(0) ||
      this.actionTokenAssetId == null
    )
      return false;
    //if supply base token
    if (
      this.action === ACTION_TYPE.SUPPLY &&
      this.actionTokenAssetId === this.baseToken.assetId
    ) {
      const balance = this.rootStore.accountStore.findBalanceByAssetId(
        this.baseToken.assetId
      );
      if (balance == null) return false;
      return balance.balance?.gte(this.tokenAmount);
    }
    //if supply collateral
    if (
      this.action === ACTION_TYPE.SUPPLY &&
      this.collaterals
        .map(({ assetId }) => assetId)
        .includes(this.actionTokenAssetId)
    ) {
      const balance = this.rootStore.accountStore.findBalanceByAssetId(
        this.actionTokenAssetId
      );
      if (balance == null) return false;
      return balance.balance?.gte(this.tokenAmount);
    }
    //if withdraw base
    if (
      this.action === ACTION_TYPE.WITHDRAW &&
      this.actionTokenAssetId === this.baseToken.assetId
    ) {
      if (this.suppliedBalance == null || this.suppliedBalance.eq(0))
        return false;
      return this.suppliedBalance.gte(this.tokenAmount);
    }

    return true;
  }

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

  get borrowApr() {
    if (this.borrowRate == null) return null;
    const rate = BN.formatUnits(this.borrowRate, 18);
    return (
      rate.times(365).times(24).times(60).times(60).times(100).toFormat(2) + "%"
    );
  }

  get supplyApr() {
    if (this.supplyRate == null) return null;
    const rate = BN.formatUnits(this.supplyRate, 18);
    return (
      rate.times(365).times(24).times(60).times(60).times(100).toFormat(2) + "%"
    );
  }

  get totalLiquidity() {
    if (this.marketBasic == null) return null;
    const { total_borrow_base, total_supply_base } = this.marketBasic;
    const value = new BN(total_supply_base.toString()).minus(
      total_borrow_base.toString()
    );
    return "$" + BN.formatUnits(value, this.baseToken.decimals).toFormat(2);
  }
}
