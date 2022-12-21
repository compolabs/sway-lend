import styled from "@emotion/styled";
import React, { HTMLAttributes } from "react";
import { Column, Row } from "@src/components/Flex";
import SizedBox from "@components/SizedBox";
import Text from "@components/Text";
import Balance from "@src/entities/Balance";
import SquareTokenIcon from "@components/SquareTokenIcon";

interface IProps extends HTMLAttributes<HTMLDivElement> {
  token: Balance;
  withClickLogic?: boolean;
  hidden?: boolean;
}

const Root = styled.div<{ withClickLogic?: boolean }>`
  position: relative;
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-sizing: border-box;
  width: 100%;
  cursor: ${({ withClickLogic }) => (withClickLogic ? "pointer" : "default")};
  padding: 10px 24px;

  :hover {
    background: ${({ withClickLogic, theme }) =>
      withClickLogic && `${theme.colors.primary100}`};
  }
`;
const DefaultIcon = styled.div`
  width: 40px;
  height: 40px;
  color: ${({ theme }) => theme.colors.primary100};
  border: 1px solid ${({ theme }) => theme.colors.primary100};
  border-radius: 8px;
`;
const Name = styled.div`
  font-style: normal;
  font-weight: normal;
  font-size: 14px;
  line-height: 20px;
  color: ${({ theme }) => theme.colors.primary800};
`;
const Symbol = styled.div`
  font-style: normal;
  font-weight: normal;
  font-size: 12px;
  line-height: 16px;
  color: ${({ theme }) => theme.colors.primary650};

  text-transform: uppercase;
`;
const Gradient = styled.div`
  display: flex;
  bottom: 0;
  left: 0;
  top: 0;
  width: 100%;
  position: absolute;
  background: ${({ theme }) => theme.colors.gradient};
  z-index: 10;
  cursor: not-allowed;
`;
const TokenInfo: React.FC<IProps> = ({ token, hidden, ...rest }) => {
  return (
    <Root {...rest}>
      {hidden && <Gradient />}
      <Row>
        {token.logo ? (
          <SquareTokenIcon size="small" src={token.logo} />
        ) : (
          <DefaultIcon />
        )}
        <SizedBox width={8} />
        <Column>
          <Name>{token.name}</Name>
          <Symbol>{token.symbol}</Symbol>
        </Column>
      </Row>
      <Column alignItems="flex-end">
        <Text style={{ whiteSpace: "nowrap" }} textAlign="right">
          {token.formatBalance}
        </Text>
        <Text
          style={{ whiteSpace: "nowrap" }}
          textAlign="right"
          type="secondary"
          size="small"
        >
          {token.formatUsdnEquivalent}
        </Text>
      </Column>
    </Root>
  );
};
export default TokenInfo;
