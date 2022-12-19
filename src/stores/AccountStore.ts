import RootStore from "@stores/RootStore";
import { makeAutoObservable } from "mobx";

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
  }

  public address: string | null = null;
  setAddress = (address: string | null) => (this.address = address);

  serialize = (): ISerializedAccountStore => ({
    address: this.address,
    loginType: this.loginType,
  });

  public loginType: LOGIN_TYPE | null = null;
  setLoginType = (loginType: LOGIN_TYPE | null) => (this.loginType = loginType);

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
