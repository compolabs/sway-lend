import styled from "@emotion/styled";
import React from "react";
import SizedBox from "@components/SizedBox";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import Text from "@src/components/Text";
import { Row } from "@src/components/Flex";
import Button from "@components/Button";
import Divider from "@src/components/Divider";
import { observer } from "mobx-react-lite";
import TokenInput from "@components/TokenInput/TokenInput";
import BN from "@src/utils/BN";
import useCollapse from "@components/Collapse";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  //margin-top: 24px;
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
  margin-bottom: 24px;
`;

const ActionTab: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  const stats = [
    { title: "Supply APY", value: "60.17%" },
    { title: "Borrow APY", value: "12.32%" },
    { title: "Total liquidity", value: "$354,285.32" },
  ];

  const handleCancelClick = () => {
    vm.setAction(null);
    vm.setActionTokenAssetId(null);
  };

  const { getCollapseProps } = useCollapse({
    isExpanded: vm.action != null,
    duration: 500,
  });
  return (
    <Root>
      <Card {...getCollapseProps()}>
        <Text fitContent weight={600} type="secondary" size="small">
          {vm.actionName} {vm.token.symbol}
        </Text>
        <SizedBox height={16} />
        <TokenInput
          decimals={vm.token.decimals}
          amount={vm.tokenAmount ?? BN.ZERO}
          setAmount={vm.setTokenAmount}
          assetId={vm.token.assetId}
        />
        <SizedBox height={8} />
        <Row>
          <Button kind="secondary" fixed onClick={handleCancelClick}>
            Cancel
          </Button>
          <SizedBox width={8} />
          <Button fixed>{vm.actionName}</Button>
        </Row>
      </Card>
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
