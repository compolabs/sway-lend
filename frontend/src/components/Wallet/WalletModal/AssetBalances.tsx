import React, { HTMLAttributes } from "react";
import { observer } from "mobx-react-lite";
import InvestRow from "@components/InvestRow";
import { Column } from "@components/Flex";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import Button from "@components/Button";
import { ReactComponent as NotFoundIcon } from "@src/assets/notFound.svg";
import styled from "@emotion/styled";
import { useStores } from "@stores";
import Skeleton from "react-loading-skeleton";
import { FAUCET_URL } from "@src/constants";
import BN from "@src/utils/BN";

interface IProps extends HTMLAttributes<HTMLDivElement> {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  min-height: 400px;
`;

const AssetsBalances: React.FC<IProps> = () => {
  const { accountStore, pricesStore } = useStores();
  if (accountStore.assetBalances === null)
    return (
      <Root style={{ padding: "0 24px" }}>
        <Skeleton height={56} style={{ marginBottom: 8 }} count={3} />
      </Root>
    );
  return (
    <Root>
      {accountStore.balances.length !== 0 ? (
        accountStore.balances.map((b) => {
          const price =
            pricesStore.tokensPrices != null
              ? pricesStore.tokensPrices[b.assetId]
              : BN.ZERO;
          return (
            <InvestRow
              key={b.assetId}
              logo={b.logo}
              topLeftInfo={b.name}
              topRightInfo={b.formatBalance}
              bottomLeftInfo={"$ " + price.toFormat(2)}
              bottomRightInfo={b.formatUsdnEquivalent}
            />
          );
        })
      ) : (
        <Column justifyContent="center" alignItems="center" crossAxisSize="max">
          <SizedBox height={16} />
          <NotFoundIcon />
          <Text type="secondary" size="medium" textAlign="center">
            You don’t have any assets on your wallet.
            <br />
            Buy ETH to start trading.
          </Text>
          <SizedBox height={16} />
          <Button
            size="medium"
            onClick={() =>
              window.open(
                `${FAUCET_URL}/?address=${accountStore.address}`,
                "blank"
              )
            }
          >
            Buy Eth
          </Button>
          <SizedBox height={100} />
        </Column>
      )}
    </Root>
  );
};
export default observer(AssetsBalances);
