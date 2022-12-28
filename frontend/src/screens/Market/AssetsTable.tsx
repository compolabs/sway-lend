import React, { useMemo, useState } from "react";
import { useStores } from "@stores";
import { Column, Row } from "@components/Flex";
import SquareTokenIcon from "@components/SquareTokenIcon";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import Button from "@components/Button";
import Scrollbar from "@components/Scrollbar";
import Table from "@components/Table";
import { useMarketVM } from "@screens/Market/MarketVm";
import styled from "@emotion/styled";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
`;
const AssetsTable: React.FC<IProps> = () => {
  const { pricesStore } = useStores();
  const vm = useMarketVM();
  const [tokens, setTokens] = useState<any>([]);
  useMemo(() => {
    setTokens(
      vm.marketTokens.map((t) => ({
        asset: (
          <Row>
            <SquareTokenIcon size="small" src={t.logo} alt="logo" />
            <SizedBox width={16} />
            <Column crossAxisSize="max">
              <Text size="medium" fitContent style={{ whiteSpace: "nowrap" }}>
                {t.name}
              </Text>
              <Text
                fitContent
                style={{ whiteSpace: "nowrap" }}
                type="secondary"
                size="small"
              >
                $ {pricesStore.getTokenPrice(t.assetId).toFormat(2)}
              </Text>
            </Column>
          </Row>
        ),
        supplied: (
          <Column crossAxisSize="max">
            <Text fitContent style={{ whiteSpace: "nowrap" }}>
              10 000 {t.symbol} / 71 312 {t.symbol}
            </Text>
            <Text fitContent style={{ whiteSpace: "nowrap" }} type="secondary">
              $ 10 000 / 71 312
            </Text>
          </Column>
        ),
        apy: (
          <Text fitContent style={{ whiteSpace: "nowrap" }}>
            20.32% / 49.51%
          </Text>
        ),
        total: (
          <Column crossAxisSize="max">
            <Text fitContent style={{ whiteSpace: "nowrap" }}>
              71.7M {t.symbol} / 35M {t.symbol}
            </Text>
            <Text fitContent style={{ whiteSpace: "nowrap" }} type="secondary">
              $ 71.7M
            </Text>
          </Column>
        ),
        btn: (() => {
          const functions = vm.tokenBtnsClick[t.assetId];
          if (t.symbol === "USDC") {
            return (
              <Row>
                <Button fixed size="medium" onClick={functions[0]}>
                  Supply
                </Button>
                <SizedBox width={8} />
                <Button fixed size="medium" onClick={functions[0]}>
                  Borrow
                </Button>
              </Row>
            );
          }
          return (
            <Button fixed size="medium" onClick={functions[0]}>
              Supply
            </Button>
          );
        })(),
      }))
    );
  }, [pricesStore.tokensPrices, vm.marketTokens]);
  //todo change deps
  const columns = React.useMemo(
    () => [
      {
        Header: "Asset",
        accessor: "asset",
      },
      {
        Header: "Supplied / Balance",
        accessor: "supplied",
      },
      {
        Header: "Supply APY / Borrow APY",
        accessor: "apy",
      },
      {
        Header: "Total Supply / Debt",
        accessor: "total",
      },
      {
        Header: " ",
        accessor: "btn",
      },
    ],
    [pricesStore, vm.marketTokens]
  );
  return (
    <Root>
      <Text weight={500} size="big">
        All assets
      </Text>
      <SizedBox height={16} />
      <Scrollbar style={{ maxWidth: "calc(100vw - 32px)", borderRadius: 16 }}>
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
  );
};
export default AssetsTable;
