import RootStore from "@stores/RootStore";
import { makeAutoObservable, reaction } from "mobx";
import { Address, Provider, Wallet } from "fuels";
import { IToken, NODE_URL, TOKENS_LIST } from "@src/constants";
import Balance from "@src/entities/Balance";
import BN from "@src/utils/BN";
import { Mnemonic } from "@fuel-ts/mnemonic";

export enum LOGIN_TYPE {
  FUEL_WALLET = "FUEL_WALLET",
  GENERATE_FROM_SEED = "GENERATE_FROM_SEED",
}

export interface ISerializedAccountStore {
  address: string | null;
  loginType: LOGIN_TYPE | null;
  seed: string | null;
}

class AccountStore {
  public readonly rootStore: RootStore;

  //todo add change address if account was changed in extension

  constructor(rootStore: RootStore, initState?: ISerializedAccountStore) {
    this.rootStore = rootStore;
    makeAutoObservable(this);
    if (initState) {
      this.setLoginType(initState.loginType);
      this.setAddress(initState.address);
      this.setSeed(initState.seed);
      console.log("initState", initState);
    }
    this.updateAccountBalances().then();
    setInterval(this.updateAccountBalances, 10 * 1000);
    reaction(
      () => this.address,
      () => Promise.all([this.updateAccountBalances()])
    );
  }

  public address: string | null = null;
  setAddress = (address: string | null) => (this.address = address);

  public seed: string | null = null;
  setSeed = (seed: string | null) => (this.seed = seed);

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

      return new Balance({ balance, ...asset });
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
    seed: this.seed,
  });

  login = async (loginType: LOGIN_TYPE) => {
    this.setLoginType(loginType);
    switch (loginType) {
      // case LOGIN_TYPE.FUEL_WALLET:
      //   await this.loginWithFuelWallet();
      //   break;
      case LOGIN_TYPE.GENERATE_FROM_SEED:
        await this.generateAccountWithSeed();
        break;
      default:
        return;
    }
  };
  disconnect = async () => {
    this.setAddress(null);
    this.setSeed(null);
    this.setLoginType(null);
  };

  loginWithFuelWallet = async () => {
    const config = { url: NODE_URL };
    const fuel = window.fuel as any;
    const res = await fuel?.connect(config);
    if (!res) {
      this.rootStore.notificationStore.notify("User denied", {
        type: "error",
      });
      return;
    }
    const accounts = await window.fuel?.accounts();
    if (accounts != null && accounts.length > 0) {
      this.setAddress(accounts[0]);
    }
  };

  getFormattedBalance = (token: IToken): string | null => {
    const balance = this.findBalanceByAssetId(token.assetId);
    if (balance == null) return null;
    return BN.formatUnits(balance.balance ?? BN.ZERO, token.decimals).toFormat(
      2
    );
  };
  getBalance = (token: IToken): BN | null => {
    const balance = this.findBalanceByAssetId(token.assetId);
    if (balance == null) return null;
    return BN.formatUnits(balance.balance ?? BN.ZERO, token.decimals);
  };

  get isLoggedIn() {
    return this.address != null;
  }

  generateAccountWithSeed = () => {
    const mn = Mnemonic.generate();
    const seed = Mnemonic.mnemonicToSeed(mn);
    const wallet = Wallet.fromPrivateKey(seed, NODE_URL);
    this.setAddress(wallet.address.toAddress());
    this.setSeed(seed);
    this.rootStore.settingsStore.setLoginModalOpened(false);
    this.rootStore.notificationStore.notify("Go to faucet page to mint ETH");
  };

  get wallet() {
    if (this.seed == null) return null;
    const provider = new Provider(NODE_URL);
    return Wallet.fromPrivateKey(this.seed, provider);
  }
}

export default AccountStore;
