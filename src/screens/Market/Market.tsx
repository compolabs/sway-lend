import styled from "@emotion/styled";
import React, { useMemo, useState } from "react";
import Text from "@components/Text";
import SizedBox from "@components/SizedBox";
import { Observer } from "mobx-react-lite";
import { MarketVMProvider, useMarketVM } from "@screens/Market/MarketVm";
import Layout from "@components/Layout";
import Scrollbar from "@components/Scrollbar";
import Table from "@components/Table";
import { Column, Row } from "@src/components/Flex";
import SquareTokenIcon from "@components/SquareTokenIcon";
import Button from "@components/Button";
import BN from "@src/utils/BN";
import { useStores } from "@stores";
import { FAUCET_URL } from "@src/constants";

interface IProps {}

const Root = styled.div<{ apySort?: boolean; liquiditySort?: boolean }>`
  display: flex;
  flex-direction: column;
  justify-content: center;
  box-sizing: border-box;
  padding: 0 16px;
  width: 100%;
  min-height: 100%;
  max-width: calc(1160px + 32px);
  margin-bottom: 24px;
  margin-top: 40px;
  text-align: left;
  @media (min-width: 880px) {
    margin-top: 56px;
  }

  .apy-group {
    width: 20px;
    height: 20px;
    transform: ${({ apySort }) => (apySort ? "scale(1)" : "scale(1, -1)")};
  }

  .liquidity-group {
    width: 20px;
    height: 20px;
    transform: ${({ liquiditySort }) =>
      liquiditySort ? "scale(1)" : "scale(1, -1)"};
  }
`;

const MarketImpl: React.FC<IProps> = () => {
  const vm = useMarketVM();
  const { accountStore } = useStores();
  const [tokens, setTokens] = useState<any>([]);

  return (
    <Layout>
      <Observer>
        {() => (
          <Root>
            <Text weight={500} size="large">
              Market for Fuel Network
            </Text>
            <SizedBox height={16} />

            <Scrollbar
              style={{ maxWidth: "calc(100vw - 32px)", borderRadius: 16 }}
            ></Scrollbar>
          </Root>
        )}
      </Observer>
    </Layout>
  );
};

const Market: React.FC<IProps> = () => (
  <MarketVMProvider>
    <MarketImpl />
  </MarketVMProvider>
);
export default Market;
