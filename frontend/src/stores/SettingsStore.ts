import RootStore from "@stores/RootStore";
import { THEME_TYPE } from "@src/themes/ThemeProvider";
import { makeAutoObservable } from "mobx";
import { getCurrentBrowser } from "@src/utils/getCurrentBrowser";

export interface ISerializedSettingsStore {
  selectedTheme: THEME_TYPE | null;
}

class SettingsStore {
  public readonly rootStore: RootStore;

  constructor(rootStore: RootStore, initState?: ISerializedSettingsStore) {
    this.rootStore = rootStore;
    makeAutoObservable(this);
    if (initState != null) {
      initState.selectedTheme != null &&
        (this.selectedTheme = initState.selectedTheme);
    }
  }

  selectedTheme: THEME_TYPE = THEME_TYPE.DARK_THEME;

  toggleTheme = (): void => {
    this.selectedTheme =
      this.selectedTheme === THEME_TYPE.LIGHT_THEME
        ? THEME_TYPE.DARK_THEME
        : THEME_TYPE.LIGHT_THEME;
  };

  serialize = (): ISerializedSettingsStore => ({
    selectedTheme: this.selectedTheme,
  });

  walletModalOpened: boolean = false;
  setWalletModalOpened = (s: boolean) => (this.walletModalOpened = s);

  loginModalOpened: boolean = false;
  setLoginModalOpened = (s: boolean) => (this.loginModalOpened = s);

  get doesBrowserSupportsFuelWallet(): boolean {
    //todo
    //https://fuels-wallet.vercel.app/docs/browser-support/
    const browser = getCurrentBrowser();
    return ["chrome", "firefox", "brave", "edge"].includes(browser);
  }
}

export default SettingsStore;
