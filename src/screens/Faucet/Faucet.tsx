import styled from "@emotion/styled";
import React from "react";
import Text from "@components/Text";
import SizedBox from "@components/SizedBox";
import { Observer } from "mobx-react-lite";
import { FaucetVMProvider, useFaucetVM } from "@screens/Faucet/FaucetVm";
import Layout from "@components/Layout";
import Card from "@src/components/Card";
import FaucetBtn from "@screens/Faucet/FaucetBtn";
import { useStores } from "@stores";
import centerEllipsis from "@src/utils/centerEllipsis";
import TokenInput from "@components/TokenInput/TokenInput";
import { TOKENS_BY_ASSET_ID } from "@src/constants";
import { Row } from "@src/components/Flex";

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
const Subtitle = styled(Text)`
  @media (min-width: 880px) {
    max-width: 560px;
  }
`;
const FaucetImpl: React.FC<IProps> = () => {
  const vm = useFaucetVM();
  const { accountStore } = useStores();
  return (
    <Layout>
      <Observer>
        {() => (
          <Root>
            <Text weight={500} size="large">
              Faucet for Fuel Network
            </Text>
            <SizedBox height={4} />

            <SizedBox height={24} />
            <Card>
              <Subtitle fitContent>Choose token to be minted</Subtitle>
              {accountStore.address != null ? (
                <Row>
                  <SizedBox height={4} />
                  <Subtitle size="medium" fitContent>
                    {`Target address is ${centerEllipsis(
                      accountStore.address,
                      10
                    )}`}
                    <SizedBox height={16} />
                    <TokenInput
                      setAssetId={(v) =>
                        vm.setTokenForMint(TOKENS_BY_ASSET_ID[v])
                      }
                      decimals={vm.tokenForMint.decimals}
                      assetId={vm.tokenForMint.assetId}
                      balances={vm.tokensForMint ?? []}
                    />
                  </Subtitle>
                </Row>
              ) : (
                <Subtitle size="medium" fitContent>
                  Connect wallet to mint
                </Subtitle>
              )}
              <SizedBox height={16} />
              <FaucetBtn />
            </Card>
          </Root>
        )}
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
