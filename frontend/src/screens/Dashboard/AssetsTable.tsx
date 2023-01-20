import styled from "@emotion/styled";
import { observer } from "mobx-react-lite";
import React from "react";
import TokenIcon from "@components/TokenIcon";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import { Column, Row } from "@src/components/Flex";
import Symbol from "@components/Symbol";
import { ACTION_TYPE, useDashboardVM } from "@screens/Dashboard/DashboardVm";
import { useStores } from "@stores";
import Tooltip from "@components/Tooltip";
import TokenInfo from "@screens/Dashboard/TokenInfo";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  width: 100%;
`;
const TokenRow = styled.div<{ selected?: boolean }>`
  display: grid;
  grid-template-columns: 6fr 6fr 4fr;
  padding: 8px 16px;
  align-items: center;
  justify-content: space-between;

  background: ${({ theme }) => theme.colors.dashboard.tokenRowColor};
  border-radius: 4px;
  margin-bottom: 2px;

  :hover {
    cursor: pointer;
    background: ${({ theme }) => theme.colors.dashboard.tokenRowSelected};
  }
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
  const { accountStore } = useStores();
  const vm = useDashboardVM();
  const handleAssetClick = (action: ACTION_TYPE, assetId: string) => {
    vm.setTokenAmount(null);
    vm.setAction(action);
    vm.setMode(0);
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
      {vm.collaterals.map((token) => {
        const userBalance = accountStore.getBalance(token);
        const canWithdraw = false;
        const canSupply = userBalance != null && userBalance.gt(0);
        return (
          <TokenRow key={token.assetId}>
            <Tooltip content={<TokenInfo assetId={token.assetId} />}>
              <Row alignItems="center">
                <TokenIcon size="small" src={token.logo} />
                <SizedBox width={20} />
                <Column>
                  <Text weight={600}>{token.name}</Text>
                  <Text weight={500} type="secondary">
                    {token.symbol}
                  </Text>
                </Column>
              </Row>
            </Tooltip>
            <div />
            <Row justifyContent="flex-end" alignItems="center">
              <Text type="secondary" size="small" fitContent>
                0.000
              </Text>
              <SizedBox width={24} />
              <Symbol
                type="plus"
                disabled={!canSupply}
                onClick={() =>
                  canSupply &&
                  handleAssetClick(ACTION_TYPE.SUPPLY, token.assetId)
                }
              />
              <SizedBox width={8} />
              <Symbol
                type="minus"
                disabled={!canWithdraw}
                onClick={() =>
                  canWithdraw &&
                  handleAssetClick(ACTION_TYPE.WITHDRAW, token.assetId)
                }
              />
            </Row>
          </TokenRow>
        );
      })}
    </Root>
  );
};
export default observer(AssetsTable);
