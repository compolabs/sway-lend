import RootStore from "@stores/RootStore";
import { makeAutoObservable, reaction, when } from "mobx";
import {
  Address,
  Mnemonic,
  Provider,
  Wallet,
  WalletLocked,
  WalletUnlocked,
} from "fuels";
import { IToken, NODE_URL, TOKENS_LIST } from "@src/constants";
import BN from "@src/utils/BN";
import Balance from "@src/entities/Balance";
import { Fuel, FuelWalletProvider } from "@fuel-wallet/sdk";

export enum LOGIN_TYPE {
  FUEL_WALLET = "Fuel Wallet",
  FUEL_DEV = "Fuel Wallet Development",
  FUELET = "Fuelet Wallet",
  GENERATE_SEED = "Generate seed",
}

export interface ISerializedAccountStore {
  address: string | null;
  loginType: LOGIN_TYPE | null;
  seed: string | null;
}

class AccountStore {
  public readonly rootStore: RootStore;
  public provider: Provider | null = null;

  public get initialized() {
    return this.provider != null;
  }

  private setProvider = (provider: Provider | null) =>
    (this.provider = provider);

  constructor(rootStore: RootStore, initState?: ISerializedAccountStore) {
    makeAutoObservable(this);

    this.rootStore = rootStore;
    if (initState) {
      this.setLoginType(initState.loginType);
      this.setAddress(initState.address);
      this.setSeed(initState.seed);
    }

    this.initFuel();
    this.initProvider();
    when(() => this.provider != null, this.updateAccountBalances);
    setInterval(this.updateAccountBalances, 10 * 1000);
    reaction(
      () => this.address,
      () => Promise.all([this.updateAccountBalances()])
    );
  }

  seed: string | null = null;
  setSeed = (seed: string | null) => (this.seed = seed);

  initFuel = () => {
    const fuel = new Fuel();
    this.setFuel(fuel);
    fuel.on(
      fuel.events.connectors,
      (connectors: Record<number, { name: string }>) => {
        const arr: any[] = Object.values(connectors).map((c) => c.name);
        this.setListConnectors(arr);
      }
    );
    fuel?.on(this.fuel?.events?.currentAccount, (address: string) =>
      this.setAddress(address)
    );
    fuel?.on(this.fuel?.events?.network, this.handleNetworkEvent);
  };

  listConnectors: string[] = [];
  setListConnectors = (value: string[]) => (this.listConnectors = value);

  fuel: any = null;
  setFuel = (fuel: any) => (this.fuel = fuel);

  initProvider = async () => {
    Provider.create(NODE_URL)
      .then((provider) => this.setProvider(provider))
      .catch(console.error);
  };

  public assetBalances: Balance[] | null = null;
  setAssetBalances = (v: Balance[] | null) => (this.assetBalances = v);

  updateAccountBalances = async () => {
    if (this.address == null) {
      this.setAssetBalances([]);
      return;
    }
    const address = Address.fromString(this.address);
    const balances = (await this.provider?.getBalances(address)) ?? [];
    const assetBalances = TOKENS_LIST.map((asset) => {
      const t = balances.find(({ assetId }) => asset.assetId === assetId);
      const balance = t != null ? new BN(t.amount.toString()) : BN.ZERO;
      if (t == null)
        return new Balance({ balance, usdEquivalent: BN.ZERO, ...asset });

      return new Balance({ balance, ...asset });
    });
    this.setAssetBalances(assetBalances);
  };

  getBalance = (token: IToken): BN | null => {
    const balance = this.findBalanceByAssetId(token.assetId);
    if (balance == null) return null;
    return balance.balance ?? BN.ZERO;
  };

  getFormattedBalance = (token: IToken): string | null => {
    const balance = this.findBalanceByAssetId(token.assetId);
    if (balance == null) return null;
    return BN.formatUnits(balance.balance ?? BN.ZERO, token.decimals).toFormat(
      2
    );
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

  handleNetworkEvent = (network: FuelWalletProvider) => {
    if (network.url !== NODE_URL) {
      this.rootStore.notificationStore.toast(
        `Please change network url to Testnet Beta 4`
      );
    }
  };

  public address: string | null = null;
  setAddress = (address: string | null) => (this.address = address);

  public loginType: LOGIN_TYPE | null = null;
  setLoginType = (loginType: LOGIN_TYPE | null) => (this.loginType = loginType);

  serialize = (): ISerializedAccountStore => ({
    address: this.address,
    loginType: this.loginType,
    seed: this.seed,
  });

  login = async (loginType: LOGIN_TYPE) => {
    this.setLoginType(loginType);
    if (loginType === LOGIN_TYPE.GENERATE_SEED) {
      console.log("genertate seed", loginType);
      this.loginWithMnemonicPhrase();
      return;
    }
    await this.loginWithWallet(loginType);
  };

  disconnect = async () => {
    try {
      if (this.loginType === LOGIN_TYPE.GENERATE_SEED) {
        this.setSeed(null);
        this.setAddress(null);
        return;
      }
      this.fuel?.disconnect();
    } catch (e) {
      this.setAddress(null);
      this.setSeed(null);
      this.setLoginType(null);
      return;
    }
    this.setAddress(null);
    this.setLoginType(null);
  };

  loginWithWallet = async (connector: LOGIN_TYPE) => {
    try {
      await this.fuel.selectConnector(connector);
      await this.fuel.connect();
      const account = await this.fuel.currentAccount();
      const provider = await this.fuel.getProvider();
      if (provider.url !== NODE_URL) {
        this.rootStore.notificationStore.toast(
          `Please change network url to beta 4`
        );
      }
      this.setAddress(account);
    } catch (e) {
      this.rootStore.notificationStore.toast("e?.toString()", {
        type: "error",
      });
      return;
    }
  };

  loginWithMnemonicPhrase = () => {
    const mnemonic = Mnemonic.generate(16);
    console.log("mnemonic", mnemonic);
    this.setSeed(mnemonic);
    const seed = Mnemonic.mnemonicToSeed(mnemonic);
    if (this.provider == null) return;
    const wallet = Wallet.fromPrivateKey(seed, this.provider);
    this.setAddress(wallet.address.toAddress());
    this.rootStore.notificationStore.toast(
      "You can copy your seed in account section",
      { type: "info" }
    );
  };

  get isLoggedIn() {
    return this.address != null;
  }

  getWallet = async (): Promise<WalletLocked | WalletUnlocked | null> => {
    if (this.loginType === LOGIN_TYPE.GENERATE_SEED) {
      if (this.seed == null) return null;
      const seed = Mnemonic.mnemonicToSeed(this.seed);
      const provider = await Provider.create(NODE_URL);
      return Wallet.fromPrivateKey(seed, provider);
    }
    if (this.address == null || this.fuel == null) return null;
    return this.fuel.getWallet(this.address);
  };

  get walletToRead(): WalletLocked | null {
    return this.provider == null
      ? null
      : Wallet.fromAddress(
          "fuel1m56y48mej3366h6460y4rvqqt62y9vn8ad3meyfa5wkk5dc6mxmss7rwnr",
          this.provider ?? ""
        );
  }

  get addressInput(): null | { value: string } {
    if (this.address == null) return null;
    return { value: Address.fromString(this.address).toB256() };
  }

  get addressB256(): null | string {
    if (this.address == null) return null;
    return Address.fromString(this.address).toB256();
  }
}

export default AccountStore;
