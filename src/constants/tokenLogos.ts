import eth from "@src/assets/tokens/ethereum.svg";
import usdt from "@src/assets/tokens/usdt.svg";
import bnb from "@src/assets/tokens/BNB.svg";
import btc from "@src/assets/tokens/bitcoin.svg";
import busd from "@src/assets/tokens/BUSD.svg";
import usdc from "@src/assets/tokens/usdc.svg";
import unknown from "@src/assets/notFound.svg";

const tokenLogos: Record<string, string> = {
  ETH: eth,
  USDT: usdt,
  BNB: bnb,
  BTC: btc,
  BUSD: busd,
  USDC: usdc,
  UNKNOWN: unknown,
};

export default tokenLogos;
