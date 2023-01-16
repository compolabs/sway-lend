import styled from "@emotion/styled";
import React from "react";
import SizedBox from "@components/SizedBox";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import Text from "@src/components/Text";
import { Row } from "@src/components/Flex";
import Button from "@components/Button";
import Divider from "@src/components/Divider";
import { observer } from "mobx-react-lite";
import { TOKENS_BY_SYMBOL } from "@src/constants";
import TokenInput from "@components/TokenInput/TokenInput";
import BN from "@src/utils/BN";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  margin-top: 24px;
  @media (min-width: 880px) {
    margin-top: 0;
  }
`;
const Card = styled.div`
  display: flex;
  flex-direction: column;
  padding: 20px;
  background-color: ${({ theme }) => theme.colors.dashboard.cardBackground};
  border-radius: 4px;
`;

const ActionTab: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  const stats = [
    { title: "Supply APY", value: "60.17%" },
    { title: "Borrow APY", value: "12.32%" },
    { title: "Total liquidity", value: "$354,285.32" },
  ];
  const handleSupplyUsdcClick = () => {
    vm.setAction("supply");
    vm.setActionTokenAssetId(TOKENS_BY_SYMBOL.USDC.assetId);
  };

  return (
    <Root>
      {vm.action != null ? (
        <Card style={{ marginBottom: 24 }}>
          <Text fitContent weight={600} type="secondary" size="small">
            {vm.actionName}
          </Text>
          <SizedBox height={16} />
          <TokenInput
            decimals={vm.token.decimals}
            amount={vm.tokenAmount ?? BN.ZERO}
            setAmount={vm.setTokenAmount}
            assetId={vm.token.assetId}
          />
          <SizedBox height={8} />
          <Button fixed>{vm.actionName}</Button>
        </Card>
      ) : (
        <Row style={{ marginBottom: 24 }}>
          <Button onClick={handleSupplyUsdcClick} fixed>
            Supply USDC
          </Button>
          <SizedBox width={8} />
          <Button disabled fixed>
            Borrow USDC
          </Button>
        </Row>
      )}

      <Card>
        <SizedBox height={24} />
        <Text weight={600} type="secondary" size="small">
          Position summary
        </Text>
        <SizedBox height={16} />
        <Divider />
        <SizedBox height={12} />
        {stats.map(({ value, title }) => (
          <Row style={{ marginBottom: 12 }}>
            <Text weight={600}>{title}</Text>
            <Text textAlign="right" weight={600}>
              {value}
            </Text>
          </Row>
        ))}
      </Card>
    </Root>
  );
};
export default observer(ActionTab);
