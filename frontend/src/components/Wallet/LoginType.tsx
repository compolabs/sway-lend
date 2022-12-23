import styled from "@emotion/styled";
import React, { HTMLAttributes } from "react";
import Text from "@components/Text";
import { LOGIN_TYPE } from "@stores/AccountStore";

interface IProps extends HTMLAttributes<HTMLDivElement> {
  title: string;
  icon: string;
  type: LOGIN_TYPE;
}

const Root = styled.div<{ disable?: boolean }>`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  padding: 16px 24px;
  border: 1px solid ${({ theme }) => theme.colors.primary100};
  box-sizing: border-box;
  border-radius: 12px;
  margin: 4px 0;
  cursor: ${({ disable }) => (disable ? "not-allowed" : "pointer")};
`;
const Icon = styled.img`
  width: 24px;
  height: 24px;
  display: flex;
  flex-direction: column;
`;

const LoginType: React.FC<IProps> = ({ title, icon, type, ...rest }) => {
  return (
    <Root {...rest} disable={rest.onClick == null}>
      <Text size="medium" weight={500}>
        {title}
      </Text>
      <Icon src={icon} alt={type} />
    </Root>
  );
};
export default LoginType;
