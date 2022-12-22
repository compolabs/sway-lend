import RootStore from "@stores/RootStore";
import { makeAutoObservable, reaction } from "mobx";
import { Provider, Address } from "fuels";
import { NODE_URL, TOKENS_LIST } from "@src/constants";
import Balance from "@src/entities/Balance";
import BN from "@src/utils/BN";

export enum LOGIN_TYPE {
  FUEL_WALLET = "FUEL_WALLET",
}

export interface ISerializedAccountStore {
  address: string | null;
  loginType: LOGIN_TYPE | null;
}

class AccountStore {
  public readonly rootStore: RootStore;

  constructor(rootStore: RootStore, initState?: ISerializedAccountStore) {
    this.rootStore = rootStore;
    makeAutoObservable(this);
    if (initState) {
      this.setLoginType(initState.loginType);
      this.setAddress(initState.address);
    }
    // this.setAddress(
    //   "fuel10m84z5x0qnzjtkmlpj37gplw2n2wp65cypg45758cp5s6vv5cvysqy5v9l"
    // );
    this.updateAccountBalances().then();
    setInterval(this.updateAccountBalances, 10 * 1000);
    reaction(
      () => this.address,
      () => Promise.all([this.updateAccountBalances()])
    );
  }

  public address: string | null = null;
  setAddress = (address: string | null) => (this.address = address);

  public loginType: LOGIN_TYPE | null = null;
  setLoginType = (loginType: LOGIN_TYPE | null) => (this.loginType = loginType);

  public assetBalances: Balance[] | null = null;
  setAssetBalances = (v: Balance[] | null) => (this.assetBalances = v);

  updateAccountBalances = async () => {
    if (this.address == null) {
      this.setAssetBalances([]);
      return;
    }
    const provider = new Provider(NODE_URL);
    const address = Address.fromString(this.address);
    const balances = await provider.getBalances(address);
    const assetBalances = TOKENS_LIST.map((asset) => {
      const t = balances.find(({ assetId }) => asset.assetId === assetId);
      const balance = t != null ? new BN(t.amount.toString()) : BN.ZERO;
      if (t == null)
        return new Balance({ balance, usdEquivalent: BN.ZERO, ...asset });

      const usdEquivalent =
        BN.formatUnits(t.amount?.toString(), asset.decimals).times(
          asset.defaultPrice ?? 0
        ) ?? BN.ZERO;
      return new Balance({ balance, usdEquivalent, ...asset });
    });

    this.setAssetBalances(assetBalances);
  };
  findBalanceByAssetId = (assetId: string) =>
    this.assetBalances &&
    this.assetBalances.find((balance) => balance.assetId === assetId);

  get balances() {
    const { accountStore } = this.rootStore;
    return TOKENS_LIST.map((t) => {
      const balance = accountStore.findBalanceByAssetId(t.assetId);
      return balance ?? new Balance(t);
    })
      .filter((v) => v.usdEquivalent != null && v.usdEquivalent.gt(0))
      .sort((a, b) => {
        if (a.usdEquivalent == null && b.usdEquivalent == null) return 0;
        if (a.usdEquivalent == null && b.usdEquivalent != null) return 1;
        if (a.usdEquivalent == null && b.usdEquivalent == null) return -1;
        return a.usdEquivalent!.lt(b.usdEquivalent!) ? 1 : -1;
      });
  }

  serialize = (): ISerializedAccountStore => ({
    address: this.address,
    loginType: this.loginType,
  });

  login = async (loginType: LOGIN_TYPE) => {
    this.setLoginType(loginType);
    switch (loginType) {
      case LOGIN_TYPE.FUEL_WALLET:
        await this.loginWithFuelWallet();
        break;
      default:
        return;
    }
  };
  disconnect = async () => {
    switch (this.loginType) {
      case LOGIN_TYPE.FUEL_WALLET:
        await window.FuelWeb3?.disconnect();
        break;
      default:
        return;
    }
    this.setAddress(null);
  };

  loginWithFuelWallet = async () => {
    const config = { url: process.env.REACT_APP_PUBLIC_PROVIDER_URL };
    const res = await window.FuelWeb3?.connect(config);
    if (!res) {
      this.rootStore.notificationStore.notify("User denied", {
        type: "error",
      });
      return;
    }
    const accounts = await window.FuelWeb3?.accounts();
    if (accounts != null && accounts.length > 0) {
      this.setAddress(accounts[0]);
    }
  };
}

export default AccountStore;
