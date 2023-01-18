import styled from "@emotion/styled";
import { observer } from "mobx-react-lite";
import React from "react";
import TokenIcon from "@components/TokenIcon";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import { Column, Row } from "@src/components/Flex";
import Symbol from "@components/Symbol";
import { TAction, useDashboardVM } from "@screens/Dashboard/DashboardVm";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
`;
const TokenRow = styled.div<{ selected?: boolean }>`
  //display: flex;
  //flex-direction: row;
  display: grid;
  grid-template-columns: 6fr 6fr 4fr;
  //padding: 12px 16px;
  padding: 8px 16px;
  align-items: center;
  justify-content: space-between;

  background: ${({ theme, selected }) =>
    selected
      ? theme.colors.dashboard.tokenRowColor
      : theme.colors.dashboard.tokenRowSelected};
  border-radius: 4px;
  margin-bottom: 2px;
`;
const Header = styled.div`
  display: grid;
  grid-template-columns: 6fr 6fr 4fr;
  padding: 12px 16px;
  border-radius: 4px;
  margin-bottom: 2px;
  background: ${({ theme }) => theme.colors.dashboard.tokenRowColor};
`;

const AssetsTable: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  const handleAssetClick = (action: TAction, assetId: string) => {
    console.log("click");
    vm.setAction(action);
    vm.setActionTokenAssetId(assetId);
  };
  return (
    <Root>
      <Header>
        <Text size="small" type="secondary">
          Collateral asset
        </Text>
        <Text />
        <Text size="small" type="secondary">
          Protocol balance
        </Text>
      </Header>
      {vm.collaterals.map(({ logo, symbol, name, assetId }) => (
        <TokenRow selected={vm.actionTokenAssetId === assetId}>
          <Row alignItems="center">
            <TokenIcon size="small" src={logo} />
            <SizedBox width={20} />
            <Column>
              <Text weight={600}>{name}</Text>
              <Text weight={500} type="secondary">
                {symbol}
              </Text>
            </Column>
          </Row>
          <div />
          <Row justifyContent="flex-end" alignItems="center">
            <Text type="secondary" size="small" fitContent>
              0.000
            </Text>
            <SizedBox width={24} />
            <Symbol
              type="plus"
              onClick={() => handleAssetClick("supply", assetId)}
            />
            <SizedBox width={8} />
            <Symbol
              type="minus"
              onClick={() => handleAssetClick("withdraw", assetId)}
            />
          </Row>
        </TokenRow>
      ))}
    </Root>
  );
};
export default observer(AssetsTable);
