import styled from "@emotion/styled";
import React from "react";
import SizedBox from "@components/SizedBox";
import { Observer } from "mobx-react-lite";
import { MarketVMProvider } from "@screens/Market/MarketVm";
import Layout from "@components/Layout";
import AssetsTable from "./AssetsTable";
import { Column } from "@components/Flex";
import MarketInfo from "@screens/Market/MarketInfo";

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
const Container = styled(Column)`
  width: 100%;

  & > :last-of-type {
    margin-top: 16px;
  }

  @media (min-width: 880px) {
    & > :last-of-type {
      margin-bottom: 16px;
    }

    flex-direction: column-reverse;
  }
`;
const MarketImpl: React.FC<IProps> = () => {
  return (
    <Layout>
      <Observer>
        {() => (
          <Root>
            <Container>
              <MarketInfo />
            </Container>
            <SizedBox height={40} />
            <AssetsTable />
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
