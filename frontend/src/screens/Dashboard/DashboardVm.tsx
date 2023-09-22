import React, { PropsWithChildren, useMemo } from "react";
import { makeAutoObservable, reaction } from "mobx";
import { RootStore, useStores } from "@stores";
import { useVM } from "@src/hooks/useVM";
import {
  EXPLORER_URL,
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
import { CollateralConfigurationOutput } from "@src/contracts/MarketAbi";
import { Contract } from "fuels";
import centerEllipsis from "@src/utils/centerEllipsis";

const ctx = React.createContext<DashboardVm | null>(null);

export enum ACTION_TYPE {
  SUPPLY = "SUPPLY",
  BORROW = "BORROW",
  REPAY = "REPAY",
  WITHDRAW = "WITHDRAW",
}

export const DashboardVMProvider: React.FC<PropsWithChildren> = ({
  children,
}) => {
  const rootStore = useStores();
  const store = useMemo(() => new DashboardVm(rootStore), [rootStore]);
  return <ctx.Provider value={store}>{children}</ctx.Provider>;
};
//todo fix maxbtn

export const useDashboardVM = () => useVM(ctx);

class DashboardVm {
  public rootStore: RootStore;

  constructor(rootStore: RootStore) {
    this.rootStore = rootStore;
    makeAutoObservable(this);
    this.updateMarketState().then(() => this.setInitialized(true));
    setInterval(this.updateMarketState, 20 * 1000);
    reaction(
      () => [
        this.rootStore.settingsStore.version,
        this.rootStore.accountStore.address,
      ],
      () => this.updateMarketStateWhenVersionOrAddressChanged()
    );
  }

  rejectUpdateStatePromise?: () => void;
  setRejectUpdateStatePromise = (v: any) => (this.rejectUpdateStatePromise = v);

  loading: boolean = false;
  private _setLoading = (l: boolean) => (this.loading = l);

  initialized: boolean = false;
  private setInitialized = (l: boolean) => (this.initialized = l);

  suppliedBalance: BN | null = null;
  setSuppliedBalance = (l: BN | null) => (this.suppliedBalance = l);

  supplyRate: BN | null = null;
  setSupplyRate = (l: BN | null) => (this.supplyRate = l);

  possibleSupplyRate: BN | null = null;
  setPossibleSupplyRate = (l: BN | null) => (this.possibleSupplyRate = l);

  borrowRate: BN | null = null;
  setBorrowRate = (l: BN | null) => (this.borrowRate = l);

  possibleBorrowRate: BN | null = null;
  setPossibleBorrowRate = (l: BN | null) => (this.possibleBorrowRate = l);

  baseTokenReserve: BN | null = null;
  setBaseTokenReserve = (l: BN | null) => (this.baseTokenReserve = l);

  totalLiquidity: BN | null = null;
  setTotalLiquidity = (l: BN | null) => (this.totalLiquidity = l);

  borrowedBalance: BN | null = null;
  setBorrowedBalance = (l: BN | null) => (this.borrowedBalance = l);

  maxBorrowBaseTokenAmount: BN | null = null;
  setMaxBorrowBaseTokenAmount = (l: BN | null) =>
    (this.maxBorrowBaseTokenAmount = l);

  get fixedMaxBorrowedAmount() {
    if (this.maxBorrowBaseTokenAmount == null) return BN.ZERO;
    return this.maxBorrowBaseTokenAmount;
  }

  collateralBalances: Record<string, BN> | null = null;
  setCollateralBalances = (l: Record<string, BN> | null) =>
    (this.collateralBalances = l);

  collateralReserves: Record<string, BN> | null = null;
  setCollateralReserves = (l: Record<string, BN> | null) =>
    (this.collateralReserves = l);

  assetsConfigs: Record<string, CollateralConfigurationOutput> | null = null;
  setAssetsConfigs = (
    l: Record<string, CollateralConfigurationOutput> | null
  ) => (this.assetsConfigs = l);

  collateralsData: Record<string, CollateralConfigurationOutput> | null = null;
  setCollateralData = (
    l: Record<string, CollateralConfigurationOutput> | null
  ) => (this.collateralsData = l);

  // initMarketSignedContract = async () => {
  //   const { accountStore } = this.rootStore;
  //   if (accountStore.address == null) return;
  //   const wallet = await accountStore.getWallet();
  //   const { market } = this.rootStore.settingsStore.currentVersionConfig;
  //   if (wallet != null) {
  //     const marketContract = MarketAbi__factory.connect(market, wallet);
  //     this.setMarketContractSigned(marketContract);
  //   }
  // };

  updateMarketState = async () => {
    const { accountStore } = this.rootStore;
    if (accountStore.address == null) return;
    const wallet = accountStore.walletToRead;
    if (wallet == null) return;
    const { market } = this.rootStore.settingsStore.currentVersionConfig;
    const marketContract = MarketAbi__factory.connect(market, wallet);
    if (this.rejectUpdateStatePromise != null) this.rejectUpdateStatePromise();

    const promise = new Promise((resolve, reject) => {
      this.rejectUpdateStatePromise = reject;
      resolve(
        Promise.all([
          this.getAssetsConfig(marketContract),
          this.updateTotalCollateralInfo(marketContract),
          this.updateAccountBalances(marketContract),
          this.updateSupplyAndBorrowRates(marketContract),
          // this.updateMarketBasic(marketContract),
          //error
          this.updateMaxBorrowAmount(marketContract),
          this.updateUserCollateralBalances(marketContract),
          this.updateTotalBaseTokenReserve(marketContract),
          this.updateTotalLiquidity(marketContract),
        ])
      );
    });

    promise
      .catch((v) => console.error(v))
      .finally(() => {
        this.setInitialized(true);
        this.setRejectUpdateStatePromise(undefined);
      });
  };

  getAssetsConfig = async (marketContract: MarketAbi) => {
    const { addressInput } = this.rootStore.accountStore;
    if (addressInput == null) return;
    const result = await marketContract.functions
      .get_collateral_configurations()
      .simulate();

    if (result.value != null) {
      const v = result.value.reduce((acc, res, index) => {
        if (res == null) return acc;
        return { ...acc, [res.asset_id]: res };
      }, {});
      this.setAssetsConfigs(v);
    }
  };

  updateTotalCollateralInfo = async (marketContract: MarketAbi) => {
    const { addressInput } = this.rootStore.accountStore;
    if (addressInput == null) return;
    const collaterals = this.collaterals;

    const functions = collaterals.map((b) =>
      marketContract.functions.totals_collateral(b.assetId).simulate()
    );
    const data = await Promise.all(functions);
    if (data.length > 0) {
      const v = data.reduce((acc, res, index) => {
        if (res == null) return acc;
        const assetId = collaterals[index].assetId;
        return { ...acc, [assetId]: new BN(res.value.toString()) };
      }, {});
      this.setCollateralReserves(v);
    }
  };

  updateMarketStateWhenVersionOrAddressChanged = async () => {
    if (this.rootStore.accountStore.address == null) {
      this.resetData();
      this.setInitialized(true);
      return;
    }
    this.setInitialized(false);
    this.setAction(null);
    this.setActionTokenAssetId(null);
    await this.updateMarketState();
  };

  updateAccountBalances = async (marketContract: MarketAbi) => {
    const { addressInput } = this.rootStore.accountStore;
    if (addressInput == null) return;
    const { value } = await marketContract.functions
      .get_user_supply_borrow(addressInput)
      .simulate();
    if (value == null) return;
    this.setSuppliedBalance(new BN(value[0].toString()));
    this.setBorrowedBalance(new BN(value[1].toString()));
  };

  // updateMarketBasic = async (marketContract: MarketAbi) => {
  //   const { addressInput } = this.rootStore.accountStore;
  //   if (addressInput == null) return;
  //   const { value } = await marketContract.functions
  //     .get_market_basics()
  //     .simulate();
  //   this.setMarketBasic(value);
  // };

  updateTotalBaseTokenReserve = async (marketContract: MarketAbi) => {
    const { value } = await marketContract.functions
      .balance_of(this.baseToken.assetId)
      .simulate();
    this.setBaseTokenReserve(new BN(value.toString()));
  };

  updateTotalLiquidity = async (marketContract: MarketAbi) => {
    const result = await marketContract.functions
      .balance_of(this.baseToken.assetId)
      .simulate();
    const result2 = await marketContract.functions.get_reserves().simulate();
    this.setTotalLiquidity(
      new BN(result.value.toString()).minus(result2.value.value.toString())
    );
  };

  updateMaxBorrowAmount = async (marketContract: MarketAbi) => {
    const { addressInput } = this.rootStore.accountStore;
    if (addressInput == null) return;
    const { priceOracle } = this.rootStore.settingsStore.currentVersionConfig;
    const oracle = new Contract(
      priceOracle,
      OracleAbi__factory.abi,
      this.rootStore.accountStore.provider
    );
    const { value } = await marketContract.functions
      .available_to_borrow(addressInput)
      .addContracts([oracle])
      .simulate();
    this.setMaxBorrowBaseTokenAmount(new BN(value.toString()));
  };

  updateUserCollateralBalances = async (marketContract: MarketAbi) => {
    const { addressInput } = this.rootStore.accountStore;
    if (addressInput == null) return;
    const collaterals = this.collaterals;

    const functions = collaterals.map((b) =>
      marketContract.functions
        .get_user_collateral(addressInput, b.assetId)
        .simulate()
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

  updateSupplyAndBorrowRates = async (marketContract: MarketAbi) => {
    const { addressInput } = this.rootStore.accountStore;
    if (addressInput == null) return;
    const { value } = await marketContract.functions
      .get_utilization()
      .simulate();
    // this.setUtilization(new BN(value.toString()));
    const [borrow, supply] = await Promise.all([
      marketContract.functions.get_borrow_rate(value).simulate(),
      marketContract.functions.get_supply_rate(value).simulate(),
    ]);
    this.setBorrowRate(new BN(borrow.value.toString()));
    this.setSupplyRate(new BN(supply.value.toString()));
  };

  collaterals: IToken[] = [
    TOKENS_BY_SYMBOL.SWAY,
    TOKENS_BY_SYMBOL.ETH,
    TOKENS_BY_SYMBOL.COMP,
    TOKENS_BY_SYMBOL.BTC,
    TOKENS_BY_SYMBOL.LINK,
    TOKENS_BY_SYMBOL.UNI,
  ];

  // marketContractSigned: MarketAbi | null = null;
  // setMarketContractSigned = (v: MarketAbi | null) =>
  //   (this.marketContractSigned = v);

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

  supplyBase = async (market: MarketAbi) => {
    if (this.tokenAmount == null || this.tokenAmount.lte(0)) return;

    return market.functions
      .supply_base()
      .callParams({
        forward: {
          amount: this.tokenAmount.toString(),
          assetId: this.baseToken.assetId,
        },
      })
      .call();
  };
  withdrawBase = async (market: any) => {
    if (this.tokenAmount == null || this.tokenAmount.lte(0)) return;

    return market.functions
      .withdraw_base(this.tokenAmount.toString())
      .txParams({ gasPrice: 1 })
      .call();
  };

  supplyCollateral = async (market: MarketAbi) => {
    if (
      this.tokenAmount == null ||
      this.actionTokenAssetId == null ||
      this.tokenAmount.eq(0)
    )
      return;

    return market.functions
      .supply_collateral()
      .callParams({
        forward: {
          assetId: this.actionTokenAssetId,
          amount: this.tokenAmount.toString(),
        },
      })
      .call();
  };

  withdrawCollateral = async (market: MarketAbi) => {
    if (
      this.tokenAmount == null ||
      this.actionTokenAssetId == null ||
      this.tokenAmount.lte(0)
    )
      return;
    const { priceOracle } = this.rootStore.settingsStore.currentVersionConfig;
    const oracle = new Contract(
      priceOracle,
      OracleAbi__factory.abi,
      this.rootStore.accountStore.provider
    );

    return market.functions
      .withdraw_collateral(this.actionTokenAssetId, this.tokenAmount.toString())
      .addContracts([oracle])
      .call();
  };

  borrowBase = async (market: MarketAbi) => {
    if (
      this.tokenAmount == null ||
      this.maxBorrowBaseTokenAmount == null ||
      this.tokenAmount.lte(0)
    )
      return;
    const { priceOracle } = this.rootStore.settingsStore.currentVersionConfig;
    const oracle = new Contract(
      priceOracle,
      OracleAbi__factory.abi,
      this.rootStore.accountStore.provider
    );
    return market.functions
      .withdraw_base(this.tokenAmount.toFixed(0))
      .txParams({ gasPrice: 1 })
      .addContracts([oracle])
      .call();
  };

  onMaxBtnClick() {
    if (
      this.actionTokenAssetId == null ||
      this.maxBorrowBaseTokenAmount == null ||
      this.baseTokenReserve == null
    )
      return null;

    switch (this.action) {
      case ACTION_TYPE.SUPPLY:
        const tokenBalance = this.rootStore.accountStore.findBalanceByAssetId(
          this.actionTokenAssetId
        );
        let balance = tokenBalance?.balance ?? BN.ZERO;
        if (this.actionTokenAssetId === TOKENS_BY_SYMBOL.ETH.assetId) {
          balance = balance.minus(500);
        }

        if (
          this.actionTokenAssetId !== this.baseToken.assetId &&
          tokenBalance?.balance?.gt(
            this.currentAssetCollateralCapacityLeft ?? BN.ZERO
          )
        ) {
          balance = this.currentAssetCollateralCapacityLeft ?? BN.ZERO;
        }
        this.setTokenAmount(balance);
        break;
      case ACTION_TYPE.WITHDRAW:
        if (this.actionTokenAssetId === this.baseToken.assetId) {
          this.setTokenAmount(this.suppliedBalance ?? BN.ZERO);
        } else {
          const balance =
            this.collateralBalances == null
              ? BN.ZERO
              : this.collateralBalances[this.actionTokenAssetId];
          this.setTokenAmount(balance);
        }
        break;
      case ACTION_TYPE.BORROW:
        if (this.maxBorrowBaseTokenAmount.gt(this.baseTokenReserve)) {
          this.setTokenAmount(this.baseTokenReserve);
          return;
        }
        this.setTokenAmount(this.fixedMaxBorrowedAmount);
        break;
      case ACTION_TYPE.REPAY:
        const balance1 = this.rootStore.accountStore.findBalanceByAssetId(
          this.baseToken.assetId
        );
        balance1?.balance?.gte(this.fixedMaxBorrowedAmount)
          ? this.setTokenAmount(this.borrowedBalance)
          : this.setTokenAmount(balance1?.balance ?? BN.ZERO);
        break;
    }
  }

  get currentAssetConfig() {
    if (this.actionTokenAssetId == null || this.assetsConfigs == null)
      return null;
    return this.assetsConfigs[this.actionTokenAssetId];
  }

  get currentAssetCollateralReserve() {
    if (this.actionTokenAssetId == null || this.collateralReserves == null)
      return null;
    return this.collateralReserves[this.actionTokenAssetId];
  }

  get currentAssetCollateralCapacityLeft() {
    if (
      this.currentAssetCollateralReserve == null ||
      this.currentAssetConfig == null
    )
      return null;
    return new BN(this.currentAssetConfig.supply_cap.toString()).minus(
      this.currentAssetCollateralReserve
    );
  }

  get tokenInputBalance(): string {
    if (
      this.actionTokenAssetId == null ||
      this.maxBorrowBaseTokenAmount == null ||
      this.baseTokenReserve == null
    )
      return "";
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
      if (this.maxBorrowBaseTokenAmount.gt(this.baseTokenReserve)) {
        return BN.formatUnits(
          this.baseTokenReserve ?? 0,
          this.baseToken.decimals
        ).toFormat(2);
      }
      return BN.formatUnits(
        this.maxBorrowBaseTokenAmount ?? BN.ZERO,
        this.baseToken.decimals
      ).toFormat(2);
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
        return BN.formatUnits(balance, this.actionToken.decimals).toFormat(2);
      }
    }
    return "";
  }

  marketAction = async () => {
    const { accountStore } = this.rootStore;
    this._setLoading(true);
    let marketContract = null;
    // if (this.marketContractSigned == null) {
    // const { accountStore } = this.rootStore;
    if (accountStore.address == null) return;
    const wallet = await accountStore.getWallet();
    const { market } = this.rootStore.settingsStore.currentVersionConfig;
    if (wallet != null) {
      marketContract = MarketAbi__factory.connect(market, wallet);
    }
    // }
    if (marketContract == null) return;
    let tx = null;
    try {
      if (this.action === ACTION_TYPE.SUPPLY) {
        if (this.actionTokenAssetId === this.baseToken.assetId) {
          tx = await this.supplyBase(marketContract);
        } else {
          tx = await this.supplyCollateral(marketContract);
        }
      }
      if (this.action === ACTION_TYPE.WITHDRAW) {
        if (this.actionTokenAssetId === this.baseToken.assetId) {
          tx = await this.withdrawBase(marketContract);
        } else {
          tx = await this.withdrawCollateral(marketContract);
        }
      }
      if (this.action === ACTION_TYPE.BORROW) {
        tx = await this.borrowBase(marketContract);
      }
      if (this.action === ACTION_TYPE.REPAY) {
        tx = await this.supplyBase(marketContract);
      }
      this.notifyThatActionIsSuccessful(tx?.transactionResult.id ?? "");
      this.hideAll();
      await accountStore.updateAccountBalances();
      await this.updateMarketState();
    } catch (e) {
      const { addErrorToLog } = this.rootStore.settingsStore;
      const err = {
        fuelAddress: this.rootStore.accountStore.address,
        address: this.rootStore.accountStore.addressB256,
        timestamp: new Date().getTime().toString(),
        action: this.action,
        errorMessage: e?.toString() ?? "",
      };
      console.log(e);
      addErrorToLog(err);
      const error = JSON.parse(JSON.stringify(e)).toString();
      this.rootStore.notificationStore.toast(error.error, {
        type: "error",
        title: "Oops..",
      });
    } finally {
      this._setLoading(false);
    }
  };

  get marketActionMainBtnState() {
    if (!this.initialized) return false;
    if (
      this.tokenAmount == null ||
      this.collateralBalances == null ||
      this.borrowedBalance == null ||
      this.tokenAmount.eq(0) ||
      this.actionTokenAssetId == null
    )
      return false;
    if (this.action === ACTION_TYPE.SUPPLY) {
      //base
      if (this.actionTokenAssetId === this.baseToken.assetId) {
        const balance = this.rootStore.accountStore.findBalanceByAssetId(
          this.baseToken.assetId
        );
        if (balance == null) return false;
        return balance.balance?.gte(this.tokenAmount);
      }
      //collateral
      else {
        const balance = this.rootStore.accountStore.findBalanceByAssetId(
          this.actionTokenAssetId
        );
        if (balance == null) return false;
        if (this.currentAssetCollateralCapacityLeft?.eq(0)) return false;
        if (this.tokenAmount.gt(this.currentAssetCollateralCapacityLeft ?? 0))
          return false;
        return balance.balance?.gte(this.tokenAmount);
      }
    }
    //if withdraw
    if (this.action === ACTION_TYPE.WITHDRAW) {
      if (this.actionTokenAssetId === this.baseToken.assetId) {
        if (this.suppliedBalance == null) return false;
        return this.suppliedBalance.gte(this.tokenAmount);
      }
      //collateral
      const balance = this.collateralBalances[this.actionTokenAssetId];
      return this.tokenAmount.lte(balance);
    }
    //if borrow
    if (this.action === ACTION_TYPE.BORROW) {
      if (this.baseTokenReserve?.eq(0)) return false;
      //if reserve is let than user collateral
      if (this.baseTokenReserve?.lt(this.fixedMaxBorrowedAmount)) {
        return this.tokenAmount?.lte(this.baseTokenReserve);
      }
      return this.tokenAmount.lte(this.fixedMaxBorrowedAmount);
    }
    //if repay
    if (this.action === ACTION_TYPE.REPAY) {
      const balance = this.rootStore.accountStore.findBalanceByAssetId(
        this.baseToken.assetId
      );
      return this.tokenAmount.lte(balance?.balance ?? BN.ZERO);
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
    return BN.formatUnits(this.tokenAmount, this.actionToken.decimals).toFormat(
      2
    );
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
    if (this.borrowRate == null || this.loading) return "0.00";
    const rate = BN.formatUnits(this.borrowRate, 18);
    const coefficient = new BN(365).times(24).times(60).times(60).times(100);
    return rate.times(coefficient).toFormat(2) + "%";
  }

  get supplyApr() {
    if (this.supplyRate == null || this.loading) return "0.00";
    const rate = BN.formatUnits(this.supplyRate, 18);
    const coefficient = new BN(365).times(24).times(60).times(60).times(100);
    return rate.times(coefficient).toFormat(2) + "%";
  }

  get possibleBorrowApr() {
    if (this.possibleBorrowRate == null) return null;
    const rate = BN.formatUnits(this.possibleBorrowRate, 18);
    const coefficient = new BN(365).times(24).times(60).times(60).times(100);
    return rate.times(coefficient).toFormat(2) + "%";
  }

  get possibleSupplyApr() {
    if (this.possibleSupplyRate == null) return null;
    const rate = BN.formatUnits(this.possibleSupplyRate, 18);
    const coefficient = new BN(365).times(24).times(60).times(60).times(100);
    return rate.times(coefficient).toFormat(2) + "%";
  }

  // calcPositionSummary = async () => {
  //   if (!this.initialized) return;
  //   if (
  //     this.action == null ||
  //     this.actionTokenAssetId == null ||
  //     this.marketContract == null ||
  //     this.marketBasic == null ||
  //     this.tokenAmount == null
  //   ) {
  //     return;
  //   }
  //   if (this.tokenAmount.eq(0)) {
  //     this.setPossibleSupplyRate(null);
  //     this.setPossibleBorrowRate(null);
  //   }
  //   if (
  //     this.collaterals
  //       .map((v) => v.assetId)
  //       .includes(this.actionTokenAssetId) ||
  //     this.action === ACTION_TYPE.REPAY ||
  //     this.action === ACTION_TYPE.WITHDRAW
  //   )
  //     return;
  //
  //   const { total_borrow_base, total_supply_base } = this.marketBasic;
  //
  //   let top = new BN(total_borrow_base.toString());
  //   let bottom = new BN(total_supply_base.toString());
  //
  //   if (this.action === ACTION_TYPE.BORROW) {
  //     top = top.plus(this.tokenAmount);
  //   }
  //   if (this.action === ACTION_TYPE.SUPPLY) {
  //     bottom = bottom.plus(this.tokenAmount);
  //   }
  //   const utilization = BN.parseUnits(top.div(bottom), 18).toFixed(0);
  //
  //   const [borrow, supply] = await Promise.all([
  //     this.marketContract.functions.get_borrow_rate(utilization).get(),
  //     this.marketContract.functions.get_supply_rate(utilization).get(),
  //   ]);
  //
  //   if (this.action === ACTION_TYPE.BORROW) {
  //     this.setPossibleBorrowRate(new BN(borrow.value.toString()));
  //   }
  //   if (this.action === ACTION_TYPE.SUPPLY) {
  //     this.setPossibleSupplyRate(new BN(supply.value.toString()));
  //   }
  // };

  get tokenInputError(): string | null {
    if (
      !this.initialized ||
      this.actionTokenAssetId == null ||
      this.maxBorrowBaseTokenAmount == null ||
      this.borrowedBalance == null ||
      this.baseTokenReserve == null ||
      this.collateralBalances == null
    )
      return null;
    if (this.tokenAmount == null || this.tokenAmount.eq(0)) return null;
    if (this.action === ACTION_TYPE.SUPPLY) {
      const balance = this.rootStore.accountStore.findBalanceByAssetId(
        this.actionTokenAssetId
      );
      if (balance == null) return null;
      if (balance.balance?.lt(this.tokenAmount)) return "Insufficient balance";
    }
    if (this.action === ACTION_TYPE.WITHDRAW) {
      if (this.actionTokenAssetId === this.baseToken.assetId) {
        if (this.tokenAmount.gt(this.suppliedBalance ?? BN.ZERO))
          return "Insufficient balance";
      } else {
        const balance = this.collateralBalances[this.actionTokenAssetId];
        if (this.tokenAmount.gt(balance ?? BN.ZERO))
          return "Insufficient balance";
      }
    }
    if (this.action === ACTION_TYPE.BORROW) {
      if (this.baseTokenReserve?.eq(0)) {
        return `There is no ${this.baseToken.symbol} to borrow`;
      }
      //if reserve is less than user collateral
      if (this.maxBorrowBaseTokenAmount.gt(this.baseTokenReserve)) {
        if (this.tokenAmount?.gt(this.baseTokenReserve ?? 0)) {
          const max = BN.formatUnits(
            this.baseTokenReserve,
            this.baseToken.decimals
          ).toFormat(2);
          return `Max to borrow is ${max} ${this.baseToken.symbol}`;
        }
        return null;
      }
      if (this.tokenAmount.gt(this.fixedMaxBorrowedAmount))
        return "Insufficient balance";
    }
    if (this.action === ACTION_TYPE.REPAY) {
      const balance = this.rootStore.accountStore.findBalanceByAssetId(
        this.baseToken.assetId
      );
      if (this.tokenAmount.gt(balance?.balance ?? BN.ZERO))
        return "Insufficient balance";
    }
    return null;
  }

  notifyThatActionIsSuccessful = (link: string) => {
    let action = "";
    switch (this.action) {
      case ACTION_TYPE.BORROW:
        action = "borrowed";
        break;
      case ACTION_TYPE.REPAY:
      case ACTION_TYPE.SUPPLY:
        action = "supplied";
        break;
      case ACTION_TYPE.WITHDRAW:
        action = "withdrawn";
        break;
    }
    this.rootStore.notificationStore.toast(
      `You have successfully ${action} ${this.formattedTokenAmount} ${this.actionToken.symbol}`,
      {
        link: `${EXPLORER_URL}/transaction/${link}`,
        linkTitle: "View on Explorer",
        copyTitle: `Copy tx id: ${centerEllipsis(link)}`,
        copyText: link,
        type: "success",
        title: "Transaction is completed!",
      }
    );
  };
  hideAll = () => {
    this.setAction(null);
    this.setActionTokenAssetId(null);
    this.setTokenAmount(null);
  };

  get totalSuppliedBalance() {
    if (!this.initialized || this.collateralBalances == null) return "0.00";
    const { getTokenPrice } = this.rootStore.pricesStore;
    const baseTokenBalance = BN.formatUnits(
      this.suppliedBalance ?? BN.ZERO,
      this.baseToken.decimals
    );
    const baseTokenPrice = getTokenPrice(this.baseToken.assetId);
    const collateralBalances = Object.entries(this.collateralBalances).reduce(
      (acc, [assetId, v]) => {
        const token = TOKENS_BY_ASSET_ID[assetId];
        const balance = BN.formatUnits(v, token.decimals);
        const dollBalance = getTokenPrice(assetId).times(balance);
        return acc.plus(dollBalance);
      },
      BN.ZERO
    );
    return baseTokenBalance
      .times(baseTokenPrice)
      .plus(collateralBalances)
      .toFormat(2);
  }

  get notification() {
    if (this.action == null) return null;
    if (this.action !== ACTION_TYPE.SUPPLY) return null;
    if (this.currentAssetCollateralCapacityLeft == null) return null;
    if (this.currentAssetCollateralCapacityLeft.eq(0)) {
      return `You can't supply more ${this.actionToken.symbol} because supply capacity is reached`;
    }
    return null;
  }

  resetData = () => {
    this.setCollateralReserves(null);
    this.setSuppliedBalance(null);
    this.setBorrowedBalance(null);
    this.setBorrowRate(null);
    this.setSupplyRate(null);
    // this.setMarketBasic(null);
    this.setMaxBorrowBaseTokenAmount(null);
    this.setCollateralBalances(null);
    this.setCollateralData(null);
    this.setBaseTokenReserve(null);
  };
}
