import styled from "@emotion/styled";
import React from "react";
import Text from "@components/Text";
import SizedBox from "@components/SizedBox";
import { Observer } from "mobx-react-lite";
import { FaucetVMProvider, useFaucetVM } from "@screens/Faucet/FaucetVm";
import Layout from "@components/Layout";
import TokensFaucetTable from "@screens/Faucet/TokensFaucetTable";
import Skeleton from "react-loading-skeleton";
import { useStores } from "@stores";

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

const FaucetImpl: React.FC<IProps> = () => {
  const vm = useFaucetVM();
  const { accountStore } = useStores();
  return (
    <Layout>
      <Observer>
        {() => {
          return (
            <Root>
              <Text weight={500} size="big">
                Faucet for Fuel Network
              </Text>
              {accountStore.address == null && (
                <>
                  <SizedBox height={8} />
                  <Text>Connect wallet to mint tokens</Text>
                  <SizedBox height={8} />
                </>
              )}
              <SizedBox height={16} />
              {vm.faucetTokens.length === 0 ? (
                <Skeleton height={70} style={{ margin: 4 }} count={5} />
              ) : (
                <TokensFaucetTable />
              )}
            </Root>
          );
        }}
      </Observer>
    </Layout>
  );
};

const Faucet: React.FC<IProps> = () => (
  <FaucetVMProvider>
    <FaucetImpl />
  </FaucetVMProvider>
);
export default Faucet;
