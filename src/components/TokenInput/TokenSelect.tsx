import styled from "@emotion/styled";
import React, { HTMLAttributes } from "react";
import SizedBox from "@components/SizedBox";
import { Column, Row } from "@components/Flex";
// import { ReactComponent as ArrowDownIcon } from "@src/assets/icons/arrow.svg";
import SquareTokenIcon from "@components/SquareTokenIcon";
import { IToken } from "@src/constants";
import { useTheme } from "@emotion/react";
import Img from "@components/Img";

interface IProps extends HTMLAttributes<HTMLDivElement> {
  token?: IToken & { logo?: string };
  balance?: string;
  selectable?: boolean;
}

const Root = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  min-width: 200px;
`;

const TokenName = styled.div`
  font-weight: 500;
  font-size: 16px;
  line-height: 24px;
  color: ${({ theme }) => theme.colors.primary800};
`;

const Balance = styled.div`
  font-size: 14px;
  line-height: 20px;
  color: #8082c5;
`;

const TokenSelect: React.FC<IProps> = ({
  token,
  selectable,
  balance,
  ...rest
}) => {
  const theme = useTheme();
  return (
    <Root {...rest}>
      <Row alignItems="center">
        <SquareTokenIcon src={token?.logo} />
        <SizedBox width={8} />
        <Column justifyContent="center">
          <TokenName>{token?.symbol}</TokenName>
          <Balance>{token?.name}</Balance>
        </Column>
      </Row>
      {selectable && <Img src={theme.images.icons.arrowDown} alt="arrow" />}
    </Root>
  );
};
export default TokenSelect;
