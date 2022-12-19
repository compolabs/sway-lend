import { makeAutoObservable } from "mobx";
import AccountStore, { ISerializedAccountStore } from "@stores/AccountStore";
import SettingsStore, { ISerializedSettingsStore } from "@stores/SettingsStore";
import NotificationStore from "@stores/NotificationStore";

export interface ISerializedRootStore {
  accountStore?: ISerializedAccountStore;
  settingsStore?: ISerializedSettingsStore;
}

export default class RootStore {
  public accountStore: AccountStore;
  public settingsStore: SettingsStore;
  public notificationStore: NotificationStore;

  constructor(initState?: ISerializedRootStore) {
    this.accountStore = new AccountStore(this, initState?.accountStore);
    this.settingsStore = new SettingsStore(this, initState?.settingsStore);
    this.notificationStore = new NotificationStore(this);
    makeAutoObservable(this);
  }

  serialize = (): ISerializedRootStore => ({
    accountStore: this.accountStore.serialize(),
    settingsStore: this.settingsStore.serialize(),
  });
}
