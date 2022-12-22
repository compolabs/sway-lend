import styled from "@emotion/styled";
import React, { useMemo, useState } from "react";
import Text from "@components/Text";
import SizedBox from "@components/SizedBox";
import { Observer } from "mobx-react-lite";
import { FaucetVMProvider, useFaucetVM } from "@screens/Faucet/FaucetVm";
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

const FaucetImpl: React.FC<IProps> = () => {
  const vm = useFaucetVM();
  const { accountStore } = useStores();
  const [tokens, setTokens] = useState<any>([]);
  useMemo(() => {
    setTokens(
      vm.faucetTokens.map((t) => ({
        asset: (
          <Row>
            <SquareTokenIcon size="small" src={t.logo} alt="logo" />
            <SizedBox width={16} />
            <Column crossAxisSize="max">
              <Text fitContent style={{ whiteSpace: "nowrap" }} weight={500}>
                {t.name}
              </Text>
              <Text
                fitContent
                style={{ whiteSpace: "nowrap" }}
                type="secondary"
              >
                $ {new BN(t.defaultPrice ?? 0).toFormat()}
              </Text>
            </Column>
          </Row>
        ),
        amount: (() => {
          const amount = new BN(20);
          const dollAmount = new BN(20);
          return (
            <Column crossAxisSize="max">
              <Text fitContent style={{ whiteSpace: "nowrap" }} weight={500}>
                {`${amount.toFormat()} ${t.symbol}`}
              </Text>
              <Text
                fitContent
                style={{ whiteSpace: "nowrap" }}
                type="secondary"
              >
                $ {dollAmount.toFormat()}
              </Text>
            </Column>
          );
        })(),
        balance: "$ ",
        btn: (
          <Button
            fixed
            size="medium"
            onClick={() => {
              if (t.symbol === "ETH") {
                window.open(
                  `${FAUCET_URL}/?address=${accountStore.address}`,
                  "blank"
                );
              }
            }}
          >
            Mint
          </Button>
        ),
      }))
    );
  }, []);
  const columns = React.useMemo(
    () => [
      {
        Header: "Asset",
        accessor: "asset",
      },
      {
        Header: "Mint amount",
        accessor: "amount",
      },
      {
        Header: "My balance",
        accessor: "balance",
      },
      {
        Header: " ",
        accessor: "btn",
      },
    ],
    []
  );
  return (
    <Layout>
      <Observer>
        {() => (
          <Root>
            <Text weight={500} size="large">
              Faucet for Fuel Network
            </Text>
            <SizedBox height={16} />

            <Scrollbar
              style={{ maxWidth: "calc(100vw - 32px)", borderRadius: 16 }}
            >
              <Table
                columns={columns}
                data={tokens}
                style={{
                  whiteSpace: "nowrap",
                  width: "fitContent",
                  minWidth: "fit-content",
                }}
              />
            </Scrollbar>
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
