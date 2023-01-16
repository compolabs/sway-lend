import styled from "@emotion/styled";
import { observer } from "mobx-react-lite";
import React from "react";
import TokenIcon from "@components/TokenIcon";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import { Column, Row } from "@src/components/Flex";
import Symbol from "@components/Symbol";
import { useDashboardVM } from "@screens/Dashboard/DashboardVm";

interface IProps {}

const Root = styled.div`
  display: flex;
  flex-direction: column;
`;
const TokenRow = styled.div<{ disabled?: boolean }>`
  display: flex;
  flex-direction: row;
  padding: 8px 16px;
  align-items: center;
  justify-content: space-between;

  background: ${({ theme, disabled }) =>
    disabled
      ? theme.colors.dashboard.tokenRowDisabled
      : theme.colors.dashboard.tokenRowColor};
  ${({ disabled }) => disabled && "cursor: not-allowed;"}
  border-radius: 4px;
  margin-bottom: 2px;
`;

const AssetsTable: React.FC<IProps> = () => {
  const vm = useDashboardVM();
  return (
    <Root>
      {vm.marketTokens.map(({ logo, symbol, name }, index) => (
        <TokenRow disabled={index !== 0}>
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
          <Row justifyContent="flex-end" alignItems="center">
            <Text type="secondary" size="small" fitContent>
              0.000
            </Text>
            <SizedBox width={24} />
            <Symbol type="plus" />
            <SizedBox width={8} />
            <Symbol type="minus" />
          </Row>
        </TokenRow>
      ))}
    </Root>
  );
};
export default observer(AssetsTable);
