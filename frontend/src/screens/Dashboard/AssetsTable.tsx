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
import Skeleton from "react-loading-skeleton";
import BN from "@src/utils/BN";

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

const TokenRowSkeleton = () => (
  <TokenRow>
    <Row>
      <Skeleton width={40} height={40} style={{ borderRadius: 50 }} />
      <SizedBox width={20} />
      <Column>
        <Skeleton height={24} width={100} />
        <Skeleton height={16} width={100} />
      </Column>
    </Row>
    <div />
    <Skeleton height={16} width={100} />
  </TokenRow>
);
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
        const canSupply = userBalance != null && userBalance.gt(0);
        const walletBalance = accountStore.getFormattedBalance(token);
        const protocolBalance =
          vm.collateralBalances != null
            ? vm.collateralBalances[token.assetId]
            : BN.ZERO;
        const canWithdraw = protocolBalance.gt(0);
        const protocolBalanceFormatted = BN.formatUnits(
          protocolBalance,
          token.decimals
        ).toFormat(2);

        if (!vm.initialized) return <TokenRowSkeleton key={token.assetId} />;
        return (
          <TokenRow key={token.assetId}>
            <Tooltip content={<TokenInfo assetId={token.assetId} />}>
              <Row alignItems="center">
                <TokenIcon size="small" src={token.logo} />
                <SizedBox width={20} />
                <Column>
                  <Text weight={600}>{token.name}</Text>
                  <Text size="small" weight={600} type="secondary">
                    {accountStore.isLoggedIn
                      ? `${token.symbol} • ${walletBalance} in wallet`
                      : `${token.symbol}`}
                  </Text>
                </Column>
              </Row>
            </Tooltip>
            <div />
            <Row justifyContent="flex-end" alignItems="center">
              <Text type="secondary" size="small" fitContent>
                {protocolBalanceFormatted}
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
