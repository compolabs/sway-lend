import styled from "@emotion/styled";
import React, { HTMLAttributes } from "react";
import Text from "@components/Text";
import { LOGIN_TYPE } from "@stores/AccountStore";
import SizedBox from "@components/SizedBox";
import { useTheme } from "@emotion/react";
import Img from "@components/Img";

interface IProps extends HTMLAttributes<HTMLDivElement> {
  title: string;
  icon: string;
  type: LOGIN_TYPE;
}

const Root = styled.div<{ disable?: boolean }>`
  display: flex;
  flex-direction: row;
  width: 100%;
  justify-content: space-between;
  padding: 10px 0;
  border-bottom: 1px solid ${({ theme }) => theme.colors.divider};
  box-sizing: border-box;
  cursor: ${({ disable }) => (disable ? "not-allowed" : "pointer")};
`;
const Icon = styled.img`
  width: 24px;
  height: 24px;
  display: flex;
  flex-direction: column;
`;

const LoginType: React.FC<IProps> = ({ title, icon, type, ...rest }) => {
  const theme = useTheme();
  return (
    <Root {...rest} disable={rest.onClick == null}>
      <Icon src={icon} alt={type} />
      <SizedBox width={4} />
      <Text weight={700}>{title}</Text>
      <Img src={theme.images.icons.rightArrow} alt="rightArrow" />
    </Root>
  );
};
export default LoginType;
