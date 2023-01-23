import styled from "@emotion/styled";
import React from "react";
import { observer } from "mobx-react-lite";
import Text from "@components/Text";
import { TOKENS_BY_ASSET_ID } from "@src/constants";
import SizedBox from "@components/SizedBox";
import { Column, Row } from "@src/components/Flex";

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
  const token = TOKENS_BY_ASSET_ID[assetId];
  const tokenData = [
    { title: "Oracle price", value: "0.00" },
    { title: "Collateral factor", value: "0.00" },
    { title: "Liquidation factor", value: "0.00" },
    { title: "Oracle price", value: "0.00", divider: true },
    { title: "Protocol balance", value: "0.00" },
    { title: "Borrow capacity", value: "0.0000", divider: true },
    { title: "Wallet balance", value: "0.0000" },
    { title: "Borrow potential", value: "0.0000" },
  ];
  //todo add disconnected state
  return (
    <Root>
      <Text weight={600} size="medium">
        {token.name}
      </Text>
      <SizedBox height={12} />
      <Container crossAxisSize="max">
        {tokenData.map(({ title, value, divider }, index) => (
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
