import { IToken } from "@src/constants";
import BN from "@src/utils/BN";
import tokenLogos from "@src/constants/tokenLogos";

export interface IAssetBalance extends Omit<IToken, "logo"> {
  balance?: BN;
  usdEquivalent?: BN;
  logo?: string;
}

class Balance implements IAssetBalance {
  public readonly assetId: string;
  public readonly name: string;
  public readonly symbol: string;
  public readonly decimals: number;
  private readonly _logo?: string;
  public readonly balance?: BN;
  public readonly defaultPrice?: number;
  public readonly usdEquivalent?: BN;

  constructor(props: IAssetBalance) {
    this.name = props.name;
    this.assetId = props.assetId;
    this.symbol = props.symbol;
    this.decimals = props.decimals;
    this._logo = props.logo;
    this.balance = props.balance;
    this.defaultPrice = props.defaultPrice;
    this.usdEquivalent = props.usdEquivalent;
  }

  get logo() {
    return this._logo ?? tokenLogos[this.symbol] ?? tokenLogos.UNKNOWN;
  }

  get formatBalance() {
    if (this.balance == null) return "—";
    const value = BN.formatUnits(this.balance ?? 0, this.decimals);
    if (value.eq(0)) return value.toFormat(2);
    return value.gt(0.01) ? value.toFormat(2) : value.toFormat(6);
  }

  get formatUsdnEquivalent() {
    if (this.usdEquivalent == null) {
      return "—";
    }
    if (this.usdEquivalent.eq(0)) return `~ 0.00 $`;
    const v = this.usdEquivalent.gt(0.01)
      ? this.usdEquivalent.toFormat(2)
      : this.usdEquivalent.toFormat(6);
    return `~ ${v} $`;
  }
}

export default Balance;
