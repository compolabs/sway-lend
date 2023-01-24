import styled from "@emotion/styled";
import React from "react";
import { observer } from "mobx-react-lite";
import Text from "@components/Text";
import { TOKENS_BY_ASSET_ID } from "@src/constants";
import SizedBox from "@components/SizedBox";
import { Column, Row } from "@src/components/Flex";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";
import BN from "@src/utils/BN";
import { useStores } from "@stores";

interface IProps {
  assetId: string;
}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  background: ${({ theme }) => theme.colors.tokenTooltip.background};
  padding: 16px;
  width: 250px;
  box-sizing: border-box;
`;
const Container = styled(Column)`
  & > * {
    margin-bottom: 12px;
  }
`;

const TokenInfo: React.FC<IProps> = ({ assetId }) => {
  const { accountStore, pricesStore } = useStores();
  const vm = useDashboardVM();
  const token = TOKENS_BY_ASSET_ID[assetId];
  if (vm.collateralsData == null) return null;
  const stats = vm.collateralsData[assetId];
  const price = pricesStore.getFormattedTokenPrice(token);
  const penalty = BN.formatUnits(
    stats.liquidation_penalty.toString(),
    4
  ).toFormat(2);
  const collFactor = BN.formatUnits(
    stats.borrow_collateral_factor.toString(),
    4
  ).toFormat(2);
  const tokenData = [
    { title: "Oracle price", value: price },
    { title: "Collateral factor", value: collFactor + "%" },
    { title: "Liquidation penalty", value: penalty + "%" },
    { title: "Wallet balance", value: accountStore.getFormattedBalance(token) },
  ];
  return (
    <Root>
      <Text weight={600} size="medium">
        {token.name}
      </Text>
      <SizedBox height={12} />
      <Container crossAxisSize="max">
        {tokenData.map(({ title, value }, index) => (
          <Row key={index} alignItems="center" justifyContent="space-between">
            <Text fitContent weight={600} type="secondary">
              {title}
            </Text>
            <Text fitContent weight={600}>
              {value}
            </Text>
          </Row>
        ))}
      </Container>
    </Root>
  );
};
export default observer(TokenInfo);
