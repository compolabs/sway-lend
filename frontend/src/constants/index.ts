import tokens from "./tokens.json";
import tokenLogos from "./tokenLogos";

export const ROUTES = {
  ROOT: "/",
  FAUCET: "/faucet",
  DASHBOARD: "/dashboard",
  WALLET: "/wallet",
};

export const TOKENS_LIST: Array<IToken> = Object.values(tokens).map((t) => ({
  ...t,
  logo: tokenLogos[t.symbol],
}));
export const TOKENS_BY_SYMBOL: Record<string, IToken> = TOKENS_LIST.reduce(
  (acc, t) => ({ ...acc, [t.symbol]: t }),
  {}
);
export const TOKENS_BY_ASSET_ID: Record<string, IToken> = TOKENS_LIST.reduce(
  (acc, t) => ({ ...acc, [t.assetId]: t }),
  {}
);

export const NODE_URL = "https://beta-3.fuel.network/graphql";
export const EXPLORER_URL =
  "https://fuellabs.github.io/block-explorer-v2/beta-3/#";
export const FAUCET_URL = "https://faucet-beta-3.fuel.network";
export const SEED =
  "0x1586034a5486b0af22458ed29106865fee519b341bd260a2d9baad1d2a5c0d5b1505692de3ce6324ba5f306460d7db9a00ffebd5cefccf305da14c6c8fdde0f0";
export const CONTRACT_ADDRESSES: Record<string, IContractsConfig> = {
  "0.1.0": {
    priceOracle:
      "0xcff9283e360854a2f4523c6e5a569a9032a222b8ea6d91cdd1506f0375e5afb5",
    market:
      "0xfda921f2ca26c0515aba2a00e5bc7d4de7a7243711ec5ac325558b0e25826576",
  },
};

export interface IToken {
  logo: string;
  assetId: string;
  name: string;
  symbol: string;
  decimals: number;
}

export interface IContractsConfig {
  priceOracle: string;
  market: string;
}
