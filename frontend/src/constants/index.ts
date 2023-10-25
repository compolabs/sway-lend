import tokens from "./tokens.json";
import tokenLogos from "./tokenLogos";

export const ROUTES = {
  ROOT: "/",
  FAUCET: "/faucet",
  TUTORIALS: "/tutorials",
  TUTORIAL: "/tutorials/:tutorialId",
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
export const INDEXER_URL =
  "https://spark-indexer.spark-defi.com/api/sql/composabilitylabs/spark_indexer";

export const NODE_URL = "https://beta-4.fuel.network/graphql";
export const EXPLORER_URL =
  "https://fuellabs.github.io/block-explorer-v2/beta-3/#";
export const FAUCET_URL = "https://faucet-beta-4.fuel.network";
export const CONTRACT_ADDRESSES: IContractsConfig = {
  priceOracle:
    "0x8f7a76602f1fce4e4f20135a0ab4d22b3d9a230215ccee16c0980cf286aaa93c",
  market: "0x3fffc28bdb0a460263eeda9b56f9c5157c8048c25ed116c3a4e5cee78bb24bb9",
  tokenFactory:
    "0xd8c627b9cd9ee42e2c2bd9793b13bc9f8e9aad32e25a99ea574f23c1dd17685a",
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
  tokenFactory: string;
}
